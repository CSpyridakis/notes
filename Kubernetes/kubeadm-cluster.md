# Create K8s Cluster
> [!CAUTION]
> Kubernetes version used: **v1.33.0**


## A. Common
These steps must be executed on all nodes. Both control plane (master) and worker nodes.

### A1. Install dependencies
```bash
sudo apt install -y curl apt-transport-https ca-certificates gpg
```

### A2. Disable swap
```bash
sudo swapoff -a
sudo sed -i.bak '/\sswap\s/s/^/#/' /etc/fstab
```

### A3. Install containerd
> [!NOTE]
> Kubernetes removed native Docker support in version 1.24 and above. 
> More info [here](https://kubernetes.io/blog/2020/12/02/dont-panic-kubernetes-and-docker/)

```bash
sudo apt update
sudo apt install -y containerd
sudo mkdir -p /etc/containerd
containerd config default | sudo tee /etc/containerd/config.toml > /dev/null
sudo sed -i 's/SystemdCgroup = false/SystemdCgroup = true/' /etc/containerd/config.toml
sudo systemctl restart containerd
sudo systemctl enable --now containerd
```

### A4. Install kubeadm, kubelet and kubectl
Refer to the official installation guide: read [this](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/install-kubeadm/)

```bash
sudo apt-get update
sudo apt-get install -y apt-transport-https ca-certificates curl gpg
curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.33/deb/Release.key | sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg
echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.33/deb/ /' | sudo tee /etc/apt/sources.list.d/kubernetes.list
sudo apt-get update
sudo apt-get install -y kubelet kubeadm kubectl
sudo apt-mark hold kubelet kubeadm kubectl
sudo systemctl enable --now kubelet
```

### A5. IP forwarding
Kubernetes requires IP forwarding to be enabled on the host so that pods and services can route traffic properly.

```bash
# Temporarily enable IP forwarding (effective immediately)
sudo sysctl -w net.ipv4.ip_forward=1
# Permanently enable it (survives reboot)
echo "net.ipv4.ip_forward = 1" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

---

## B. Master

### B1. Initialize the Cluster
```bash
sudo kubeadm init
```

> [!IMPORTANT]
> After this step a line should be appeared similar to this:
> ```bash
> kubeadm join ...
> ```
>
> This command should be used by the **workers** to establish a connection into the cluster

> [!TIP]
> You can print the join command by executing:
> ```bash
> kubeadm token create --print-join-command
> ```

### B2. K8s configuration
This allows the user to interact with the cluster using kubectl.

```bash
mkdir -p $HOME/.kube
sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config
``` 

### B3. Container Network Interface (CNI)
Kubernetes does not install a default network plugin. One must be applied manually: read [this](https://kubernetes.io/docs/concepts/cluster-administration/networking/#how-to-implement-the-kubernetes-networking-model)

#### Options:
**i. Flannel**
```bash
kubectl apply -f https://github.com/flannel-io/flannel/releases/latest/download/kube-flannel.yml
```

**ii. Calico**
```bash
kubectl apply -f https://docs.projectcalico.org/manifests/calico.yaml
```

### B4. Install helm
Helm is a Kubernetes package manager, useful for deploying complex applications.

```bash
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
```

### B5. Dashboard

```bash
```

---

## C. Verification

### C1. Dummy deployment

```yml
# inside the nginx-deployment.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
spec:
  # Number of pod replicas
  replicas: 3                          
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:latest
        ports:
        - containerPort: 80

---

apiVersion: v1
kind: Service
metadata:
  name: nginx-service
spec:
  # Exposes the app on each Node's IP at a static port
  type: NodePort
  selector:
    app: nginx
  ports:
  - port: 80                         # Service port
    targetPort: 80                   # Container port
    nodePort: 30080                  # External access port on the node (e.g., http://<nodeIP>:30080)
```

### C2. Run deploymnt

```bash
kubectl apply -f nginx-deployment.yml
```

### C3. Check the deployment and service

```bash
kubectl get pods                # Check pods
kubectl get deployments         # Check deployment
kubectl get svc nginx-service   # Check service
```

### C4. Access Application
```web
http://<external-node-IP>:30080
```