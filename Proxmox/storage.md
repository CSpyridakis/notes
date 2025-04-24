# Storage

In proxmox you can mount storage from remote servers (**shared storage**) to share resources.


> [!INFO]
> Since Proxmox runs on top of linux, you can use **samba** or **NFS**.

---

## Target

We can create [nfs](https://en.wikipedia.org/wiki/Network_File_System) network storage in a technology, like:
- Synology
- TrueNAS
- etc..

Then, mount the storage in Proxmox.

---

## Use Case

We can use this approach to store files related to:
- **Shared Storage** (High Availability)
- **Back-ups** (Disaster recovery)

> [!NOTE]
> Keep in mind that, in most cases, backups occur occasionally, while shared storage is continuously used.
> 
> Therefore, having a high-speed connection between your Proxmox nodes and the shared storage is essential for optimal performance.
> 
> Consider using 10 GbE connections instead of standard Gigabit Ethernet, especially if you're working with high-throughput workloads or fast storage that should be accessible from multiple VMs.

---

### Connect NFS to the Datacenter

As a prerequisite, you must already have an NFS storage available in your network. 

> [!IMPORTANT]
> Make sure that the NFS storage in the server can be accessed by **root** user.

`Datacenter` > `Storage` > `Add` > `NFS`
- `ID`: A name for this share
- `Server`: IP or Domain
- `Export`: Server share dir full path
- `Content`: What we plan to store there
    - For **BUs** select: `Disk image`, `Container template`, `Backup` and `Snippets`
    - For **Shared Storate** select: `Disk image`, `Container template` and `Container`

