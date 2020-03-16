# Mac Notes

## Check port
```bash
# check if PORT is currently using
$ lsof -i :PROT
```

## Kill problem
```bash
# kill a problem that is using PORT
$ kill -9 PORT
```

## Disable update when install package by homebrew
```bash
$ export HOMEBREW_NO_AUTO_UPDATE=true
```

## Context Menu
```bash
# Set the number of items required to create the Services submenu
# Here 999 is just an example
$ write -g NSServicesMinimumItemCountForContextSubmenu -int 999
```

