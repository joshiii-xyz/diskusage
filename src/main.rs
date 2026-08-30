use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to scan
    #[arg(default_value = ".")]
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = PathBuf::from(&args.path);
    let entries = fs::read_dir(&path)?;

    let mut sizes: Vec<(String, u64)> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            let size = if meta.is_dir() {
                dir_size(&e.path()).ok()?
            } else {
                meta.len()
            };
            Some((e.file_name().to_string_lossy().into_owned(), size))
        })
        .collect();

    sizes.sort_by(|a, b| b.1.cmp(&a.1));

    for (name, size) in sizes {
        println!("{:>10}  {}", format_size(size), name);
    }
    Ok(())
}

fn dir_size(path: &std::path::Path) -> Result<u64> {
    let mut total = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        total += if meta.is_dir() {
            dir_size(&entry.path())?
        } else {
            meta.len()
        };
    }
    Ok(total)
}

fn format_size(size: u64) -> String {
    if size >= 1_073_741_824 {
        format!("{:.1}G", size as f64 / 1_073_741_824.0)
    } else if size >= 1_048_576 {
        format!("{:.1}M", size as f64 / 1_048_576.0)
    } else if size >= 1024 {
        format!("{:.1}K", size as f64 / 1024.0)
    } else {
        format!("{}B", size)
    }
}
