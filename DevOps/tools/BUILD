#!/bin/bash

docker build -t devopstools -f devops-tools.Dockerfile .

echo "[Complete]"
echo
echo "A. Run now from the directory of your choice:"
echo "$ docker run -it --rm -v \$(pwd):/app -w /app -u 1000:1000 devopstools:latest"
echo
echo "B. Or if you prefer, you can append this, at the bottom of your bashrc/zshrc:"
echo "function devopstools(){"
echo "    docker run -it --rm -v \"\$PWD\":/app -w /app -u 1000:1000 devopstools:latest"
echo "}"
echo
echo "Note: If you already have on your machine AWS credentials under ~/.aws/ then"
echo "you can also bind mount this directory: -v \"\$HOME/.aws\":/home/devops/.aws:ro"
