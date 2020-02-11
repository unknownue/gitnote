---
tags: [Command Line, Linux]
title: Linux Commands
created: '2020-01-18T12:18:31.628Z'
modified: '2020-02-11T02:55:11.895Z'
---

# Linux Commands

## Show file sizf
```shell
$ ls -hl
```

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
# Download with username and password
$ wget https://resource.url -O target_path --http-user=user --http-passwd=passwd
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

## Sed
insert the content of file1 to file2 at line 10
```shell
sed -i '10r path/to/file1' path/to/file2
```

insert the content of file1 to the end of file2
```shell
cat path/to/file1 >> path/to/file2
```

Replace characters from ORIGIN_STRING to NEW_STRING for FILE
```shell
$ sed 's/ORIGIN_STRING/NEW_STRING/' FILE
```

