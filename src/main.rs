use clap::Parser;
use std::io::{self, BufReader, Write};
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

    /// Ignore hidden files and directories
    #[arg(short = 'i', long = "ignore")]
    ignore: bool,

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

    let normalize = |e: &str| e.trim().trim_start_matches('.').to_lowercase();

    if let Some(ref ext) = file_ext {
        if excluded_exts.iter().any(|e| normalize(e) == *ext) {
            return false;
        }
    } else if excluded_exts.iter().any(|e| normalize(e).is_empty()) {
        return false;
    }

    if !allowed_exts.is_empty() {
        match file_ext {
            Some(ref ext) => allowed_exts.iter().any(|e| normalize(e) == *ext),
            None => false,
        }
    } else {
        true
    }
}

fn process_file(
    file_path: &std::path::Path,
    headers: bool,
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    if headers {
        writeln!(stdout, "===> {} <===", file_path.display())?;
    }

    let file = std::fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);
    io::copy(&mut reader, stdout)?;

    Ok(())
}

fn main() {
    let args = Args::parse();
    let mut has_error = false;
    let mut stdout = io::stdout();

    for path in &args.paths {
        let walker = WalkDir::new(path).into_iter();
        
        let filtered_walker = walker.filter_entry(|e| !args.ignore || !is_hidden(e));

        for entry in filtered_walker {
            match entry {
                Ok(val) => {
                    if val.file_type().is_file() {
                        let file_path = val.path();

                        if !matches_extension(file_path, &args.ext, &args.exclude) {
                            continue;
                        }

                        if let Err(e) = process_file(file_path, args.headers, &mut stdout) {
                            if e.kind() == io::ErrorKind::BrokenPipe {
                                std::process::exit(0);
                            }
                            eprintln!("rcat: {}: {}", file_path.display(), e);
                            has_error = true;
                        }
                    }
                }
                Err(e) => {
                    if let Some(path) = e.path() {
                        eprintln!("rcat: {}: {}", path.display(), e);
                    } else {
                        eprintln!("rcat: {}", e);
                    }
                    has_error = true;
                }
            }
        }
    }

    if let Err(e) = stdout.flush() {
        if e.kind() == io::ErrorKind::BrokenPipe {
            std::process::exit(0);
        }
    }

    if has_error {
        std::process::exit(1);
    }
}
