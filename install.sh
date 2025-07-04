#!/usr/bin/env bash

# 🌸────────────────────────────────────
# 🌸 ElainaScan Installer by YURI08 🌸
# 🌸────────────────────────────────────

set -e

echo -e "\e[95m🌸 ElainaScan Auto Installer 🌸\e[0m"
echo "────────────────────────────────────────"

# 🩷 Check Rust
if ! command -v cargo &>/dev/null; then
    echo -e "\e[91m[!] Rust chưa cài, đang cài rustup...\e[0m"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
else
    echo -e "\e[92m[*] Rust đã cài sẵn.\e[0m"
fi

# 🩷 Update repo nếu có
if [ -d ".git" ]; then
    echo -e "\e[92m[*] Đang pull ElainaScan repo...\e[0m"
    git pull
else
    echo -e "\e[91m[!] Không thấy repo Git, bỏ qua bước pull.\e[0m"
fi

# 🩷 Build release
echo -e "\e[92m[*] Building ElainaScan (release)...\e[0m"
cargo build --release

# 🩷 Strip giảm size
echo -e "\e[92m[*] Strip binary giảm size...\e[0m"
strip target/release/elainascan || true

# 🩷 Copy vào /usr/local/bin
echo -e "\e[92m[*] Copy elainascan vào /usr/local/bin...\e[0m"
sudo cp target/release/elainascan /usr/local/bin/elainascan

# 🩷 Hoàn tất
echo -e "\n\e[95m🌸 ElainaScan đã sẵn sàng! 🌸\e[0m"
echo "────────────────────────────────────────"
echo -e "\e[96m👉 Chạy thử:\e[0m"
echo -e "   elainascan --target 13.92.97.133 --ports 80,443 --banner --finger --vuln --report report.json"
echo ""
echo -e "\e[93m✨ YURI08 chúc bạn pentest vui vẻ!\e[0m"
