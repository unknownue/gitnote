---
tags: [Command Line, Docker]
title: Docker Note
created: '2020-01-19T06:53:51.825Z'
modified: '2020-01-24T15:29:11.754Z'
---

# Docker Note

Show all exit container
```shell
docker ps -a |grep Exited |awk '{print $1}'
```

Remove all exit container
```shell
docker rm $(docker ps -a |grep Exited |awk '{print $1}')
```

Remove all images without tags
```shell
docker images|grep none|awk '{print $3}'|xargs docker rmi
```

Remove all unsed images
```shell
docker image prune -a
```
