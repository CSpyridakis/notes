# Multipass
**Multipass** is a lightweight VM manager developed by Canonical, designed to quickly launch and manage Ubuntu VMs on your local machine.

It’s like a super-simplified alternative to tools like **VirtualBox**, **Docker**, or **Vagrant**, but specifically for Ubuntu and Ubuntu images.

## Installation

```
sudo snap install multipass
```

## Commom Commands

| Command | Description | Example |
|--------|-------------|---------|
| `multipass find` | List all available Ubuntu images | `multipass find` |
| `multipass launch <image-name> -c <cpus> -d <disk> -m <ram> -n <name>` | Launch a new instance with specified resources and name | `multipass launch 22.04 -c 2 -m 2G -d 10G -n my-vm` |
| `multipass list` | Show all instances and their current status | `multipass list` |
| `multipass info <instance>` | Show detailed information about a specific instance | `multipass info my-vm` |
| `multipass shell <instance>` | Open an interactive shell inside the instance | `multipass shell my-vm` |
| `multipass exec <instance> -- <command>` | Run a command inside the instance from the host | `multipass exec my-vm -- ls /home/ubuntu` |
| `multipass stop <instance>` | Stop a running instance | `multipass stop my-vm` |
| `multipass start <instance>` | Start a stopped instance | `multipass start my-vm` |
| `multipass delete <instance>` | Delete a stopped instance | `multipass delete my-vm` |
| `multipass purge` | Remove all deleted instances and reclaim disk space | `multipass purge` |
| `multipass set <parameter>=<value>` | Set configuration options for Multipass | `multipass set local.driver=qemu` |

--- 

## Known issues
```
ERROR: ld.so: object 'libgtk3-nocsd.so.0' from LD_PRELOAD cannot be preloaded (failed to map segment from shared object): ignored.
```

\* libgtk3-nocsd is commonly used to disable client-side decorations (CSD) in GTK3 applications, forcing them to use traditional window manager decorations.