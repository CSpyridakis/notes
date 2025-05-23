# Disk manipulation
---

## fdisk
A command-line utility to create, delete, and modify disk partitions on a Linux system.

### List disks
`sudo fdisk -l`

### To list the partitions on a disk:
`fdisk -l <disk_path>`

### Modify partitions to disk
`sudo fdisk <disk_path>`

---

## cfdisk
TUI Partition Manager, to create, delete, resize (non-destructively), and manage partitions, , great for MBR/GPT, visually intuitive.

`sudo cfdisk`

---

## parted

Resize, move, and create partitions, GPT and MBR support.

`sudo parted /dev/sdX`

---

## mkfs
`mkfs` (make file system) is a command used to create a file system on a partition or disk.

### Format partition to `ext4`
`sudo mkfs.ext4 <partition_path>`
or
`sudo mkfs -t <filesystem_type> <partition_path>`
where `filesystem_type` can be `ext4`, `xfs`, etc.

---

## mount 
Is used to attach a file system to a specified mount point. 

### To temporarily mount a file system:
`sudo mount <partition_path> <mount_point_path>`

### Permanent mounting (persistent after reboot)
1. Edit `/etc/fstab`, and append this line:
```
<partition_path>  <mount_point_path> <filesystem_type> defaults 0 0 
```
2. Run `mount -a` to mount it.

3. Check if it is mounted by running `df -h`.

Extra options:
- `-f`: Fake skip the mount syscall.
- `-v`: Verbose, print also some messages.

---

## umount 
`umount` is used to unmount a mounted file system.

To unmount a file system:
`umount <mount_point_path>`

---

## lsblk
List all block devices, providing details about disks and partitions.

### Usage
`sudo lsblk`

---

## partprobe
Inform the kernel of partition table changes.

### Usage
`sudo partprobe <disk_path>`