use count::count_file_lines;
use clap::{Parser};

#[derive(Parser)]
/// Count lines or words in the specified files
struct Args {
    /// Count words instead of lines
    #[arg(short, long)]
    words: bool,
    /// Files to be counted
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    for file_path in args.files {
        let output = count_file_lines(&file_path)?;

        if args.words {
            println!("{file_path}: {} words", output.words);
        } else {
            println!("{file_path}: {} lines", output.lines);
        }
    }

    Ok(())
}
