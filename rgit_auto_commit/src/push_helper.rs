use std::env;
use std::path::Path;
use git2::{Cred, Direction, PushOptions, RemoteCallbacks, Repository, RepositoryState};
use crate::auto_commit_errors::Errors;

pub(crate) fn push_to_remote(repo: &Repository) -> Result<(), Errors> {

    println!("{:?}, is: {:?}", repo.path(), repo.state());
    if repo.state() == RepositoryState::Clean {
        return Ok(())
    }

    let head_name = match repo.head() {
        Ok(h) => h.name().unwrap().to_string(),
        Err(e) => return Err(Errors::ReferenceError("HEAD REFERENCE not found".to_string()))
    };

    let mut auth_callbacks = RemoteCallbacks::new();
    auth_callbacks.credentials(| _url, uname, _allowed_type | {
        let home = env::var("HOME").unwrap();
        // let pub_key = Path::new(format!("{}/.ssh/id_ed25519.pub", home.as_str()).as_str());
        let prv_key_path_str = format!("{}/.ssh/id_ed25519", home.as_str());
        let prv_key = Path::new(&prv_key_path_str);
        let user_name = match uname {
            Some(u) => u,
            None => "git"
        };
        return Cred::ssh_key(user_name, None, prv_key, None);
    });

    let mut push_callbacks = RemoteCallbacks::new();
    push_callbacks.push_update_reference(| refname, status | {
        assert_eq!(refname, head_name.as_str());
        Ok(())
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(push_callbacks);

    let mut origin = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(e) => return Err(Errors::RemotesError(e))
    };
    let mut remote_connection = match origin.connect_auth(Direction::Push, Some(auth_callbacks), None) {
        Ok(c) => c,
        Err(e) => return Err(Errors::RemotesError(e))
    };
    match remote_connection.remote().push(&[&head_name], Some(&mut push_options)) {
        Ok(p) => return Ok(p),
        Err(e) => return Err(Errors::PushError(e))
    };

    /* for remote_opt in remotes.iter() {
        if let Some(remote_str) = remote_opt {
            let remote_callback = RemoteCallbacks::new();
            let mut remote = match repo.find_remote(remote_str) {
                Ok(r) => r,
                Err(e) => return Err(Errors::RemotesError(e))
            };
            let mut remote_connection = match remote.connect_auth(Direction::Push, Some(remote_callback), None) {
                Ok(c) => c,
                Err(e) => return Err(Errors::RemotesError(e))
            };
            match remote_connection.remote().push(&[], None) {
                Ok(p) => return Ok(p),
                Err(e) => return Err(Errors::PushError(e))
            };
        }
    } */

    Ok(())
}