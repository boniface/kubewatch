# **kubewatch**  

**Automatically apply Kubernetes manifest changes with a file watcher**  

`kubewatch` is a lightweight Linux daemon that monitors a directory for changes to Kubernetes manifests (`*.yaml`/`*.yml`) and automatically applies them to your cluster using `kubectl apply`. Perfect for development, GitOps workflows, or syncing local changes to a cluster without manual intervention.  

---

## **Features**  
- 🕵️ **Real-time file watching** – Detects changes in specified directories.  
- ⚡ **Auto-apply** – Runs `kubectl apply -f` on modified manifests.  
- 🔧 **Configurable paths** – Set your watched directory via config file.  
- 🖥️ **Systemd service** – Runs as a background daemon (`systemctl` support).  
- 🔒 **Kubernetes RBAC-aware** – Uses local `kubeconfig` for authentication.  

---

## **Installation**  

### **Debian/Ubuntu (via APT )**

You can install Hash Release directly from our APT repository:

```sh
# Add our GPG key
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://boniface.github.io/kubewatch/apt-repo/public.gpg | sudo gpg --dearmor -o /etc/apt/keyrings/kubewatch-archive-keyring.gpg

# Add repository to sources
echo "deb [signed-by=/etc/apt/keyrings/kubewatch-archive-keyring.gpg] https://boniface.github.io/kubewatch/apt-repo stable main" | sudo tee /etc/apt/sources.list.d/kubewatch.list

# Update package lists
sudo apt-get update

# Install kubewatch
sudo apt-get install kubewatch
```

### **Debian/Ubuntu (via .deb package)**  
```sh
# Download and install the package  
wget https://example.com/kubewatch.deb  
sudo apt install ./kubewatch.deb  

### Start and Enable the Service

After installation, you can start and enable the systemd service:

```sh
sudo systemctl start kubewatch
sudo systemctl enable kubewatch
```


### **Manual Install (Any Linux)**  
```sh
# Clone the repo  
git clone https://github.com/your-repo/kubewatch.git  
cd kubewatch  

# Install dependencies (if needed)  
sudo apt install inotify-tools kubectl  

# Install and start  
sudo make install  
sudo systemctl start kubewatch  
```  

---

## **Configuration**  
Edit `/etc/kubewatch/config.yaml`:  
```yaml
watchDir: "/path/to/manifests"  # Directory to monitor  
kubeconfig: "/home/user/.kube/config"  # Optional: Custom kubeconfig  
logLevel: "info"                 # debug, info, warn, error  
```  

Restart to apply changes:  
```sh
sudo systemctl restart kubewatch  
```  

---

## **Usage**  
1. Place Kubernetes manifests in the configured `watchDir`.  
2. On file save, `kubewatch` detects changes and runs:  
   ```sh
   kubectl apply -f /path/to/changed/file.yaml  
   ```  
3. Check logs:  
   ```sh
   journalctl -u kubewatch -f  
   ```  

---

## **FAQ**  
### **Q: Does this work with `kustomize` or Helm?**  
A: No—it only applies raw YAML files. For Helm/Kustomize, consider a CI/CD pipeline.  

### **Q: How is this different from Flux/ArgoCD?**  
A: `kubewatch` is a **simple, local-file watcher**, not a full GitOps solution. Ideal for dev environments.  

### **Q: Can I restrict which files are watched?**  
A: Yes! Use `includePattern` in the config (e.g., `"*.yaml"`).  

---

## **Contributing**  
PRs welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.  

---

## **License**  
MIT © Hashcode (Z) Limited

---

### **Why Use `kubewatch`?**  
- ✅ **No Git commits needed** – Great for rapid local testing.  
- ✅ **Zero dependencies** – Just `kubectl` and `inotify`.  
- ✅ **KISS (Keep It Simple)** – No complex operators or YAML templating.  

