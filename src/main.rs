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

    /// Include hidden files and directories
    #[arg(short = 'a', long = "all")]
    all: bool,

    /// Print file path banners before each file's contents
    #[arg(short = 'H', long = "headers")]
    headers: bool,
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
        let walker = WalkDir::new(path).into_iter();

        let filtered_walker = walker.filter_entry(|e| {
            args.all || !is_hidden(e)
        });

        for entry in filtered_walker {
            match entry {
                Ok(val) => {
                    if val.file_type().is_file() {
                        let file_path = val.path();
                        
                        if args.headers {
                            println!("==> {} <==", file_path.display());
                        }

                        match std::fs::read_to_string(file_path) {
                            Ok(contents) => print!("{}", contents),
                            Err(e) => eprintln!("Error reading file: {}", e),
                        }
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
