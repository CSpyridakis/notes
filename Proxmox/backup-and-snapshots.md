# Backup and Snapshots

| Feature                          | Proxmox Backup                          | Proxmox Snapshot                        |
|----------------------------------|-----------------------------------------|-----------------------------------------|
| **Purpose**                      | Full backups for disaster recovery | Save VM/CT state at a specific point in time |
| **Speed**                        | **Slower** (involves packing/unpacking and copying data) | **Fast** (just revert to the snapshot state) |
| **Use Case**                     | Long-term backup, offsite storage, disaster recovery | Temporary rollback point before updates or changes |
| **Storage Location**             | **Same local** or **external** storage (e.g., PBS server, NFS, CIFS) | **Same local** storage where VM/CT exist  |
| **Data Type**                    | Compressed, deduplicated backup archives | Raw disk state + RAM (optional)         |
| **Resource Usage**               | More CPU and I/O during compression and transfer | Minimal unless snapshot is retained long-term |
| **Retention Policy**            | Managed by Proxmox Backup Server (PBS)  | Manual snapshot deletion required       |
| **RAM State**                   | Optional (if included in backup)        | Optional (if selected during snapshot)  |
| **Network Impact**              | High if stored offsite or on remote PBS | None (local operation)                  |

---

## > Snapshots

### A. Take a snapshot
1. Select the VM/CT of desire
2. `Snapshots`
3. `Take Snapshot`

> [!NOTE]
> If you do not select `Include RAM`
> The VM/CT will need to start again after rolling back.

### B. Restore snapshot
1. Select the VM/CT of desire
2. `Snapshots`
3. Select the snapshot you want to rollback to
4. `Rollback`

---

## > Backups

### 1. VM level

#### A. Create a Backup
1. Select the VM/CT of desire
2. `Backup`
3. `Backup now`
   - `Storage`: Where to store the bu (can also be an external location).
   - `Mode`: How the bu will be created (Ηow much downtime will have. More downtime -> Increase consistency).

> [!ΝΟΤΕ]
> It is a good practice to have installed `qemu agent` in the VMs
> so, the bu process to be more consistent.

#### B. Restore a Backup
1. Select the VM/CT of desire
2. `Backup`
3. Select the Backup you want to restore to
4. `Restore`

### 2. Datacenter Level

Can automatically backup specific Nodes.

1. Select `Datacenter`
2. `Backup`
3. `Add`