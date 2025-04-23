# Installation

## 1. Target Hard Disk
During installation, click on `Options` to customize the `filesystem` (e.g., **ZFS**) and set up any required **RAID** configuration.

> [!Note]
> **ZFS** is recommended if the system has more than 32GB of RAM. 
> It is ideal for production environments and systems requiring high performance.
>
> - ZFS is memory-intensive (recommended: 1 GB RAM per 1 TB of storage).
> - Mixing disk sizes or replacing drives requires careful planning.
> - Avoid using hardware RAID with ZFS — use ZFS-native RAID-Z instead.

## 2. Administration
Specify an email address for receiving system notifications and alerts.

## 3. Network Configuration
- **Hostname**: The name of the Proxmox server.
- **IP Address**: Assign a static IP address outside of the router's DHCP range.

## 4. Web Access
After installation, access the Proxmox Web GUI:

`http://<your-ip>:8006`

**Default Credentials:**
- **Username**: `root`
- **Password**: Set during installation
