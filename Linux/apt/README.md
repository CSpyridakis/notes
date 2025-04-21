# APT package manager
`apt` (Advanced Package Tool) is a package manager for Debian-based Linux distributions like:
- Ubuntu
- Debian
- Linux Mint
- Kali Linux, etc.

What it does:
- Installs, removes, updates, and manages software packages.
- Resolves dependencies automatically.
- Works with **.deb** packages (Debian package format).
- Internally uses tools like dpkg, apt-get, and apt-cache.

Backend tool: **dpkg**.

## Common Commands
| Command                                           | Description                                                                                       |
|--------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `apt search <package-name>`                      | Search for a package                                                                              |
| `sudo apt search linux-generic-hwe`              | Search for kernel-related packages                                                                |
| `apt show <package-name>`                        | Get detailed info about a package                                                                 |
| `sudo apt install <package-name>`                | Install a package                                                                                 |
| `sudo apt remove <package-name>`                 | Remove a package (keep config files)                                                              |
| `sudo apt purge <package-name>`                  | Remove a package and its config files                                                             |
| `sudo apt update`                                | Update package lists (metadata)                                                                   |
| `sudo apt upgrade`                               | Upgrade all installed packages                                                                    |
| `sudo apt full-upgrade`                          | Full upgrade (may remove/install packages)                                                        |
| `sudo apt install --only-upgrade <package-name>` | Upgrade only a specific package                                                                   |
| `apt list --installed`                           | List all installed packages                                                                       |
| `apt list -a <package-name>`                     | List available versions of a package                                                              |
| `sudo apt-mark hold <package>`                   | Do not update/upgrade this package (e.g. useful for preventing kernel updates)                         |
| `sudo apt-mark unhold <package>`                 | Revert the previous hold                                                                          |
| `sudo apt clean`                                 | Clean local repository of downloaded package files                                                |
| `sudo apt autoremove`                            | Remove unnecessary packages                                                                       |
| `sudo apt --fix-broken install`                  | Fix broken dependencies                                                                           |
| `sudo dpkg-reconfigure <package-name>`           | Reconfigure a package                                                                             |
