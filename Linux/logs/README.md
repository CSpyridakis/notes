# Logs

Most of the logs exist under `/var/log/` dir.


**Binary log files:**
| Binary file | Command to read log | Distro | Description |
| ----------- | ------------------- | ------ | ----------- | 
| **/var/log/wtmp** |  `last <username>` | RH/Debian |  Shows the login history of users | 
| **/var/log/lastlog** |  `lastlog`  | RH/Debian |  Displays the most recent login of all users |
| **/var/log/btmp** | `lastb -adF` | RH/Debian |  Bad login attempts (Check malicious activity) |


**Text log files:**
| Text file | Distro | Description | Tricks/Extra info |
| --------- | ------ | ----------- | ------ | 
| **/var/log/auth.log** | Debian | Authorization attempts | Can be used to trubleshoot SSH login attempts that fail | 
| **/var/log/secure** | RH | Authorization attempts  | * Not enabled by default in some distros, need to use journalctl
| **/var/log/syslog** | Debian | System messages and logs |
| **/var/log/messages** | RH |  System messages and logs | 