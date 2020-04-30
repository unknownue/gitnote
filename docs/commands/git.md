# git

Revert last commit in local

```bash
$ git reset HEAD^ --soft
```

Checkout a specific tag

```bash
# List all tag
$ git tag -l
# checkout a tag
$ git checkout tags/<tag_name>
# checkout a tag in a branch
$ git checkout tags/<tag_name> -b <branch_name>
```

Modify history commit message

```bash
$ git rebase -i HEAD~5
(git) # Modify pick to edir
$ git commit --amend
$ git rebase --continue
```

### Submodule

```bash
# Pull all submodule
$ git submodule update --remote --recursive 
```
