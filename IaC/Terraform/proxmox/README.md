# Terraform - Proxmox

## Generate an API token
1. Login to Proxmox Web UI.
2. `Datacenter` > `Permissions` > `Users` > `Add`
   - `User name`: `terraform`.
   - `Realm`: `Proxmox VE authentication`.
3. `Datacenter` > `Permissions` > `Add` > `User Permission`
   - `Path`: `/`
   - `User`: Your new user (e.g. `terraform@pve`)
   - `Role`: `Administrator`
4. `Datacenter` > `Permissions` > `API Tokens` > `Add`
   - `User`: Select the user created
   - `Token ID`: `tf_token`
   - `Privilege Separation`: **Not Selected**
5. A new `Token Secret` will be created. 
   - The `Token ID` should look like this: `terraform@pve!tf_token`
6. Copy both the `Token ID` and the `Secret`. You’ll use them in `terraform.tfvars`. 