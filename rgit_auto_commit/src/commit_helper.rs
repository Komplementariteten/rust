use std::path::Path;
use git2::{Index, IndexAddOption, Repository, Status, StatusEntry, StatusOptions};
use log::{debug, info};
use walkdir::WalkDir;
use chrono::{Local};
use crate::auto_commit_errors::Errors;

pub (crate)fn search_repositories<P: AsRef<Path>>(base_dir: P) -> Result<Vec<Repository>, Errors>{
    if base_dir.as_ref().exists() {
        debug!("searching Repositories under {}", base_dir.as_ref().to_str().unwrap())
    } else {
        return Err(Errors::CantOpenBaseSearchDir)
    }

    let mut pathes = Vec::new();
    for entry in WalkDir::new(base_dir).follow_links(false).into_iter().filter_map(Result::ok).filter(| e | e.file_type().is_dir() && e.path().ends_with(".git")) {
        let repo = match Repository::open(entry.path().parent().unwrap().to_path_buf()) {
            Ok(r) => r,
            Err(e) => return Err(Errors::OpenRepositoryError(e))
        };
        pathes.push(repo);
    }

    return Ok(pathes);
}

pub (crate)fn process_repository(repo: &Repository) -> Result<bool, Errors>{

    let mut option = StatusOptions::new();
    option.include_ignored(false);
    option.include_untracked(true);

    let states = match repo.statuses(Some(&mut option)) {
        Ok(s) => s,
        Err(e) => return Err(Errors::RepositoryStatusError(e))
    };

    if states.iter().any(| s | s.status() == Status::CONFLICTED) {
        return Err(Errors::HasUnsolvedConflictWithRemote)
    }

    let sig = match repo.signature() {
        Ok(s) => s,
        Err(e) => return Err(Errors::UserSignatureError(e))
    };

    let mut commit_pending = false;
    for state in states.iter() {
        let r = match state.status() {
            Status::WT_NEW => add_to_index(&repo, &state),
            Status::WT_MODIFIED => add_to_index(&repo, &state),
            Status::WT_DELETED => delete_from_index(&repo, &state),
            _ => /* no handle needed*/ Ok(()),
        };
        if r.is_err() {
            info!("{:?}", r.err().unwrap())
        } else {
            commit_pending = true;
        }
    }


    if commit_pending {
        let oid = {
            let mut index = match repo.index() {
                Ok(i) => i,
                Err(e) => return Err(Errors::IndexError(e))
            };
            match index.write_tree() {
                Ok(tree_id) => tree_id,
                Err(e) => return Err(Errors::WriteTreeError(e))
            }
        };
        let tree = match repo.find_tree(oid) {
            Ok(t) => t,
            Err(_) => panic!("Tree not found after write")
        };

        let head = match repo.head() {
            Ok(h_oid) => h_oid.target().unwrap(),
            Err(_e) => return Err(Errors::ReferenceError("HEAD REF not found".to_string()))
        };
        let parent = repo.find_commit(head).unwrap();

        match repo.commit(Some("HEAD"), &sig, &sig, format!("Auto commit @{}", Local::now().format("%d-%m-%Y %H:%M")).as_str(), &tree, &[&parent]) {
            Ok(c) => {
                info!("auto-commit:{} for {}", c, repo.path().to_str().unwrap());
                return Ok(true)
            },
            Err(e) => return Err(Errors::CommitError(e))
        }
    }
    Ok(false)
}

fn delete_from_index(repo: &Repository, status: &StatusEntry) -> Result<(), Errors> {
    let mut index = match repo.index() {
        Ok(i) => i,
        Err(e) => return Err(Errors::IndexError(e))
    };
    let path_str = match status.path() {
        Some(p) => p,
        None => return Err(Errors::IndexPathError)
    };
    return match index.remove_path(Path::new(path_str)) {
        Ok(_) => match index.write() {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::IndexError(e))
        },
        Err(e) => Err(Errors::IndexError(e))
    };
}

fn add_to_index(repo: &Repository, status: &StatusEntry) -> Result<(), Errors> {
    let mut index = match repo.index() {
        Ok(i) => i,
        Err(e) => return Err(Errors::IndexError(e))
    };
    let path = match status.path() {
        Some(p) => Path::new(p),
        None => return Err(Errors::IndexPathError)
    };
    return match index.add_path(path) {
        Ok(_) => match index.write() {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::IndexError(e))
        },
        Err(_e) => add_all_to_index(&mut index)
    };
}

fn add_all_to_index(index: &mut Index) -> Result<(), Errors> {
    match index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None) {
        Ok(_) => match index.write() {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::IndexError(e))
        },
        Err(e) => Err(Errors::IndexError(e))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use chrono::{Local};
    use git2::Repository;
    use crate::commit_helper::{process_repository, search_repositories};

    #[test]
    fn date_time_test() {
        println!("{}", Local::now().to_rfc2822());
        println!("{}", Local::now().to_rfc3339());
        println!("{}", Local::now().format("%d-%m-%Y %H:%M"));
    }

    #[test]
    fn process_repository_ok() {
        let repo = Repository::open("/home/me/Workspace/Github/cpp").unwrap();
        let r = process_repository(&repo);
        assert!(r.is_ok());
    }

    #[test]
    fn search_repositories_is_ok() {
        let result = search_repositories(Path::new("/home/me/Workspace/Github"));
        assert!(result.is_ok());
    }
}