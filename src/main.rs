mod config;
mod db;
mod scan;
use clap::{Parser, ArgAction};
use std::{path::{Path, PathBuf}, process::exit};
use config::Config;
use db::DB;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
  #[clap(short, long, action = ArgAction::SetTrue)]
  pub recursive: bool,
  #[clap(required = true, num_args = 1..)]
  pub target: Vec<PathBuf>,
}

fn main() {
  let args = Args::parse();

  if args.target.is_empty() {
    eprintln!("Error: no target specified");
    exit(1);
  }

  let config = match Config::new() {
    Ok(config) => config,
    Err(e) => {
      eprintln!("Error: {}", e);
      exit(1);
    }
  };

  let mut db = DB::new();
  for db_dir in config.database_dir.iter() {
    db.load(&PathBuf::from(db_dir)).unwrap();
  }

  for arg in args.target.iter() {
    let target_path = Path::new(arg);
    if target_path.is_dir() {
      db.scan_dir(target_path, args.recursive);
    } else {
      db.scan(target_path);
    }
  }

  db.summary.end();
  db.summary.print();
}
