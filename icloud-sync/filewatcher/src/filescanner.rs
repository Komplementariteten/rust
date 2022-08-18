use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::{Metadata, read, read_dir};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct PathFileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub crc32: u32
}

#[derive(Debug)]
struct DirIteratorOpts {
    get_crc32:bool,
    follow_symlinks: bool,
    // ToDo: Implement this later
    // name_filter: Option<Vec<String>>,
}

#[derive(Debug)]
struct DirIterator {
    file_cache: Vec<PathFileEntry>,
    dir_stack: Vec<PathBuf>,
    opts: DirIteratorOpts,
}

impl Iterator for DirIterator {
    type Item = PathFileEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.file_cache.is_empty() {
            if let Some(dir) = self.dir_stack.pop() {
                // Ensure Dir is readable
                let rd = match read_dir(dir) {
                    Ok(dir) => dir,
                    Err(e) => panic!("{}", e.to_string())
                };

                for dir_item in rd.filter_map(| dir| dir.ok())
                    .filter(| dir | dir.metadata().is_ok()) {
                    let meta = dir_item.metadata().unwrap();
                    if meta.is_symlink().eq(&self.opts.follow_symlinks) {
                        self.evaluate_cur(dir_item.path(), meta);
                    }
                }
            } else {
                // No directory left to visist
                return None
            }
        }
        self.file_cache.pop()
    }
}

/* impl IntoIterator for DirIterator {
    type Item = ();
    type IntoIter = ();

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
} */

impl DirIterator {
    fn evaluate_cur(&mut self, d: PathBuf, m: Metadata) {
        if m.is_dir() {
            self.dir_stack.push(d)
        } else {
            let mut  hash: u32 = 0;
            if self.opts.get_crc32 {
                let data = read(&d);
                if data.is_ok() {
                    hash = crc32fast::hash(&data.unwrap());
                }
            }
            self.file_cache.push(PathFileEntry {
                path: d,
                crc32: hash,
                size: m.size(),
                modified: m.modified().expect("modified-time not supported on your os"),
                created: m.created().expect("created-time not supported on your os"),
            })
        }
    }
    fn new(p: PathBuf, follow_symlinks: bool, with_hash: bool) -> DirIterator {
        let opts = DirIteratorOpts {
            follow_symlinks: follow_symlinks,
            get_crc32: with_hash
        };
        let d_stack = vec![p];
        DirIterator {
            opts: opts,
            file_cache: Vec::new(),
            dir_stack: d_stack
        }
    }
}

pub fn scan_ordered<P: AsRef<Path>>(p: P, with_hash: bool) -> Option<Vec<PathFileEntry>> {
    let iter = DirIterator::new(p.as_ref().to_path_buf(), false, with_hash);
    let mut res_vec = Vec::<PathFileEntry>::new();
    for entry in iter {
        res_vec.push(entry);
    }
    if res_vec.is_empty() {
        None
    } else {
        res_vec.sort_by(| a, b | b.modified.borrow().cmp(a.modified.borrow()));
        Some(res_vec)
    }
}
pub fn scan<P: AsRef<Path>>(p: P, with_hash: bool) -> Option<Vec<PathFileEntry>> {
    let iter = DirIterator::new(p.as_ref().to_path_buf(), false, with_hash);
    let mut res_vec = Vec::<PathFileEntry>::new();
    for entry in iter {
        res_vec.push(entry);
    }
    if res_vec.is_empty() {
        None
    } else {
        Some(res_vec)
    }
}


pub fn scan_hm<P: AsRef<Path>>(p: P) -> Option<HashMap<PathBuf, PathFileEntry>> {
    let iter = DirIterator::new(p.as_ref().to_path_buf(), false, true);
    let mut res_vec = HashMap::<PathBuf, PathFileEntry>::new();
    for entry in iter {
        res_vec.insert(entry.path.clone(), entry);
    }
    if res_vec.is_empty() {
        None
    } else {
        Some(res_vec)
    }
}