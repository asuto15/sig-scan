use std::{fs, path::Path};
use sha::{sha1::Sha1, sha256::Sha256, utils::{Digest, DigestExt}};
use crate::scan::{Scan, ScanResult};

pub const EXT: &str = "hsb";

#[derive(Debug)]
pub struct HSB {
  pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
  pub sha: String,
  pub size: Option<u64>,
  pub name: String,
}

impl HSB {
  fn new() -> Result<Self, Box<dyn std::error::Error>> {
    let entries = vec![];
    Ok(HSB { entries })
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
    let mut hsb = HSB::new()?;
    hsb.load(path)?;
    Ok(hsb)
  }
}

impl Scan for HSB {
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

    let sha1 = Sha1::default().digest(&contents.as_slice()).to_hex();
    let sha256 = Sha256::default().digest(&contents.as_slice()).to_hex();
    let size = contents.len() as u64;

    self.entries.iter().find(|entry| (entry.sha == sha1 || entry.sha == sha256) &&
      entry.size.map_or(
        true,
        |s| s == size
      )
    )
      .map_or(
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
  pub fn new(sha: &str, size: Option<u64>, name: &str) -> Self {
    Entry {
      sha: sha.to_string(),
      size,
      name: name.to_string(),
    }
  }

  pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = s.split(':').collect();

    match parts.len() {
      3 => return Ok(Entry::new(parts[0], Some(parts[1].parse()?), parts[2])),
      4 => {
        match parts[3] {
          "73" => return Ok(Entry::new(parts[0], None, parts[2])),
          _ => return Err("invalid entry with 4 element".into()),
        }
      },
      _ => return Err("invalid entry".into()),
    }
  }
}