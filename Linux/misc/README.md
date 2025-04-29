# General misc notes

## > Default CLI editor
To set the default editor set this ENV var
```bash
export EDITOR=vim
```

or
```bash
echo "export EDITOR=vim" >> ~/.bashrc && source ~/.bashrc
```

## `sudoedit` vs `sudo vim`


## > Laptop LID

[Source](https://askubuntu.com/questions/15520/how-can-i-tell-ubuntu-to-do-nothing-when-i-close-my-laptop-lid)

Action to do when closing the Laptop lid (Do nothing):
1. Edit: `sudoedit /etc/systemd/logind.conf`
2. Then make sure that this configuration is available: `HandleLidSwitch=ignore`
3. Restart daemon `sudo systemctl restart systemd-logind`

## > `sudoedit` vs `sudo -e` vs `sudo vim`

| Command       | Editor runs as | Safe? | Uses `$EDITOR` | Temporary copy? | Recommended |
|---------------|----------------|--------|----------------|------------------|-------------|
| `sudoedit`    | User       | Yes | Yes         | Yes           | Yes      |
| `sudo -e`     | User       | Yes | Yes         | Yes           | Yes      |
| `sudo vim`    | Root       | No  | No          | No            | No       |

> [!CAUTION]
> Always use `sudoedit` or `sudo -e` for editing system files.

> [!NOTE]
> `sudo -e` is just a different syntax/alias provided by `sudoedit`