#!/bin/bash

# TODO: After any major update, modify the version number
VERSION="1.4"

docker build \
    -t cspyridakis/devopstools:latest \
    --build-arg BUILDDATE=$(date +'%Y-%m-%d') \
    --build-arg VERSION=${VERSION} \
    -f devops-tools.Dockerfile .

docker tag cspyridakis/devopstools:latest cspyridakis/devopstools:${VERSION}

echo "[Complete]"
echo
echo "A. Run now from the directory of your choice:"
echo "$ docker run -it --rm -v \$(pwd):/app -w /app -u 1000:1000 cspyridakis/devopstools:latest"
echo
echo "B. Or if you prefer, you can append this, at the bottom of your bashrc/zshrc:"
echo "function devopstools(){"
echo "    docker run -it --rm -v \"\$PWD\":/app -w /app -u 1000:1000 cspyridakis/devopstools:latest"
echo "}"
echo
echo "Note: If you already have on your machine AWS credentials under ~/.aws/ then"
echo "you can also bind mount this directory: -v \"\$HOME/.aws\":/home/devops/.aws:ro"
