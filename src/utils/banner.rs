use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}};

pub fn show() {
    let banner_lines = vec![
        "╔════════════════════════════════════════════════════════════╗",
        "║                   🌸 ElainaScan 🌸                         ║",
        "║   High-Speed CVE & Port Vulnerability Scanner (Rust)      ║",
        "║      By Yuri08 | For Pentesters | MIT License             ║",
        "╚════════════════════════════════════════════════════════════╝",
        "Features: Fast TCP Scan • Banner Grab • OS Fingerprint • CVE Check • NSE",
        "──────────────────────────────────────────────────────────────",
    ];

    let colors = vec![
        Color::Magenta,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
    ];

    let mut stdout = stdout();

    for (i, line) in banner_lines.iter().enumerate() {
        execute!(
            stdout,
            SetForegroundColor(colors[i % colors.len()]),
            Print(line),
            ResetColor,
            Print("\n")
        ).unwrap();
        sleep(Duration::from_millis(100)); // Hiệu ứng typing
    }
}
