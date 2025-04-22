# IPMI

[Intelligent Platform Management Interface](https://en.wikipedia.org/wiki/Intelligent_Platform_Management_Interface) is a standard for remote server management, mainly used in data centers and enterprise environments.

IPMI lets you manage a server remotely even when it's 
- Powered off
- Crashed, 
- Or has no OS installed.

It works below the OS level, giving you out-of-band management capabilities.

## How It Works

It runs on a separate microcontroller called the **BMC (Baseboard Management Controller)**.

The BMC operates independently of the main CPU/OS and usually has its own
Network port (or shares one with the server).

## Accessing IPMI

You can use:
- Web Interface (most BMCs have a web UI)
- CLI tools like [ipmitool](https://github.com/ipmitool/ipmitool)
- Redfish (a newer alternative to IPMI)

E.g. 
```
ipmitool -I lanplus -H <IP> -U <user> -P <pass> power status
```

## Examples of IPMI

- [Dell iDRAC](https://www.dell.com/en-us/lp/dt/open-manage-idrac)
- [Supermicro IPMI](https://www.supermicro.com/en/solutions/management-software/bmc-resources)
- [Lenovo IMM](https://lenovopress.lenovo.com/tips0849-imm2-support-on-lenovo-servers)
- [ASRock Rack IPMI](https://www.asrockrack.com/support/IPMI.pdf)

