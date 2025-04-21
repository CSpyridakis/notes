# Bootloader

Bootloader's goal is to load into memory the kernel and then pass the control to the OS.

**GRUB2** is one of the well known bootloaders.

```mermaid
flowchart LR

bios["BIOS/UEFI"] --> grub["Bootloader"] --> kernel["Kernel"]
```

GRUB2 configuration location: `/etc/default/grub`

Update configuration

Debian based:
```bash
sudo update-grub
```

Red Hat based
```bash
sudo grub2-mkconfig -o /boot/grub2/grub.cfg
```

Both will update `/boot/grub/grub.cfg`

> [!IMPORTANT]
> **NEVER** update manually the `/boot/grub/grub.cfg`