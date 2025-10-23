use clap::Parser;
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;
use yansi::Paint;

#[derive(Parser)]
#[command(name = "diffdirs")]
#[command(about = "Compare two directories and show missing paths")]
struct Args {
    /// First directory to compare
    dir_a: String,
    /// Second directory to compare
    dir_b: String,
}

fn collect_paths(dir: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let mut paths = HashSet::new();
    let root = Path::new(dir);

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let relative = path.strip_prefix(root)?;
            paths.insert(relative.to_string_lossy().to_string());
        }
    }

    Ok(paths)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let paths_a = collect_paths(&args.dir_a)?;
    let paths_b = collect_paths(&args.dir_b)?;

    // Paths only in dirA
    for path in paths_a.difference(&paths_b) {
        println!("{}", Paint::red(format!("Only in {}: {}", args.dir_a, path)));
    }

    // Paths only in dirB
    for path in paths_b.difference(&paths_a) {
        println!("{}", Paint::red(format!("Only in {}: {}", args.dir_b, path)));
    }

     // Add this summary after the loops
    println!(
        "\nSummary: {} unique in {}, {} unique in {}",
        paths_a.difference(&paths_b).count(),
        args.dir_a,
        paths_b.difference(&paths_a).count(),
        args.dir_b
    );

    Ok(())
}
