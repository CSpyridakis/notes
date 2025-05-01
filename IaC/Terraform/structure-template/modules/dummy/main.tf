resource "null_resource" "example" {
  count = var.example_count

  provisioner "local-exec" {
    command = "echo Hello from ${var.example_name}!"
  }
}
