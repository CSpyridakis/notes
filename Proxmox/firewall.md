# Firewall

Different Firewall levels
- Datacenter
- Node
- VM/CT

> [!IMPORTANT]
> In order to work each level, the firewall for this layer should be enabled! 

Default Interface: `vmbr0`

> [!CAUTION]
> If Datacenter firewall rules is not set properly, access to Proxmox web UI can break.
> 
> E.g. By enabling Datacenter firewall without adding rules
>
> To fix this issue (if happens): 
> 1. Connect via CLI
> 2. Edit `/etc/pve/firewall/cluster.fw`. Set `enable: 0`
>
> To prevent it, add a Datacenter Rule:
> - `Direction`: In
> - `Action`: ACCEPT
> - `Interface`: Give your interface
> - `Enable`: [x]
> - `Protocol`: tcp
> - `Dest. port`: 8006

> [!TIP]
> You may want to enable also icmp protocol in Datacenter level, so you are able to ping the server.

## Add rule
Select the component (Datacenter, Node or VM/CT) > `Firewall` > `Add`
- `Direction`: In, our or Forward
- `Action`: ACCEPT, REJECT or DROP
- `Interface`: The target interface
- `Enable`: If it is enabled
- `Protocol`: Protocols like tcp, udp, icmp
- `Macro`: Predefined macros like SSH
- `Source`: Use CIDR format (/32 for single IP)

## Enable/Disable Firewall
Select the component (Datacenter, Node or VM/CT) > `Firewall` > `Options` > `Firewall` > `Edit` > Set to on/off