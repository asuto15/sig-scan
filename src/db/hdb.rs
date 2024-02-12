use std::{fs, path::Path};
use md5;
use crate::scan::{Scan, ScanResult};

pub const EXT: &str = "hdb";

#[derive(Debug)]
pub struct HDB {
  pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
  pub md5: String,
  pub size: u64,
  pub name: String,
}

impl HDB {
  fn new() -> Result<Self, Box<dyn std::error::Error>> {
    let entries = vec![];
    Ok(HDB { entries })
  }

  fn load(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let mut entries: Vec<Entry> = vec![];
    for line in contents.lines() {
      let entry = Entry::from_str(line)?;
      entries.push(entry);
    }
    self.entries = entries;
    Ok(())
  }

  pub fn from(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let mut hdb = HDB::new()?;
    hdb.load(path)?;
    Ok(hdb)
  }
}

impl Scan for HDB {
  fn scan(&self, target_path: &Path, res: Option<&mut ScanResult>) -> ScanResult {
    let contents = match fs::read(target_path) {
      Ok(contents) => contents,
      Err(_) => return ScanResult::Error{
        path: target_path.to_path_buf(),
        desc: "unable to read file".to_string(),
      }
    };

    if contents.is_empty() {
      return ScanResult::Empty(target_path.to_path_buf());
    }

    let md5 = md5::compute(&contents);
    let md5 = format!("{:x}", md5);
    let size = contents.len() as u64;

    self.entries.iter().find(|entry| entry.md5 == md5 && entry.size == size).map_or(
      ScanResult::Ok(target_path.to_path_buf()),
      |entry| ScanResult::Invalid {
        path: target_path.to_path_buf(),
        name: match res {
          Some(ScanResult::Invalid { path:_, name }) => {
            let mut name = name.clone();
            name.push(entry.name.clone());
            name
          },
          _ => vec![entry.name.clone()],
        },
      },
    )
  }
}

impl Entry {
  pub fn new(md5: &str, size: u64, name: &str) -> Self {
    Entry {
      md5: md5.to_string(),
      size,
      name: name.to_string(),
    }
  }

  pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
      return Err("invalid entry".into());
    }
    let md5 = parts[0];
    let size = parts[1].parse()?;
    let name = parts[2];
    Ok(Entry::new(md5, size, name))
  }
}