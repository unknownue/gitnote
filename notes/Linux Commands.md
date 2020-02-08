---
tags: [Command Line, Linux]
title: Linux Commands
created: '2020-01-18T12:18:31.628Z'
modified: '2020-02-08T09:16:58.881Z'
---

# Linux Commands

## Query the number of threads of processor
```shell
$ nproc
```

## Check port
```shell
$ ss -lntpd | grep :22
```

## View process status
```shell
$ top
```

## Download
```shell
$ wget https://resource.url -O target_path
```

## Chmod
```shell
$ chmod 777 file
```

## 7Z

```shell
# 7z a archive.7z path_to_dir/path_to_file
$ 7z a archive.7z abc/
```

```shell
# 7z x archive.7z
$ 7z x archive.7z
```

## Mount
```shell
# mount source target
$ mount /some/where/to/mount /mnt

# unmount
$ mount /mnt
```

## file content insert
insert the content of file1 to file2 at line 10
```shell
sed -i '10r path/to/file1' path/to/file2
```

insert the content of file1 to the end of file2
```shell
cat path/to/file1 >> path/to/file2
```
