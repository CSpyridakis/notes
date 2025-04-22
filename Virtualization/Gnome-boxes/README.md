# Gnome boxes

**GNOME Boxes** is a simple VM management application designed for GNOME desktop environments. 

It lets you **easily** create, access, and manage VMs or remote desktops with a modern, user-friendly GUI.

Think of it as a **lightweight**, **user-friendly** alternative to VirtualBox or virt-manager, especially suited for beginners and casual users.

## Installation

### Debian
```
sudo apt install gnome-boxes
```

### Red Hat
```
sudo dnf install gnome-boxes
```

## Under the Hood
| Component | Purpose   |
| --------- | --------- |
| libvirt | Provides VM lifecycle management
| QEMU/KVM | Runs the actual VMs
| SPICE/VNC | Enables remote desktop protocols
| .iso images | Used to install guest OS