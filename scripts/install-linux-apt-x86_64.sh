#!/bin/bash

wget -O- https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB \
| gpg --dearmor | sudo tee /usr/share/keyrings/oneapi-archive-keyring.gpg > /dev/null

# add signed entry to apt sources and configure the APT client to use Intel repository:
echo "deb [signed-by=/usr/share/keyrings/oneapi-archive-keyring.gpg] https://apt.repos.intel.com/oneapi all main" | sudo tee /etc/apt/sources.list.d/oneAPI.list

sudo apt install build-essential gcc dirmngr ca-certificates software-properties-common apt-transport-https dkms curl -y
curl -fSsL https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2404/x86_64/3bf863cc.pub | sudo gpg --dearmor | sudo tee /usr/share/keyrings/nvidia-drivers.gpg > /dev/null 2>&1
echo 'deb [signed-by=/usr/share/keyrings/nvidia-drivers.gpg] https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2404/x86_64/ /' | sudo tee /etc/apt/sources.list.d/nvidia-drivers.list
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-keyring_1.0-1_all.deb
sudo dpkg -i cuda-keyring_1.0-1_all.deb
sudo apt update
sudo apt install cuda cudnn
sudo apt install intel-basekit
sudo apt clean
sudo apt autoremove --purge

ONEDNN_VERSION=3.1.1
curl -L -O https://github.com/oneapi-src/oneDNN/archive/refs/tags/v${ONEDNN_VERSION}.tar.gz
tar xf *.tar.gz && rm *.tar.gz
cd oneDNN-*
INSTALL_DIR="$GITHUB_WORKSPACE/onednn-install"
mkdir -p "$INSTALL_DIR"
cmake  -DCMAKE_INSTALL_PREFIX="$INSTALL_DIR" -DCMAKE_BUILD_TYPE=Release -DONEDNN_LIBRARY_TYPE=STATIC -DONEDNN_BUILD_EXAMPLES=OFF -DONEDNN_BUILD_TESTS=OFF -DONEDNN_ENABLE_WORKLOAD=INFERENCE -DONEDNN_ENABLE_PRIMITIVE="CONVOLUTION;REORDER" -DONEDNN_BUILD_GRAPH=OFF .
make -j$(nproc) install
cd ..
rm -r oneDNN-*

sudo apt install libnccl2 libnccl-dev
sudo apt install openmpi-bin libopenmpi-dev
sudo apt clean
sudo apt autoremove --purge
