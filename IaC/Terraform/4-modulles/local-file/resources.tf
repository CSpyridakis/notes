resource "local_file" "foo" {
  filename = var.filename
  content  = var.filecontext
}