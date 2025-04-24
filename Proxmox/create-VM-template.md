# Create a VM Template

## > The 'Traditional' way to create a Template
### 0. Create VM
First we need to create a VM (read [this](./create-VM.md)) to achieve that.

Then, connect to a CLI of the VM created and follow the next steps.

### 1. Clean Up the Guest System
Remove Unique System Identifiers. SSH host keys and machine-id are unique to each system. 
If left unchanged:

- All VMs cloned from the template will share the same identity, which poses security risks and networking conflicts.
- SSH connections may raise security warnings due to reused host keys.

Perform the following steps from **inside the guest VM** before converting it to a template:

#### For Debian-based systems:
```bash
# Update package list
sudo apt update -y
sudo apt dist-upgrade -y   # Optional

# Install QEMU guest agent for better VM integration
sudo apt install -y qemu-guest-agent

# Ensure cloud-init is installed for automated provisioning
sudo apt install -y cloud-init

# Enable and start the QEMU guest agent
sudo systemctl enable --now qemu-guest-agent

# Remove existing SSH host keys (they will be regenerated on first boot)
sudo rm -rf /etc/ssh/ssh_host_*

# Clear machine-id (should be empty)
sudo truncate -s 0 /etc/machine-id

# Ensure /var/lib/dbus/machine-id is a symlink to /etc/machine-id
# sudo ln -sf /etc/machine-id /var/lib/dbus/machine-id

# Make sure that /var/lib/dbus/machine-id is a symlink of /etc/machine-id
ls -l /var/lib/dbus/machine-id

# Make sure that /etc/machine-id is empty
cat /etc/machine-id

# Clean up unnecessary packages and cached files
sudo apt clean
sudo apt autoremove -y

# Power off the VM
sudo poweroff
```

### 2. Convert the VM to a Template
Right-click the powered-off VM > Select `Convert to Template`.

### 3. Hardware updates
1. Select the newly created Template.
2. Go to the `Hardware` tab.
3. `CD/DVD` > `Edit` > `Do not use any media` > `OK`.
4. `Add` > `CloudInit Drive`
   1. `Bus/Device`: **IDE**
   2. `Storage`: Choose your desired storage (e.g., local-lvm)
   3. `Add`
5. `Cloud-Init` > Edit values such as User, Password, etc > `Regenerate Image`

### 4. Create a VM from the Template

1. Right-click on the Template.
2. Select Clone.
3. Set the following:
   - **Mode**: Full Clone (recommended to make a complete copy)
   - Assign VM ID, Name, and Target Node
4. Click Clone

---

## > Create Template using a Cloud Image & Cloud Init
1. Follow the [process](./create-VM.md) to create an new VM, i.e. `Create VM` However! Regarding the following:
  - `OS`: **Do not use any media**
  - `Disk`: Remove disk (*No Disks*)
2. Select VM > `Hardware` > `Add` > `CloudInit Drive` > `Storage`: The target storage > `Add`
3. Select VM > `Cloud-Init` > Set values:
   - Set Username, Password, etc..
   - **Do not forget to fix `IP Config`**: Select `IP Config` > `Edit` > `IPv4` to **DHCP**
   - `Regenerate Image`
4. Start a CLI session on the same Node that this VM exists and run:
   - Find the URL of the Cloud Image you wish to install (e.g. for [Ubuntu](https://cloud-images.ubuntu.com/) you can select the minimal .img file)
   - `qm set <VMID> -serial0 socket --vga serial0`: Add serial console, this will enable us to view the process later
   - `wget <url-of-the-cloud-image>`: Download image
   - `mv <cloud-image>.img <cloud-image>.qcow2`: Convert cloud image
   - `qemu-img resize <cloud-image>.qcow2 <desired-size>G`: Resize image to the desired size
   - `qm importdisk <VMID> <cloud-image>.qcow2 <target-storage>` Import image to VM (target-storage if using ext4 could be local-lvm)
5. Attach disk to VM
   - Select VM > `Hardware` > `Unused Disk` > `Edit this` > [See](./create-VM.md#4-disks) > `Add`
6. Update Boot order: Select VM > `Options` > `Boot Order` > `Edit`
   - Enable the new disk
   - Reorder it: **cdrom** --> **cloud-image** --> Any other disks
7. Right click on the VM > `Convert to Template`