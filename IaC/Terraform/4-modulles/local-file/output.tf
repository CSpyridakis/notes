output "output_filename" {
  description = "File name"
  value       = local_file.foo.filename
  sensitive   = false
}

output "output_filecontext" {
  description = "File filecontext"
  value       = local_file.foo.content
  sensitive   = true
}

