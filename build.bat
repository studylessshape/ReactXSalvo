@echo off
set TARGET_FOLDER="build"
if exist "%TARGET_FOLDER%" (
    echo Remove all build files
    rmdir build /S /Q
)
mkdir build
mkdir build\frontend
echo Enter frontend...
cd frontend
npm install
npm run build
xcopy dist ..\build\static\ /Y /E
echo Enter server...
cd ..\server
cargo build --release
copy "target\release\server.exe" "..\build"
cd ..