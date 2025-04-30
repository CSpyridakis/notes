# HashiCorp Vault

> [!CAUTION]
> THIS DOCUMENT HAS YET TO BE VALIDATED! ONLY NOTES HAVE BEEN TAKEN!
> BEFORE TAKING FOR FACT, NEED TO TEST IT! 

Using HashiCorp Vault to manage secrets for Terraform is a secure and scalable approach.

---

## 1. Install and Start HashiCorp Vault
**Debian**
```bash
sudo apt update
sudo apt install vault
```

Dev mode (for testing only):
```bash
vault server -dev
```

Expected output
```
Root Token: s.your-root-token
```

## 2. Set Up Vault CLI Access

Export environment variables to interact with Vault:
```bash
export VAULT_ADDR='http://127.0.0.1:8200'
export VAULT_TOKEN='s.your-root-token'
```

## 3. Store Your Secret (e.g. proxmox)
```bash
vault kv put secret/proxmox password='your-secure-password'
```

To retrieve manually:
```bash
vault kv get secret/proxmox
```

## 4. Use Vault in Terraform
### Step 1: Add Vault Provider
```yml
terraform {
  required_providers {
    vault = {
      source  = "hashicorp/vault"
      version = "~> 3.0"
    }
  }
}
```

### Step 2: Declare the Vault Provider

```yml
provider "vault" {
  address = "http://127.0.0.1:8200"
  token   = var.vault_token
}
```

### Step 3: Fetch Secret from Vault

```yml
data "vault_kv_secret_v2" "proxmox_creds" {
  mount = "secret"
  name  = "proxmox"
}
```

### Step 4: Use the Secret

```yml
provider "proxmox" {
  pm_api_url      = var.proxmox_api_url
  pm_user         = var.proxmox_user
  pm_password     = data.vault_kv_secret_v2.proxmox_creds.data["password"]
  pm_tls_insecure = true
}
```

### Step 5: Add to variables.tf

```yml
variable "vault_token" {
  description = "Vault token"
  sensitive   = true
}
variable "proxmox_api_url" {}
variable "proxmox_user" {}
```

### Summary of Best Practices

Never hardcode credentials
Use Vault with versioned secrets
Mark all secret variables as sensitive
Use Vault Agent or Kubernetes auth for automation later