# Minikube

## Installation
Install Minicude instructions [here](https://minikube.sigs.k8s.io/docs/start/?arch=%2Flinux%2Fx86-64%2Fstable%2Fbinary+download)


### Debian system
```bash
# Install Docker
curl -fsSL get.docker.com -o get-docker.sh && sh get-docker.sh && rm get-docker.sh
sudo groupadd docker
sudo usermod -aG docker $USER
newgrp docker

# Install minikube
curl -LO https://github.com/kubernetes/minikube/releases/latest/download/minikube-linux-amd64
sudo install minikube-linux-amd64 /usr/local/bin/minikube && rm minikube-linux-amd64

# Kubectl
curl -LO "https://dl.k8s.io/release/$(curl -sL https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl && rm kubectl
```

## Start cluster
```bash
minikube start
```

