# Create VM

---

## 1. General

| **Field**         | **Description**                                                                 |
|-------------------|---------------------------------------------------------------------------------|
| `Node`            | Select the physical node in the `Datacenter` where you want to create the VM.  |
| `VM ID`           | Must be unique. Can Use ID Scheme E.g.:                                            |
|                   | - 100–200: VMs                                                                  |
|                   | - 201–300: Containers (CT)                                                      |
|                   | - 301–400: VM Templates                                                         |
|                   | - 401–500: CT Templates                                                         |
|                   | - Extend as needed for your environment.                                        |
| `Name`            | Use a clear, descriptive name (e.g. `webserver-01`, `db-node01`).               |
| `Resource Pool`   | Optional. Use if you want to group VMs under a defined resource pool.           |

---

## 2. OS

| **Field**                | **Description**                                                                 |
|--------------------------|---------------------------------------------------------------------------------|
| `Storage`                | Select where the VM disk should be stored (e.g., `local-lvm`, `zfs`, etc.).     |
| `ISO image`              | Choose the ISO file for the OS installation.                                    |
| `Guest OS > Type/Version`| Set the correct OS type and version. Required for optimal VM configuration.     |

---

## 3. System

- **QEMU Agent**  
  Enable this option for improved VM integration (e.g., IP reporting, shutdown control).  
  Requires `qemu-guest-agent` to be installed inside the guest OS.

📌 [QEMU Agent Documentation](https://pve.proxmox.com/wiki/Qemu-guest-agent)

### Install QEMU Agent (Debian/Ubuntu):

```bash
sudo apt update
sudo apt install qemu-guest-agent
sudo systemctl enable --now qemu-guest-agent
```

---

## 4. Disks

- **Discard (Trim)**: Enable if the VM is stored on an SSD. Helps with storage optimization.  
- **Recommended Disk Sizes**:
  - Linux Server: ~15 GB (adjust as needed)
  - Linux Desktop: 20–30 GB minimum
  - Windows: 60 GB or more

---

## 5. CPU

- Start with **1 core**.
- Increase if the VM requires more performance.

---

## 6. Memory

| **Use Case**       | **Recommended Memory** |
|--------------------|------------------------|
| Linux Server       | 2 GB                   |
| Linux Desktop      | 4 GB                   |
| Windows Desktop    | 4–8 GB                 |

Adjust based on usage and load.

---

## 7. Network

- **Best Practice**:  
  - Separate **management** network from **VM** network for better security and performance (read [here](./networking.md)).
  - Use `virtio` for better network performance.