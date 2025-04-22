# Proxmox VE (PVE)

A powerful open-source hypervisor for managing Virtual Machines (VMs) and Linux Containers LXD (CTs).

**Support:**
- [Proxmox Wiki](https://pve.proxmox.com/wiki)
- [Proxmox Community Forum](https://forum.proxmox.com)

---

## > Installation

### 1. Target Hard Disk
During installation, click on `Options` to customize the `filesystem` (e.g., **ZFS**) and set up any required **RAID** configuration.

> [!Note]
> **ZFS** is recommended if the system has more than 32GB of RAM. 
> It is ideal for production environments and systems requiring high performance.
>
> - ZFS is memory-intensive (recommended: 1 GB RAM per 1 TB of storage).
> - Mixing disk sizes or replacing drives requires careful planning.
> - Avoid using hardware RAID with ZFS — use ZFS-native RAID-Z instead.

### 2. Administration
Specify an email address for receiving system notifications and alerts.

### 3. Network Configuration
- **Hostname**: The name of the Proxmox server.
- **IP Address**: Assign a static IP address outside of the router's DHCP range.

### 4. Web Access
After installation, access the Proxmox Web GUI:

`http://<your-ip>:8006`

**Default Credentials:**
- **Username**: `root`
- **Password**: Set during installation

---

## > Post-Installation

### Update Node
1. Select the node (top left panel).
2. Go to `Updates` > `Refresh`.

### Upgrade Node
1. `Updates` > `Refresh`
2. `Updates` > `Upgrade`
3. `Updates` > `Refresh`
4. Reboot the node.

### Subscription Repository
If you don't plan to purchase a subscription:

1. Go to `Updates` > `Repositories` > `Add`.
2. Select `No-subscription` repository > `Add`.
3. [Upgrade node](#upgrade-node).

If you do have a subscription:
- Select the node > `Subscription` > `Upload Subscription Key`.

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
