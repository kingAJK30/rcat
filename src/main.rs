use clap::Parser;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(
    name = "rcat",
    author = "KingAJK30",
    version,
    about = "Recursively traverses directories and concatenates file contents",
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

    /// Only include files with these extensions
    #[arg(short = 'e', long = "ext", value_delimiter = ',')]
    ext: Vec<String>,

    /// Exclude files with these extensions
    #[arg(short = 'E', long = "exclude", value_delimiter = ',')]
    exclude: Vec<String>,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "." && s != "..")
        .unwrap_or(false)
}

fn matches_extension(
    file_path: &std::path::Path,
    allowed_exts: &[String],
    excluded_exts: &[String],
) -> bool {
    let file_ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase());

    if let Some(ref ext) = file_ext {
        if excluded_exts
            .iter()
            .any(|e| e.trim_start_matches('.').to_lowercase() == *ext)
        {
            return false;
        }
    } else if excluded_exts.contains(&"".to_string()) {
        return false;
    }

    if !allowed_exts.is_empty() {
        match file_ext {
            Some(ref ext) => allowed_exts
                .iter()
                .any(|e| e.trim_start_matches('.').to_lowercase() == *ext),
            None => false,
        }
    } else {
        true
    }
}

fn main() {
    let args = Args::parse();

    for path in &args.paths {
        let walker = WalkDir::new(path).into_iter();

        let filtered_walker = walker.filter_entry(|e| args.all || !is_hidden(e));

        for entry in filtered_walker {
            match entry {
                Ok(val) => {
                    if val.file_type().is_file() {
                        let file_path = val.path();

                        if !matches_extension(file_path, &args.ext, &args.exclude) {
                            continue;
                        }

                        if args.headers {
                            println!("===> {} <===", file_path.display());
                        }

                        match std::fs::read(file_path) {
                            Ok(bytes) => {
                                let contents = String::from_utf8_lossy(&bytes);
                                print!("{}", contents);
                            }
                            Err(e) => eprintln!("Error reading file: {}", e),
                        }
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
