resource "local_file" "foo" {
    filename = "${path.module}/dummy-file"
    content  = "This file contains dummy data"
}