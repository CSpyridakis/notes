# Systemd Timer

A **timer** unit in systemd is like a modern, more powerful version of cron. 

It allows you to schedule tasks (services) to run:
- At specific times or intervals
- After system boot
- After a certain delay

**Timers** work in combination with **.service** files, the timer triggers the service.

Example:
`backup.timer` → triggers `backup.service`

---

## Timer creation

### A. Create service
```
# in /etc/systemd/system/my-dummy-timer.service
[Unit]
Description=Some Description for service

[Service]
Type=oneshot
ExecStart=<path/to/the/startup/binary>
```
Use Type=oneshot for short-lived tasks like scripts or backup jobs.

### B. Create timer

```
# in /etc/systemd/system/my-dummy-timer.timer
[Unit]
Description=Some Description for timer

[Timer]
 
# Optional if timer and service share the same name
Unit=my-dummy-timer.service

<TIMER_OPTION>=<value>  # See below

[Install]
WantedBy=timers.target
```

| **TIMER_OPTION**        | **Example**                  | **Description**                                                              |
|-------------------------|------------------------------|------------------------------------------------------------------------------|
| `OnCalendar`            | `*-*-* 02:00:00` **\***           | Run at a specific date/time (daily at 2:00 AM in this case)                 |
| `OnCalendar`            | `Mon *-*-* 09:00:00` **\***       | Run every Monday at 9:00 AM                                                 |
| `OnBootSec`             | `10min`                      | Run a service 10 minutes after the system boots                             |
| `OnStartupSec`          | `5min`                       | Run 5 minutes after systemd initialization completes                        |
| `OnUnitActiveSec`       | `1h`                         | Run 1 hour after the unit was last activated                                |
| `OnActiveSec`           | `1min`                       | Run 1 minute after the timer was last activated                             |
| `AccuracySec`           | `1s`                         | Defines how accurate the timer should be (default: 1min)                    |
| `RandomizedDelaySec`    | `30min`                      | Add random delay to spread load (useful in clusters or scripts)             |
| `Persistent`            | `true`                       | Run missed executions when the system was powered off at scheduled time     |

**\*** These follow the format that is available during `systemd-analyze timestamp now` execution.
As an example we can use:
```
systemd-analyze calendar '*-*-* *:0,10,20,30,40,50:00'
```
To verify the values passed.

Other available options:
- `hourly`
- `minutely`

---

## Common commands
- List active timers: `sudo systemctl list-timers`
