# This is a local variable
locals {
    files_directory = "${path.module}"
    file_extension  = "txt"
}

resource "local_file" "foo" {
    filename = "${local.files_directory}/${var.resource_name}.${local.file_extension}"
    
    # This is also a way to add multiline content
    content  = <<EOT
    Hey there, this is a multiline
    Content, inside the ${var.resource_name} file
    
    ------------------------------------------
    Option 1:
        - resource_name: ${var.resource_name}
    Option 2:
        - resource_content: ${var.resource_content}
    Option 3:
        - resource_extra: ${var.resource_extra}
    Option 4:
        - resource_tfvars: ${var.resource_tfvars}
        - resource_custom_tfvars: ${var.resource_custom_tfvars}
        - resource_auto_tfvars: ${var.resource_auto_tfvars}
    Option 5:
        - resource_env: ${var.resource_env}
    
    EOT
}

# This is an output variable
# This will print in the console during `terraform apply` the filename
output "filename" {
    description = "This is an output variable"
    value = local_file.foo.filename   # ->  provider.identifier.resource
    sensitive = false
}


