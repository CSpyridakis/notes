terraform {
    # ===============================================
    # Specify here the backend
    # ===============================================

    # Option 1: Local Backend (default)
    backend "local" {
        path = "./statefile/terraform.tfstate"
    }

    # Option 2: Remote Backend (e.g. S3)
    # backend "s3" {
    #     region  = "us-east-1"
    #     bucket  = "bucket-name"
    #     key     = "path/to/key"
    # }
    # ===============================================

    required_providers {
        local = {
        source = "hashicorp/local"
        version = "2.5.2"
        }
    }
}

provider "local" {
    # Configuration options
}