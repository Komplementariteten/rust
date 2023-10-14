use git2::{Direction, Repository};
use crate::auto_commit_errors::Errors;

pub(crate) fn push_to_remote(repo: &Repository) -> Result<(), Errors> {
    let remotes = match repo.remotes() {
        Ok(r) => r,
        Err(e) => return Err(Errors::RemotesError(e))
    };

    for remote_opt in remotes.iter() {
        if let Some(remote_str) = remote_opt {
            let mut remote = match repo.find_remote(remote_str) {
                Ok(r) => r,
                Err(e) => return Err(Errors::RemotesError(e))
            };
            let mut remote_connection = match remote.connect_auth(Direction::Push, None, None) {
                Ok(c) => c,
                Err(e) => return Err(Errors::RemotesError(e))
            };
            match remote_connection.remote().push(&["HEAD"], None) {
                Ok(p) => return Ok(p),
                Err(e) => return Err(Errors::PushError(e))
            };
        }
    }

    Ok(())
}