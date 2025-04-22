# Create a CT template

## 1. Prepare CT

Remove Unique System Identifiers. SSH host keys and machine-id are unique to each system. 
If left unchanged:

- All VMs cloned from the template will share the same identity, which poses security risks and networking conflicts.
- SSH connections may raise security warnings due to reused host keys.

### For Debian-based systems:
```bash
# Make sure that the system is updated
sudo apt update -y
sudo apt dist-upgrade -y # Optional

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
sudo apt autoremove

# Power off the CT
sudo poweroff
```

## 2. Convert the CT to a Template
Right-click the powered-off CT > Select `Convert to Template`.

## 3. Create a CT from the Template

1. Right-click on the Template.
2. Select Clone.
3. Set the following:
   - **Mode**: Full Clone (recommended to make a complete copy)
   - Assign CT ID, Hostname, and Target Node
4. Click Clone