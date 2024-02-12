pub mod hdb;
pub mod hsb;
use std::{fs, path::{Path, PathBuf}};
use crate::scan::{Scan, ScanResult, Summary};

pub struct DB {
  pub hdb: Option<hdb::HDB>,
  pub hsb: Option<hsb::HSB>,
  pub summary: Summary,
}

impl DB {
  pub fn new() -> Self {
    DB {
      hdb: None,
      hsb: None,
      summary: Summary::new(),
    }
  }

  pub fn load(&mut self, db_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let paths = get_paths(db_dir)?;

    for path in paths {
      match get_db_type(&path) {
        Ok(DBType::HDB) => {
          self.hdb = Some(hdb::HDB::from(&path)?);
          self.summary.known += self.hdb.as_ref().unwrap().entries.len() as u64;
        }
        Ok(DBType::HSB) => {
          self.hsb = Some(hsb::HSB::from(&path)?);
          self.summary.known += self.hsb.as_ref().unwrap().entries.len() as u64;
        }
        _ => {}
      }
    }

    Ok(())
  }

  #[allow(dead_code)]
  pub fn scan(&mut self, target_path: &Path) {
    // target must be a file
    if target_path.is_dir() {
      return;
    }

    let mut res: ScanResult = ScanResult::Ok(target_path.to_path_buf());
    match &self.hdb {
      Some(hdb) => {
        res = hdb.scan(target_path, Some(&mut res));
      },
      None => {},
    };

    match &self.hsb {
      Some(hsb) => {
        res = hsb.scan(target_path, Some(&mut res));
      },
      None => {},
    };

    self.summary.data_scanned += fs::metadata(target_path).unwrap().len() as f64 / 1024.0;
    res.print();
    self.summary.update(&res);
  }

  #[allow(dead_code)]
  pub fn scan_dir(&mut self, target_dir: &Path, is_recursive: bool) {
    let paths = get_paths(target_dir).unwrap();
    for path in paths {
      if path.is_dir() {
        if is_recursive {
          self.scan_dir(&path, is_recursive);
        }
      } else {
        self.scan(&path);
      }
    }
    self.summary.scanned_dirs += 1;
  }
}

pub enum DBType {
  HDB,
  HSB,
}

fn get_paths(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
  let paths = fs::read_dir(dir)?
    .filter_map(Result::ok)
    .map(|e| e.path())
    .collect();

  Ok(paths)
}

fn get_db_type(path: &Path) -> Result<DBType, Box<dyn std::error::Error>> {
  let ext = path.extension().and_then(std::ffi::OsStr::to_str).unwrap_or("");
  match ext {
    hdb::EXT => Ok(DBType::HDB),
    hsb::EXT => Ok(DBType::HSB),
    _ => Err("invalid db type".into()),
  }
}

