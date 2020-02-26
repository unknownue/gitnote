---
tags: [Command Line, Linux]
title: Shell Commands
created: '2020-01-18T12:18:31.628Z'
modified: '2020-02-26T10:15:13.862Z'
---

# Shell Commands

## Echo
```shell
# output something to std
$ echo "Hello world"
# Rewrite the file content(if not exist, create it)
$ echo "**FILE CONTENT**" > target.txt
# Append content to the end of file
$ echo "**ADDITIONAL CONTENT**" >> target.txt
# Output to stderr
$ >&2 echo "Here is stderr"
```

## View disk usage
```shell
# Show file size of current directory
$ ls -hl
# Show disk usage
$ df -hl
# Show the file and directroy size of current folder
$ du -hl --max-depth=1
```

## Search
```shell
# Search current directory
$ grep -rn "hello,world!" ./
```

## Query the number of threads of processor
```shell
$ nproc
```

## Query CPU info
```shell
$ cat /proc/cpuinfo
```

## Query process status
```shell
# View all proess realtime status
$ top
# query process by port
$ ss -lntpd | grep :22  # Linux
$ lsof -i TCP:6000      # macOS
# query process by NAME
$ ps -ef | grep -m1 NAME
# Get process ID
$ ps -ef | grep -m1 socat | awk '{ print $2 }'
# Kill process
$ kill -9 PROCESS_ID
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

Search local network device
```shell
$ nmap -sP 192.168.1.0/24
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
$ sed -i '10r path/to/file1' path/to/file2
```

insert the content of file1 to the end of file2
```shell
$ cat path/to/file1 >> path/to/file2
```

Replace characters from ORIGIN_STRING to NEW_STRING for FILE
```shell
$ sed -i 's/ORIGIN_STRING/NEW_STRING/' FILE
```

## Path
```shell
# Convert a path to an absolute path without symlinks
$ realpath some/path

# Return the directory name of given path
$ dirname some/path

# Delete the path prefix and trailing extension of given path
$ basename some/path
```

## Get IP address
```shell
# IPV4
$ ip -4 address show | grep inet | grep -v 127.0.0 | awk '{print $2}' | cut -d'/' -f1
# IPV6
$ ip -6 address show | grep inet6 | awk '{print $2}' | cut -d'/' -f1
```

