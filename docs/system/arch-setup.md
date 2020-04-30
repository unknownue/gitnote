# Arch Linux Setup

## Color

```bash
# Add color to pacman
$ nvim /etc/pacman.conf
(nvim) $ # Uncomment the line with `Color`

# Add color to ls
$ alias ls="ls --color"
```

## Install basic package

```bash
# Upgrade system
$ pacman -Syu
# Install package
$ pacman -S man base-devel git
# Display system info
$ pacman -S neofetch

# Import archlinuxcn source
# https://mirror.tuna.tsinghua.edu.cn/help/archlinuxcn/
$ nvim /etc/pacman.conf
$ pacman -Syy
$ pacman -S archlinuxcn-keyring
$ pacman -Sy
```

## User

```bash
$ useradd -m -G wheel unknownue
$ passwd unknownue
$ ln -s /usr/bin/nvim /usr/bin/vi
$ visudo
(visudo) $ # Uncomment line "%wheel ALL=(ALL) ALL"
```

## TRIM

```bash
# Enable TRIM support on SSD
$ systemctl enable fstrim.timer
```

## Shutdown system

```bash
$ shutdown -h now
```

## Connect to network(by eth0)

```bash
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

```bash
$ pacman -S xorg xorg-server

# For KDE
$ pacman -S plasma-meta kde-applications-meta
$ pacman -S networkmanager net-tools
# Login display
$ pacman -S sddm sddm-kcm
# Delete all package about xfce4
$ pacman -Rcs xfce4

$ systemctl enable sddm
$ systemctl enable NetworkManager

# Now the desktop is available
$ systemctl start sddm
```

### Disable baloo(For file searching)

See also https://wiki.archlinux.org/index.php/Baloo

```bash
$ balooctl suspend
$ balooctl disable
```

### Global Menu

1. Desktop right click > Add panel > Application Menu Bar
2. Install widget
   
   ```bash
   $ pacman -S plasma5-applets-active-window-control
   ```
   
   3.Add this widget to left side of global menu
   4.open its setting, and choose `Hide titlebar for maximized windows`

## Graphics

See also http://tieba.baidu.com/p/6340530678?red_tag=p3174950699

### INTEL Driver

```bash
# Install dirver for intel
$ pacman -S xf86-video-intel
```

### NVIDIA Driver

```bash
# Install dirver for nvidia
$ pacman -S nvidia

# Install NVIDIA driver first
# Get GPU address
$ lspci | grep -E "VGA|3D"
$ nvim /usr/share/sddm/scripts/Xsetup
(nvim) $ # as follows
# Config xorg.conf
$ nvim /etc/X11/xorg.conf
(nvim) $ # Add as follows
# After /etc/X11/xorg.conf has been set
$ nvidia-xconfig

$ nvim /etc/mkinitcpio.conf
(nvim) $ # as follows
$ mkinitcpio -p linux
$ nvim /etc/default/grub
(nvim) $ # as follows
$ grub-mkconfig > /boot/grub/grub.cfg

# Try load nvidia driver(optional)
$ modprobe nvidia nvidia_uvm nvidia_drm nvidia_modeset
# If failed uninstall all nvidia driver and try others

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
GRUB_CMDLINE_LINUX_DEFAULT="quiet"
GRUB_CMDLINE_LINUX="nvidia-drm.modeset=1"
```

/usr/share/sddm/scripts/Xsetup

```bash
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

### Graphics switcher

```bash
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

```bash
$ pacman -S cuda cudnn
$ reboot
```

## Other Drivers

```bash
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

### OpenSSH

```bash
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

### Mac Remote Develement

```bash
(archlinux) $ nvim /etc/ssh/sshd_config
(nvim) $ # as follows
(mac) $ nvim /private/etc/ssh/ssh_config
(nvim) $ # ForwardX11 yes
(mac) $ ssh -X root@192.168.0.x
(mac) $ export LIBGL_ALWAYS_INDIRECT=1
```

(archlinux) /etc/ssh/sshd_config

```
X11Forwarding yes
X11DisplayOffset 10
```

(mac) /private/etc/ssh/ssh_config

```
ForwardX11 yes
```

## Shadowsocks

```bash
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

```bash
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

```bash
$ pacman -S --noconfirm docker docker-compose
$ systemctl enable docker
$ systemctl start docker
# Add current user to docker group
$ groupadd docker
$ sudo gpasswd -a $USER docker
$ newgrp docker
```

## Trojan

See also https://wiki.archlinux.org/index.php/Trojan

```bash
$ pacman -S --noconfirm trojan
# Config as https://github.com/Acris/docker-shadowsocks-libev
```
