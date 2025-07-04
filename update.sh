#!/usr/bin/env bash

# 🌸────────────────────────────
# 🌸 ElainaScan Updater
# 🌸────────────────────────────

set -e

echo -e "\e[95m🌸 ElainaScan Updater 🌸\e[0m"
echo "─────────────────────────────────"

# Kiểm tra git repo
if [ -d ".git" ]; then
    echo -e "\e[92m[*] Pulling latest changes từ GitHub...\e[0m"
    git pull
else
    echo -e "\e[91m[!] Không tìm thấy repo Git, không thể update.\e[0m"
    exit 1
fi

# Build lại
echo -e "\e[92m[*] Building ElainaScan (release)...\e[0m"
cargo build --release

# Strip giảm size
echo -e "\e[92m[*] Strip binary...\e[0m"
strip target/release/elainascan || true

# Copy lại vào /usr/local/bin
echo -e "\e[92m[*] Cập nhật elainascan vào /usr/local/bin...\e[0m"
sudo cp target/release/elainascan /usr/local/bin/elainascan

echo -e "\n\e[95m🌸 Update ElainaScan thành công, sẵn sàng pentest! 🌸\e[0m"
