#!/bin/bash

# Update system
yum update -y

# Install HTTP server
yum install -y httpd
systemctl enable httpd
systemctl start httpd

# Add a dummy page
echo "<p>If you are here, then Congrats! There is a working http server on your machine.</p>" \
    > /var/www/html/index.html


