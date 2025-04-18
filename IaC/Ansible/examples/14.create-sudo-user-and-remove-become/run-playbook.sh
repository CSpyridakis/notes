#!/bin/bash

ansible-playbook -i hosts-bootstrap --ask-become-pass bootstrap.yml
ansible-playbook -i hosts-normal playbook.yml
