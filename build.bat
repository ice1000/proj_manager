@echo off
cargo clean
REM cargo build --release
cargo build
cd ./target/debug
move *.exe ../../
cd ../../
echo ===============================================
echo ===============================================
echo ===============================================
echo ================build success==================
echo ===============================================
echo ===============================================
echo ===============================================
proj_manager