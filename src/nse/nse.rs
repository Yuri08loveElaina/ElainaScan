use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use anyhow::Result;
use std::fs::File;

pub fn run_nmap_nse(target: &str, ports: &str, scripts: Option<&str>, report: Option<&str>) -> Result<()> {
    let mut cmd = Command::new("nmap");

    cmd.arg("-p").arg(ports);

    if let Some(script_list) = scripts {
        cmd.arg("--script").arg(script_list);
    } else {
        cmd.arg("--script").arg("vuln");
    }

    cmd.arg(target);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().expect("Failed to capture stdout");

    let reader = BufReader::new(stdout);
    let mut report_file = if let Some(path) = report {
        Some(File::create(path)?)
    } else {
        None
    };

    for line in reader.lines() {
        let line = line?;
        if line.contains("open") {
            println!("\x1b[92m{}\x1b[0m", line); // xanh lá cho port open
        } else if line.contains("CVE") || line.contains("|") || line.contains("VULNERABLE") {
            println!("\x1b[93m{}\x1b[0m", line); // vàng cho vuln
        } else {
            println!("{}", line);
        }

        if let Some(file) = report_file.as_mut() {
            writeln!(file, "{}", line)?;
        }
    }

    let status = child.wait()?;
    if !status.success() {
        eprintln!("\x1b[91m[!] Nmap exited with non-zero status: {:?}\x1b[0m", status);
    } else {
        println!("\x1b[94m[*] Nmap NSE scan completed.\x1b[0m");
    }

    Ok(())
}
