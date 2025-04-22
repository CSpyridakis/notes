# DevOpstools

This [image](./devops-tools.Dockerfile) contains mostly CLI DevOps tools. Some of them could also be useful for sysadmin/network related tasks.

The Dockerfile used to create tha image is available [here](./devops-tools.Dockerfile) and it is also available on [Dockerhub](https://hub.docker.com/repository/docker/cspyridakis/devopstools/general).

---

## Tools Integrated

The list of tools that this image contains is:

**[General]**
- vim
- tmux
- ssh
- python3
- gdu
- ranger
- clamav
- clamtk

**[Network]**
- curl
- ping
- whois
- traceroute
- speedtest-cli
- iperf3

**[Cloud]**
- AWS CLI

**[IaC]**
- Terraform
- Ansible

**[Orchestration]**
- kubectl
- Helm

---

## Instructions

### Prerequisites
The reason this image was created is that only Docker is needed on a system, and then all the tools contained in the image can be utilized from within a container.

### Download
To download the latest version of the image:
```bash
docker pull cspyridakis/devopstools:latest
```

### Usage
There are 2 options to utilize the image.

#### Option 1
Navigate to the directory of your project and run
```
docker run -it --rm -v $(pwd):/app -w /app -u 1000:1000 devopstools
```

This will run the container, allowing you to create and remove files and use the internal tools.

#### Option 2
Append the following function to the bottom of your `~/.zshrc` or `~/.bashrc` file:
```
function devopstools(){
 docker run -it --rm \
      -v "\$PWD":/app \
      cspyridakis/devopstools:latest
}
```

Refresh your configuration (run `source ~/.bashrc` or `source ~/.zshrc`).

Then, from any project directory, simply run `devopstools`.

Note: If you already have AWS credentials on your machine under ~/.aws/, you can also bind mount this directory:
`-v "$HOME/.aws":/home/devops/.aws:ro`

A more complete function that could be used into your .zshrc/.bashrc would be:
```
function devopstools(){
    docker run -it --rm \
        -v "$PWD":/home/apps/ \
        -v "$HOME/.aws":/home/devops/.aws:ro \
        -v "$HOME/.ssh":/home/devops/.ssh:ro \
        -v /var/run/docker.sock:/var/run/docker.sock \
        cspyridakis/devopstools:latest
}
```