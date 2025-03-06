# Ventoy

## 1. Download 
Download [Ventoy](https://www.ventoy.net/en/download.html) and unzip it.

## 2. Determine the desired USB
```bash
sudo parted -l
```

## 3. Install Ventoy
Navigate to the `Ventoy` dir, from there run the following commands.
```bash
sudo bash Ventoy2Disk.sh -I -r SIZE_MB /dev/sdX         # Where X is the desired device
```
Where:
-  SIZE_MB: The size of free space to preserve at the end of the disk (Can be used as a normal USB stick to store files, or in another way).

## 4. [Optional] Partition free space
Create a new partition at the optional preserve space that exist at the end of the disk.

## 5. Put ISOs
Copy ISOs in the `Ventoy` disk.

### Some Useful ISOs would be:

#### Tools
* [gparted-live-1.7.0-1-amd64.iso](https://gparted.org/download.php)
* [systemrescue-11.03-amd64.iso](https://www.system-rescue.org/Download/)
* [rescuezilla-2.5.1-64bit.noble.iso](https://rescuezilla.com/download)
* [clonezilla-live-3.2.0-5-amd64.iso](https://clonezilla.org/downloads.php)
* [HBCD_PE_x64.iso](https://www.hirensbootcd.org/download/)
* [KNOPPIX_V9.1CD-2021-01-25-EN.iso](https://www.knopper.net/knoppix-mirrors/index-en.html)

#### Debian Based
* [debian-12.9.0-amd64-netinst.iso](https://www.debian.org/distrib/)
* [ubuntu-24.04.2-desktop-amd64.iso](https://ubuntu.com/download/desktop)
* [ubuntu-24.04.2-live-server-amd64.iso](https://ubuntu.com/download/server)
* [linuxmint-22.1-cinnamon-64bit](https://www.linuxmint.com/)

#### RPM
* [Fedora-Workstation-Live-x86_64-41-1.4.iso](https://fedoraproject.org/workstation/download)

#### Arch based
* [archlinux-2025.03.01-x86_64.iso](https://archlinux.org/download/)
* [manjaro-gnome-24.2.1-241216-linux612.iso](https://manjaro.org/products/download/x86)

#### Windows
* [Win10_22H2_EnglishInternational_x64v1.iso](https://www.microsoft.com/el-gr/software-download/windows10)
* [Win11_24H2_EnglishInternational_x64](https://www.microsoft.com/el-gr/software-download/windows11)

#### Security
* [kali-linux-2024.4-installer-amd64.iso](https://www.kali.org/get-kali/#kali-platforms)
* [Parrot-security-6.3.2_amd64.iso](https://www.parrotsec.org/download/)

#### Servers
* [proxmox-ve_8.3-1.iso](https://www.proxmox.com/en/downloads/proxmox-virtual-environment/iso)
* [TrueNAS-SCALE-24.10.2.iso](https://www.truenas.com/download-truenas-scale/)

#### Firewalls
* [pfSense-CE-2.7.2-RELEASE-amd64.iso](https://www.pfsense.org/download/)
