use std::fs;
use std::path::PathBuf;
use std::io;
use clap::Parser;

/// PROGRAM (Rust compiled .rs) FOR GRIDSEARCHING PDF FILES AND MOVING THEM TO DESTINATION DIRECTORY
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder to search (ex: C:\Users\user\Documents\...)
    #[arg(short, long)]
    folder: PathBuf,

    /// Search patterns (space-separated)
    #[arg(short, long)] 
    patterns: Vec<String>,

    /// Destination folder (ex: C:\Users\user\Documents\destination\)
    #[arg(short, long)] 
    destination: PathBuf,
}

fn main() {
    let args: Args = Args::parse();

    let search_patterns: Vec<&str> = args.patterns.iter().map(|s| s.as_str()).collect();

    let entries = fs::read_dir(args.folder).expect("Failed to read directory");
    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "pdf" {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if search_patterns.iter().any(|pattern| file_name.contains(pattern)) {
                    let dest_path = args.destination.join(path.file_name().unwrap());
                    fs::rename(path, dest_path).expect("Failed to move file");
                }
            }
        }
    }

    println!("Press enter to exit");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
}