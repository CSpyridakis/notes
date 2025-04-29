# see: https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/instance

# ======================================================
# Automatically detect AMI
# ======================================================

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

# ======================================================
# Create key pair
# ======================================================
resource "aws_key_pair" "ec2_ubuntu_instance_key_pair" {
  key_name   = var.ec2_key_pair_name
  public_key = var.ec2_key_pair_public_key
}

# ======================================================
# Having found an AMI create the EC2
# ======================================================

resource "aws_instance" "ec2_ubuntu_instance" {
  ami                         = data.aws_ami.ami_ubuntu.id
  instance_type               = "t2.micro"
  associate_public_ip_address = true
  availability_zone           = "us-east-1a"

  # Will use this key pair to connect
  key_name = var.ec2_key_pair_name # You need to have this key pair available in your AWS console  (see above)

  # Specify the security group
  vpc_security_group_ids = [aws_security_group.ec2_ubuntu_instance_security_group.id]

  tags = {
    Name    = "ec2-tf-ubuntu-experiments"
    Project = "TF-experiments"
  }
}

# ======================================================
# Specify Security Group
# ======================================================

resource "aws_security_group" "ec2_ubuntu_instance_security_group" {
  name        = "ubuntu-experiments-tf-sg"
  description = "EC2 Ubuntu TF Instance ingress and egress rules"

  tags = {
    Name = "ec2_ubuntu_instance_rules"
  }
}

# ======================================================
# Security Group rules
# ======================================================

# SSH
resource "aws_vpc_security_group_ingress_rule" "ec2_ubuntu_allow_ssh" {
  security_group_id = aws_security_group.ec2_ubuntu_instance_security_group.id
  cidr_ipv4         = "0.0.0.0/0" # If you need a specific IP, modify this 
  from_port         = 22
  ip_protocol       = "tcp"
  to_port           = 22
}

resource "aws_vpc_security_group_ingress_rule" "ec2_ubuntu_allow_tls_ipv4" {
  security_group_id = aws_security_group.ec2_ubuntu_instance_security_group.id
  cidr_ipv4         = "0.0.0.0/0"
  from_port         = 443
  ip_protocol       = "tcp"
  to_port           = 443
}

resource "aws_vpc_security_group_ingress_rule" "ec2_ubuntu_allow_tls_ipv6" {
  security_group_id = aws_security_group.ec2_ubuntu_instance_security_group.id
  cidr_ipv6         = "::/0"
  from_port         = 443
  ip_protocol       = "tcp"
  to_port           = 443
}

resource "aws_vpc_security_group_ingress_rule" "ec2_ubuntu_allow_http_ipv4" {
  security_group_id = aws_security_group.ec2_ubuntu_instance_security_group.id
  cidr_ipv4         = "0.0.0.0/0"
  from_port         = 80
  ip_protocol       = "tcp"
  to_port           = 80
}

resource "aws_vpc_security_group_ingress_rule" "ec2_ubuntu_allow_http_ipv6" {
  security_group_id = aws_security_group.ec2_ubuntu_instance_security_group.id
  cidr_ipv6         = "::/0"
  from_port         = 80
  ip_protocol       = "tcp"
  to_port           = 80
}

resource "aws_vpc_security_group_egress_rule" "ec2_ubuntu_allow_all_traffic_ipv4" {
  security_group_id = aws_security_group.ec2_ubuntu_instance_security_group.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "-1" # semantically equivalent to all ports
}

resource "aws_vpc_security_group_egress_rule" "ec2_ubuntu_allow_all_traffic_ipv6" {
  security_group_id = aws_security_group.ec2_ubuntu_instance_security_group.id
  cidr_ipv6         = "::/0"
  ip_protocol       = "-1" # semantically equivalent to all ports
}
