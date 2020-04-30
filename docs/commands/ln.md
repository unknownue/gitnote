# ln

!> Absolute path must be use!

### create hard link

```bash
$ ln /data/ln/src /data/ln/dst
```

### create soft link

```bash
$ ln -s /data/ln/src /data/ln/dst
# link all files from src to dst
$ ln -s /data/ln/src/* /data/ln/dst
# force to delete existed links
$ ln -sf /data/ln/src /data/ln/dst
```
