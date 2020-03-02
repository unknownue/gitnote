---
tags: [Command Line]
title: 'Command: git'
created: '2020-01-27T16:05:00.692Z'
modified: '2020-03-02T01:38:37.687Z'
---

# Command: git

Revert last commit in local
```shell
$ git reset HEAD^ --soft
```

Checkout a specific tag
```shell
# List all tag
$ git tag -l
# checkout a tag
$ git checkout tags/<tag_name>
# checkout a tag in a branch
$ git checkout tags/<tag_name> -b <branch_name>
```

Modify history commit message
```shell
$ git rebase -i HEAD~5
(git) # Modify pick to edir
$ git commit --amend
$ git rebase --continue
```
