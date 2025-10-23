use clap::Parser;
use serde::Serialize;
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;
use yansi::Paint;

#[derive(Serialize)]
struct DiffResult {
    only_in_a: Vec<String>,
    only_in_b: Vec<String>,
    summary: Summary,
}

#[derive(Serialize)]
struct Summary {
    unique_in_a: usize,
    unique_in_b: usize,
}

#[derive(Parser)]
#[command(name = "diffdirs")]
#[command(about = "Compare two directories and show missing paths")]
struct Args {
    /// First directory to compare
    dir_a: String,
    /// Second directory to compare
    dir_b: String,
    /// Compare only directories (not files)
    #[arg(long)]
    dirs: bool,
    /// Output results in JSON format
    #[arg(long)]
    json: bool,
    /// Maximum depth to traverse (0 = only root)
    #[arg(long)]
    depth: Option<usize>,
}

fn collect_paths(dir: &str, dirs_only: bool, depth: Option<usize>) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let mut paths = HashSet::new();
    let root = Path::new(dir);

    let mut walker = WalkDir::new(dir);
    if let Some(d) = depth {
        walker = walker.max_depth(d);
    }

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if dirs_only && entry.file_type().is_dir() {
            let relative = entry.path().strip_prefix(root)?;
            if !relative.as_os_str().is_empty() {
                paths.insert(relative.to_string_lossy().to_string());
            }
        } else if !dirs_only && entry.file_type().is_file() {
            let relative = entry.path().strip_prefix(root)?;
            paths.insert(relative.to_string_lossy().to_string());
        }
    }

    Ok(paths)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let paths_a = collect_paths(&args.dir_a, args.dirs, args.depth)?;
    let paths_b = collect_paths(&args.dir_b, args.dirs, args.depth)?;

    let only_in_a: Vec<String> = paths_a.difference(&paths_b).cloned().collect();
    let only_in_b: Vec<String> = paths_b.difference(&paths_a).cloned().collect();

    if args.json {
        let result = DiffResult {
            only_in_a,
            only_in_b,
            summary: Summary {
                unique_in_a: paths_a.difference(&paths_b).count(),
                unique_in_b: paths_b.difference(&paths_a).count(),
            },
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        // Paths only in dirA
        for path in &only_in_a {
            println!(
                "{}",
                Paint::blue(format!("Only in {}: {}", args.dir_a, path))
            );
        }

        // Paths only in dirB
        for path in &only_in_b {
            println!(
                "{}",
                Paint::red(format!("Only in {}: {}", args.dir_b, path))
            );
        }

        println!(
            "\nSummary: {} unique in {}, {} unique in {}",
            Paint::blue(paths_a.difference(&paths_b).count()),
            args.dir_a,
            Paint::red(paths_b.difference(&paths_a).count()),
            args.dir_b
        );
    }

    Ok(())
}
