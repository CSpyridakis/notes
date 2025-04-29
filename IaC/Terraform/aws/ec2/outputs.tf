output "output_ubuntu_instance" {
  value = "ssh -i ~/.ssh/${var.ec2_key_pair_name} ubuntu@${aws_instance.ec2_ubuntu_instance.public_ip}"
}