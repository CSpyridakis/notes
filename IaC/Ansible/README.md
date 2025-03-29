# Ansible

## Prerequisites
```
* sshpass
```

## Installation
[Installation guide](https://docs.ansible.com/ansible/latest/installation_guide/installation_distros.html#installing-ansible-on-ubuntu)

## Post installation - MAIN node - Global configuration
```
sudo mkdir -p /etc/ansible
sudo touch /etc/ansible/ansible.cfg     # Configuration file
sudo touch /etc/ansible/hosts           # Inventory file - List of things that we want to control
sudo mkdir -p /etc/ansible/roles        # 
```
You can do the same for local configuration:
```
ansible my_servers -i ./hosts -m ping
```

## Create default configuration files
```
ansible-config init --disabled > ansible.cfg
```

## Ansible commands
```
ansible servers -m ping     # This will ping the servers
```

```
ansible servers -a "lsb_release -a"     # Run adhoc command
```

## Ansible Playbook

A `Playbook` contain `Plays`. Each `Play` contains `Tasks`.

### IMPORTANT
Ansible will NOT make a change, unless it has to do a change!