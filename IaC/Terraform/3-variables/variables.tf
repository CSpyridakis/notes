# =================================================================
# Ways to Pass Input Variables in Terraform
# 
# There are the following ways to pass input variables:
# 
#   1. Using a `variable` block WITH a default value.
# 
#   2. Using a `variable` block WITHOUT a default value.
#      In this case, during execution, Terraform will prompt for its value.
# 
#   3. Using a `variable` block and passing the value via `-var` during execution.
#       E.g.: $ terraform plan/apply -var="resource_name=value"
# 
#   4. Specifying variable values in a terraform.tfvars file.
#
#       Note 1: You can give the file a different name, but you must specify 
#       it during execution:
#       $ terraform plan/apply -var-file="path/to/your/varfile.tfvars"
#       
#       Note 2: Alternatively, you can create *.auto.tfvars files.
#       This is useful for different environments like prod/dev
#
#   5. By creating an Environment variable titled: `TF_VAR_<name>`
#       THIS IS THE BEST PRACTICE FOR MANAGING AUTHENTICATION/CREDENTIALS
# -----------------------------------------------------------------
#
# Variable Precedence: 
#   The priority that will be followed if a variable is specified in multiple ways
#
#   a) -var or -var-file            /\  Highest
#   b) terraform.tfvars             ||
#   c) *.auto.tfvars                ||
#   d) TF_VAR_<name>                ||
#   e) variable.tf: default         \/  Lowest
# =================================================================

# Some examples

# Option 1: 
variable "resource_name" {
    type =  string
    default = "default"
}

# Option 2: 
variable "resource_content" {
    type =  string
}

# Option 3:
# Run: $ terraform apply -var="resource_extra=during-runtime-using-var"
variable "resource_extra" {
    type =  string
    default = "default"
}

# Option 4:
variable "resource_tfvars" {
    type =  string
    default = "default"
}

# Option 4a:
# Run: $ terraform apply -var-file="./custom.tfvars"
variable "resource_custom_tfvars" {
    type =  string
    default = "default"
}

# Option 4b:
variable "resource_auto_tfvars" {
    type =  string
    default = "default"
}

# Option 5:
# Run: $ export TF_VAR_resource_env="This is an ENV var" ; terraform apply
variable "resource_env" {
    type =  string
    default = "default"
}
