use std::path::Path;

use git2::{Error, Remote, Repository};

fn main() {
    match get_repo() {
        Ok(()) => (),
        Err(_e) => (),
    }
}

fn get_remote(repo: &Repository) -> Result<Remote, Error> {
    let head = repo.head()?;
    let refname = head.name().ok_or(Error::from_str("Head not found"))?;
    if let Ok(name) = repo.branch_upstream_remote(refname) {
        repo.find_remote(name.as_str().ok_or(Error::from_str("Not utf-8"))?)
    } else {
        repo.find_remote("origin")
    }
}

fn get_name(repo: &Repository) -> Option<&str> {
    repo.path().parent()?.file_name()?.to_str()
}

fn get_repo() -> Result<(), Error> {
    let repo = Repository::open_from_env()?;
    let remote = get_remote(&repo);
    let url = match &remote {
        Ok(remote) => remote.url(),
        Err(_e) => get_name(&repo),
    };
    // Valid urls:
    //ssh://[user@]host.xz[:port]/path/to/repo.git/
    //git://host.xz[:port]/path/to/repo.git/
    //http[s]://host.xz[:port]/path/to/repo.git/
    //ftp[s]://host.xz[:port]/path/to/repo.git/
    //
    // Trim the protocol, along with the user `git` and `github.com`
    // We also trim the `.git` at the end. This isn't the full URL,
    // but it should provide enough information to identify the repo
    // at a glance.
    let url = url
        .unwrap_or("")
        .trim_start_matches("ssh://")
        .trim_start_matches("git://")
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .trim_start_matches("ftp://")
        .trim_start_matches("ftps://")
        .trim_start_matches("git@")
        .trim_start_matches("github.com")
        .trim_start_matches('/')
        .trim_start_matches(':')
        .trim_end_matches('/')
        .trim_end_matches(':')
        .trim_end_matches(".git");

    println!("{}", url);
    Ok(())
}
