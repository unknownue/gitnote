# Shell Commands

## Echo
```bash
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
```bash
# Show file size of current directory
$ ls -hl
# Show disk usage
$ df -hl
# Show the file and directroy size of current folder
$ du -hl --max-depth=1
```

## Search
```bash
# Search current directory
$ grep -rn "hello,world!" ./
```

## Query the number of threads of processor
```bash
$ nproc
```

## Query CPU info
```bash
$ cat /proc/cpuinfo
```

## Query process status
```bash
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
```bash
$ wget https://resource.url -O target_path
# Download with username and password
$ wget https://resource.url -O target_path --http-user=user --http-passwd=passwd
```

## Chmod
```bash
$ chmod 777 file
```

## 7Z

```bash
# 7z a archive.7z path_to_dir/path_to_file
$ 7z a archive.7z abc/
```

```bash
# 7z x archive.7z
$ 7z x archive.7z
```

Search local network device
```bash
$ nmap -sP 192.168.1.0/24
```

## Mount
```bash
# mount source target
$ mount /some/where/to/mount /mnt

# unmount
$ mount /mnt
```

## Sed
insert the content of file1 to file2 at line 10
```bash
$ sed -i '10r path/to/file1' path/to/file2
```

insert the content of file1 to the end of file2
```bash
$ cat path/to/file1 >> path/to/file2
```

Replace characters from ORIGIN_STRING to NEW_STRING for FILE
```bash
$ sed -i 's/ORIGIN_STRING/NEW_STRING/' FILE
```

## Path
```bash
# Convert a path to an absolute path without symlinks
$ realpath some/path

# Return the directory name of given path
$ dirname some/path

# Delete the path prefix and trailing extension of given path
$ basename some/path
```

## Get IP address
```bash
# IPV4
$ ip -4 address show | grep inet | grep -v 127.0.0 | awk '{print $2}' | cut -d'/' -f1
# IPV6
$ ip -6 address show | grep inet6 | awk '{print $2}' | cut -d'/' -f1
```

