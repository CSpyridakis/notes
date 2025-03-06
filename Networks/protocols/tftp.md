# Trivial File Transfer Protocol (TFTP)

> [!NOTE]
> **tftp** protocol typicaly operates on Port: **69** and uses **UDP**

## 1. Install both server and client
```bash
sudo apt update
sudo apt install tftpd-hpa tftp-hpa
```

## 2. Configure tftp server (daemon)
```bash
sudo nano /etc/default/tftpd-hpa
```

It should contain something like:
```bash
TFTP_USERNAME="tftp"
TFTP_DIRECTORY="/srv/tftp"
TFTP_ADDRESS="192.168.0.66:69"
TFTP_OPTIONS="--secure --verbose"
```
Where:
- **TFTP_USERNAME**: The user under which the service runs.
- **TFTP_DIRECTORY**: The directory that will serve as the root for TFTP transfers (make sure this directory exists).
- **TFTP_ADDRESS**: The IP and port (UDP port 69 is standard for TFTP).
- **TFTP_OPTIONS**: The --secure option restricts file access to the TFTP_DIRECTORY.

*If the directory **TFTP_DIRECTORY** doesn’t exist, create it:*
```bash
sudo mkdir -p /srv/tftp
sudo chown -R tftp:tftp /srv/tftp
```

## 3. Ensure Operation
```bash
# Restart the TFTP service
sudo systemctl restart tftpd-hpa

# Check the status of the TFTP service
sudo systemctl status tftpd-hpa

# Confirm that it's listening on the correct IP
sudo netstat -ulnp | grep tftp
```

## 4. Firewall
Make sure that for firewall allows connections
### 4a. Ubuntu based
```bash
sudo ufw allow 69/udp
```

## 5. Verification
In order to evaluate that both the server and client work.

1. Add files in the **TFTP_DIRECTORY** dir.
2. Run `tftp 192.168.0.66`
3. Execute:
    ```bash
    verbose on
    mode binary 
    status
    get <filename>
    ```
4. Make sure that you can receive the file

## 6. Server logs
To view server logs run:

### Using syslog
```bash
sudo tail -f /var/log/syslog | grep tftpd
```

### Using journalctl (if tftpd-hpa is managed by systemd)
```bash
sudo journalctl -u tftpd-hpa -f
```