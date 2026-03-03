use std::{env};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let log_file_path = "logbook.txt";


    if args.is_empty() {
        if let Some(content) = logbook::read(log_file_path)? {
            print!("{}", content)
        } else {
            println!("Logbook is empty")
        }
    } else {
        logbook::append(log_file_path, args.join(" ").as_str())?;
    }

    Ok(())
}
