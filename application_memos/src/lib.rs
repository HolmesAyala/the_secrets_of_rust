use std::fmt::{Display, Formatter};
use std::io::{BufWriter};
use std::fs;
use std::fs::File;
use std::io::{BufReader};
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum MemoStatus {
    Pending,
    Done
}

impl Display for MemoStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MemoStatus::Pending => "Pending",
                MemoStatus::Done => "Done"
            }
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Memo {
    text: String,
    status: MemoStatus
}

impl Memo {
    pub fn create(text: String, status: MemoStatus) -> Self {
        Self { text, status }
    }

    pub fn create_pending(text: String) -> Self {
        Self { text, status: MemoStatus::Pending }
    }

    pub fn create_done(text: String) -> Self {
        Self { text, status: MemoStatus::Done }
    }

    pub fn mark_as_done(&mut self) {
        self.status = MemoStatus::Done;
    }

    fn is_pending(&self) -> bool {
        self.status == MemoStatus::Pending
    }
}

impl Display for Memo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] - {}", self.status, self.text)
    }
}

pub struct Memos {
    file_path: PathBuf,
}

impl Memos {
    pub fn create(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn open(&self) -> anyhow::Result<Vec<Memo>> {
        if !fs::exists(&self.file_path)? {
            return Ok(vec![]);
        }

        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        let items: Vec<Memo> = serde_json::from_reader(reader)?;

        Ok(items)
    }

    pub fn sync(&self, memos: &Vec<Memo>) -> anyhow::Result<()> {
        let file = File::create(&self.file_path)?;

        serde_json::to_writer(BufWriter::new(file), memos)?;

        Ok(())
    }

    pub fn add_pending(&self, text: String) -> anyhow::Result<()> {
        let mut memos = self.open()?;
        memos.push(Memo::create_pending(text));
        self.sync(&memos)?;
        Ok(())
    }

    pub fn add_done(&self, text: String) -> anyhow::Result<()> {
        let mut memos = self.open()?;
        memos.push(Memo::create_done(text));
        self.sync(&memos)?;
        Ok(())
    }

    pub fn mark_as_done(&self, match_text: String) -> anyhow::Result<Vec<Memo>> {
        let mut memos = self.open()?;
        let mut memos_as_done: Vec<Memo> = vec![];

        for memo in &mut memos {
            if memo.is_pending() && memo.text.to_lowercase().contains(&match_text.to_lowercase()) {
                memo.mark_as_done();

                memos_as_done.push(memo.clone())
            }
        }

        self.sync(&memos)?;

        Ok(memos_as_done)
    }

    pub fn purge_done(&self) -> anyhow::Result<()> {
        let mut memos = self.open()?;
        memos.retain(|memo| memo.is_pending());
        self.sync(&memos)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;
    use tempfile::tempdir;
    use crate::{Memo, Memos};

    #[test]
    fn then_it_should_read_memos_content() {
        let file_path = PathBuf::from_str("./tests/data/memos_mock.json")
            .expect("Could not find memos mock.json");
        let memos = Memos::create(file_path);

        let memos = memos.open()
            .expect("Failed to read mock file");
        
        let memos_expected = vec![Memo::create_pending("One record".to_string()), Memo::create_pending("Second record".to_string())];
        
        assert_eq!(memos, memos_expected, "The memos retrieved are not correct");
    }

    #[test]
    fn given_a_missing_file_then_it_should_return_empty() {
        let file_path = PathBuf::from_str("tests/unknown.json")
            .expect("Could not find memos mock.txt");
        let memos = Memos::create(file_path);

        let memos = memos.open()
            .expect("Failed to read mock file");

        assert!(memos.is_empty(), "There should be no memos retrieved");
    }

    #[test]
    fn then_it_should_sync_memos() {
        let temp_dir = tempdir()
            .expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("memos_mock.json");
        let memos = Memos::create(file_path);

        // First writing

        let mut memos_list = vec![
            Memo::create_pending("One record".to_string()),
            Memo::create_pending("Second record".to_string())
        ];

        memos.sync(&memos_list).expect("Failed to sync memos");

        let memos_saved = memos.open().expect("Failed to open memos saved");

        assert_eq!(memos_list, memos_saved, "The memos saved are not correct");

        // Second writing

        memos_list.push(Memo::create_pending("New memo".to_string()));

        memos.sync(&memos_list).expect("Failed to sync memos");

        let memos_saved = memos.open().expect("Failed to open memos saved");

        assert_eq!(memos_list, memos_saved, "The memos saved are not correct");

    }

    #[test]
    fn then_it_should_mark_as_done() {
        let temp_dir = tempdir()
            .expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("memos_mock.json");

        let memos = Memos::create(file_path.clone());

        memos.add_pending("First task".to_string()).expect("Failed to add pending memo");
        memos.add_pending("Second task".to_string()).expect("Failed to add pending memo");
        memos.add_pending("Sub SECOND sub task".to_string()).expect("Failed to add pending memo");

        let memos_as_done = memos.mark_as_done("second".to_string()).expect("Failed to mark as done");

        let memos_as_done_expected = vec![
            Memo::create_done("Second task".to_string()),
            Memo::create_done("Sub SECOND sub task".to_string())
        ];

        assert_eq!(memos_as_done, memos_as_done_expected, "The memos retrieved are not correct");

        let memos_saved = memos.open().expect("Failed to open memos saved");

        let memos_expected = vec![
            Memo::create_pending("First task".to_string()),
            Memo::create_done("Second task".to_string()),
            Memo::create_done("Sub SECOND sub task".to_string())
        ];

        assert_eq!(memos_saved, memos_expected, "The memos saved are not correct");
    }

    #[test]
    fn then_it_should_not_return_as_done_memos_already_as_done() {
        let temp_dir = tempdir()
            .expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("memos_mock.json");

        let memos = Memos::create(file_path.clone());

        memos.add_done("First task".to_string()).expect("Failed to add done memo");
        memos.add_done("Second task".to_string()).expect("Failed to add done memo");

        let memos_as_done = memos.mark_as_done("task".to_string()).expect("Failed to mark as done");

        let memos_as_done_expected = vec![];

        assert_eq!(memos_as_done, memos_as_done_expected, "The memos retrieved are not correct");
    }

    #[test]
    fn then_it_should_purge_done_memos() {
        let temp_dir = tempdir()
            .expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("memos_mock.json");

        let memos = Memos::create(file_path.clone());

        memos.add_done("First task".to_string()).expect("Failed to add done memo");
        memos.add_pending("Second task".to_string()).expect("Failed to add pending memo");
        memos.add_done("Third task".to_string()).expect("Failed to add done memo");

        memos.purge_done().expect("Failed to purge done memos");

        let memos_saved = memos.open().expect("Failed to open memos saved");

        let memos_expected = vec![Memo::create_pending("Second task".to_string())];

        assert_eq!(memos_saved, memos_expected, "The memos saved are not correct");
    }
}