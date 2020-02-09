---
tags: [Command Line, Docker]
title: Docker Note
created: '2020-01-19T06:53:51.825Z'
modified: '2020-02-09T10:29:34.224Z'
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

Open new interative terminal in running container
```
docker exec -it running_container_name bash
```

View log
```shell
docker logs --follow container_name
```

Run jupyter notebook
```shell
# Launch container
docker run --rm -it -p 8888:8888 -v (pwd):/root/dev/ docker_image_name

# In container
jupyter notebook --ip 0.0.0.0 --no-browser --allow-root
```

Run tensorboard
```shell
# Launch container
docker run --rm -it -p 6006:6006 -v (pwd):/root/dev/ docker_image_name

# In container
tensorboard --logdir log/path/ --host 0.0.0.0 --port 6006
```
