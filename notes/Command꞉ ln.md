---
title: 'Command: ln'
created: '2020-03-04T02:10:29.537Z'
modified: '2020-03-04T02:44:02.467Z'
---

# Command: ln

## **Absolute path must be use!**

### create hard link
```shell
$ ln /data/ln/src /data/ln/dst
```

### create soft link
```shell
$ ln -s /data/ln/src /data/ln/dst
# link all files in dst
$ ln -s /data/ln/src /data/ln/dst
```
