# DietPi-Dashboard
A web dashboard for DietPi

## Compiliation
### Prereq:

```sh
dietpi-software install 9 16 17 188 # Install Node.js (webpage), Build-Essential (make), Git (git clone), and Go (binary), respectively
npm install -g yarn # Install Yarn package manager, for node dependencies
```
Now log out and log back in to register GOPATH

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
