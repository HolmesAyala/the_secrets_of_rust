use std::path::PathBuf;
use std::str::FromStr;
use clap::Parser;
use memos::{Memos};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    done: bool,
    #[arg(short, long)]
    purge: bool,
    text: Vec<String>
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file_path = PathBuf::from_str("./memos.json")
        .expect("Could not find memos mock.txt");
    let memos = Memos::create(file_path);

    if args.done {
        let match_text = args.text.join(" ");
        let memos_as_done = memos.mark_as_done(match_text)?;

        println!("Memos marked as done:");

        for memo in &memos_as_done {
            println!("{}", memo);
        }
    } else if args.purge {
        memos.purge_done()?;

        let memos_list = memos.open()?;

        println!("Memos pending:");

        for memo in &memos_list {
            println!("{}", memo);
        }
    } else if args.text.is_empty() {
        let memos_list = memos.open()?;

        println!("Memos:");

        for memo in &memos_list {
            println!("{}", memo);
        }
    } else {
        let memo_text = args.text.join(" ");
        memos.add_pending(memo_text.to_string())?;
        println!("Memo added: {}", memo_text);
    }
    
    Ok(())
}
