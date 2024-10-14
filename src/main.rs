use anyhow::{anyhow, Context, Result};
use clap::Parser;
use fancy_regex::Regex;
use git2::{Repository, StatusOptions};
use once_cell::sync::Lazy;
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long)]
    json_file: String,
}

static COMPILED_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?<![a-zA-Z_])t\(\s*(['"])(.*?)\1\s*\)"#).unwrap());

fn main() -> Result<()> {
    let args = Cli::parse();
    let repo = Repository::open(".").context("Couldn't open the repository")?;

    let mut status_opts = StatusOptions::new();
    status_opts
        .include_untracked(true)
        .recurse_ignored_dirs(false);
    let statuses = repo
        .statuses(Some(&mut status_opts))
        .context("Couldn't get the status of the repository")?;

    let translations = statuses
        .iter()
        .map(|file| process_file(file.path().unwrap()))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<HashSet<_>>();

    parse_and_add_translations(&translations, &args.json_file)?;
    println!("Done adding translations to {}, you should review the newly added translations entries and add the correct values.", args.json_file);

    Ok(())
}

fn parse_and_add_translations(translations: &HashSet<String>, json_file: &str) -> Result<()> {
    let json_content = fs::read_to_string(json_file)
        .with_context(|| format!("Couldn't read the JSON file {}", json_file))?;
    let mut json: HashMap<String, Value> =
        serde_json::from_str(&json_content).with_context(|| "Couldn't parse the JSON content")?;

    for translation in translations {
        json.entry(translation.clone()).or_insert_with(|| {
            println!("Adding translation: {}", translation);
            Value::String(String::from("TODO"))
        });
    }

    let json_content =
        serde_json::to_string_pretty(&json).context("Couldn't serialize the JSON")?;
    fs::write(json_file, json_content)
        .with_context(|| format!("Couldn't write the JSON file {}", json_file))?;

    Ok(())
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
    let translations = COMPILED_RE
        .captures_iter(&content)
        .filter_map(|cap| {
            cap.ok()
                .and_then(|c| c.get(2).map(|m| m.as_str().to_string()))
        })
        .collect();
    Ok(translations)
}
