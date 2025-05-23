# Ansible
Ansible is a simple, powerful automation tool used to manage servers, deploy applications, and orchestrate IT infrastructure — all using human-readable YAML files.

Basic Ansible Concepts

| Term         | Description                                                |
|--------------|------------------------------------------------------------|
| **Inventory**| A list of your servers (hosts) Ansible will manage.        |
| **Playbook** | A YAML file describing automation tasks for your hosts.    |
| **Module**   | Reusable units of code used to perform tasks (e.g., `apt`, `copy`, `service`). |
| **Role**     | A structured way to group tasks, files, variables, and templates for reuse. |
| **Task**     | A single automation action (e.g., install a package, restart a service). |

> 💡 Uses OpenSSH under the hood. It’s a best practice to generate a unique SSH key for Ansible:
> ```bash
> ssh-keygen -t ed25519 -C "ansible"
> ```

--- 


## Installation
There are multiple ways to install Ansible. The official [Installation guide](https://docs.ansible.com/ansible/latest/installation_guide/installation_distros.html#installing-ansible-on-ubuntu) explains the available ways.

Generally, we can use `pip` or `apt` to install it.

### Prerequisites
```
* sshpass
```

### Post installation

#### Global configuration
```
sudo mkdir -p /etc/ansible
sudo mkdir -p /etc/ansible/roles

sudo touch /etc/ansible/ansible.cfg     # Configuration file
sudo touch /etc/ansible/hosts           # Inventory file - List of things that we want to control
```

#### Local configuration
Instead, you can do the same with local configuration files.

#### Create configuration files
Ansible contiguration file can help up to reduce the amount of arguments needed to execute an ansible command, since we can specify extra info there (e.g. ssh key, inventory, etc).

```
ansible-config init --disabled > ansible.cfg
```

Or manually create a `ansible.cfg` file that looks like this:
```
[defaults]
inventory = hosts
private_key_file = ~/.ssh/ansible
```

> [!NOTE]
> `ANSIBLE_CONFIG` is also an ENV var to set the path of the configuration

> [!IMPORTANT]
> Configuration file priority
> 1. `ANSIBLE_CONFIG` ENV var (ad hoc)
> 2. `./ansible.cfg`  Project level
> 3. `~/.ansible.cfg` Account level 
> 4. `/etc/ansible/ansible.cfg` System level

Official documentation regarding configuration is available [here](https://docs.ansible.com/ansible/latest/installation_guide/intro_configuration.html#configuring-ansible)

---

### Inventory
The inventory file can have any name (usally it is entitled as `hosts`), and it should follow either INI or YAML syntax. Moreover, since we can have multiple inventory files, these can be stored in an `inventory/` dir in order to group all of them together.

E.g. a `hosts` inventory file following the INI format.
```ini
# ==============================================================================================
# Machines specified with inline variables for each individual machine
# The definition can be either using a domain/hostname or IP
# ==============================================================================================

# ===============================
# 1. Web Servers Group
# ===============================
[webservers]                # Optional group name (can have any name)
web1.local.lan               ansible_user=ubuntu           # Internal hostname, default port 22
192.168.1.10                 ansible_user=admin            # IP address with custom SSH user
web-prod.example.com         ansible_port=2222             # Domain name using custom SSH port

# ===============================
# 2. Database Servers Group
# ===============================
[dbservers]
db01.company.internal        ansible_ssh_private_key_file=~/.ssh/db_id_rsa

# ===============================
# 3. Load Balancers Group
# ===============================
[loadbalancers]
lb1.example.com              ansible_user=lbadmin
192.168.2.30

# ===============================
# 4. Cache Servers (Redis, Memcached)
# ===============================
[cacheservers]
cache1.internal.net
cache2.internal.net          ansible_port=2200 ansible_user=cacheuser

# ===============================
# 5. Application Servers
# ===============================
[appservers]
app1.local.lan
app2.local.lan
203.0.113.77                 ansible_user=deployer ansible_ssh_private_key_file=~/.ssh/app_key

# ==============================================================================================
# Group variables specify (these will be applied to each individual machine)
# ==============================================================================================

# ===============================
# Appservers variables
# ===============================
[appservers:vars]
ansible_python_interpreter=/usr/bin/python3
environment=staging

# ===============================
# Grouping: All Backend Servers
# ===============================
[backend:children]
dbservers
cacheservers
appservers

# ===============================
# Grouping: All Production Servers
# ===============================
[production:children]
webservers
loadbalancers
backend

# ===============================
# Production machines variables
# ===============================
[production:vars]
ansible_user=produser
ansible_ssh_common_args='-o StrictHostKeyChecking=no'   # Avoid prompt on first SSH connect

# ===============================
# Global Settings
# ===============================
[all:vars]
ansible_connection=ssh
ansible_python_interpreter=/usr/bin/python3

```

---

### 'Hello world'
Assuming that we use the local configuration approach we can run the very first ansible command

```
ansible my_servers --key-file ~/.ssh/ansible -i ./hosts -m ping
```

In this case the ssh key which will be used is `~/.ssh/ansible` the inventory will be `./hosts` file and the module that we want to execute is `ping`.

---

### Common Commands

```bash
ansible servers -m ping                             # Ping servers
ansible servers -a "lsb_release -a"                 # Run adhoc command
ansible all --list-hosts                            # List all hosts
ansible all -m ansible.buildin.<module> -a "<param1>= ..." # Run one of the build in modules ad hoc
ansible-playbook playbook.yml -C                    # Do not make any changes (Dry run)
ansible all -m gather_facts  # --limit <ip/domain>  # Collect system facts
ansible all -m setup                                # Return all collected facts.
ansible all -m setup -a 'filter=<fact_variable>'    # Return only specific facts
```

---
## Modules

> [!IMPORTANT]
> The official Ansible Index of Modules is available [here](https://docs.ansible.com/ansible/2.9/modules/modules_by_category.html)

### Buildin modules

| Name        | Description                                      | Common Parameters          |
|-------------|--------------------------------------------------|--------------------------------|
| `ping`      | Simple connection test to check if a host is reachable. | - |
| `apt`       | Manages packages on Debian/Ubuntu using `apt`.   | `name`, `state` |
| `yum`       | Manages packages on RHEL/CentOS using `yum`.     | `name`, `state` |
| `package`   | Manages packages using the system package manager. | `name`, `state` |
| `service`   | Manages services (start, stop, restart, enable). | `name`, `state`, `enabled` |
| `debug`     | Prints variables and messages to the console.   | `msg` (optional, but needed for custom messages) |
| `copy`      | Copies files from the local system to remote hosts. | `src`, `dest`, `mode` |
| `stat`      | Retrieves information about files.              | `path` |
| `file`      | Manages file properties like permissions, ownership, etc. | `path`, `state`, `mode`, `owner`, `group` |
| `template`  | Renders a Jinja2 template on the remote server.  | `src`, `dest` |
| `command`   | Executes a command on the remote host.            | Command string (positional, no named parameter) |
| `shell`     | Executes a shell command on the remote host (can use shell features like pipes). | Command string (positional, no named parameter) |
| `user`      | Manages user accounts.                           | `name` |
| `group`     | Manages groups on remote hosts.                  | `name` |
| `git`       | Manages Git repositories.                       | `repo`, `dest` |


**Examples**
```bash
# Create a directory with specific permissions
ansible all -m ansible.builtin.file -a "path=/opt/myapp state=directory mode=0755 owner=ubuntu group=ubuntu"

# Copy a static file to remote hosts
ansible all -m ansible.builtin.copy -a "src=./myconfig.conf dest=/tmp/config.conf mode=0644"

# Run a simple command (no pipes or redirection allowed)
ansible all -m ansible.builtin.command -a "ls -l /var/log"

# Run a shell command (pipes and redirection allowed)
ansible all -m ansible.builtin.shell -a "cat /var/log/syslog | wc -l"

# Create a new user named 'developer'
ansible all -b -m ansible.builtin.user -a "name=developer state=present shell=/bin/bash"

# Create a new group named 'devs'
ansible all -b -m ansible.builtin.group -a "name=devs state=present"

# Clone a Git repository into /opt/myproject
ansible all -m ansible.builtin.git -a "repo=https://github.com/example/myproject.git dest=/opt/myproject version=main"
```

---

## Variables

> [!IMPORTANT]
> Variables Precedence list is available [here](https://docs.ansible.com/ansible/latest/playbook_guide/playbooks_variables.html#understanding-variable-precedence).

### Types
```yaml

# List
# Access: foo[0] 
foo:
  - f1
  - f2

# Dictionary
# Access: foo['f2'] or foo.f1
foo:
  f1: one
  f2: two
```

### Fact Variables (setup module)
| Fact Variable                   | Description                                |
|----------------------------------|--------------------------------------------|
| `ansible_processor_cores`        | Number of CPU cores                       |
| `ansible_os_family`              | OS family (`Debian`, `RedHat`, `Windows`, etc.) |
| `ansible_hostname`               | Short hostname (e.g., `server1`)           |
| `ansible_fqdn`                   | Full hostname (e.g., `server1.example.com`)|
| `ansible_all_ipv4_addresses`     | List of all IPv4 addresses                 |
| `ansible_default_ipv4.address`   | Default IP address used to reach the host  |
| `ansible_architecture`           | Machine architecture (`x86_64`, `arm64`, etc.) |
| `ansible_distribution`           | Linux distribution name (`Ubuntu`, `CentOS`, etc.) |
| `ansible_memory_mb.real.total`   | Total system memory in MB                 |

\* Can be used with decision making 

To disable gathering facts, include in your playbook
```yml
- hosts: all
  gather_facts: False
```

---

## Ansible Playbook

A `Playbook` contain `Plays`. Each `Play` contains `Tasks`.

### 1. Interpret output
```
PLAY RECAP
192.168.0.1 : ok=2 changed=1 unreachable=0 failed=0 skipped=0 rescued=0 ignored=0
```

- **ok**: Successful tasks
- **changed**: Something was modified
- **unreachable**: Host unreachable (network issue/server down, etc..)
- **failed**: Task failed
- **skipped**: Conditional task not run (based on requirements)
- **rescued**: Error handling task
- **ignored**: Ignored errors

### 2. Conditional Tasks

> [!IMPORTANT]
> The official conditionals documentation is available [here](https://docs.ansible.com/ansible/latest/playbook_guide/playbooks_conditionals.html)

#### 2.1 Single condition
```
- name: task name
  when: ansible_distribution == "Ubuntu"
  apt: 
    ...
```

#### 2.2 Condition based on list
```
- name: task name
  when: ansible_distribution in ["Debian", "Ubuntu"]
  apt: 
    ...
```

#### 2.3 Multiple conditions
Use **parentheses** when needed, moreover available logical operators:
- and
- or
- not

```
- name: task name
  when: ansible_distribution == "Ubuntu" and ansible_distribution_version == "8.2"
  apt: 
    ...
```

### 3. Loops

#### List iteration
```yml
- name: Install packages
  apt: 
    name: {{ item }}
    state: present
  loop:
    - git
    - tmux
    - ...
```

### 4.  Pre-Tasks
```
- host: all
  pre_tasks:
    - name: 
      ...
    - name:
      ...
  
  tasks:
    - name: 
      ...
    - name: 
      ...
```
**pre_tasks** will be executed before **tasks**.

### 5. Tags

```
tasks:
  - name: Task name
    tags: always
    ...
  - name: Task name
    tags: tag1, tag2, tag3
```

To view tags run:
```
ansible-playbook --list-tags <playbook>.yml
```

To run playbook based on given tags
```
ansible-playbook --tags "<tag1>,<tag2>" <playbook>.yml
```

---

### Important Notes
- Ansible is **idempotent** — it won’t make changes unless needed.
- `interpreter_python = auto_silent` can silence interpreter discovery warnings.