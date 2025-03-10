# OpenWrt

## 🔗 List of compatible devices
Find the [list](https://openwrt.org/toh/start) of compatible devices.

## 📙 Defaults
### A. Credentials
Username: `root`
Password: `root`

### B. Gateway IP
`192.168.1.1`

---

## 🚩 Common Issues

### 1. SSH connection error
If this error is appeared during ssh connection:
```bash
Unable to negotiate with 192.168.0.1 port 22: no matching host key type found. Their offer: ssh-rsa
```

This error occurs because recent OpenSSH clients have disabled the older ssh-rsa algorithm (which uses SHA-1) by default for security reasons.

As a workaround, temporarily accept the ssh-rsa algorithm for this connection:

```bash
ssh -oHostKeyAlgorithms=+ssh-rsa -oPubkeyAcceptedAlgorithms=+ssh-rsa root@192.168.0.1
```

If you want to frequently connect to this device, include these lines in your `~/.ssh/config` file.

```bash
Host 192.168.0.1
    HostKeyAlgorithms +ssh-rsa
    PubkeyAcceptedAlgorithms +ssh-rsa
```