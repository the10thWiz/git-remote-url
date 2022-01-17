
use git2::{Error, Repository};

fn main() {
    match get_repo() {
        Ok(()) => (),
        Err(_e) => (),
    }
}

fn get_repo() -> Result<(), Error> {
    let repo = Repository::open_from_env()?;
    let head = repo.head()?;
    let refname = head.name().ok_or(Error::from_str("Head not found"))?;
    let remote = if let Ok(name) = repo.branch_upstream_remote(refname) {
        repo.find_remote(name.as_str().unwrap())?
    } else {
        repo.find_remote("origin")?
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
    let url = remote.url()
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
