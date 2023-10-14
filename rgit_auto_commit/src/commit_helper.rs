use std::path::{Path, PathBuf};
use git2::{Repository, Status, StatusEntry, StatusOptions};
use log::{debug, info};
use walkdir::WalkDir;
use crate::auto_commit_errors::Errors;

pub (crate)fn search_repositories<P: AsRef<Path>>(base_dir: P) -> Result<Vec<PathBuf>, Errors>{
    if base_dir.as_ref().exists() {
        debug!("searching Repositories under {}", base_dir.as_ref().to_str().unwrap())
    } else {
        return Err(Errors::CantOpenBaseSearchDir)
    }

    let mut pathes = Vec::new();
    for entry in WalkDir::new(base_dir).follow_links(false).into_iter().filter_map(Result::ok).filter(| e | e.file_type().is_dir() && e.path().ends_with(".git")) {
        pathes.push(entry.path().parent().unwrap().to_path_buf());
    }

    return Ok(pathes);
}

pub (crate)fn process_repository(path: PathBuf) -> Result<(), Errors>{
    let repo = match Repository::open(path) {
        Ok(r) => r,
        Err(e) => return Err(Errors::OpenRepositoryError(e))
    };

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
            println!("{:?}: {:?}",state.status(), state.path().unwrap());
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
            Err(e) => return Err(Errors::ReferenceError("HEAD REF not found".to_string()))
        };
        let parent = repo.find_commit(head).unwrap();

        match repo.commit(Some("HEAD"), &sig, &sig, "Auto commit", &tree, &[&parent]) {
            Ok(c) => {
                info!("auto-commit:{} for {}", c, repo.path().to_str().unwrap());
                return Ok(())
            },
            Err(e) => return Err(Errors::CommitError(e))
        }
    }

    Ok(())
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
    let path_str = match status.path() {
        Some(p) => p,
        None => return Err(Errors::IndexPathError)
    };
    return match index.add_path(Path::new(path_str)) {
        Ok(_) => match index.write() {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::IndexError(e))
        },
        Err(e) => Err(Errors::IndexError(e))
    };
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::git_helper::{process_repository, search_repositories};

    #[test]
    fn process_repository_ok() {
        let r = process_repository(Path::new("/home/me/Workspace/Github/cpp").to_path_buf());
        assert!(r.is_ok());
    }

    #[test]
    fn search_repositories_is_ok() {
        let result = search_repositories(Path::new("/home/me/Workspace/Github"));
        assert!(result.is_ok());
    }
}