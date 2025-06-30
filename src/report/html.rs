use serde::Serialize;
use std::fs::File;
use std::io::Write;
use anyhow::Result;

pub fn write_html_report<T: Serialize>(data: &[T], path: &str) -> Result<()> {
    let mut file = File::create(path)?;

    let header = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>ElainaScan Report</title>
<style>
body { font-family: monospace; background: #0e0e0e; color: #e0e0e0; padding: 20px; }
table { width: 100%; border-collapse: collapse; margin-top: 20px; }
th, td { border: 1px solid #444; padding: 8px; text-align: left; }
th { background: #1f1f1f; }
tr:nth-child(even) { background: #1a1a1a; }
pre { white-space: pre-wrap; word-wrap: break-word; }
</style>
</head>
<body>
<h1>ElainaScan Report</h1>
"#;

    file.write_all(header.as_bytes())?;

    let json = serde_json::to_value(data)?;
    let arr = json.as_array().ok_or_else(|| anyhow::anyhow!("Expected JSON array"))?;

    if arr.is_empty() {
        file.write_all(b"<p>No data available.</p>")?;
    } else {
        let keys = arr[0].as_object().unwrap().keys().collect::<Vec<_>>();

        file.write_all(b"<table><thead><tr>")?;
        for key in &keys {
            file.write_all(format!("<th>{}</th>", key).as_bytes())?;
        }
        file.write_all(b"</tr></thead><tbody>")?;

        for obj in arr {
            let obj = obj.as_object().unwrap();
            file.write_all(b"<tr>")?;
            for key in &keys {
                let value = obj.get(*key).map(|v| v.to_string()).unwrap_or_default();
                file.write_all(format!("<td><pre>{}</pre></td>", html_escape(&value)).as_bytes())?;
            }
            file.write_all(b"</tr>")?;
        }

        file.write_all(b"</tbody></table>")?;
    }

    let footer = r#"
</body>
</html>
"#;
    file.write_all(footer.as_bytes())?;

    Ok(())
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
