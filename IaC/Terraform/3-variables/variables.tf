# This is an input variable
variable "resource_name" {
    type =  string
    default = "file-name"   # If you comment out this line, then during `terraform apply` a prompt will ask for its value
}