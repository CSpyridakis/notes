# SSH

---

## Enable SSH Server (Debian)

```bash
sudo apt install openssh-server
sudo systemctl enable ssh
sudo systemctl start ssh
sudo ufw allow ssh
sudo ufw enable
```

## Enable SSH Server (Red Hat)
```bash
sudo dnf install -y openssh-server
sudo systemctl enable sshd
sudo systemctl start sshd
sudo systemctl status sshd
sudo firewall-cmd --permanent --add-service=ssh
sudo firewall-cmd --reload
# sudo firewall-cmd --list-all # Just to comfirm
```

---

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

---

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

---

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

---

## Client Configuration
Check SSH Configuration
`ssh -G <hostname>`

Global configuration filepath
`/etc/ssh/ssh_config`

### SSH client `Include`
In order to include in the `~/.ssh/config` another configuration use the `Include` keyword.

E.g.

```
# In ~/.ssh/config
Include servers/config

Host ...
```

Then in the servers/config
```
Host <server-alias>
  HostName <IP-address-or-domain>
  User <user-name>                  # Not important
  Port <port-number>                # Not important
  IdentityFile ~/.ssh/<file>        # Not important

  IdentitiesOnly yes        # Optional
  IdentityAgent none        # Optional
  AddKeysToAgent no         # Optional
```

---

## Server Configuration
Location `/etc/ssh/sshd_config`

### Enhance server security

#### 1. Change port to another one
```
Port <port_num>
```

#### 2. Disable both password and keyboard-interactive
```
PasswordAuthentication no
ChallengeResponseAuthentication no
UsePAM no
```

#### 3. Disable root connection
```
PermitRootLogin no
```

#### Restart sshd
```
sudo systemctl restart ssh
```

#### Use a firewall
Use a firewall to allow only to specific ports connections.

---

## Other connections
Remote Command Execution
`ssh <user>@<hostname/ip> "<command>"`

---

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

