use anyhow::{Result, anyhow};
use std::process::{Command, Stdio};
use std::str;
use std::time::{Instant, Duration};
use colored::*;
use std::thread::sleep;
use regex::Regex;

#[derive(Debug)]
pub struct NseResult {
    pub ip: String,
    pub port: u16,
    pub script: String,
    pub output: String,
}

pub struct NmapNseConfig<'a> {
    pub scripts: Vec<&'a str>,
    pub timeout: u64,
    pub retries: u8,
    pub timing_template: &'a str,
    pub script_args: Option<Vec<&'a str>>,
}

pub fn run_nse_scan(ip: &str, port: u16, config: &NmapNseConfig) -> Result<Vec<NseResult>> {
    let mut results = Vec::new();
    let target = format!("{} -p {}", ip, port);

    for &script in &config.scripts {
        let mut attempt = 0;
        let mut success = false;

        while attempt < config.retries && !success {
            attempt += 1;
            let mut cmd = Command::new("nmap");
            cmd.args([
                "-sV",
                "--script",
                script,
                ip,
                "-p",
                &port.to_string(),
                "--script-timeout",
                &format!("{}s", config.timeout),
                config.timing_template,
            ]);

            if let Some(args) = &config.script_args {
                for arg in args {
                    cmd.args(["--script-args", arg]);
                }
            }

            let start = Instant::now();
            println!(
                "{}",
                format!(
                    "[NSE] Running script '{}' on {}:{} (Attempt {}/{})",
                    script, ip, port, attempt, config.retries
                )
                .blue()
            );

            let output = cmd.output();

            match output {
                Ok(output) => {
                    let duration = start.elapsed();
                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!(
                            "{}",
                            format!(
                                "[NSE] Script '{}' on {}:{} failed: {}",
                                script, ip, port, stderr.trim()
                            )
                            .red()
                        );
                        if attempt < config.retries {
                            println!(
                                "{}",
                                format!(
                                    "[NSE] Retrying '{}' on {}:{} after 3s",
                                    script, ip, port
                                )
                                .yellow()
                            );
                            sleep(Duration::from_secs(3));
                            continue;
                        } else {
                            return Err(anyhow!(
                                "Nmap NSE scan failed after {} attempts: {}",
                                config.retries,
                                stderr.trim()
                            ));
                        }
                    }

                    let stdout = str::from_utf8(&output.stdout)?.to_string();
                    let filtered_output = parse_nmap_output(&stdout);
                    if filtered_output.trim().is_empty() {
                        println!(
                            "{}",
                            format!(
                                "[NSE] Script '{}' on {}:{} completed in {:.2?}, but no relevant output.",
                                script, ip, port, duration
                            )
                            .yellow()
                        );
                    } else {
                        println!(
                            "{}",
                            format!(
                                "[NSE] Script '{}' on {}:{} completed in {:.2?}.",
                                script, ip, port, duration
                            )
                            .green()
                        );
                    }

                    results.push(NseResult {
                        ip: ip.to_string(),
                        port,
                        script: script.to_string(),
                        output: filtered_output,
                    });
                    success = true;
                }
                Err(e) => {
                    println!(
                        "{}",
                        format!(
                            "[NSE] Error executing script '{}' on {}:{} => {}",
                            script, ip, port, e
                        )
                        .red()
                    );
                    if attempt < config.retries {
                        println!(
                            "{}",
                            format!(
                                "[NSE] Retrying '{}' on {}:{} after 5s",
                                script, ip, port
                            )
                            .yellow()
                        );
                        sleep(Duration::from_secs(5));
                    } else {
                        return Err(anyhow!("Nmap execution failed after {} attempts: {}", config.retries, e));
                    }
                }
            }
        }
    }
    Ok(results)
}

fn parse_nmap_output(raw: &str) -> String {
    let lines: Vec<&str> = raw.lines().collect();
    let mut useful_lines = Vec::new();

    let vuln_marker = Regex::new(r"(?i)CVE|VULNERABLE|vulnerability|exploit|http|smb|ftp").unwrap();
    let ignore_marker = Regex::new(r"^#|^|_|\s+").unwrap();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if vuln_marker.is_match(trimmed) && !ignore_marker.is_match(trimmed) {
            useful_lines.push(trimmed);
        }
    }

    useful_lines.join("\n")
}

pub fn display_nse_results(results: &[NseResult]) {
    for result in results {
        println!(
            "{}",
            format!(
                "\n[NSE-RESULT] {}:{} [{}]\n{}",
                result.ip,
                result.port,
                result.script,
                result.output
            )
            .bright_cyan()
        );
    }
}

pub fn get_nse_profiles(profile_name: &str) -> Vec<&'static str> {
    match profile_name {
        "http_vuln" => vec![
            "http-vuln-cve2006-3392",
            "http-vuln-cve2017-5638",
            "http-vuln-cve2014-3704",
            "http-slowloris",
            "http-shellshock",
        ],
        "smb_vuln" => vec![
            "smb-vuln-ms17-010",
            "smb-vuln-cve-2017-7494",
            "smb-vuln-regsvc-dos",
        ],
        "default" => vec!["vulners", "vuln"],
        _ => vec!["vulners"],
    }
}

pub fn scan_target_with_profile(ip: &str, port: u16, profile: &str) -> Result<()> {
    let scripts = get_nse_profiles(profile);
    let config = NmapNseConfig {
        scripts,
        timeout: 20,
        retries: 2,
        timing_template: "-T4",
        script_args: None,
    };

    let results = run_nse_scan(ip, port, &config)?;
    display_nse_results(&results);

    Ok(())
}
