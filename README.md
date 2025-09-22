# Kubewatch

[![CI](https://github.com/boniface/kubewatch/actions/workflows/tests.yml/badge.svg)](https://github.com/boniface/kubewatch/actions/workflows/tests.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Automatically apply Kubernetes manifest changes with a file watcher**

`kubewatch` is a lightweight Linux daemon that monitors a directory for changes to Kubernetes manifests (`*.yaml`/`*.yml`) and automatically applies them to your cluster using `kubectl apply`. Perfect for development, GitOps workflows, or syncing local changes to a cluster without manual intervention.

---

## Features

- 🕵️ **Real-time file watching** – Detects changes in specified directories.
- ⚡ **Auto-apply** – Runs `kubectl apply -f` on modified manifests.
- 🔧 **Configurable paths** – Set your watched directory via config file.
- 🖥️ **Systemd service** – Runs as a background daemon (`systemctl` support).
- 🔒 **Kubernetes RBAC-aware** – Uses local `kubeconfig` for authentication.

---

## Installation

### Debian/Ubuntu (via APT)

You can install Kubewatch directly from the APT repository:

```sh
# Add our GPG key
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://boniface.github.io/kubewatch/public.gpg | sudo gpg --dearmor -o /etc/apt/keyrings/kubewatch-archive-keyring.gpg

# Add repository to sources
echo "deb [signed-by=/etc/apt/keyrings/kubewatch-archive-keyring.gpg] https://boniface.github.io/kubewatch stable main" | sudo tee /etc/apt/sources.list.d/kubewatch.list

# Update package lists
sudo apt-get update

# Install kubewatch
sudo apt-get install kubewatch
```

### Debian/Ubuntu (via .deb package)

```sh
# Download and install the package
wget https://example.com/kubewatch.deb
sudo apt install ./kubewatch.deb
```

### Start and Enable the Service

After installation, you can start and enable the systemd service:

```sh
sudo systemctl start kubewatch
sudo systemctl enable kubewatch
```

### Manual Install (Any Linux)

```sh
# Clone the repo
git clone https://github.com/boniface/kubewatch.git
cd kubewatch

# Build and install
cargo build --release
sudo cp target/release/kubewatch /usr/local/bin/
```

---

## Configuration

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

## Usage

1.  Place Kubernetes manifests in the configured `watchDir`.
2.  On file save, `kubewatch` detects changes and runs:
    ```sh
    kubectl apply -f /path/to/changed/file.yaml
    ```
3.  Check logs:
    ```sh
    journalctl -u kubewatch -f
    ```

---

## Contributing

PRs welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.