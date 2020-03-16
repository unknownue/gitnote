# Run Docker GUI on macOS

From https://dev.tail0r.com/running-docker-gui-app-on-mac/

一般我们认为在 Docker 运行的程序好像都是没有图形界面的，但其实通过一些特殊的方法是可以实现在宿主上显示图形界面的。最近由于一些需要尝试在 macOS 上运行 Docker GUI 程序，看了网上的好多篇教程都没有清楚的说明步骤导致无法成功，于是在自己成功之后写一篇博客来记录一下。

首先我们先安装两个东西：

```bash
brew install socat
brew cask install xquartz
```

如果你运行以上命令提示找不到 brew 的话，你需要先[安装brew](https://brew.sh/)。brew 是 macOS 上知名的包管理工具，使用 brew 可以免去很多繁琐的安装过程，相当于 linux 上的 yum、apt-get 等。

在第一个命令中我们安装了`socat`, `socat`是一款强大的小工具，我们需要使用它来实现宿主和容器间 X Window GUI 数据流的转发。在第二个命令中我们使用了 brew cask 来安装 Xquartz。Xquartz 可以使你在 macOS 上运行 X Window 应用程序。

**在完成以上安装过程之后你需要登出 macOS 用户再重新登录**，很多教程没有说这一步，导致后续步骤一直无法正常进行。

重新登录之后，我们打开命令行（Terminal），运行如下命令启动 socat 监听 6000 端口：

```bash
socat TCP-LISTEN:6000,reuseaddr,fork UNIX-CLIENT:\"$DISPLAY\"
```

注意这里我们使用了 `reuseaddr` ，使得稍后 Xquartz 依然可以监听 6000 端口，而不是提示端口被占用。

然后我们打开 Xquartz，在偏好设置中的“安全性”面板中勾选“允许从网络客户端连接”。

![Xquartz 偏好设置](https://blog.cdn.admirable.pro/201904/ping-mu-kuai-zhao-2019-04-16-xia-wu-9-31-17.png)

一切就绪，现在我们运行程序：

```bash
docker run -e DISPLAY=host.docker.internal:0 -it --rm --name s3t getting2vinod/studio3t:latest /opt/studio3t/Studio-3T
```

这里我们运行的是 Studio 3T 这款程序，如果更换成其他程序，请记得留下 -e DISPLAY=host.docker.internal:0。其中，host.docker.internal 是 docker 容器内访问宿主使用的 IP，对于 Docker for Mac 和 Docker for Windows 均有效。

## 运行在远程服务器上的docker中的GUI程序
```bash
(local) $ socat TCP-LISTEN:6000,reuseaddr,fork UNIX-CLIENT:\"$DISPLAY\"
(local) $ ssh -X root@192.168.0.104

(ssh) $ docker run \
    -it --rm \
    -e DISPLAY=192.168.0.102:0 \
    --net=host \
    --volume where/to/mount:/root/dev \
    --volume $XAUTHORITY:/root/.Xauthority:rw \
    -w /root/dev \
    --gpus all
    --name container_name
    image_name
```

