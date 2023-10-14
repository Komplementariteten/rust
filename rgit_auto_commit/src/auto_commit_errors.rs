#[derive(Debug)]
pub enum Errors {
    CantOpenBaseSearchDir,
    OpenRepositoryError(git2::Error),
    RepositoryStatusError(git2::Error),
    HasUnsolvedConflictWithRemote,
    IndexError(git2::Error),
    IndexPathError,
    UserSignatureError(git2::Error),
    WriteTreeError(git2::Error),
    CommitError(git2::Error),
    ReferenceError(String),
    RemotesError(git2::Error),
    PushError(git2::Error),
}