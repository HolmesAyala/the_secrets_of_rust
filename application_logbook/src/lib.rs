//! Records text in a logbook file, also allows
//! the retrieval of the logbook content
use std::io::Write;
use std::fs::{exists, read_to_string, File};
use std::path::Path;

/// Reads the contents of the logbook file at `path`.
///
/// Returns [`None`] if the file does not exist or is empty.
///
/// # Errors
///
/// Returns any error from [`std::fs::exists`] or [`std::fs::read_to_string`].
pub fn read(path: impl AsRef<Path>) -> anyhow::Result<Option<String>> {
    if !exists(&path)? {
        return Ok(None);
    }

    let file_content = read_to_string(path)?;

    if file_content.is_empty() {
        return Ok(None);
    }

    Ok(Some(file_content))
}

/// Appends `msg` to the logbook file at `path`.
/// It creates the file if it is not created.
///
/// # Errors
///
/// Returns any error from [`open`](std::fs::OpenOptions::open)
/// or [`writeln!`].
pub fn append(path: impl AsRef<Path>, msg: &str) -> anyhow::Result<()> {
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{}", msg)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use crate::{append, read};

    #[test]
    fn given_a_missing_file_then_it_should_return_none() {
        let content_optional = read("unknown_file.txt")
            .expect("Something went wrong while reading the file");

        assert_eq!(content_optional, None, "expected None");
    }

    #[test]
    fn given_a_file_with_empty_content_then_it_should_return_none() {
        let content_optional = read("tests/data/empty_logbook_file.txt")
            .expect("Something went wrong while reading the file");

        assert_eq!(content_optional, None, "expected None");
    }

    #[test]
    fn given_a_file_with_content_then_it_should_return_some() {
        let content = read("tests/data/logbook_file.txt")
            .expect("Something went wrong while reading the file")
            .expect("The file content must be present");

        assert_eq!(content, "Day 1: My mock text", "expected Day 1: My mock text");
    }

    #[test]
    fn given_file_not_created_then_it_should_create_and_include_the_content() {
        let temp_dir = tempdir()
            .expect("Something went wrong while creating a temporary directory");
        let file_path = temp_dir.path().join("new_logbook.txt");

        append(&file_path, "Day 25: My new mock text")
            .expect("The append must work");

        let content = read(file_path)
            .expect("The logbook content reading must work")
            .expect("The file content must be present");

        assert_eq!(content, "Day 25: My new mock text\n", "expected Day 25: My new mock text\n")
    }

    #[test]
    fn given_file_created_then_it_should_include_the_new_content() {
        let temp_dir = tempdir()
            .expect("Something went wrong while creating a temporary directory");
        let file_path = temp_dir.path().join("new_logbook.txt");

        append(&file_path, "Day 25: My new mock text")
            .expect("The append must work");

        let content = read(&file_path)
            .expect("The logbook content reading must work")
            .expect("The file content must be present");

        assert_eq!(content, "Day 25: My new mock text\n", "expected Day 25: My new mock text\n");

        append(&file_path, "Day 26: My other new mock text")
            .expect("The append must work");

        let content = read(&file_path)
            .expect("The logbook content reading must work")
            .expect("The file content must be present");

        assert_eq!(content, "Day 25: My new mock text\nDay 26: My other new mock text\n", "expected Day 25: My new mock text\nDay 26: My other new mock text\n");
    }
}
