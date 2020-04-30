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
# Output multi-lines to file
$ (
cat <<EOF

contents

EOF
) > path/to/file
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
# Split file
$ curl --range 0-199999999 -o ubuntu-iso.part2 http://mirror.pnl.gov/releases/15.04/ubuntu-15.04-desktop-amd64.iso
$ curl --range 200000000-399999999 -o ubuntu-iso.part2 http://mirror.pnl.gov/releases/15.04/ubuntu-15.04-desktop-amd64.iso
$ curl --range 400000000-599999999 -o ubuntu-iso.part3 http://mirror.pnl.gov/releases/15.04/ubuntu-15.04-desktop-amd64.iso
$ curl --range 600000000-799999999 -o ubuntu-iso.part4 http://mirror.pnl.gov/releases/15.04/ubuntu-15.04-desktop-amd64.iso
$ curl --range 800000000-999999999 -o ubuntu-iso.part5 http://mirror.pnl.gov/releases/15.04/ubuntu-15.04-desktop-amd64.iso
$ curl --range 1000000000- -o ubuntu-iso.part6 http://mirror.pnl.gov/releases/15.04/ubuntu-15.04-desktop-amd64.iso
$ cat ubuntu-iso.part? > ubuntu-15.04-desktop-amd64.iso
```

## Chmod

```bash
$ chmod 777 file
```

## 7z

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

## Display a file

```bash
# Output the content of file
$ cat filename
# Output first 100 lines(from 5 to 10 lines)
$ sed -nn '5,10p' filename
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

# aria2

```bash
# Example: aria2c --ftp-user=username --ftp-passwd=password sftp://remote_ip:port/path/to/target/file
aria2c --ftp-user=root --ftp-passwd=1111 sftp://111.111.11.11:34432/home/user/files/hello.txt
```
