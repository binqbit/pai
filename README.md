# ShellAI

## Shell with AI in terminal

# Example
```shell
sai install all libs for node crypto
sai run this project
sai create new backend app on rust for blockchain
```

# Flags
```shell
sai [-flags] or [command]

[--key, -k] - set openai key
sai --key 1234567890qwertyuiopasdfghjklzxcvbnm

[--version, -v] - view sai version
sai --version

[--help, -h] - view help
```

# Install
```shell
cargo build --release
mkdir build
mkdir build/config

# liunx
cp target/release/sai build/sai
export PATH=$PATH:$(pwd)/build

# windows
copy target/release/sai.exe build/sai.exe
set PATH=%PATH%;%cd%/build
```
