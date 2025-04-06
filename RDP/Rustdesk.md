# Rustdesk

RustDesk is a free, open-source remote desktop application that allows users to access and control computers remotely.

Download Rustdesk from [here](https://github.com/rustdesk/rustdesk/releases/tag/1.3.9)

## Internal LAN access
Make sure that you have enabled the following setting.
`Settings` > `Security` > `Security` > [x] `Enable direct IP access`

## Login screen
Based on [this](https://rustdesk.com/docs/en/client/linux/#login-screen), Wayland is not supported.
Hence, **uncomment** `WaylandEnable=false` in `/etc/gdm/custom.conf` or `/etc/gdm3/custom.conf`, then reboot.