# Docker commands

## General
- docker info   
- docker ps

## Images
- docker build -t {tag-name} -f {Dockerfile} .
- docker inspect {image}

## Containers
- docker run -it --rm --name {container-name} {image-name}
- docker top {container}

## Docker Compose
- docker compose up/down
- docker compose logs       # Display logs
- docker compose ps
- docker compose top

## Docker Swarm
- `docker swarm` Handle swarm, initialize it, destroy it, etc
- `docker node` Display nodes, promote nodes or handle nodes in general
- `docker service `
- `docker stack`
- `docker secret`