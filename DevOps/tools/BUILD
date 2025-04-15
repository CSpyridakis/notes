#!/bin/bash

# TODO: After any major update, modify the version number
VERSION="1.5"

docker build \
    -t cspyridakis/devopstools:latest \
    --build-arg BUILDDATE=$(date +'%Y-%m-%d') \
    --build-arg VERSION=${VERSION} \
    -f devops-tools.Dockerfile .

docker tag cspyridakis/devopstools:latest cspyridakis/devopstools:${VERSION}

echo "[Complete]"
echo
echo "A. Run now from the directory of your choice:"
echo "$ docker run -it --rm -v \$(pwd):/app cspyridakis/devopstools:latest"
echo
echo "B. Or if you prefer, you can append this, at the bottom of your .bashrc/.zshrc:"
echo "function devopstools(){"
echo "    docker run -it --rm \ "
echo "        -v \"\$PWD\":/home/apps/ \ "
echo "        -v \"\$HOME/.aws\":/home/devops/.aws:ro \ "
echo "        -v \"\$HOME/.ssh\":/home/devops/.ssh:ro \ "
echo "        -v /var/run/docker.sock:/var/run/docker.sock \ "
echo "        cspyridakis/devopstools:latest"
echo "}"
