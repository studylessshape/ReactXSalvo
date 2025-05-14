#!/bin/bash
# Generate by mita-deepseek - https://metaso.cn/

TARGET_FOLDER="build"

# 检查并删除现有构建目录
if [ -d "$TARGET_FOLDER" ]; then
    echo "Remove all build files"
    rm -rf "$TARGET_FOLDER"  # 使用强制递归删除目录 [[8]]
fi

# 创建目录结构
mkdir -p "${TARGET_FOLDER}/frontend"  # 递归创建多级目录 [[5,8]]

echo "Enter frontend..."
cd frontend
npm install
npm run build
# 复制构建产物到目标目录，保持目录结构
mkdir -p "../${TARGET_FOLDER}/static"  # 确保目标目录存在
cp -r dist/* "../${TARGET_FOLDER}/static/"  # 递归复制文件 [[16]]

echo "Enter server..."
cd ../server
cargo build --release
# 复制二进制文件到构建目录
mkdir -p "../${TARGET_FOLDER}"  # 确保目标目录存在
cp "target/release/server" "../${TARGET_FOLDER}/"  # 注意Linux可执行文件无.exe后缀 [[16]]

cd ..  # 返回项目根目录