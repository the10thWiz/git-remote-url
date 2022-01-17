# git-remote-url

Prints the remote url of a repository. This is intended to be run inside a
prompt to identify the repository.

The remote url used is the remote of the upstream branch, otherwise defaulting to the `origin`
remote if the current branch doesn't have an upstream branch. If there is no remote found, 
the directory name of the repository root is used instead.

For convience, the protocol, `.git`, `git` ssh user, and `github.com` are stripped from the url.
