resource "local_file" "foo" {
    count = 2   # Number of files

    filename = "${path.module}/dummy-file-${count.index}"
    content  = "This file contains dummy data ${count.index}"
}