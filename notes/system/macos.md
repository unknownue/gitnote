---
tags: [macOS]
title: Mac Notes
created: '2020-02-10T13:57:35.284Z'
modified: '2020-03-02T07:56:54.475Z'
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

## Context Menu
```shell
# Set the number of items required to create the Services submenu
# Here 999 is just an example
$ write -g NSServicesMinimumItemCountForContextSubmenu -int 999
```

## Temporarily disable homebrew update
```shell
$ export HOMEBREW_NO_AUTO_UPDATE=true
```
