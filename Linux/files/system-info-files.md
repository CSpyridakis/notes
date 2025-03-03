# System information files

## Important Files
|    Info   |    File     |
| ----------| ----------- |
| CPU info  | `/proc/cpuinfo` | 
| Memory info | `/proc/meminfo` |
| Operating System info | `/etc/os-release` |
| Linux Standard Base | `/etc/lsb-release` |
| Kernel info | `/proc/version` |
| Hostname | `/proc/sys/kernel/hostname` |
| OS type  | `/proc/sys/kernel/ostype` |
| Motherboard  | `/sys/class/dmi/id/board_name` |
| BIOS Version | `/sys/class/dmi/id/bios_version` |
| System users    | `/etc/passwd` |
| System groups    | `/etc/group` |
| System user password hashes   | `/etc/shadow` * |

\* Requires sudo privileges to access it

## Misc Files

| **Category**           | **File/Path**                        | **Description**                                                                 |
|-------------------------|--------------------------------------|---------------------------------------------------------------------------------|
| **System Uptime**       | `/proc/uptime`                      | Shows how long the system has been running and idle.                           |
| **Kernel Parameters**   | `/proc/cmdline`                     | Kernel boot parameters passed during system startup.                           |
| **Mount Points**        | `/proc/mounts`                      | Displays all mounted file systems.                                             |
| **Block Devices**       | `/proc/devices`                     | List of device types, including block and character devices.                   |
| **Network Interfaces**  | `/proc/net/dev`                     | Statistics about network interfaces, such as sent and received packets.        |
| **System Statistics**   | `/proc/stat`                        | General statistics like CPU utilization, interrupts, and context switches.     |
| **Process Info**        | `/proc/[pid]/status`                | Detailed information about a specific process by its PID.                      |
| **Open Files**          | `/proc/[pid]/fd/`                   | Lists all open file descriptors for a specific process.                        |
| **Default Runlevel**    | `/etc/inittab`                      | Defines the system's default runlevel (on older systems).                      |
| **Loaded Modules**      | `/proc/modules`                     | Lists currently loaded kernel modules.                                         |
| **Filesystem Info**     | `/proc/filesystems`                 | Lists supported file systems.                                                  |
| **Disk Statistics**     | `/proc/diskstats`                   | Provides detailed information about disk I/O.                                  |
| **System Logs**         | `/var/log/syslog` or `/var/log/messages` | System and kernel logs for debugging and monitoring.                          |
| **Device Information**  | `/sys/block/`                       | Provides information about block devices like disks.                           |
| **PCI Devices**         | `/proc/bus/pci` or `lspci`          | Information about PCI devices connected to the system.                         |
| **USB Devices**         | `/proc/bus/usb` or `lsusb`          | Information about USB devices connected to the system.                         |
| **Kernel Features**     | `/boot/config-$(uname -r)`          | Configuration file for the running kernel version.                             |
