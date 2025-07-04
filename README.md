# 🌸 ElainaScan

![Rust](https://img.shields.io/badge/Rust-%23dea584.svg?style=for-the-badge&logo=rust&logoColor=white)
![MIT License](https://img.shields.io/github/license/Yuri08loveElaina/ElainaScan?style=for-the-badge)
![ElainaScan](https://img.shields.io/badge/Fast%20Port%20Scanner-ElainaScan-ff69b4?style=for-the-badge)

---

## 🚀 Giới thiệu

**ElainaScan** là **High-speed CVE & Port Vulnerability Scanner** phát triển bằng **Rust**, kết hợp tốc độ **Masscan**, xác thực port chuẩn như **RustScan**, và mở rộng detect **service / OS / vuln / NSE** như **Nmap**.

**Phục vụ:**  
✨ **Pentest thực chiến**  
✨ **Recon Bug Bounty**  
✨ **CTF & Lab an toàn thông tin**

---

## ⚡ Tính năng

✅ Quét port TCP tốc độ cao, concurrency tùy chỉnh.  
✅ Xác thực port tránh false positive.  
✅ Banner grabbing (HTTP/SSH/FTP).  
✅ OS & service version detection.  
✅ Vuln check local CVE DB.  
✅ Auto chạy Nmap NSE nếu cần.  
✅ Xuất kết quả **JSON / CSV / HTML**.  
✅ Gọn nhẹ, dễ build cross-platform.

---

## 🛠️ Cài đặt

### Yêu cầu:
- Rust + Cargo (hoặc dùng Docker)
- Git

### Clone repo:
```bash
git clone https://github.com/Yuri08loveElaina/ElainaScan.git
cd ElainaScan
```

### Build release:
```bash
cargo build --release
```

File **build xong sẽ nằm tại:**
```bash
./target/release/elainascan
```

---

## 🐳 Chạy bằng Docker (khuyến nghị nếu không muốn cài Rust)

Build:
```bash
docker build -t elainascan .
```

Chạy:
```bash
docker run --rm elainascan --target 13.92.97.133 --ports 80,443 --banner --finger --vuln
```

---

## ✨ Cách sử dụng

### ⚡ Quét port + banner grab:
```bash
./elainascan --target 192.168.1.10 --ports 22,80,443 --banner
```

### ⚡ Quét kèm OS detect:
```bash
./elainascan --target 192.168.1.10 --ports 22,80,443 --finger
```

### ⚡ Quét vuln (CVE local DB):
```bash
./elainascan --target 192.168.1.10 --ports 22,80,443 --vuln
```

### ⚡ Quét NSE:
```bash
./elainascan --target 192.168.1.10 --ports 80,443 --nse --nse-scripts vuln
```

### ⚡ Xuất báo cáo:
```bash
./elainascan --target 192.168.1.10 --ports 22,80,443 --banner --finger --vuln --report report.json
```

---

## ✨ Tính năng nâng cao

- **`--concurrency <num>`**: Tùy chỉnh luồng quét song song.
- **`--nse-scripts <script>`**: Chạy script NSE cụ thể (nmap phải cài sẵn).
- **`--report <file>`**: Xuất kết quả ra file JSON/CSV/HTML tự động.

---

## ❤️ Góp ý / Issue

Nếu phát hiện bug hoặc cần thêm tính năng:
- Mở [Issue](https://github.com/Yuri08loveElaina/ElainaScan/issues)
- Hoặc fork & PR

---

## 📜 License

**MIT License**

---

## ✨ Yuri08 chúc bạn pentest vui vẻ!
