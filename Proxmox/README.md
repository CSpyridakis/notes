# Proxmox VE (PVE)

A powerful open-source hypervisor for managing Virtual Machines (VMs) and Linux Containers LXD (CTs).

**Support:**
- [Proxmox Wiki](https://pve.proxmox.com/wiki)
- [Proxmox Community Forum](https://forum.proxmox.com)

---

## > Installation
For Installation related topics read [this](./installation.md).

---

## > Post-Installation
For Post-Installation related topics read [this](./post-installation.md).

---

## > Create VM/CT

- To create a VM read [this](./create-VM.md).
- To create a CT read [this](./create-CT.md).

---

## > Templates

### VM Templates
Create VM templates to standardize VM creation and reduce setup time.

Read [this](./create-VM-template.md)

### Container Templates
Use CT templates for lightweight, fast deployment of containerized environments.

Read [this](./create-CT-template.md)

---

## > Clustering

> [!IMPORTANT]
> Datacenter-wide settings will override individual node configurations.

Use clustering to manage multiple Proxmox nodes from a single interface.



---



## > Migration

### VM Migration
VMs can be **live migrated** between nodes (e.g., Node A → Node B) without downtime, thanks to Proxmox's built-in HA (High Availability) support.

### Container Migration
Container (CT) migration requires the container to be shut down. **Live migration** is not supported for containers.
