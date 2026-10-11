use clap::Parser;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(
    name = "rcat",
    author,
    version,
    about = "Recursively traverses directories and concatenates file contents",
    long_about = None
)]
struct Args {
    /// Target paths (files or directories)
    #[arg(required = true)]
    paths: Vec<String>,

    #[arg(short = 'a', long = "all")]
    all: bool,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "." && s != "..")
        .unwrap_or(false)
}

fn main() {
    let args = Args::parse();
    
    for path in &args.paths {
        println!("Traversing: {}", path);

        let walker = WalkDir::new(path).into_iter();

        let filtered_walker = walker.filter_entry(|e| {
            args.all || !is_hidden(e)
        });

        for entry in filtered_walker {
            match entry {
                Ok(val) => println!("  -> {}", val.path().display()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
