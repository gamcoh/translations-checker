use clap::Parser;
use git2::Repository;
use std::path::Path;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long)]
    json_file: String,
}

fn main() {
    // Use `clap` to handle arguments if needed
    let args = Cli::parse();
    println!("{:?}", args);

    let repo = Repository::open(".").expect("Couldn't open the repository");
    let index = repo.index().expect("Unable to get index");

    println!("{:?}", index.path());

    // Iterate over the staged files
    for entry in index.iter() {
        let path = Path::new(std::str::from_utf8(&entry.path).unwrap());
        println!("Staged file: {:?}", path);
    }
}
