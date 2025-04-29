variable "ec2_key_pair_name" {
  description = "The name of the key pair that we will use to connect to the EC2"
  type        = string
  default     = ""
}

variable "ec2_key_pair_public_key" {
  description = "The key pair public value"
  type        = string
  default     = ""
}