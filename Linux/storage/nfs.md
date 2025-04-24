# Network File Storage

> [!IMPORTANT]
> NFS (v3 and v4) by default doesn't support password authentication like Samba does.
> So you can restrict access by IP/subnet rather than using usernames and passwords.

----

## Server

### 1. Create parent dir
The Directory in which all shared folders will exist

### 2. Create subdirs
These folders should be located in the parent dir, and will be all the sharable folders.

### 3. Install software

**Debian**
```bash
sudo apt update -y
sudo apt install -y nfs-kernel-server
```

### 4. Configure nfs server

Create a backup file (in case it is needed)
```bash
sudo cp /etc/exports /etc/exports.bak   
```

Then edit `/etc/exports/` like this:
```txt
<full/path/of/each/subdir> <network_identifier>/<subnet_mask>(rw,no_subtree_ckeck)
```

E.g.
```
/home/user/nfs_shares/files/ 192.168.0.0/255.255.255.0(rw,no_subtree_ckeck)
/home/user/nfs_shares/documents/ 192.168.0.0/255.255.255.0(rw,no_subtree_ckeck)
/home/user/nfs_shares/backups/ 192.168.0.0/255.255.255.0(rw,no_subtree_ckeck)
```

### 5. Restart nfs server
```bash
sudo systemctl restart nfs-kernel-server
systemctl status nfs-kernel-server
```

---

## Client

### 1. Install software

**Debian**
```bash
sudo apt update -y
sudo apt install -y nfs-common
```

### 2. Verify server connectivity

```bash
showmount --exports <server-ip> # This should display all the subdirs that exist in /etc/exports
```

### 3. Preparation
As with server, create a parent dir and subdirs that will be attached to remote dirs.

### 4. Mount remote shares
```bash
sudo mount <server-ip>:<server/dir/full/path> <local/dir/full/path>
```
Now you should be able to use remote dirs.

### 5. Verify mounts
```bash
df -h
# Or run
mount
```

### 6. Unmount 
```bash
sudo umount <local/dir/full/path>
```