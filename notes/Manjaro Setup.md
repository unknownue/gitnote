---
tags: [Linux]
title: Manjaro Setup
created: '2020-02-29T12:48:44.874Z'
modified: '2020-02-29T13:18:23.010Z'
---

# Manjaro Setup

## Common configuration
```shell
# Update mirror list
$ pacman-mirrors -i -c China -m rank
$ Server = http://mirrors.163.com/archlinux/$repo/os/$arch

# Disabling multilib
# https://wiki.archlinux.org/index.php/Official_repositories#multilib
$ pacman -R $(comm -12 <(pacman -Qq | sort) <(pacman -Slq multilib | sort))
$ nvim /etc/pacman.conf # Remove "#[multilib]" segment
```
