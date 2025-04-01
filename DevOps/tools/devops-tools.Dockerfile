FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies
RUN apt-get update && \
    apt-get install -y \
        curl \
        unzip \
        gnupg \
        lsb-release \
        software-properties-common \
        python3 \
        python3-pip \
        sudo \
        git \
        ssh \
        vim \
        tmux \
        iputils-ping \
        ca-certificates \
        apt-transport-https && \
    rm -rf /var/lib/apt/lists/*

# Install AWS CLI v2
RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
    unzip awscliv2.zip && \
    ./aws/install && \
    rm -rf awscliv2.zip aws

# Install Ansible
RUN pip3 install --no-cache-dir ansible

# Install Terraform
RUN curl -fsSL https://apt.releases.hashicorp.com/gpg | gpg --dearmor -o /usr/share/keyrings/hashicorp.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/hashicorp.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" \
    > /etc/apt/sources.list.d/hashicorp.list && \
    apt-get update && \
    apt-get install -y terraform packer && \
    rm -rf /var/lib/apt/lists/*
# Set an alias for tf="terraform"
RUN echo 'alias tf="terraform"' >> ~/.bashrc

# Install kubectl
RUN curl -LO "https://dl.k8s.io/release/$(curl -sL https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl" && \
    install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl && \
    rm kubectl

# Install Helm
RUN curl https://baltocdn.com/helm/signing.asc | gpg --dearmor -o /usr/share/keyrings/helm.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/helm.gpg] https://baltocdn.com/helm/stable/debian/ all main" \
    > /etc/apt/sources.list.d/helm-stable-debian.list && \
    apt-get update && \
    apt-get install -y helm && \
    rm -rf /var/lib/apt/lists/*

# Add user and group with UID and GID 1000
RUN groupadd -g 1000 devops && \
    useradd -m -u 1000 -g devops -s /bin/bash devops
# Set an alias for tf="terraform"
RUN echo 'alias tf="terraform"' >> /home/devops/.bashrc

# Optional: add sudo access if needed
RUN echo "devops ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set default shell to bash
SHELL ["/bin/bash", "-c"]

# Default command
CMD ["bash"]

