use clap::Parser;
use git2::Repository;
use std::path::Path;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long)]
    json_file: String,
}

fn main() {
    let args = Cli::parse();
    println!("JSON file: {}", args.json_file);

    let repo = Repository::open(".").expect("Couldn't open the repository");
    let index = repo.index().expect("Couldn't get the index");

    // Iterate over the staged files
    for entry in index.iter() {
        let path = Path::new(std::str::from_utf8(&entry.path).unwrap());
        process_file(path);
    }
}

fn process_file(file: &Path) {
    // Read the file and do something with it
    println!("Processing file: {:?}", file);
}
