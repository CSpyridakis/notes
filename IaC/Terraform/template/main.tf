# CAUTION: THIS IS NOT A WORKING .TF FILE. IT SHOULD ONLY BE USED AS A
# REFFERENCE TO DESCRIBE TERRAFORM CONCEPTS

# ==================================================================
# Initial Terraform setup
# ==================================================================

# When `terraform init` is executed, this part is getting set up

terraform {
    # Here include all the needed providers
    required_providers {
        # As an example we will use here the local provider and the AWS provider

        # A. Local provider
        local = {
            source = "haspicorp/local"
            version = "2.5.1"
        }

        # B. AWS provider
        aws = {
            source = "hashicorp/aws"
            version = "5.65.0"
        }
    }
}

# ==================================================================
# Configure providers
# ==================================================================

# A. Configure local provider
provider "local" {

}

# B. Configure AWS provider
provider "aws" {
    region = "us-east-1"
}

# ==================================================================
# Configure Resources
# ==================================================================

# Syntax: 
# resource "type" "resource_name" {
# 
# }
# Remember `resource_name` is only a reference to Terraform, not the
# actual resource name.

resource "aws_instance" "web" {
    ami             = "ami-someami"
    instance_type   = "t2.micro"
}