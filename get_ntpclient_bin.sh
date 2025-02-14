#!/bin/bash

# 定义仓库的所有者、仓库名称
owner="katteXu"
repo="ntpclient"
# 指定保存路径
save_path="./src-tauri/bin"

# 创建保存路径（如果不存在）
mkdir -p "$save_path"

# 获取最新版本信息
release_info=$(curl -s "https://api.github.com/repos/$owner/$repo/releases/latest")

# 提取 assets 信息
assets=$(echo "$release_info" | jq -c '.assets[]')

# 遍历 assets 并下载
while read -r asset; do
    asset_name=$(echo "$asset" | jq -r '.name')
    asset_url=$(echo "$asset" | jq -r '.browser_download_url')
    echo -e "\033[32m Downloading \033[0m$asset_name..."
    curl -s -L -o "$save_path/$asset_name" "$asset_url"
    if [ $? -eq 0 ]; then 
        echo -e "\033[33m $asset_name \033[0m downloaded successfully."
    else
        echo "Failed to download $asset_name."
    fi
done <<< "$assets"

cd src-tauri/bin
chmod +x ntpclient*