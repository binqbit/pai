# PackageAI

## Package manager with AI in terminal

# Example
```shell
pai install all libs for node crypto
pai run this project
pai create new backend app on rust for blockchain
```

# Flags
```shell
pai [-flags] or [command]

[--key, -k] - set openai key
pai --key 1234567890qwertyuiopasdfghjklzxcvbnm

[--version, -v] - view pai version
pai --version

[--help, -h] - view help
```

# Install
```shell
cargo build --release
mkdir build
mkdir build/config

# liunx
cp target/release/pai build/pai
export PATH=$PATH:$(pwd)/build

# windows
copy target/release/pai.exe build/pai.exe
set PATH=%PATH%;%cd%/build
```
