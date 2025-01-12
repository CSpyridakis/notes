
List keys
`ssh-add -l`

Generate new key
`ssh-keygen -t rsa -b 4096 -C <email@domain.com>`

Copy key to remote
`ssh-copy-id -i $<key_path> <user>@<hostname/ip>`

Use a specific key during connection
`ssh -o "IdentitiesOnly=yes" -i <key_path> -p <port> <user><hostname/ip>`

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

Check SSH Configuration
`ssh -G <hostname>`

Remote Command Execution
`ssh <user>@<hostname/ip> "<command>"`

Local Port Forwarding (Forward a port from the remote machine to your local machine)
`ssh -L <local_port>:<remote_host>:<remote_port> -N <user>@<hostname/ip>` # Use -N if you don’t need a shell session

Remote Port Forwarding: (Forward a port from your local machine to a remote machine)
`ssh -R <remote_port>:<local_host>:<local_port> -N <user>@<hostname/ip>` # Use -N if you don’t need a shell session

Create an SSH SOCKS proxy for dynamic port forwarding
`ssh -D <local_port> <user>@<hostname>`

Global configuration filepath
`/etc/ssh/ssh_config`

--- 

Use ssh key on Github

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
```
5. Evaluate that it is working by running `ssh -T git@github.com`
6. For security run `chmod 400 ~/.ssh/<keyname>`