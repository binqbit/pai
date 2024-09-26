@echo off
cargo build --release
mkdir bin
copy .\target\release\pai.exe .\bin\pai.exe