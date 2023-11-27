# PackageAI

## Package manager with AI in terminal

# Example
```shell
pai install all libs for node crypto
pai run this project
pai add redis container to docker config
pai create new backend app on rust for blockchain
pai tell me about of this project
```

# Flags
```shell
pai [-flags] task

[--key] - set openai key
pai --key 1234567890qwertyuiopasdfghjklzxcvbnm

[--model] - set gpt model
pai --model gpt-4

[--version, -v] - view pai version
pai --version

[--help, -h] - view help
```

# Install
```shell
git clone http://github.com/binqbit/pai.git
cd pai
cargo build --release

# liunx
mkdir build
mkdir build/config
cp target/release/pai build/pai
cp config/vocab.json build/config/vocab.json
cp config/merges.txt build/config/merges.txt
export PATH=$PATH:$(pwd)/build

# windows
mkdir build
mkdir build/config
copy target/release/pai.exe build/pai.exe
copy config/vocab.json build/config/vocab.json
copy config/merges.txt build/config/merges.txt
set PATH=%PATH%;%cd%/build
```
