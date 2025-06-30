# 🌸 ElainaScan

![Rust](https://img.shields.io/badge/Rust-%23dea584.svg?style=for-the-badge&logo=rust&logoColor=white)
![MIT License](https://img.shields.io/github/license/Yuri08loveElaina/ElainaScan?style=for-the-badge)
![ElainaScan](https://img.shields.io/badge/Fast%20Port%20Scanner-ElainaScan-ff69b4?style=for-the-badge)

---

## 🚀 Giới thiệu

**ElainaScan** là công cụ **Port Scanner + Vuln Scanner siêu nhanh, chính xác, đẹp mắt** phát triển bằng **Rust**, kết hợp:
- Tốc độ quét của **Masscan**.
- Xác thực port mở chuẩn như **RustScan**.
- Định hướng mở rộng detect service, OS fingerprint, vuln scan như **Nmap**.

**Mục tiêu:** Phục vụ **pentest thực chiến, recon Bug Bounty, CTF, lab nghiên cứu an toàn thông tin**.

---

## ⚙️ Tính năng

✅ Quét port TCP tốc độ cao, concurrency tùy chỉnh.  
✅ Xác thực port tránh false positive.  
✅ Banner grab, OS fingerprint, CVE check local.  
✅ Tích hợp chạy NSE nếu cần.  
✅ In kết quả màu dễ đọc trên terminal.  
✅ Gọn nhẹ, dễ build cross-platform.

---

## 🛠️ Cài đặt

Yêu cầu:
- Rust + Cargo

Clone repo:
```bash
git clone https://github.com/Yuri08loveElaina/ElainaScan.git
cd ElainaScan
```

Build Release:
```bash
cargo build --release
```

File chạy:
```bash
./target/release/elainascan
```

---

## 🚀 Cách chạy

Quét banner grab:
```bash
./target/release/elainascan --target 192.168.1.10 --ports 22,80,443 --banner --report banner_192.json
```

Chạy nhanh:
```bash
cargo run --release -- --target 192.168.1.10 --ports 80,443 --banner
```

---

**Chúc vui vẻ ;). 🩶**
