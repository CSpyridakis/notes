# Load average

How busy is the system.

> [!TIP]
> **Rule of thumb**: Load average should be ≤ number of CPU cores.

## Useful Commands
| Command               | Description                                                                 |
|-----------------------|-----------------------------------------------------------------------------|
| `uptime`              | Shows how long the system has been running, number of users, and load average. |
| `cat /proc/loadavg`   | Displays the same load averages as `uptime`, running / total processes and last PID. |
| `top`                 | Interactive process viewer that also shows load averages and CPU usage.     |
| `nproc`               | Displays the number of processing units (cores) available.                  |


## Output
```bash
load average: 0.02, 0.20, 0.02
# Load average is shown for 1, 5, and 15 minutes.
```

Normalized to the number of CPUs, load 1 per Core.

**How to interpret:**
| Load Avg | Cores (nproc) | Interpretation
| ---------| ------------- | --------------
| 1.0 | 1 | Full CPU usage
| 2.0 | 1 | Overloaded
| 0.5 | 1 | Underloaded (50% utilization)
| 4.0 | 4 | Fully utilized (ideal scenario)
| 6.0 | 4 | Overloaded (may experience latency/slowness)