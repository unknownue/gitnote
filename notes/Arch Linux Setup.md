---
tags: [Linux]
title: Arch Linux Setup
created: '2020-02-06T18:15:10.214Z'
modified: '2020-02-08T14:04:04.470Z'
---

# Arch Linux Setup

## Color
```shell
# Add color to pacman
$ nvim /etc/pacman.conf
(nvim) $ # Uncomment the line with `Color`

# Add color to ls
$ alias ls="ls --color"
```

## Install basic package
```shell
# Upgrade system
$ pacman -Syu
# Install package
$ pacman -S man base-devel git
# Display system info
$ pacman -S neofetch
```

## User
```shell
$ useradd -m -G wheel unknownue
$ passwd unknownue
$ ln -s /usr/bin/nvim /usr/bin/vi
$ visudo
(visudo) $ # Uncomment line "%wheel ALL=(ALL) ALL"
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

# GUI
https://wiki.archlinux.org/index.php/KDE
```shell
$ pacman -S xorg xorg-server

# For KDE
$ pacman -S plasma-meta kde-applications-meta
$ pacman -S networkmanager net-tools
# Login display
$ pacman -S sddm sddm-kcm

$ systemctl enable sddm
$ systemctl enable NetworkManager

# Use NVIDIA driver for GUI(optional)
# Install NVIDIA driver first
# Get GPU address
$ lspci | grep -E "VGA|3D"
$ nvim /usr/share/sddm/scripts/Xsetup
(nvim) $ # as follows
# Config xorg.conf
$ nvim /etc/X11/xorg.conf
(nvim) $ # Add as follows
```

/usr/share/sddm/scripts/Xsetup
```shell
#!/bin/sh
# Xsetup - run as root before the login dialog appear
xrandr --setprovideroutputsource modesetting NVIDIA-0
xrandr --auto
```

/etc/X11/xorg.conf(The BusID is needed to calculate)
```
Section "Module"
    Load "modesetting"
EndSection

Section "Device"
    Identifier "nvidia"
    Driver "nvidia"
    BusID "1:0:0"
    Option "AllowEmptyInitialConfiguration"
EndSection
```

### Global Menu
1. Desktop right click > Add panel > Application Menu Bar
2. Install widget
```shell
$ pacman -S plasma5-applets-active-window-control
```
3.Add this widget to left side of global menu
4.open its setting, and choose `Hide titlebar for maximized windows`


## Graphics
### INTEL driver
```shell
# Install dirver for intel
$ pacman -S xf86-video-intel
```

### NVIDIA
```shell
# Install dirver for nvidia
$ pacman -S nvidia nvidia-libgl
# After /etc/X11/xorg.conf has been set
$ nvidia-xconfig

# Try load nvidia driver
$ modprobe nvidia nvidia_uvm nvidia_drm nvidia_modeset
# If failed uninstall all nvidia driver and try others

$ nvim /etc/mkinitcpio.conf
(nvim) $ # as follows
$ nvim /etc/default/grub
(nvim) $ # as follows
$ grub-mkconfig > /boot/grub/grub.cfg

# Check if nvidia dirver is running
$ lsmod | grep nvidia
$ nvidia-smi -q
```

/etc/mkinitcpio.conf
```
MODULES=(nvidia nvidia_modeset nvidia_uvm nvidia_drm)
```

/etc/default/grub
```
GRUB_CMDLINE_LINUX_DEFAULT="quiet nvidia-drm.modeset=1"
```

### Graphics switcher
```shell
# See which GPU is working now
$ lspci | grep VGA

# Install driver switch
$ pacman -S bbswitch
$ modprobe bbswitch
# Open the nvidia driver
$ tee /proc/acpi/bbswitch <<< ON
# Close the nvidia driver
$ tee /proc/acpi/bbswitch <<< OFF
# Check switch state
$ cat /proc/acpi/bbswitch

# Duel GPU switcher
$ pacman -S bumblebee
$ sudo usermod -a -G bumblebee unknownue
$ nvim /etc/bumblebee/bumblebee.conf
(nvim) $ # as follows
$ systemctl start bumblebeed
$ systemctl enable bumblebeed
```

/etc/bumblebee/bumblebee.conf
```
Driver=nvidia
[driver-nvidia]
PMMethod=bbswitch
```

### cuDNN
See https://developer.nvidia.com/cuda-gpus
```shell
$ pacman -S cuda cudnn
$ reboot
```

## Other Drivers
```shell
# Audio
$ pacman -S alsa-utils pulseaudio pulseaudio-alsa

# Power Management
$ pacman -S tlp tlp-rdw
$ systemctl enable tlp
$ systemctl enable tlp-sleep
$ systemctl mask systemd-rfkill.service
$ systemctl mask systemd-rfkill.socket
$ systemctl start tlp.service

$ pacman -S smartmontools
# show battery info
$ tlp-stat --battery
# show disk info
$ tlp-stat --disk
# show pci-e info
$ tlp-stat --pcie
# show GPU info
$ tlp-stat --graphics
# show CPU info
$ tlp-stat --processor
# show system info
$ tlp-stat --system
# show fan info
$ tlp-stat --temp
# show usb info
$ tlp-stat --usb
# total report
$ tlp-stat
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

## Shadowsocks
```shell
$ pacman -S shadowsocks-libev
$ mkdir /etc/shadowsocks
$ nvim /etc/shadowsocks/config.json
(nvim) $ # as follows
$ systemctl start shadowsocks-libev@config
# Query log by
$ journalctl -u shadowsocks@config

# Config global proxy
$ pacman -S privoxy
$ nvim /etc/privoxy/config
(nvim) $ # Chang and modify line"# forward-socks5t / 127.0.0.1:1080 ."
$ systemctl start privoxy
# In terminal
$ export http_proxy="127.0.0.1:8118"
$ export https_proxy="127.0.0.1:8118"

# Close shadowsocks
$ systemctl start shadowsocks-libev@config
$ systemctl start privoxy

# Launch on front
$ ss-local -c /etc/shadowsocks/config.json
```

/etc/shadowsocks/config.json
```json
{
	"server":"remote-shadowsocks-server-ip-addr",
	"server_port":443,
	"local_address":"127.0.0.1",
	"local_port":1080,
	"password":"your-passwd",
	"timeout":300,
	"method":"chacha20-ietf",
	"fast_open":false,
	"workers":1
}
```

## BBR
```shell
# the kernel version must be >= 4.9
$ uname -r
# Check if bbr module exist
$ modinfo tcp_bbr
# Enable bbr
$ modprobe tcp_bbr
# Default to use bbs
$ echo "tcp_bbr" > /etc/modules-load.d/80-bbr.conf
$ echo "net.ipv4.tcp_congestion_control=bbr" >> /etc/sysctl.d/80-bbr.conf
$ echo "net.core.default_qdisc=fq" >> /etc/sysctl.d/80-bbr.conf
# Check if success
$ lsmod | grep bbr
```

## Docker
```shell
$ pacman -S --noconfirm docker docker-compose
$ systemctl enable docker
$ systemctl start docker
# Add current user to docker group
$ groupadd docker
$ sudo gpasswd -a $USER docker
$ newgrp docker
```

# Vulkan

## Trojan
See also https://wiki.archlinux.org/index.php/Trojan
```shell
$ pacman -S --noconfirm trojan
# Config as https://github.com/Acris/docker-shadowsocks-libev
```

