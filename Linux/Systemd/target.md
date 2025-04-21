# Systemd targets

In systemd, a target is a special kind of unit that groups other units together to manage the system's state, like booting into graphical mode, multi-user mode, rescue mode, etc.

---

## Common targets: 

| Target | Description   |
| -----  | ------------- |
| `graphical.target` | Multi-user system with GUI
| `multi-user.target` | Multi-user system without GUI
| `rescue.target` | Minimal system for maintenance (single-user mode)
| `emergency.target` | Bare minimum system with no services except systemd and a shell
| `default.target` | The default target used at boot (symlink to one of the above)

---

## Common commands:

- Check your current target: `sudo systemctl get-default`
- List available targets: `sudo systemctl list-units --type target --all`
- Switch current target: `sudo systemctl isolate <target-name>.target`
- Change default target: `sudo systemctl set-default <target-name>.target`
- See which services are tied to a target: `sudo systemctl list-dependencies <target-name>.target`

---

## How it works
Assume the following `.service` file
```
...
[Install]
WantedBy=<target-name>.target
```

The idea is that at the point we execute: `sudo systemctl enable <service-name>` 

A **symlink** will be created in `/etc/systemd/system/<target-name>.target.wants/<service-name>.service`. 

Hence, all services need to be started at a given target will be located in the same place.