use clap::Parser;

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
}

fn main() {
    let args = Args::parse();
    
    // Placeholder for your core logic
    println!("Running rcat with args: {:#?}", args);
}
