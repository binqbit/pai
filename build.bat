@echo off
cargo build --release
mkdir bin
copy .\target\release\sai.exe .\bin\sai.exe