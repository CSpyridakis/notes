# see: https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/instance

# Automatically detect AMI
data "aws_ami" "ami_ubuntu" {
  most_recent = true

  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-amd64-server-*"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }

  owners = ["099720109477"] # Canonical
}

# Having found an AMI create the EC2
resource "aws_instance" "ec2_ubuntu_instance" {
  ami           = data.aws_ami.ami_ubuntu.id
  instance_type = "t2.micro"
  associate_public_ip_address = true
  
  # Will use this key pair to connect
  key_name = "tf_key_pair"              # You need to have this key pair available in your AWS console 

  tags = {
    Name = "ec2-tf-ubuntu-experiments"
  }
}

# Specify also a security group
# resource "aws_security_group" "security_group_ec2" {
#   name   = "sg-tf-ubuntu-experiments"
#   vpc_id = aws_vpc.main.id

#   # Allow HTTPS from everywhere
#   ingress = {  
#     descri = ""
#     from_port   = 443
#     to_port     = 443
#     protocol    = "tcp"
#     cidr_blocks = ["0.0.0.0/0"]
#   }

#   # Allow HTTPS from everywhere
#   ingress = {  
#     descri = ""
#     from_port   = 443
#     to_port     = 443
#     protocol    = "tcp"
#     cidr_blocks = ["0.0.0.0/0"]
#   }
#   egress  = []
# }

# output "output_ubuntu_insance" {
#   connection_ubuntu_insance = "ssh -i ~/.ssh/tf_key_pair.pem ubuntu@${ubuntu_insance}"
# }
