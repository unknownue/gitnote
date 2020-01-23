---
tags: [Command Line]
title: Linux common command
created: '2020-01-18T12:18:31.628Z'
modified: '2020-01-19T09:16:08.268Z'
---

# Linux common command

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

## file content insert
insert the content of file1 to file2 at line 10
```shell
sed -i '10r path/to/file1' path/to/file2
```

insert the content of file1 to the end of file2
```shell
cat path/to/file1 >> path/to/file2
```
