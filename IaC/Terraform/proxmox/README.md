# Terraform - Proxmox

## What we will need
1. A Working Proxmox VE Datacenter
2. A Connection URL (`https://<ip-or-domain>:8006/api2/json`)
3. A VM template
4. An API Token

## Generate terraform credentials
1. Login to Proxmox Web UI.
2. Create Terraform Role
   `Datacenter` > `Roles` > `Create` > `Name`: **TerraformRole**
   This is the **best practice**, for temp tests you can also use `Administrator` even though it is not recommended. Make sure that the role will have the following `Privileges`:
   - `Datastore.AllocateSpace`
   - `Datastore.Audit`
   - `Pool.Allocate`
   - `SDN.Use`
   - `Sys.Audit`
   - `Sys.Console`
   - `Sys.Modify`
   - `Sys.PowerMgmt`
   - `VM.Allocate`
   - `VM.Audit`
   - `VM.Clone`
   - `VM.Config.CDROM`
   - `VM.Config.CPU`
   - `VM.Config.Cloudinit`
   - `VM.Config.Disk`
   - `VM.Config.HWType`
   - `VM.Config.Memory`
   - `VM.Config.Network`
   - `VM.Config.Options`
   - `VM.Migrate`
   - `VM.Monitor`
   - `VM.PowerMgmt`
   Or run 
   ```bash
   pveum role add TFRole -privs "Datastore.Allocate Datastore.AllocateSpace Datastore.AllocateTemplate Datastore.Audit Pool.Allocate Sys.Audit Sys.Console Sys.Modify SDN.Use VM.Allocate VM.Audit VM.Clone VM.Config.CDROM VM.Config.Cloudinit VM.Config.CPU VM.Config.Disk VM.Config.HWType VM.Config.Memory VM.Config.Network VM.Config.Options VM.Migrate VM.Monitor VM.PowerMgmt User.Modify"
   ``` 
3. Create Terraform Group (this will have the **TerraformRole** permissions)
   `Datacenter` > `Groups` > `Create` > `Name`: **terraformgroup**
   Or run
   ```bash
   pveum groupadd terraformers --comment "Terraform automation group"
   ```
4. Assign Role to Group
   `Datacenter` > `Permissions` > `Add` > `Group Permission`
   - `Path`: `/`
   - `Group`: Your new user (e.g. `terraform@pve`)
   - `Role`: `TerraformRole`
   Or run
   ```bash
   pveum aclmod / -group terraformers -role TFRole
   ```
5. Create Terraform Users
   `Datacenter` > `Permissions` > `Users` > `Add`
   - `User name`: `terraform`.
   - `Group`: **terraformgroup**
   - `Realm`: `Linux PAM standard auth`.
   Or run
   ```bash
   pveum useradd <username>@pam --comment "Terraform automation user"
   pveum usermod <username>@pam -group terraformers
   ```
6. Create API token for a User
   `Datacenter` > `Permissions` > `API Tokens` > `Add`
   - `User`: Select the user created
   - `Token ID`: `tf_token`
   - `Privilege Separation`: **Not Selected**
   Or run
   ```bash
   pveum user token add <username>@pam provider --privsep=0
   ``` 
7. A new `Token Secret` will be created. 
   - The `Token ID` should look like this: `terraform@pam!tf_token`
8. Copy both the `Token ID` and the `Secret`. You’ll use them in `terraform.tfvars`. 