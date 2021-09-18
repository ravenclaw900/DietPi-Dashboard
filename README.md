# DietPi-Dashboard
A web dashboard for DietPi

[![CodeFactor](https://www.codefactor.io/repository/github/ravenclaw900/dietpi-dashboard/badge/main)](https://www.codefactor.io/repository/github/ravenclaw900/dietpi-dashboard/overview/main)

## Compiliation
### Prereq:

```sh
dietpi-software install 9 16 17 # Install Node.js (webpage), Build-Essential (make and gcc), and Git (git clone), respectively
npm install -g yarn # Install Yarn package manager, for node dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Install Rust (backend)
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
