# Create bucket
resource "aws_s3_bucket" "tf_s3_private_bucket" {
  bucket = "this-example-private-bucket-1234321"

  tags = {
    Name        = "My bucket"
    Environment = "Dev"
  }
}

# Upload all files from s3-files/ directory
resource "aws_s3_object" "tf_s3_private_objects" {
  # This is the name of the bucket
  bucket = aws_s3_bucket.tf_s3_private_bucket.bucket

  # Parse each file in the ./s3-files/ dir
  for_each = fileset("./s3-files", "**")

  # The name of the file when uploaded
  key    = "${each.key}"    # The name of each file
  
  # The path of the file
  source = "./s3-files/${each.key}"
}