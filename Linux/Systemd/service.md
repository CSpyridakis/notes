# Systemd Service

A **service** in systemd is a type of unit file that tells the system how to **start**, **stop**, **reload**, and manage a background **service** or **process**, e.g. a web server, database, or custom script.

---

## Service file
Service file example. An enhanced [.service](./service-template.service) example is located here.
```
[Unit]
# Contains a more general configuration for the unit

Description=Some description
Documentation=Link or man page


[Service]
# Defines how the service should behave, like start/stop/reload commands executed

Type=simple
ExecStart=<path/to/the/target/executable>

[Install] 
# Install ~= enable/disable
# In which target we will enable the service if the service is enabled
WantedBy=<target-name>.target
```

