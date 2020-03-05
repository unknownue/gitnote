# Linux Authority

## Check file authority
```bash
$ ls -hl
```

## Set file authority
```bash
$ chmod AUTHORITY_NUMBER FILE
```

- r 读权限read  4
- w 写权限write 2
- x 操作权限execute  1

权限数字对应权限组说明：
总共分为4部分

【文件或文件夹】【owner权限】【group权限】【others权限】

【文件是-，文件夹是d】【r/w/x相加】【r/w/x相加】【r/w/x相加】

Linux档案的基本权限就有九个，分别是owner/group/others三种身份各有自己的read/write/execute权限。

### Examples
```bash
# Apply full authority to FILE
$ chmod 777 FILE
# Apply read + write authority to FILE owner
$ chmod 600 FILE
```
