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
    EOT
}

# This is an output variable
# This will print in the console during `terraform apply` the filename
output "filename" {
    value = local_file.foo.filename   # ->  provider.identifier.resource
}


