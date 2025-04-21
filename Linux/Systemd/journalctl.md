# Systemd journal

The **journal** in Linux refers to the logging system used by **systemd**.

It collects logs from the **kernel**, **system services**, **applications**, and **users**, all in one place.

These logs are stored in a **binary** format (not plain text like older log files in **/var/log**), and they can be accessed using the **journalctl** command.

`systemd-journald` component caprutes logs, `journalctl` tool is used to read these logs.

---

## Features of systemd Journal
| Feature | Description |
| ------- | ----------- | 
| Structured logs | Entries have metadata: timestamps, PIDs, unit names, etc.
| Binary format | Faster and more efficient, but not human-readable without journalctl
| Persistent storage | Can be configured to persist across reboots (see below)
| Filtering/searching | Powerful built-in filtering by time, service, priority, etc.
| Security | Logs can be restricted to root or specific users

---

## Utilization
To activate journal just add this part in your .service file.
```
[Service]
StandardOutput=journal
StandardError=journal
```
This ensures that anything printed by the service to stand streams (e.g., echo, print, stderr) goes into the systemd journal, accessible via journalctl.

---

## Making Journal Logs Persistent

By default, logs might only be stored in memory.

To keep them between reboots:
```
sudo mkdir -p /var/log/journal
sudo systemd-tmpfiles --create --prefix /var/log/journal
sudo systemctl restart systemd-journald
```

Now logs will be saved under `/var/log/journal/` and survive reboots.

---

## Common commands
- Show all logs (from oldest to newest): `journalctl`
- List all boot processes: `journalctl --list-boots`
- Show logs from the current boot: `journalctl -b # <boot-idx>`
- Show logs for a specific unit: `journalclt -u <unit-name>`
- Follow live log updates: `journalclt -f`
- Show logs since some date: `journalclt --since "YYYY-MM-DD HH:MM:SS"` 
- Show logs until some date: `journalclt --until "YYYY-MM-DD HH:MM:SS"` 
- Show logs filtered by an identifier: `journalctl -t <identifier>`
- Show logs for specific user: `journalctl _UID=<userid>`
- Show the most recent logs with details: `journalctl -xe`
- Send message to journalctl: `echo "message" | systemd-cat -t 'identifier'`