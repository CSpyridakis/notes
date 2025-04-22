# Create CT

Proxmox supports LXC containers, which are more lightweight than virtual machines (VMs) but still preserve state and behave similarly in many use cases.

Before creating a container, you must first download a CT template, just like with VMs.

---

## Download a CT template

**Procedure**: Storage (e.g. local) > `CT Templates` > `Templates`
Search for the desired image (e.g., ubuntu), then click `Download`.

---

## Create CT

### 1. General


| **Field**         | **Description**                                                                 |
|------------------|----------------------------------------------------------------------------------|
| `Node`           | Select the Proxmox node where the container will be created.                    |
| `CT ID`          | Must be unique. Can use a structured range:                                         |
|                  | - 100–200: VMs                                                                  |
|                  | - 201–300: Containers (CTs)                                                     |
|                  | - 301–400: VM Templates                                                         |
|                  | - 401–500: CT Templates                                                         |
| `Hostname`       | Choose a meaningful name to identify the container.                             |
| `Password`       | Set a strong root password for access.                                          |
| `Resource Pool`  | (Optional) Assign to a resource pool if using them.                             |

### 2. Template

Select the CT template you downloaded in the previous step.


### 3. Disks

Allocate disk space **~15GB** is typically sufficient for most container use cases. 
Adjust as needed.

### 4. CPU

Start with **1 core**. 
Increase if the container shows signs of high CPU usage or performance issues.

### 5. Memory

Allocate **~1GB RAM** for basic Linux usage. 
Monitor and increase based on performance requirements.

### 6. Network

Enable **DHCP** for both **IPv4** and **IPv6**, unless static IPs are required.

### 7. DNS

By default, DNS is inherited from the host. Override only if needed.

---

## Connecting to the CT

### Access Credentials

- **Username:** `root`  
- **Password:** As set during Step 1

### SSH Access

To allow secure SSH access, create a non-root user:

```bash
useradd -m -s /bin/bash <username>
usermod -aG sudo <username>
passwd <username>
```
