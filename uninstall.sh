#!/usr/bin/env bash

# 🌸────────────────────────────
# 🌸 ElainaScan Uninstaller
# 🌸────────────────────────────

echo -e "\e[95m🌸 ElainaScan Uninstaller 🌸\e[0m"
echo "─────────────────────────────────"

# Xóa binary
if [ -f "/usr/local/bin/elainascan" ]; then
    echo -e "\e[92m[*] Đang gỡ elainascan khỏi /usr/local/bin...\e[0m"
    sudo rm /usr/local/bin/elainascan
    echo -e "\e[92m[+] Đã gỡ ElainaScan.\e[0m"
else
    echo -e "\e[91m[!] ElainaScan không tồn tại trong /usr/local/bin.\e[0m"
fi

# Hỏi xóa thư mục repo nếu cần
read -p "🩷 Xóa thư mục repo ElainaScan hiện tại không? (y/N): " confirm
if [[ "$confirm" == "y" || "$confirm" == "Y" ]]; then
    cd ..
    rm -rf ElainaScan
    echo -e "\e[92m[+] Đã xóa thư mục ElainaScan.\e[0m"
else
    echo -e "\e[93m[-] Giữ lại thư mục ElainaScan.\e[0m"
fi

echo -e "\n\e[95m🌸 Đã hoàn tất uninstall ElainaScan! 🌸\e[0m"
