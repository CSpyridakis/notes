# SSHFS

**SSHFS** stands for **SSH Filesystem** and it lets you mount a remote filesystem over **SSH** on your local machine as if it were a local folder.

It’s part of **FUSE** **(Filesystem in Userspace)**, meaning you don't need kernel-level access, just a user with SSH access to the remote system.

With **SSHFS**, you can browse, open, and edit files on a remote server securely over SSH as if they were on your local computer.

Key Features

- Secure (uses SSH encryption).
- Lightweight and easy to set up.
- No need for Samba, NFS, or FTP servers.
- Read/write access (if permissions allow).
- Great for development, backups, or remote file management.

## Installation

Debian
```bash
sudo apt install sshfs
```

Red Hat
```bash
sudo dnf install sshfs
```

## Mount
```bash
sshfs <user>@<ip-or-domain>:</remote/path> </local/mountpoint>
```

### If there are errors
#### Step 1.
Make sure that fuse group exists and that your user is part of it
```
sudo groupadd fuse
sudo usermod -aG fuse $USER
```

#### Step 2.
Update configuration
Edit `sudo vim /etc/fuse.conf` and uncomment `user_allow_other` option.

#### Step 3
Try using `allow_other` option.
```bash
sshfs -o allow_other,uid=$(id -u),gid=$(id -g) \
  <user>@<ip-or-domain>:</remote/path> </local/mountpoint>
```

\* See this [url](https://askubuntu.com/questions/123215/sshfs-is-mounting-filesystems-as-another-user)

#### Step 4
Trubleshoot

```bash
sshfs -o allow_other,debug,sshfs_debug,loglevel=debug,uid=$(id -u),gid=$(id -g) \
  <user>@<ip-or-domain>:</remote/path> </local/mountpoint>
```

## Verify 
```bash
mount
```

## Unmount
```bash
umount </local/mountpoint>
# or preferred for this 
fusermount -u </local/mountpoint>
```