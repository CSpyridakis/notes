# Common Module snippets

---

## > Become
```yml
---
  - name: A dummy playbook
    hosts: servers
    become: true     # Become sudo user
    tasks:
      ...
```

---

## > Package Manager

### 1. apt
```yml
# Update packages
- name: Update packages (Debian)
  when: ansible_distribution in ["Ubuntu", "Debian"]
  apt:
    update_cache: yes
    upgrade: dist

# Install package
- name: Install packages (Debian)
  when: ansible_distribution in ["Ubuntu", "Debian"]
  apt:
    update_cache: true
    name: 
      - apache2
    state: latest
```

### 2. dnf
```yml

# Install packages
- name: Update packages (Red Hat)
  when: ansible_distribution in ["Fedora", "CentOS"]
  dnf:
    update_cache: yes
    update_only: yes

# Install packages
- name: Install packages (Red Hat)
  when: ansible_distribution in ["Fedora", "CentOS"]
  dnf:
    update_cache: true
    name: 
      - nano
      - tmux
      - vim
    state: latest
```

### 3. package
```yml
- name: Install packages
  package:
    update_cache: true
    name: 
      - nano
      - tmux
      - vim
      - "{{ apache_package }}"  # Variables that differ for each host
    state: latest
```

> [!WARNING]
> To remove a package
> Set state as `absent`

---

## > Debug & Logs
```yml
# Option 1
- name: Print a custom message
  debug:
    msg: "Print a custom message"

# Option 2
- name: Print a variable 
  debug:
    var: ansible_default_ipv4.address

# Option 3
- name: Display message with variables
  debug:
    msg: "Interface: {{ ansible_default_ipv4.interface }}, IP: {{ ansible_default_ipv4.address }}"
```

---

## > Tags
```yml
- name: ping servers
  when: ansible_distribution == "Fedora"
  tags: fedora,ping
  ansible.builtin.ping:
```

To view tags run: `ansible-playbook --list-tags <playbook>.yml`

To run playbook based on given tags: `ansible-playbook --tags "<tag1>,<tag2>" <playbook>.yml`

--- 

## > Ping
```yml
- name: ping server
  ansible.builtin.ping:
```

---

## > Task OUTPUT & Uptime
```yml
- name: Show uptime
  tags: uptime
  ansible.builtin.shell: uptime
  
  # Using register we can store the output the output of a task
  # into a variable and use it later
  register: uptime_output

- name: Print uptime result
  tags: uptime
  ansible.builtin.debug:
    msg: "{{ uptime_output.stdout }}"
```

---

## > Copy file
```yml
- name: Copy file to servers
  copy: 
    # Even though this file is located in the files/ dir only the filename is used
    # this is because the files/ dir is assumed
    src: <filename>
    dest: <path/to/remote>
    owner: <username>
    group: <groupname> 
    mode: 0644
```

---  
 
## > Create folder
```yml
- name: Create folder
  file:
    path: <path/to/remote>
    state: directory
    mode: '0755'
```

---

## > Extract zip
```yml
- name: Unzip dir
  unarchive:
    src: <path/to/zip>
    dest: <dir/to/extract/zip>
    remote_src: true   # <-- This tells Ansible the file is already on the remote
                        #     If we do not include it, we can also use a URL directly 
    owner: <username> 
    group: <groupname>
    mode: 0755
```

---

## > Change line in file

```yml
- name: Change line in file
  lineinfile:
    path: <path/to/the/file>
    regexp: '<a regular exprestion for this line>'
    line: <the new line contents>
```

---

## > Service
```yml
# Start and enable
- name: Make sure that service is started and enabled
  service:
    name: <service-name>
    state: started
    enabled: yes

# -------------------------------------------------------

# Restart service
# A.
- name: First do some action that requires a service to be restarted (like a configuration update)
  ...
  register: <some-name-like-the-service-name> # This is a local reference

# B.
- name: Restart service
  when: <the-same-name-like-the-one-in-register>.changed
  service:
    name: <service-name>
    state: restarted

```
---

## > Create User

```yml
- name: Create a new user
  user:
    name: <user-name>
    groups: root
```

## > Make user sudo
```yml

# Option 1
- name: Add sudoers file for user
  copy:
    src: sudoer_<username>
    dest: /etc/sudoers.d/<username>
    owner: root
    group: root
    mode: 0440
  
# Option 2
- name: Create sudoer file for user
  lineinfile:
    path: /etc/sudoers.d/<username>
    line: "<username> ALL=(ALL) NOPASSWD: ALL"
    create: yes
    mode: '0440'
```

```
# In sudoer_<username>
<username> ALL=(ALL) NOPASSWD: ALL
```

## > SSH keys
```yml
- name: Add SSH key for user
  authorized_key:
    user: <user-name>
    key: "<copy here your public key>"
```

## > Roles
```yml
- hosts: all
  roles:
    - <role-name>
```

This requires a `roles/` dir following this structure, that will contain taskbooks.

```
roles
├── <role1>
|   └── tasks
|       └── main.yml
└── <role2>
    ├── files
    |   └── some-file 
    └── tasks
        └── main.yml
```

> [!TIP]
> To automatically create a template structure of a role dir run this command:
> ```bash
> cd roles/
> ansible-galaxy init <role-name>
> ```

Overwrite role vars
```yml
- hosts: all
  roles:
    - role: <role-name> 
      vars:
        <var1-name>: <var1-value>
        <var2-name>: <var2-value>
        <var3-name>: <var3-value>
```

> [!NOTE]
> Community created roles  are available [here](https://galaxy.ansible.com/ui/)

Use community roles:

```yml
# Use it directly
- hosts: all
  roles:
    - <ansible-galaxy-role-name>
```
Before using it you can also first download it (the code snippet above, will do it automatically)

```bash
ansible-galaxy role install <ansible-galaxy-role-name>
```

## > Variables
There are 3 ways to define variables

### 1. In the inventory file
```ini
[servers]
192.168.0.1   some_kind_of_var=value
```

### 2. In a hosts_var/groups_var file

```yml
host_var_specific_to_this: This_server_is_78
```

### 3. In the playbook
```yml
  - hosts: all
    vars:
      host_var_example: "This kind of var is not recommended"
```

### 4. vars_files
```yml
# Include them in the target playbook
vars_files:
  - /vars_files/other_vars.yml
```

## > Mark changed
This tells Ansible that this task should be marked as "changed" no matter what happens.

```yml    
- name: some kind of task
  changed_when: true
  ...
```


## > Handlers

### 1. Roles level
**Trigger**
```yml
- name: This task will trigger a handler
  # This is required to trigger a handler
  # REMEMBER! This will be triggered, only if a change occured
  notify: trigger-demo-handler
  ...
```

**Handler** (./roles/<role>/handlers/main.yml)
```yml
# The name of the handler task, should be the same
# as the reference used during `notify`
- name: trigger-demo-handler
  ...
```

```
roles
└── <role1>
    ├── handler
    |   └── main.yml 
    ├── files
    |   └── some-file 
    └── tasks
        └── main.yml
```

### 2. Root level handler

**Trigger**
```yml
- hosts: all
  # -----------------------------
  # Import handler
  handlers:
    - import_tasks: handlers.yml
  # -----------------------------

  tasks:
    - name: This task will trigger a handler
      # This is required to trigger a handler
      # REMEMBER! This will be triggered, only if a change occured
      notify: trigger-demo-handler
      ...
```

**Handler** (handlers.yml)
```yml
# The name of the handler task, should be the same
# as the reference used during `notify`
- name: trigger-demo-handler
  ...
```

```
.
├── handlers.yml 
├── playbook.yml 
└── inventory
```

## > Templates
Templates can be used in a similar way as files, however, they can contain variables that will change ad hoc. Hence, they can be used for configuration files that need to be different on each machine, etc.

> [!NOTE]
> Templates use jinja2 format (.j2).

**Task**
```yml
- name: Some kind of name
  template:
    src: "<your/local/template/file/path>"
    dest: "<the/target/path>"
    owner: <username>
    group: <groupanme>
    mode: 0644
```

**Template file** (./templates/dummy-template.j2)
```j2
Some random value: {{ template-dummy-variable }}
```

**Host/Group var** (per host)
```ini
template-dummy-variable: 
```

---

## > AWS
```bash
export AWS_ACCESS_KEY_ID=''
export AWS_SECRET_ACCESS_KEY=''
```

Playbook
```yml
- hosts: localhost
  gather_facts: False
  tasks:
    ...
```