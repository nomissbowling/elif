#![doc(html_root_url = "https://docs.rs/elif/0.0.2")]
//! file and directory walker for Rust
//!

use std::error::Error;
use std::fs;
// use fs_extra;
use std::path::PathBuf;

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

/// walk dir entries (with compare)
pub fn walk_dir_entries(ignores: &Vec<String>, bpaths: &Vec<&str>, dep: u64) ->
  Result<u64, Box<dyn Error>> {
  let mut total: u64 = 0;
  let depth = String::from_utf8((0..dep).into_iter().map(|_| 0x20).collect())?;
//  println!("{}+{} ({})", depth, bpaths[0], bpaths[1]);
  let Ok(pde) = read_dir_entries(bpaths[0]) else {
    return Err(format!("no dir entries in [{}]", bpaths[0]).into());
  };
  let Ok(qde) = read_dir_entries(bpaths[1]) else {
    return Err(format!("no dir entries in [{}]", bpaths[1]).into());
  };
  for pe in pde {
    let p = &pe.path();
    let mut q = PathBuf::from(""); // dummy
    let mut f = false;
    for qe in &qde {
      let qpb = &qe.path();
      if qpb.file_name() == p.file_name() { q = qpb.clone(); f = true; break; }
    }
    print!("{}", if f {"T"}else{"F"});
    print!("{}{}", depth, if p.is_dir() {"+"}else{"-"});
    // println!("{:?}", pe); // fs::DirEntry
    // println!("{:?}", pe.file_name()); // OsString
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
      let e = pe.file_name().to_str().expect("invalid_name").to_string();
      if !ignores.contains(&e) {
        let s = p.to_str().expect("invalid_path");
        let t = q.to_str().expect("invalid_path");
        total += walk_dir_entries(ignores, &vec![s, t], dep + 1)?;
      }
    }
  }
  Ok(total)
}

/// macro walk dir entries (with compare)
#[macro_export]
macro_rules! walk_dir_entries {
  ($ignores: expr, $bpaths: expr) => {
    walk_dir_entries($ignores, $bpaths, 0)
  };
  ($ignores: expr) => {
    walk_dir_entries($ignores, vec![".", "."], 0)
  };
}

/// walker (with compare)
pub fn walker(dirs: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
  let ignores = vec![".git", "target"].into_iter().map(|s|
    s.to_string()).collect();
  let f = |pbs: &Vec<&str>| {
    println!("[{}] - [{}]", pbs[0], pbs[1]);
    let t = match walk_dir_entries!(&ignores, pbs) {
    Err(e) => { eprintln!("{:?}", e); 0 },
    Ok(t) => t
    };
    println!("total: {}", t);
  };
  let mut pbs = dirs.iter().map(|p| p.to_str().unwrap()).collect::<Vec<_>>();
  f(&pbs);
  pbs.reverse();
  f(&pbs);
  Ok(())
}

/// take2
pub fn take2<T>(args: T) -> Vec<PathBuf> where T: Iterator<Item = String> {
  let mut dirs = Vec::<PathBuf>::new();
  for a in args {
    let p = PathBuf::from(a);
    if !p.exists() { continue; }
    if !p.is_dir() { continue; }
    dirs.push(p);
  }
  println!("dirs: {}", dirs.len());
  for (i, p) in dirs.iter().enumerate() {
    println!("dirs[{}]: {}", i, p.display());
  }
  if dirs.len() < 2 {
    println!("Usage: {} a dir0 dir1 ...", env!("CARGO_PKG_NAME"));
    return vec!["src", "src"].into_iter().map(|s| PathBuf::from(s)).collect();
  }
  dirs
}

/// tests
#[cfg(test)]
mod tests {
  use super::*;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_walker() {
    assert_eq!(walker(take2(std::env::args().skip(1))).unwrap(), ());
  }
}
