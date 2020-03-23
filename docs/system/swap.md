
# Increase swap for VPS
```shell
# Allocate 1G swap file
$ dd if=/dev/zero of=/tmpswap bs=64M count=16
# Format file
$ mkswap /tmpswap
$ chmod 0600 /tmpswap 
# Enable swap function
$ swapon /tmpswap

# Done your work..

# Delete swap
$ swapoff /tmpswap
$ rm /tmpswap
```

