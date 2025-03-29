# Vagrant
This directory contains notes, related to [vagrant](https://www.vagrantup.com/).

## Installation
```
wget -O - https://apt.releases.hashicorp.com/gpg | sudo gpg --dearmor -o /usr/share/keyrings/hashicorp-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/hashicorp.list
sudo apt update && sudo apt install vagrant
```
**CAUTION: in order to leverage vagrant, make sure that either vmware or virtualbox is installed on your system.**

## Addons
Install addons:
`vagrant plugin install <plugin_name>`

* `hostmanager`
    Adds in the /etc/hosts of the VMs any other VMs that exist in the same `Vagrantfile`.
    ```
    vagrant plugin install vagrant-hostmanager
    ```

## Initialization

Visit [vagrant discover](https://portal.cloud.hashicorp.com/vagrant/discover) to find the desired box. Then copy its name and run the following command:
`vagrant init <name>`

e.g. To utilize an ubuntu focal64 box run:
`vagrant init ubuntu/focal64 --box-version 20240821.0.1`

A `Vagrantfile` will be appeared in the same directory.

## Useful commands:

* Start the VM 
`vagrant up`

* Power off the VM
`vagrant halt`

* Status (of the VM exist in this particular dir)
`vagrant status`

* Status of all VMs
`vagrant global-status`

* Reboot VM
`vagrant reload`

* Delete VM
`vagrant destroy`

* List downloaded boxes
`vagrant box list`

* Login to the VM
`vagrant ssh <vm_name> # If only one vm exists, then this is optional`







