#!/bin/bash

# Instructions based on https://sluijsjes.nl/2024/05/18/coral-and-nvidia-passthrough-for-proxmox-lxc-to-install-frigate-video-surveillance-server/


# ==================================================================
# HOST SIDE:
# ==================================================================

# ----------------------------------------------------
# 1. Deactivating the Nouveau driver:
# Disable the Nouveau driver
echo -e "blacklist nouveau\noptions nouveau modeset=0" | tee /etc/modprobe.d/blacklist-nouveau.conf
# Update the initramfs
update-initramfs -u
# Reboot the system
reboot

# ----------------------------------------------------
# 2. Preparing the system to compile and install the new driver
apt install pve-headers-$(uname -r)
apt-get install make gcc libvulkan1 pkg-config

# ----------------------------------------------------
# 3. Installing the new Nvidia driver
# https://www.nvidia.com/en-us/drivers/details/237853/
NVIDIA_DRIVER_VERSION=550.142
wget https://download.nvidia.com/XFree86/Linux-x86_64/${NVIDIA_DRIVER_VERSION}/NVIDIA-Linux-x86_64-${NVIDIA_DRIVER_VERSION}.run
chmod +x NVIDIA-Linux-x86_64-${NVIDIA_DRIVER_VERSION}.run
./NVIDIA-Linux-x86_64-${NVIDIA_DRIVER_VERSION}.run --no-questions --disable-nouveau

# ----------------------------------------------------
# 4. Enabling the NVIDIA drivers using udev rules
echo -e '\n# load nvidia modules\nnvidia\nnvidia_uvm\nnvidia-drm\nnvidia-uvm' | tee /etc/modules-load.d/modules.conf
update-initramfs -u -k all

# Edit udev rules
nano /etc/udev/rules.d/70-nvidia.rules
# Add these:
# KERNEL=="nvidia", RUN+="/bin/bash -c '/usr/bin/nvidia-smi -L && /bin/chmod 666 /dev/nvidia*'"
# KERNEL=="nvidia_uvm", RUN+="/bin/bash -c '/usr/bin/nvidia-modprobe -c0 -u && /bin/chmod 0666 /dev/nvidia-uvm*'"
# SUBSYSTEM=="module", ACTION=="add", DEVPATH=="/module/nvidia", RUN+="/usr/bin/nvidia-modprobe -m"

# To prevent the Nvidia driver/kernel files from stopping when the GPU is not in use, install the Nvidia persistence service. It became available to us after we installed the new drivers.
cp /usr/share/doc/NVIDIA_GLX-1.0/samples/nvidia-persistenced-init.tar.bz2 .
tar -xjf nvidia-persistenced-init.tar.bz2
# Remove old, if any (to avoid masked service)
rm /etc/systemd/system/nvidia-persistenced.service
# Install
./nvidia-persistenced-init/install.sh
timeout 1 systemctl status nvidia-persistenced.service
rm -rf nvidia-persistenced-init*

# ----------------------------------------------------
# 5. Done! Reboot and test
reboot

# ----------------------------------------------------
# 6. Verify everything ok
nvidia-smi
timeout 1 systemctl status nvidia-persistenced.service
ls -alh /dev/nvidia*
# Expected output:
        # crw-rw-rw- 1 root root 195,   0 Dec 27 17:27 /dev/nvidia0
        # crw-rw-rw- 1 root root 195, 255 Dec 27 17:27 /dev/nvidiactl
        # crw-rw-rw- 1 root root 195, 254 Dec 27 17:27 /dev/nvidia-modeset
        # crw-rw-rw- 1 root root 510,   0 Dec 27 17:27 /dev/nvidia-uvm
        # crw-rw-rw- 1 root root 510,   1 Dec 27 17:27 /dev/nvidia-uvm-tools

        # /dev/nvidia-caps:
        # total 0
        # drw-rw-rw-  2 root root     80 Dec 27 17:27 .
        # drwxr-xr-x 20 root root   4.5K Dec 27 17:27 ..
        # cr--------  1 root root 239, 1 Dec 27 17:27 nvidia-cap1
        # cr--r--r--  1 root root 239, 2 Dec 27 17:27 nvidia-cap2

# ==================================================================
# LXC SIDE:
# ==================================================================

# ----------------------------------------------------
# 1. Stop container

# ----------------------------------------------------
# 2. Configuring the LXC Container
# Edit /etc/pve/lxc/1xx.conf and add the following:
# The numbers on the cgroup2 lines are from the fifth column in the device list above (via ls -alh /dev/nvidia*).
#   lxc.cgroup2.devices.allow: c 195:* rwm
#   lxc.cgroup2.devices.allow: c 506:* rwm
#   lxc.cgroup2.devices.allow: c 509:* rwm
#   lxc.cgroup2.devices.allow: c 511:* rwm
#   lxc.mount.entry: /dev/nvidia0 dev/nvidia0 none bind,optional,create=file
#   lxc.mount.entry: /dev/nvidiactl dev/nvidiactl none bind,optional,create=file
#   lxc.mount.entry: /dev/nvidia-modeset dev/nvidia-modeset none bind,optional,create=file
#   lxc.mount.entry: /dev/nvidia-uvm dev/nvidia-uvm none bind,optional,create=file
#   lxc.mount.entry: /dev/nvidia-uvm-tools dev/nvidia-uvm-tools none bind,optional,create=file

# ----------------------------------------------------
# 3. Install Frigate 
LXD_ID=101 # TODO: replace
bash -c "$(wget -qLO - https://github.com/tteck/Proxmox/raw/main/misc/frigate-support.sh)" -s ${LXD_ID}


# ==================================================================
# INSTALL DRIVER IN CONTAINER:
# ==================================================================

# ----------------------------------------------------
# 1. Start container

# ----------------------------------------------------
# 3. Installing the new Nvidia driver
# https://www.nvidia.com/en-us/drivers/details/237853/
NVIDIA_DRIVER_VERSION=550.142
wget https://download.nvidia.com/XFree86/Linux-x86_64/${NVIDIA_DRIVER_VERSION}/NVIDIA-Linux-x86_64-${NVIDIA_DRIVER_VERSION}.run
chmod +x NVIDIA-Linux-x86_64-${NVIDIA_DRIVER_VERSION}.run
./NVIDIA-Linux-x86_64-${NVIDIA_DRIVER_VERSION}.run --no-questions --disable-nouveau