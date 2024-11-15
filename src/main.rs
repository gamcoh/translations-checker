use anyhow::{anyhow, Context, Result};
use clap::Arg;
use clap::Command;
use fancy_regex::Regex;
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::{collections::HashSet, fs, path::Path};

static COMPILED_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?<![a-zA-Z_])t\(\s*(['"])(.*?)\1\s*\)"#).unwrap());
static COMPILED_HOOK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(useTranslation|getTranslations)\(("|')(.+?)\2\)"#).unwrap());

fn main() -> Result<()> {
    let matches = Command::new("translation_checker")
        .version(env!("CARGO_PKG_VERSION"))
        .about("This hook checks for missing translations in TypeScript and JavaScript files.")
        .author("Gamliel Cohen @gamcoh")
        .arg(
            Arg::new("files")
                .help("path(s) to files to check")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("json-file")
                .help("The JSON file to use")
                .long("json-file")
                .required(true),
        )
        .get_matches();

    let mut translations = HashSet::new();
    if let Some(files) = matches.get_many::<String>("files") {
        for file in files {
            translations.extend(process_file(file)?);
        }
    }

    if translations.is_empty() {
        return Ok(());
    }

    if let Some(translation_file) = matches.get_one::<String>("json-file") {
        let count = parse_and_add_translations(&translations, translation_file)?;

        if count > 0 {
            eprintln!(
                "Found {} missing translations. TODOs added to the JSON file.",
                count
            );
            std::process::exit(1);
        }
    }

    Ok(())
}

fn parse_and_add_translations(translations: &HashSet<String>, json_file: &str) -> Result<u16> {
    let json_content = fs::read_to_string(json_file)
        .with_context(|| format!("Couldn't read the JSON file {}", json_file))?;
    let mut json: IndexMap<String, Value> =
        serde_json::from_str(&json_content).with_context(|| "Couldn't parse the JSON content")?;

    let mut missing_translations_count = 0;
    for translation in translations {
        if !json.contains_key(translation) {
            missing_translations_count += 1;
            println!("Adding translation: {}", translation);
            json.insert(translation.to_string(), Value::String(String::from("TODO")));
        }
    }

    if missing_translations_count == 0 {
        return Ok(0);
    }

    let json_content =
        serde_json::to_string_pretty(&json).context("Couldn't serialize the JSON")?;
    fs::write(json_file, json_content)
        .with_context(|| format!("Couldn't write the JSON file {}", json_file))?;

    Ok(missing_translations_count)
}

fn process_file(file: &str) -> Result<Vec<String>> {
    let path = Path::new(file);
    let metadata =
        fs::metadata(path).with_context(|| format!("Failed to get metadata for {}", file))?;
    if metadata.is_dir() {
        let entries =
            fs::read_dir(path).with_context(|| format!("Couldn't read directory {}", file))?;
        let mut res = Vec::new();
        for entry in entries {
            let entry = entry.with_context(|| "Couldn't get directory entry")?;
            let path_str = entry
                .path()
                .to_str()
                .ok_or_else(|| anyhow!("Invalid path"))?
                .to_string();
            res.extend(process_file(&path_str)?);
        }
        Ok(res)
    } else {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ["jsx", "tsx", "js", "ts"].contains(&ext) {
            extract_translations_from_file(&path)
        } else {
            Ok(vec![])
        }
    }
}

fn extract_translations_from_file(file: &Path) -> Result<Vec<String>> {
    let content =
        fs::read_to_string(file).with_context(|| format!("Couldn't read file {:?}", file))?;

    let translations_hooks = COMPILED_HOOK_RE
        .captures_iter(&content)
        .filter_map(|cap| {
            cap.ok().and_then(|c| {
                c.get(3).map(|m| {
                    let mut r = m.as_str().to_string();
                    r.push_str(".");
                    r
                })
            })
        })
        .collect::<Vec<String>>();

    if translations_hooks.len() > 1 {
        return Err(anyhow!(
            "Found more than one translation hook in file {:?}",
            file
        ));
    }

    let translations = COMPILED_RE
        .captures_iter(&content)
        .filter_map(|cap| {
            cap.ok().and_then(|c| {
                c.get(2).map(|m| {
                    let mut r = m.as_str().to_string();

                    if translations_hooks.is_empty() {
                        return r;
                    }

                    r.insert_str(0, translations_hooks[0].as_str());
                    r
                })
            })
        })
        .collect();
    Ok(translations)
}
