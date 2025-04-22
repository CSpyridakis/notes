# Create a VM Template

## 1. Clean Up the Guest System
Remove Unique System Identifiers. SSH host keys and machine-id are unique to each system. 
If left unchanged:

- All VMs cloned from the template will share the same identity, which poses security risks and networking conflicts.
- SSH connections may raise security warnings due to reused host keys.

Perform the following steps from **inside the guest VM** before converting it to a template:

### For Debian-based systems:
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

## 2. Convert the VM to a Template
Right-click the powered-off VM > Select `Convert to Template`.

## 3. Hardware updates
1. Select the newly created Template.
2. Go to the `Hardware` tab.
3. `CD/DVD` > `Edit` > `Do not use any media` > `OK`.
4. `Add` > `CloudInit Drive`
   1. `Bus/Device`: **IDE**
   2. `Storage`: Choose your desired storage (e.g., local-lvm)
   3. `Add`
5. `Cloud-Init` > Edit values such as User, Password, etc > `Regenerate Image`

## 4. Create a VM from the Template

1. Right-click on the Template.
2. Select Clone.
3. Set the following:
   - **Mode**: Full Clone (recommended to make a complete copy)
   - Assign VM ID, Name, and Target Node
4. Click Clone