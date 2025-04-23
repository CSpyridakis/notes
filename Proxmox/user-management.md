# User management

Default user: `root@pam`

## Realm
Proxmox VE supports multiple user authentication realms, which define how users are authenticated when accessing the system. Each realm type serves different use cases, ranging from local user management to centralized directory services

| Realm                  | pam                                      | pve                                                   |
|------------------------|------------------------------------------|--------------------------------------------------------|
| Description            | Linux PAM (Proxmox VE)                   | Proxmox VE Authentication Server                       |
| Login name example     | root@pam                                 | admin@pve                                              |
| Authentication method  | Uses system users on the Proxmox host (/etc/passwd and /etc/shadow) | Users managed within the Proxmox web interface (not system users) |
| When to use            | 1. For local administration, especially for the root user. 2. When you don't have or need external authentication. 3. Ideal for initial setup or single-node deployments. | 1. For managing users separately from the OS. 2. Useful in multi-user environments where you want non-root users with different permissions. 3. Allows granular permission assignment using Proxmox's roles and ACLs.
| Limitations            | 1. Harder to manage in large environments. 2. Not centralized, i.e. credentials only work on the node they're created on. | Credentials stored locally, not integrated with external systems.
| **Common Use Cases**        | **Access via SSH the server**  | **Manage VMs via web interface**
| Notes                       | 1. It is not mandatory to add **pam** users in the proxmox web ui. 2. **pam** users are not created by default on the underlying devices. | 

---

## Groups

Groups will handle User permissions

### Create Group

`Datacenter` > `Permissions` > `Groups` > `Create`

### Give Permissions

`Datacenter` > `Permissions` > `Add` > `Group Permission`

- `Path`: The resources that is can access
- `Group`: The desired Group
- `Role`: The permissions that this Group has on the given resource (Path)
 
---

## Users

### Create User

`Datacenter` > `Permissions` > `Users` > `Add`

> [!NOTE]
> For **pam** users, remember that an extra step is required
> Create on the underlying device the user
>
> Open the CLI of the target device and run
> ```
> useradd -m -s /bin/bash <username>
> passwd <username>
> ```

