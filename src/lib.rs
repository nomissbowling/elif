#![doc(html_root_url = "https://docs.rs/elif/0.0.1")]
//! file and directory walker for Rust
//!

use std::error::Error;
use std::fs;
// use fs_extra;
// use std::path;

/*
pub fn read_dir_entries<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
  let mut entries = fs::read_dir(path)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;
  entries.sort();
  Ok(entries)
}
*/

/// read dir entries
pub fn read_dir_entries(bpath: &str) ->
  Result<Vec<fs::DirEntry>, Box<dyn Error>> {
  let mut entries: Vec<fs::DirEntry> = Vec::new();
  // let rdir = path::Path::new(bpath).read_dir()?; // ignore Result check
  // let rdir = fs::read_dir(bpath)?; // ignore Result check
  // if let Ok(rdir) = fs::read_dir(bpath) {
  match fs::read_dir(bpath) {
  Err(e) => eprintln!("err: {}", e),
  Ok(rdir) => for item in rdir.into_iter() { // for item in rdir {
    // entries.push(item?); // fs::DirEntry // ignore Result check
    // if let Ok(entry) = item { entries.push(entry); } // fs::DirEntry
    match item {
    Err(e) => eprintln!("err: {}", e),
    Ok(entry) => entries.push(entry) // fs::DirEntry
    }
  }
  }
  // entries.sort(); // entries must be Vec<path::PathBuf>
  // entries.sort_by(|a, b| a.file_name().partial_cmp(&b.file_name()).unwrap());
  // entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
  entries.sort_by(|a, b| a.path().cmp(&b.path()));
  entries.sort_by(|a, b| a.path().is_file().cmp(&b.path().is_file()));
  Ok(entries)
}

/// walk dir entries
pub fn walk_dir_entries(ignores: &Vec<String>, bpath: &str, dep: u64) ->
  Result<u64, Box<dyn Error>> {
  let mut total: u64 = 0;
  let depth = String::from_utf8((0..dep).into_iter().map(|_| 0x20).collect())?;
//  println!("{}+{}", depth, bpath);
  let rde = read_dir_entries(bpath);
  // let lde = rde.unwrap(); // unwrap Ok([...]) ignore Result check
  // if let Ok(lde) = rde {
  match rde {
  Err(e) => eprintln!("err: {}", e),
  Ok(lde) => for de in lde {
    let p = &de.path();
    print!("{}{}", depth, if p.is_dir() {"+"}else{"-"});
    // println!("{:?}", de); // fs::DirEntry
    // println!("{:?}", de.file_name()); // OsString
    println!("{}", p.display()); // path::PathBuf
    if p.is_file() {
      let rmf = fs::metadata(p);
      match rmf {
      Err(e) => eprintln!("err: {}", e),
      Ok(mf) => {
        println!(" {}", mf.len());
        total += mf.len();
      }
      }
    }
    if p.is_dir() {
      let e = de.file_name().to_str().expect("invalid_name").to_string();
      if !ignores.contains(&e) {
        let s = p.to_str().expect("invalid_path");
        total += walk_dir_entries(ignores, s, dep + 1)?;
      }
    }
  }
  }
  Ok(total)
}

/// macro walk dir entries
#[macro_export]
macro_rules! walk_dir_entries {
  ($ignores: expr, $bpath: expr) => {
    walk_dir_entries($ignores, $bpath, 0)
  };
  ($ignores: expr) => {
    walk_dir_entries($ignores, ".", 0)
  };
}

/// walker
pub fn walker() -> Result<(), Box<dyn Error>> {
  let ignores = vec![".git", "target"].into_iter().map(|s|
    s.to_string()).collect();
  Ok(println!("total: {}", walk_dir_entries!(&ignores)?))
}

/// tests
#[cfg(test)]
mod tests {
  use super::*;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_walker() {
    assert_eq!(walker().unwrap(), ());
  }
}
