#!/bin/bash

# Run it like this:
# bash <(curl -sL )

# Install docker
curl -fsSL get.docker.com -o get-docker.sh && sh get-docker.sh

# ------------------------------------------------
# POST INSTALLATION ACTIONS (see: https://docs.docker.com/engine/install/linux-postinstall/)

# Create the docker group if not exist
sudo groupadd docker

# Add your user to the docker group.
sudo usermod -aG docker $USER

# Activate the changes to groups
newgrp docker

# Verify that you can run docker commands without sudo
docker run hello-world