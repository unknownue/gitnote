---
tags: [macOS]
title: Mac Notes
created: '2020-02-10T13:57:35.284Z'
modified: '2020-02-16T01:52:10.292Z'
---

# Mac Notes

## Check port
```shell
# check if PORT is currently using
$ lsof -i :PROT
```

## Kill problem
```shell
# kill a problem that is using PORT
$ kill -9 PORT
```

## Disable update when install package by homebrew
```shell
$ export HOMEBREW_NO_AUTO_UPDATE=true
```
