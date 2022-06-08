# pretty_branch_ps1
Custom compiled rust pretty printer for git branch 

## Usage
Add to `~/.bashrc` the following:
```bash
parse_git_branch() {
    branch=$(git branch --show-current 2> /dev/null | pretty_branch_ps1)
    [ $branch ] && echo -n " $branch " || echo -n " "
}
```

And change the `PS1`similarly to: 
```bash
export PS1="$Black$On_White \W$Blue$On_White\$(parse_git_branch)$Black$On_White> $White "
```

> Note that pretty_branch_ps1 should be in `PATH`
