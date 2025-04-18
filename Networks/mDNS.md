# Multicast DNS

---

## Approach 1: Avahi (Most Common & Cross-Distro)

### A. Installation:

#### 1.Debian/Ubuntu:
```bash
sudo apt update
sudo apt install -y avahi-daemon avahi-utils
```

#### 2. RHEL/Fedora/CentOS:
```bash
sudo dnf install -y avahi avahi-tools
```

### B. Configuration:

#### 1. Enable and start the service:
```bash
sudo systemctl enable avahi-daemon
sudo systemctl start avahi-daemon
```

#### 2. Edit `/etc/avahi/avahi-daemon.conf` if customization is needed. Example (optional):
```bash
[server]
use-ipv4=yes
use-ipv6=no
```

To advertise services, use .service files in `/etc/avahi/services/`.

### C. Verification:
```bash
# Ping
ping hostname.local

# Or
avahi-browse -a
```

---

## Approach 2: systemd-resolved (Modern, Native to systemd)

> [!IMPORTANT]
> This approach should be modified in order to include the latest
> changes of systemd, because systemd-resolve has 
> been renamed to resolvectl, however, resolvectl does not support
> --set-mdns=yes

Supported on: Debian 9+, Ubuntu 16.10+, Fedora 33+, RHEL 8+

### A. Installation:
Usually preinstalled with systemd.

### B. Enable mDNS in systemd-resolved:

1. Edit or create `/etc/systemd/resolved.conf`:

    ```bash
    [Resolve]
    MulticastDNS=yes
    ```
2. Make sure `/etc/nsswitch.conf` includes mdns:
    ```
    hosts: files mdns4_minimal [NOTFOUND=return] dns
    ```

3. Enable on a link: `sudo systemd-resolve --set-mdns=yes --interface=<interface>`

3. Then run `sudo systemctl restart systemd-resolved`

4. Based on your firewall, make sure to open UDP and TCP ports `5355`.

### C. Verification:

On the host
```
sudo systemd-resolve --status <interface>
```

On the client:
```
ping hostname.local
```

No extra daemons or tools are required if you only want basic mDNS resolution.