use anyhow::Context;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct CountOutput {
    pub lines: usize,
    pub words: usize,
}

pub fn count_file_lines(path: &str) -> anyhow::Result<CountOutput> {
    let file = File::open(path).context(format!("failed to open file. path: {}", path))?;

    let file_reader = BufReader::new(file);

    let output = count(file_reader).context("Failed counting file lines")?;

    Ok(output)
}

pub fn count(mut reader: impl BufRead) -> anyhow::Result<CountOutput> {
    let mut count = CountOutput::default();
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        count.lines += 1;
        count.words += line.split_whitespace().count();

        line.clear();
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use crate::count;
    use std::io::{BufReader, Cursor, Error, ErrorKind, Read};

    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Err(Error::new(ErrorKind::Other, "Some unknown error"))
        }
    }

    #[test]
    fn given_an_error_from_reader_then_it_should_returns_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count(reader);

        assert!(result.is_err(), "no error returned");
    }

    #[test]
    fn given_reader_with_5_words_and_3_lines_then_it_should_return_5_words_and_3_lines() {
        let reader = Cursor::new("   some  \n\t  mock \ntext here . \n");
        let output = count(reader).expect("Counting lines and words failed");

        assert_eq!(output.lines, 3);
        assert_eq!(output.words, 5);
    }
}
