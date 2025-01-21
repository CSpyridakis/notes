# Portainer

## Installation
See: 
https://community-scripts.github.io/ProxmoxVE/scripts?id=docker
https://www.youtube.com/watch?v=S8WCtqTMeO8

1. Open Proxmox Shell and run: 
```
bash -c "$(wget -qLO - https://github.com/community-scripts/ProxmoxVE/raw/main/ct/docker.sh)"
```
## Settings:
* a. Use advanced settings 
* b. Distribution: ubuntu
* c. Ubuntu version: 22.04 Jammy
* d. Container type: Unprivileged
* e. Hostname: portainer

2. Open Portainer shell and run
```
bash -c "$(wget -qLO - https://github.com/community-scripts/ProxmoxVE/raw/main/ct/docker.sh)"
```

### Reboot Portainer twice a day

1. Create a new cron job: `crontab -e`
2. `0 0,12 * * * pct stop [VMID] && pct start [VMID]`, Where VMID is the ID of your LXC container