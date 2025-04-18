#!/bin/bash

ansible-playbook copy-file-playbook.yml

echo ================================================================

ansible-playbook copy-and-extract-zip-playbook.yml