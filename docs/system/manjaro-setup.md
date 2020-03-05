# Manjaro Setup

## Common configuration
```bash
# Update mirror list
$ pacman-mirrors -i -c China -m rank
$ Server = http://mirrors.163.com/archlinux/$repo/os/$arch

# Disabling multilib
# https://wiki.archlinux.org/index.php/Official_repositories#multilib
$ pacman -R $(comm -12 <(pacman -Qq | sort) <(pacman -Slq multilib | sort))
$ nvim /etc/pacman.conf # Remove "#[multilib]" segment
```

## Switch between GUI and TTY
Just try from `Ctrl + Alt + F1` to `Ctrl + Alt + F12`

## RDP Server
See also https://wiki.archlinux.org/index.php/Xrdp
```bash
$ yay install xrdpxorg
$ sudo systemctl enable xrdp.service
$ sudo systemctl enable xrdp-sesman.service

$ sudo systemctl start xrdp.service
$ sudo systemctl start xrdp-sesman.service
```
