# Sudo permissions

## Give sudo permissions to a user

### 1. Add User to the sudo Group (Debian/Ubuntu and derivatives)
```
sudo usermod -aG sudo <username>
```

### 2. Add User to the wheel Group (Red Hat/CentOS/Fedora and derivatives)
```
sudo usermod -aG wheel <username>
```

### 3. Edit the `/etc/sudoers` File 
1. Run `sudo visudo`
2. Add a line like: `<username> ALL=(ALL:ALL) ALL`

### 4. Create a Custom File in `/etc/sudoers.d/`
1. Run `sudo visudo -f /etc/sudoers.d/<username>`
2. Content `<username> ALL=(ALL) NOPASSWD: ALL`
3. Run `sudo chmod 0440 /etc/sudoers.d/<username>` (File must have 0440 permissions)

### 5. Grant Specific Command Permissions
Instead of full access, you can give access to only certain commands:
```
<username> ALL=(ALL) /path/to/command1 /path/to/command2 command3
```