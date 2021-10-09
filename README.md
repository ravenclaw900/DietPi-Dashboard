# DietPi-Dashboard
A web dashboard for DietPi

[![CodeFactor](https://www.codefactor.io/repository/github/ravenclaw900/dietpi-dashboard/badge/main)](https://www.codefactor.io/repository/github/ravenclaw900/dietpi-dashboard/overview/main)

## Compiliation
### Prereq:

```sh
dietpi-software install 9 16 17 # Install Node.js (webpage), Build-Essential (make and gcc), and Git (git clone), respectively
npm install -g yarn # Install Yarn package manager, for node dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Install Rust (backend)
source ~/.cargo/env # Update PATH
```

### Compiling:

```sh
rm -rf DietPi-Dashboard # Remove possibly pre-download repository
git clone https://github.com/ravenclaw900/DietPi-Dashboard # Download source code
cd DietPi-Dashboard/src/frontend # Change directories
yarn # Install dependencies
cd ../.. # Change directories
make # Compile binary for your platform
./dietpi-dashboard # Run binary
```

## Compiling for all targets (release)
### Prereq:

Normal compilation prereq (see above)
```sh
rustup target add aarch64-unknown-linux-gnu arm-unknown-linux-gnueabihf armv7-unknown-linux-gnueabihf x86_64-unknown-linux-gnu # Add Rust standard libraries
apt install gcc-aarch64-linux-gnu libc-dev-arm64-cross gcc-arm-linux-gnueabihf libc6-dev-armhf-cross gcc-x86-64-linux-gnu libc6-dev-amd64-cross # Install cross-compiling toolchains
mkdir /opt/rpi ; git clone https://github.com/raspberrypi/tools /opt/rpi # Install Raspberry Pi cross-compiling toolchain
```
You also need to install UPX, however to compress the ARMv6/7 binaries you need to [compile it yourself](https://github.com/upx/upx/blob/devel/README.SRC)

### Compiling:

```sh
rm -rf DietPi-Dashboard # Remove possibly pre-download repository
git clone https://github.com/ravenclaw900/DietPi-Dashboard # Download source code
cd DietPi-Dashboard/src/frontend # Change directories
yarn # Install dependencies
cd ../.. # Change directories
make build # Compile binaries for all platforms
```
Binaries will then be avalible in the `build` directory.
