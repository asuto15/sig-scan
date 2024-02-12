use std::path::{Path, PathBuf};
use chrono::{Local, DateTime, Duration};

pub trait Scan {
  fn scan(&self, target_path: &Path, result: Option<&mut ScanResult>) -> ScanResult;
}

#[derive(Debug)]
pub enum ScanResult {
  Ok(PathBuf),
  Empty(PathBuf),
  Invalid {
    path: PathBuf,
    name: Vec<String>,
  },
  Error {
    path: PathBuf,
    desc: String,
  },
}

impl ScanResult {
  fn to_string(&self) -> String {
    match self {
      ScanResult::Ok(path) => format!("{}: OK", path.display()),
      ScanResult::Empty(path) => format!("{}: Empty file", path.display()),
      ScanResult::Invalid { path, name } => format!("{}: Invalid: {} FOUND({}/2)", path.display(), name.join(", "), name.len()),
      ScanResult::Error { path, desc } => format!("{}: Error: {}", path.display(), desc),
    }
  }

  pub fn print(&self) {
    println!("{}", self.to_string());
  }
}

pub struct Summary {
  pub known: u64,
  pub version: String,
  pub scanned_dirs: u64,
  pub scanned_files: u64,
  pub infected: u64,
  pub data_scanned: f64,
  pub data_read: f64,
  pub time: Duration,
  pub start_date: DateTime<Local>,
  pub end_date: DateTime<Local>,
}

impl Summary {
  pub fn new() -> Self {
    Summary {
      known: 0,
      version: env!("CARGO_PKG_VERSION").to_string(),
      scanned_dirs: 0,
      scanned_files: 0,
      infected: 0,
      data_scanned: 0.0,
      data_read: 0.0,
      time: Duration::hours(0),
      start_date: Local::now(),
      end_date: Local::now(),
    }
  }

  pub fn end(&mut self) {
    self.end_date = Local::now();
    self.time = self.end_date - self.start_date;
  }

  pub fn print(&self) {
    println!("----------- SCAN SUMMARY -----------");
    println!("Known viruses: {}", self.known);
    println!("Application Version: {}", self.version);
    println!("Scanned directories: {}", self.scanned_dirs);
    println!("Scanned files: {}", self.scanned_files);
    println!("Infected files: {}", self.infected);
    // println!("Data scanned: {} MB", self.data_scanned);
    // println!("Data read: {} MB", self.data_read);
    println!("Time: {}", self.display_time());
    println!("Start date: {:?}", self.start_date);
    println!("End date: {:?}", self.end_date);
  }

  pub fn display_time(&self) -> String {
    let seconds = self.time.num_seconds();
    let minutes = seconds / 60;
    if minutes == 0 {
      return format!("{} seconds", seconds)
    }
    let hours = minutes / 60;
    if hours == 0 {
      return format!("{} sec ({} m {} s)", seconds, minutes, seconds % 60)
    }
    let days = hours / 24;
    if days == 0 {
      return format!("{} sec ({} h {} m {} s)", seconds, hours, minutes % 60, seconds % 60)
    }
    format!("{} sec ({} d {} h {} m {} s)", seconds, days, hours % 24, minutes % 60, seconds % 60)
  }

  pub fn update(&mut self, res: &ScanResult) {
    match res {
      ScanResult::Ok(_) => {
        self.scanned_files += 1;
      },
      ScanResult::Empty(_) => {
        self.scanned_files += 1;
      },
      ScanResult::Invalid { path:_, name:_ } => {
        self.scanned_files += 1;
        self.infected += 1;
      },
      ScanResult::Error { path:_, desc:_ } => {
      },
    }
  }
}