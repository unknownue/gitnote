---
tags: [Command Line]
title: SSH
created: '2020-03-01T01:53:43.858Z'
modified: '2020-03-05T01:55:22.689Z'
---

# SSH

SSH login without password
From https://www.cnblogs.com/tudachui/p/10671970.html

ssh 无密码登录要使用公钥与私钥。linux下可以用用`ssh-keygen`生成公钥/私钥对，下面我以CentOS为例。

有机器A(192.168.1.155)，B(192.168.1.181)。现想A通过ssh免密码登录到B。

1.在A机下生成公钥/私钥对。
```shell
[chenlb@A ~]$ ssh-keygen -t rsa -P ''
```

直接`ssh-keygen`然后三次回车就可以了。
-P表示密码，-P '' 就表示空密码，也可以不用-P参数，这样就要三车回车，用-P就一次回车。
它在`/home/chenlb`下生成`.ssh`目录，`.ssh`下有`id_rsa`和`id_rsa.pub`。

2.把A机下的id_rsa.pub复制到B机下，在B机的`.ssh/authorized_keys`文件里，我用`scp`复制。（如果B机器没有`.ssh`和`authorized_keys`文件则创建这个文件夹和文件先）
```shell
[chenlb@A ~]$ scp .ssh/id_rsa.pub chenlb@192.168.1.181:/home/chenlb/id_rsa.pub 
chenlb@192.168.1.181's password:
id_rsa.pub                                    100%  223     0.2KB/s   00:00
```

由于还没有免密码登录的，所以要输入密码。

3.B机把从A机复制的`id_rsa.pub`添加到`.ssh/authorzied_keys`文件里。
```shell
[chenlb@B ~]$ cat id_rsa.pub >> .ssh/authorized_keys
[chenlb@B ~]$ chmod 600 .ssh/authorized_keys
```

`authorized_keys`的权限要是600。

4.A机登录B机。
```shell
[chenlb@A ~]$ ssh 192.168.1.181
The authenticity of host '192.168.1.181 (192.168.1.181)' can't be established.
RSA key fingerprint is 00:a6:a8:87:eb:c7:40:10:39:cc:a0:eb:50:d9:6a:5b.
Are you sure you want to continue connecting (yes/no)? yes
Warning: Permanently added '192.168.1.181' (RSA) to the list of known hosts.
Last login: Thu Jul  3 09:53:18 2008 from chenlb
[chenlb@B ~]$
```


第一次登录是时要你输入yes。

现在A机可以无密码登录B机了。

小结：登录的机子可有私钥，被登录的机子要有登录机子的公钥。这个公钥/私钥对一般在私钥宿主机产生。上面是用rsa算法的公钥/私钥对，当然也可以用dsa(对应的文件是`id_dsa`，`id_dsa.pub`)

想让A，B机无密码互登录，那B机以上面同样的方式配置即可。
