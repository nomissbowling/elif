#![doc(html_root_url = "https://docs.rs/elif/0.0.4")]
//! file and directory walker for Rust
//!

use std::error::Error;
use std::io::{Read, BufReader};
use std::fs;
// use fs_extra;
use std::path::PathBuf;

// use hashes::md5::hash;
use md5;
// use binascii::bin2hex;

/// get file metadata
pub fn file_meta(p: &PathBuf) -> Result<fs::Metadata, Box<dyn Error>> {
  let Ok(mf) = fs::metadata(p) else {
    return Err(format!("cannot get metadata: {}", p.display()).into())
  };
  Ok(mf)
}

/// calc md5 sum digest
pub fn md5sum(p: &PathBuf, sz: u64) -> Result<String, Box<dyn Error>> {
/*
  let s = "abcdefghijklmnopqrstuvwxyz".as_bytes();
  let mut digest = vec![0u8; 32];
  let _r = bin2hex(&hash(s).into_bytes(), &mut digest).expect("digest");
  Ok(String::from_utf8(digest).unwrap())
*/
/*
  let s = "abcdefghijklmnopqrstuvwxyz".as_bytes();
  let digest = md5::compute(s);
  Ok(format!("{:x}", digest))
*/
  let mut t: usize = 0;
  let f = fs::File::open(p)?;
  let mut rdr = BufReader::new(f);
  let mut buf = vec![0u8; 8192]; // when use read_exact() [0u8; sz as usize];
  let mut ctx = md5::Context::new();
  loop {
    match rdr.read(&mut buf) {
//    Err(Error::from(std::io::ErrorKind::UnexpectedEof)) => { break; },
    Err(_e) => { break; },
    Ok(l) => { ctx.consume(&buf[..l]); t += l; if t >= sz as usize { break; } }
    }
  }
  if t < sz as usize { return Err("can't read all".into()); }
  let digest = ctx.compute();
  Ok(format!("{:x}", digest))
}

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
pub fn walk_dir_entries(inf: bool,
  ignores: &Vec<String>, bpaths: &Vec<&str>, dep: u64) ->
  Result<u64, Box<dyn Error>> {
  let mut total: u64 = 0;
  let depth = String::from_utf8((0..dep).into_iter().map(|_| 0x20).collect())?;
//  println!("{}+{} ({})", depth, bpaths[0], bpaths[1]);
  let mut msg = vec![]; // first define for dir entries
  let mut de = vec![];
  for i in 0..2 {
    match read_dir_entries(bpaths[i]) {
    Err(e) => { msg.push(format!("no dir entries [{}] [{}]", e, bpaths[i])); },
    Ok(ent) => { de.push(ent); }
    }
  }
  if msg.len() > 0 || de.len() < 2 { return Err(msg.join("\x0A").into()); }
  for pe in &de[0] {
    let p = &pe.path(); // &PathBuf
    let mut q = PathBuf::from(""); // dummy PathBuf
    let mut f = false;
    for qe in &de[1] {
      let qpb = &qe.path();
      if qpb.file_name() == p.file_name() { q = qpb.clone(); f = true; break; }
    }
    let mut msg = vec![]; // second define for metadata and md5 sum digest
    let mut sz = vec![];
    let mut digest = vec![];
    if p.is_file() {
      for p in [p, &q].iter() {
        sz.push(match file_meta(p) {
        Err(e) => { msg.push(format!("metadata: {}", e)); 0 },
        Ok(mf) => { mf.len() }
        });
      }
      if sz[0] != sz[1] {
        for _i in 0..2 { digest.push("".to_string()); } // skip md5sum
      } else {
        for (i, p) in [p, &q].iter().enumerate() {
          digest.push(match md5sum(p, sz[i]) {
          Err(e) => { msg.push(format!("md5sum: {}", e)); "".to_string() },
          Ok(s) => { s }
          });
        }
      }
      total += sz[0];
      if msg.len() > 0 || sz.len() < 2 || digest.len() < 2 {
        // eprintln!(" ===error=== {}", msg.join("\x0A"));
        f = false;
      }
      if sz[0] != sz[1] || digest[0] != digest[1] { f = false; }
    }
    if p.is_dir() || (inf || !f) {
      print!("{}", if f {"T"}else{"F"});
      print!("{}{}", depth, if p.is_dir() {"+"}else{"-"});
      // println!("{:?}", pe); // fs::DirEntry
      // println!("{:?}", pe.file_name()); // OsString
      println!("{}", p.display()); // path::PathBuf
      if !p.is_dir() {
        for i in 0..2 { println!(" {} {}", sz[i], digest[i]); }
      }
    }
    if p.is_dir() {
      let e = pe.file_name().to_str().expect("invalid_name").to_string();
      if !ignores.contains(&e) {
        let st = [p, &q].iter().map(|p| {
          p.to_str().expect("invalid_path")
        }).collect::<Vec<_>>();
        total += walk_dir_entries(inf, ignores, &st, dep + 1)?;
      }
    }
  }
  Ok(total)
}

/// macro walk dir entries (with compare)
#[macro_export]
macro_rules! walk_dir_entries {
  ($inf: expr, $ignores: expr, $bpaths: expr) => {
    walk_dir_entries($inf, $ignores, $bpaths, 0)
  };
  ($inf: expr, $ignores: expr) => {
    walk_dir_entries($inf, $ignores, vec![".", "."], 0)
  };
}

/// walker (with compare)
pub fn walker(inf: bool, dirs: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
  let ignores = vec![".git", "target"].into_iter().map(|s|
    s.to_string()).collect();
  let f = |pbs: &Vec<&str>| {
    println!("[{}] - [{}]", pbs[0], pbs[1]);
    let t = match walk_dir_entries!(inf, &ignores, pbs) {
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
//    assert_eq!(walker(true, take2(std::env::args().skip(1))).unwrap(), ());
    assert_eq!(walker(false, take2(std::env::args().skip(1))).unwrap(), ());
  }
}
