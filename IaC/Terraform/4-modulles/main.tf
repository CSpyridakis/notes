terraform {
  required_providers {
    local = {
      source  = "hashicorp/local"
      version = "2.5.2"
    }
  }
}
# ========================================================
# Use the module
module "create_local_file" {
  # Where the module is located  
  source = "./local-file"

  # Modify internal variables
  filename    = "${path.module}/use-module-filename.txt"
  filecontext = "use-module-content"
}

# It can also be reused
module "create_local_file2" {
  # Where the module is located  
  source = "./local-file"

  # Modify internal variables
  filename    = "${path.module}/second.txt"
  filecontext = "use-module-content-2"
}

# ========================================================
# Finally can use outputs from the module
output "local" {
  value = [
    module.create_local_file.output_filename,
    module.create_local_file.output_filecontext,
  ]
}