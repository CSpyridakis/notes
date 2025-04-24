# General Proxmox settings
variable "pm_api_url" {
    description = "Proxmox API URL, e.g. https://X.X.X.X:8006/api2/json"
    type        = string
}

variable "pm_api_token_id" {
    description = "Proxmox API Token ID, e.g. terraform@pam!token"
    type        = string
}

variable "pm_api_token_secret" {
    description = "Secret for the Proxmox API Token"
    type        = string
    sensitive   = true
}

variable "pm_target_node" {
    description = "Target Proxmox node name to deploy the VM"
    type        = string
}

variable "network_bridge" {
    description = "Network bridge to connect VMs (e.g., vmbr0)"
    type        = string
}

# Template names (must exist in Proxmox)
variable "template_vm" {
    description = "Template name for VM"
    type        = string
}
