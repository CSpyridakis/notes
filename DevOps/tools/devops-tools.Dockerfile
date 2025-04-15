FROM ubuntu:22.04

ARG BUILDDATE
ARG VERSION

LABEL maintainer="Spyridakis Christos"
LABEL version="${VERSION}"

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
        sshpass \
        whois traceroute speedtest-cli gdu clamav clamtk tree figlet ranger iperf3 \
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

# --------------------------------------------------------------------
# Add a copy of my personal notes and tools repositories
# This can be handy to set quicky a ready to run experiments environment
# based on my needs. So I can have a quick reference
WORKDIR /tools-and-notes
RUN git clone https://github.com/CSpyridakis/notes.git
RUN git clone https://github.com/CSpyridakis/tmux-monitor-theme.git
RUN git clone https://github.com/CSpyridakis/dockerfiles.git
RUN git clone https://github.com/CSpyridakis/modelfiles.git
RUN git clone https://github.com/CSpyridakis/dotfiles.git
RUN git clone https://github.com/CSpyridakis/SCTT.git
RUN git clone https://github.com/CSpyridakis/homelab.git
RUN git clone https://github.com/CSpyridakis/homepage.git

# "Install" tmux monitor theme for root user
RUN wget -O /root/.tmux.conf https://gist.githubusercontent.com/CSpyridakis/0c2782d2f91f57a7d5d46ef5abf58dad/raw/45f58afe2e3992f7ba9022e9e9d7ebb10a56ca15/tmux-monitor-theme-minimal

# "Install" tmux monitor theme for devops user
WORKDIR /home/devops/
RUN wget -O /home/devops/.tmux.conf https://gist.githubusercontent.com/CSpyridakis/0c2782d2f91f57a7d5d46ef5abf58dad/raw/45f58afe2e3992f7ba9022e9e9d7ebb10a56ca15/tmux-monitor-theme-minimal

# If TMUX is not set in this terminal, then activate it by default!
RUN echo 'if [ "$TMUX" = "" ]; then tmux; fi' >> /home/devops/.bashrc

# --------------------------------------------------------------------

LABEL buildDate="${BUILDDATE}"

# Default command
CMD ["bash"]