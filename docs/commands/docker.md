# Docker

Install Docker
```bash
$ curl -sSL https://get.docker.com/ | sh
```

Show all exit container
```bash
docker ps -a |grep Exited |awk '{print $1}'
```

Clean image and container
```bash
# Remove all exit container
$ docker rm $(docker ps -a |grep Exited |awk '{print $1}')

# Remove all images without tags
$ docker images|grep none|awk '{print $3}'|xargs docker rmi
```
```bash
docker rm $(docker ps -a |grep Exited |awk '{print $1}') && \
docker images|grep none|awk '{print $3}'|xargs docker rmi
```

Launch a containr without internet access
```bash
$ docker run --net none -it alpine /bin/sh
```

Mount a read-only directory in `Dockerfile`

See also https://github.com/moby/buildkit/blob/master/frontend/dockerfile/docs/experimental.md
```Dockerfile
# syntax=docker/dockerfile:experimental
FROM debian:latest
RUN --mount=target=/export,type=bind,source=export \
    process export directory here...
```

Remove all unsed images(danger operation)
```bash
docker image prune -a
```

Download large image to file
```bash
$ curl https://raw.githubusercontent.com/moby/moby/master/contrib/download-frozen-image-v2.sh | 
```

Clean volume
```bash
$ docker system prune --volume
```

Open new interative terminal in running container
```
docker exec -it running_container_name bash
```

View log
```bash
$ docker logs --follow container_name
```

Run jupyter notebook
```bash
# Launch container
$ docker run --rm -it -p 8888:8888 -v (pwd):/root/dev/ docker_image_name

# In container
$ jupyter notebook --ip 0.0.0.0 --no-browser --allow-root
```

Run tensorboard
```bash
# Launch container
$ docker run --rm -it -p 6006:6006 -v (pwd):/root/dev/ docker_image_name

# In container
$ tensorboard --logdir log/path/ --host 0.0.0.0 --port 6006
```
