use std::env;
use std::path::{Path, PathBuf};

use build_target::{Arch, Family, Os};
use cmake::Config;

fn main() {
    use std::process::Command;

    let repo_url = "https://github.com/OpenNMT/CTranslate2.git";
    let target_dir = if let Ok(dir) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(dir)
    } else {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        PathBuf::from(manifest_dir).join("target")
    }
    .join("CTranslate2");

    let repo_exists = target_dir.exists();

    if repo_exists {
        let status = Command::new("git")
            .arg("-C")
            .arg(&target_dir)
            .arg("pull")
            .status()
            .expect("Failed to execute Git pull");

        if !status.success() {
            panic!("Failed to update external project");
        }
    } else {
        let status = Command::new("git")
            .arg("clone")
            .arg("--recursive")
            .arg(repo_url)
            .arg(&target_dir)
            .status()
            .expect("Failed to execute Git clone");

        if !status.success() {
            panic!("Failed to clone external project");
        }
    }

    let arch = build_target::target_arch().unwrap(); // eg. "x86_64", "aarch64", ...
    let family = build_target::target_family().unwrap(); // eg. "windows", "unix", ...
    let os = build_target::target_os().unwrap(); // eg. "android", "linux", ...
    let mut config = Config::new(&target_dir);
    config
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_CLI", "OFF")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5");

    match family {
        Family::Unix => {
            match os {
                Os::MacOS => {
                    config.define("CMAKE_INSTALL_RPATH_USE_LINK_PATH", "ON");
                    match arch {
                        Arch::AArch64 | Arch::Arm => {
                            config
                                .define("CMAKE_OSX_ARCHITECTURES", "arm64")
                                .define("WITH_ACCELERATE", "ON")
                                .define("WITH_MKL", "OFF")
                                .define("OPENMP_RUNTIME", "NONE")
                                .define("WITH_RUY", "ON");
                        }
                        _ => {
                            config
                                .define("ONEDNN_LIBRARY_TYPE", "STATIC")
                                .define("ONEDNN_BUILD_EXAMPLES", "OFF")
                                .define("ONEDNN_BUILD_TESTS", "OFF")
                                .define("ONEDNN_ENABLE_WORKLOAD", "INFERENCE")
                                .define("ONEDNN_ENABLE_PRIMITIVE", "CONVOLUTION;REORDER")
                                .define("ONEDNN_BUILD_GRAPH", "OFF");
                        }
                    }
                }
                _ => {
                    match arch {
                        Arch::AArch64 | Arch::Arm => {
                            config
                                .define("WITH_MKL", "OFF")
                                .define("OPENMP_RUNTIME", "COMP")
                                .define("CMAKE_PREFIX_PATH", "/opt/OpenBLAS")
                                .define("WITH_OPENBLAS", "ON")
                                .define("WITH_RUY", "ON");
                        }
                        _ => {
                            config
                                .define("CMAKE_CXX_FLAGS", "-msse4.1")
                                .define("WITH_DNNL", "ON")
                                .define("OPENMP_RUNTIME", "COMP")
                                .define("WITH_CUDA", "ON")
                                .define("WITH_CUDNN", "ON")
                                .define("CUDA_DYNAMIC_LOADING", "ON")
                                .define("CUDA_NVCC_FLAGS", "-Xfatbin=-compress-all")
                                .define("CUDA_ARCH_LIST", "Common")
                                .define("WITH_TENSOR_PARALLEL", "ON");

                            // yum-config-manager --add-repo https://developer.download.nvidia.com/compute/cuda/repos/rhel8/x86_64/cuda-rhel8.repo
                            //    # error mirrorlist.centos.org doesn't exists anymore.
                            //    sed -i s/mirror.centos.org/vault.centos.org/g /etc/yum.repos.d/*.repo
                            //    sed -i s/^#.*baseurl=http/baseurl=http/g /etc/yum.repos.d/*.repo
                            //    sed -i s/^mirrorlist=http/#mirrorlist=http/g /etc/yum.repos.d/*.repo
                            //    yum install --setopt=obsoletes=0 -y \
                            //        cuda-nvcc-12-2-12.2.140-1 \
                            //        cuda-cudart-devel-12-2-12.2.140-1 \
                            //        libcurand-devel-12-2-10.3.3.141-1 \
                            //        libcudnn9-devel-cuda-12-9.1.0.70-1 \
                            //        libcublas-devel-12-2-12.2.5.6-1 \
                            //        libnccl-devel-2.19.3-1+cuda12.2
                            //    ln -s cuda-12.2 /usr/local/cuda

                            //    ONEAPI_VERSION=2023.2.0
                            //    yum-config-manager --add-repo https://yum.repos.intel.com/oneapi
                            //    rpm --import https://yum.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB
                            //    yum install -y intel-oneapi-mkl-devel-$ONEAPI_VERSION

                            //    ONEDNN_VERSION=3.1.1
                            //    curl -L -O https://github.com/oneapi-src/oneDNN/archive/refs/tags/v${ONEDNN_VERSION}.tar.gz
                            //    tar xf *.tar.gz && rm *.tar.gz
                            //    cd oneDNN-*
                            //    cmake -DCMAKE_BUILD_TYPE=Release -DONEDNN_LIBRARY_TYPE=STATIC -DONEDNN_BUILD_EXAMPLES=OFF -DONEDNN_BUILD_TESTS=OFF -DONEDNN_ENABLE_WORKLOAD=INFERENCE -DONEDNN_ENABLE_PRIMITIVE="CONVOLUTION;REORDER" -DONEDNN_BUILD_GRAPH=OFF .
                            //    make -j$(nproc) install
                            //    cd ..
                            //    rm -r oneDNN-*

                            //    OPENMPI_VERSION=4.1.6
                            //    curl -L -O https://download.open-mpi.org/release/open-mpi/v4.1/openmpi-${OPENMPI_VERSION}.tar.bz2
                            //    tar xf *.tar.bz2 && rm *.tar.bz2
                            //    cd openmpi-*
                            //    ./configure
                            //    make -j$(nproc) install
                            //    cd ..
                            //    rm -r openmpi-*
                            //    export LD_LIBRARY_PATH="/usr/local/lib/:$LD_LIBRARY_PATH"
                        }
                    }
                }
            }
        }
        Family::Windows => {
            const CUDA_ROOT: &str = "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.2";
            // curl --netrc-optional -L -nv -o cuda.exe https://developer.download.nvidia.com/compute/cuda/12.2.2/local_installers/cuda_12.2.2_537.13_windows.exe
            // ./cuda.exe -s nvcc_12.2 cudart_12.2 cublas_dev_12.2 curand_dev_12.2
            // rm cuda.exe

            // CUDNN_ROOT="C:/Program Files/NVIDIA/CUDNN/v9.1"
            // curl --netrc-optional -L -nv -o cudnn.exe https://developer.download.nvidia.com/compute/cudnn/9.1.0/local_installers/cudnn_9.1.0_windows.exe
            // ./cudnn.exe -s
            // sleep 10
            // # Remove 11.8 folders
            // rm -rf "$CUDNN_ROOT/bin/11.8"
            // rm -rf "$CUDNN_ROOT/lib/11.8"
            // rm -rf "$CUDNN_ROOT/include/11.8"

            // # Move contents of 12.4 to parent directories
            // mv "$CUDNN_ROOT/bin/12.4/"* "$CUDNN_ROOT/bin/"
            // mv "$CUDNN_ROOT/lib/12.4/"* "$CUDNN_ROOT/lib/"
            // mv "$CUDNN_ROOT/include/12.4/"* "$CUDNN_ROOT/include/"

            // # Remove empty 12.4 folders
            // rmdir "$CUDNN_ROOT/bin/12.4"
            // rmdir "$CUDNN_ROOT/lib/12.4"
            // rmdir "$CUDNN_ROOT/include/12.4"
            // cp -r "$CUDNN_ROOT"/* "$CUDA_ROOT"
            // rm cudnn.exe

            // # See https://github.com/oneapi-src/oneapi-ci for installer URLs
            // curl --netrc-optional -L -nv -o webimage.exe https://registrationcenter-download.intel.com/akdlm/irc_nas/19078/w_BaseKit_p_2023.0.0.25940_offline.exe
            // ./webimage.exe -s -x -f webimage_extracted --log extract.log
            // rm webimage.exe
            // ./webimage_extracted/bootstrapper.exe -s --action install --components="intel.oneapi.win.mkl.devel" --eula=accept -p=NEED_VS2017_INTEGRATION=0 -p=NEED_VS2019_INTEGRATION=0 --log-dir=.

            // ONEDNN_VERSION=3.1.1
            // curl --netrc-optional -L -O https://github.com/oneapi-src/oneDNN/archive/refs/tags/v${ONEDNN_VERSION}.tar.gz
            // tar xf *.tar.gz && rm *.tar.gz
            // cd oneDNN-*
            // cmake -DCMAKE_BUILD_TYPE=Release -DONEDNN_LIBRARY_TYPE=STATIC -DONEDNN_BUILD_EXAMPLES=OFF -DONEDNN_BUILD_TESTS=OFF -DONEDNN_ENABLE_WORKLOAD=INFERENCE -DONEDNN_ENABLE_PRIMITIVE="CONVOLUTION;REORDER" -DONEDNN_BUILD_GRAPH=OFF .
            // cmake --build . --config Release --target install --parallel 6
            // cd ..
            // rm -r oneDNN-*

            //.define("CMAKE_INSTALL_PREFIX", CTRANSLATE2_ROOT)
            config.define("CMAKE_PREFIX_PATH","C:/Program Files (x86)/Intel/oneAPI/compiler/latest/windows/compiler/lib/intel64_win;C:/Program Files (x86)/oneDNN")
                .define("BUILD_CLI","OFF")
                .define("WITH_DNNL","ON")
                .define("WITH_CUDA","ON")
                .define("WITH_CUDNN","ON")
                .define("CUDA_TOOLKIT_ROOT_DIR", CUDA_ROOT)
                .define("CUDA_DYNAMIC_LOADING","ON")
                .define("CUDA_NVCC_FLAGS","-Xfatbin=-compress-all")
                .define("CUDA_ARCH_LIST","Common");
        }
        _ => unimplemented!(),
    }

    let dst = config.build();

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++17")
        .include(Path::new(&format!("{}/include", dst.display())))
        .compile("ctranslate2rs");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=ctranslate2");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/translator.h");
}
