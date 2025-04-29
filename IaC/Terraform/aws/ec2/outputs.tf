output "output_ubuntu_instance" {
  value = [
    for instance in aws_instance.ec2_ubuntu_instance : 
    "ssh -i ~/.ssh/${var.ec2_key_pair_name} ubuntu@${instance.public_ip}"
  ]
}