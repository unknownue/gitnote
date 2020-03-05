# Arch Linux Installation

# Prepare
```bash
# Downdown install iso from https://archlinux.org/download/.
# Boot this iso from the computer to install.
# vertify signature
$ pacman-key -v archlinux-version-x86_64.iso.sig
```

# Installation setup
```bash
# Set keyboard layout
$ loadkeys colemak
# Set console font
$ setfont /usr/share/kbd/consolefonts/LatGrkCyr-12x22.psfu.gz
# See wifi device
$ ip link
# Set current wifi device(assume its name is wlan0)
$ ip link set wlan0 up
# Search all wifi
$ iwlist wlan0 scan
# Allocate ip address
$ dhcpcd
# Connect internet by wpa_passphrase
$ wpa_passphrase network_name password > internet.conf
$ wpa_passphrase -c internet.conf -i wlan0
# Try ping
$ ping baidu.com
# Sync time
$ timedatectl set-ntp true
```

## Allocate storage
```bash
# check if motherboard support UEFI
$ ls /sys/firmware/efi/efivars
$ efivar -l
# Check storage device
$ fdisk -l
# Edit a disk
$ fdisk disk_name
# -----------------------------------------------
# For MBR allocation(Not support UEFI)
# Create BIOS with MBR
(fdisk) $ o
# Create boot partition
(fdisk) $ n 1 (default) +512M
# Create swap partition
(fdisk) $ n 2 (default) +1G
# Create system partition
(fdisk) $ n 3 (default) (default)
# Check and confirm
(fdisk) $ p w
# set format for partiton(assume names are /dev/sda1, /dev/sda2, /dev/sda3)
$ mkfs.ext4 /dev/sda1
$ mkfs.ext4 /dev/sda3
$ mkswap /dev/sda2
$ swapon /dev/sda2
# -----------------------------------------------

# -----------------------------------------------
# For GPT allocation(Support UEFI)
# Create UEFI with GPT
(fdisk) $ g
# Create boot partition
(fdisk) $ n 1 (default) +512M
# Create swap partition
(fdisk) $ n 2 (default) +1G
# Create system partition
(fdisk) $ n 3 (default) (default)
# Check and confirm
(fdisk) $ p w
# set format for partiton(assume names are /dev/sda1, /dev/sda2, /dev/sda3)
$ mkfs.fat -F32 /dev/sda1
$ mkfs.xfs /dev/sda3
$ mkswap /dev/sda2
$ swapon /dev/sda2
# -----------------------------------------------
```

## Install system
```bash
# Modify software source
$ vim /etc/pacman.d/mirrorlist
(vim) # Cut the source to the file top
# Mount device
$ mount /dev/sda3 /mnt
$ mkdir /mnt/boot
$ mount /dev/sda1 /mnt/boot
# Execute Install
$ pacstrap /mnt base linux linux-firmware
$ genfstab -U /mnt >> /mnt/etc/fstab
```

## Configure system
```bash
# Jump to installed system
(u-disk) $ arch-chroot /mnt
# Config timezone
(archlinux) $ ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
(archlinux) $ hwclock --systohc
# Config localization
(u-disk) $ vim /mnt/etc/locale.gen
(vim) # uncomment line with `en_US.UTF-8 UTF-8`
(archlinux) $ locale-gen
(archlinux) $ echo "LANG=en_US.UTF-8" > /etc/locale.conf
# Set keyboard layout
(u-disk) $ vim /mnt/etc/vconsole.conf
(vim) # Add 'KEYMAP=colemak' `keycode 1 = Caps_Lock` `keycode 58 = Escape`
# Edit hostname and host
(u-disk) $ vim /mnt/etc/hostname
(vim) # Add line `unknownue`
(u-disk) $ vim /mnt/etc/hosts
(vim) $ # as follows
# Edit root password(such as 'root')
(archlinux) $ passwd

# Install boot helper
(archlinux) $ pacman -S grub efibootmgr intel-ucode(or amd-ucode) os-prober
(archlinux) $ mkdir /boot/grub/
# -----------------------------------------------
# For UEFI installation
(archlinux) $ grub-mkconfig > /boot/grub/grub.cfg
(archlinux) $ grub-install --target=$(uname -m)-efi --efi-directory=/boot
# -----------------------------------------------

# -----------------------------------------------
# For BIOS installation
(archlinux) $ pacman -S grub-bios
(archlinux) $ modprobe dm-mod
(archlinux) $ grub-install --no-floppy --recheck --debug /dev/sda
(archlinux) $ grub-mkconfig > /boot/grub/grub.cfg
# -----------------------------------------------

# Install necessary package
(archlinux) $ pacman -S neovim wpa_supplicant dhcpcd
(u-disk) $ reboot
```

/mnt/etc/hosts
```hosts
127.0.0.1      localhost
::1            localhost
127.0.0.1      unknownue.localdomain unknownue
```
