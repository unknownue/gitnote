---
tags: [Linux]
title: Arch Linux
created: '2020-02-06T18:15:10.214Z'
modified: '2020-02-07T12:19:04.407Z'
---

# Arch Linux

## User
```shell
$ useradd -m -G wheel unknownue
$ passwd unknownue
```

## Color
```shell
# Add color to pacman
$ nvim /etc/pacman.conf
# Uncomment the line with `Color`

# Add color to ls
$ alias ls="ls --color"
```

## Shutdown system
```shell
$ shutdown -h now
```

## Connect to network(by eth0)
```shell
# find some way to install dhcpcd
$ pacman -S dhcpcd 

# Edit /etc/rc.conf
$ echo "interface = eth0" >> /etc/rc.conf
$ dhcpcd

$ systemctl enable dhcpcd
$ systemctl start dhcpcd

# See if network is enabled now
ping google.com
```

## SSH
```shell
$ pacman -S openssh
# Start on system launch
$ systemctl enable sshd
# Launch now
$ systemctl start sshd
# Restart
$ systemctl restart sshd

# Allow root to login
$ nvim /etc/ssh/sshd_config
# Change `#PermitRootLogin prohibit-password` to `PermitRootLogin yes`

# Modify default port
$ nvim /etc/ssh/sshd_config
# Set port by add line `Port 12596`
```

## Docker
```shell
$ pacman -S --noconfirm docker docker-compose
$ systemctl enable docker
$ systemctl start docker
```


## Trojan
See also https://wiki.archlinux.org/index.php/Trojan
```shell
$ pacman -S --noconfirm trojan
# Config as https://github.com/Acris/docker-shadowsocks-libev
```

