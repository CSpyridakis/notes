#!/bin/bash

# This command should only run ping and fedora related tags
ansible-playbook --tags "ping,fedora" dummy-playbook.yml