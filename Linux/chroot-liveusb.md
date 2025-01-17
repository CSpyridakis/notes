# Live USB chroot
There are occasions in which we need to login to a system that cannot boot properly. Maybe because it is corrupted, there is a wrong configuration, etc.

This contains the instructions to boot from a live USB to a distribution, in order to fix the problem. To achieve this we will leverage `chroot`. Before continue, create a bootable USB, boot using it and open a terminal. Then follow, these steps: 

## 0. Find disk
Use `lsblk` or `df -h`


## 1. Mount the root partition
```
sudo mount /dev/sdaX /mnt
```

## 2. If have, mount separate boot partition
```
sudo mount /dev/sdaY /mnt/boot
```

## Binding system directories.
```
sudo mount -t proc /proc /mnt/proc
sudo mount -o bind /dev /mnt/dev
sudo mount -t sysfs /sys /mnt/sys
sudo mount -o bind /run /mnt/run
sudo mount -t devpts /pts /mnt/dev/pts
```

## Chroot
```
sudo chroot /mnt /bin/bash
```

---

## DO ANYTHING you want!
Maybe install packages, remove files,etc...

---

## At the end
```
exit
sudo umount /mnt/dev/pts /mnt/run /mnt/sys mnt/dev /mnt/proc /mnt/boot /mnt
sudo reboot
```