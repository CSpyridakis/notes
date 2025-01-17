# Build and install a different linux kernel

Instructions based on this [guide](https://phoenixnap.com/kb/build-linux-kernel)

## 1. Download Linux kernel source code (kernel.org)
```
LINUX_KERNEL_INST_VERSION="6.10.9"
wget https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-${LINUX_KERNEL_INST_VERSION}.tar.xz
```

## 2. Extract the source code
```
tar xvf linux-${LINUX_KERNEL_INST_VERSION}.tar.xz
``` 

## 3. Install additional packages
```
sudo apt-get install git fakeroot build-essential ncurses-dev xz-utils libssl-dev bc flex libelf-dev bison
```

## 4. Configure Kernel
```
cd linux-${LINUX_KERNEL_INST_VERSION}
cp /boot/config-$(uname -r) .config
make menuconfig
# Make any needed changes regarding modules
```

## 5. Disable on Ubuntu conflicting security certificates
```
scripts/config --disable SYSTEM_TRUSTED_KEYS
scripts/config --disable SYSTEM_REVOCATION_KEYS
```

## 6. Build the kernel (This will take some time)
```
make -j $(nproc)
```

## 7. Install the required modules
```
sudo make modules_install
```

## 8. Install the kernel
```
sudo make install 
```

## 9.  (OPTIONAL) Update bootloader
```
# `make install` command performs this process automatically, but you can also do it manually
sudo update-initramfs -c -k ${LINUX_KERNEL_INST_VERSION}
sudo update-grub
```

## 10.  Reboot
## 11  If kernel does not change then ([reference](https://askubuntu.com/questions/82140/how-can-i-boot-with-an-older-kernel-version))
```    
# 1. Find kernel number
sudo grub-mkconfig | grep -iE "menuentry 'Ubuntu, with Linux" | awk '{print i++ " : "$1, $2, $3, $4, $5, $6, $7}'
 
# 2. Change GRUB_DEFAULT="1>N" where N required kernel 
sudo nano /etc/default/grub

# 3. Update
sudo update-grub
```
