use std::str;
use std::process::Command;
use sysinfo::{SystemExt, System, CpuExt};

pub fn memz() -> String {
    let mut sys = System::new_all();
    sys.refresh_memory();
    let total_memory: f64 = sys.total_memory() as f64;
    let used_memory: f64 = sys.used_memory() as f64;
    let percmem: f64 = used_memory/total_memory;
    return percmem.to_string()
}

pub fn dskz() -> String {
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();
    lines.next();
    if let Some(line) = lines.next() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() >= 4 {
            let used_space = fields[4].replace("%",".00").parse::<f32>().unwrap_or(0.0);
            return format!("{:.2}", used_space);
        }
    }
    return "ERR".to_string();
}


pub fn netz() -> String {
    let output = Command::new("ss")
        .arg("-tulpan")
        .output()
        .expect("failed to execute ss");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let count = stdout.lines()
        .skip(2) 
        .filter(|line| line.contains("ESTAB"))
        .count();

    format!("{}", count)
}


pub fn cpuz() -> String {
    let output = Command::new("cat")
        .arg("/proc/loadavg")
        .output()
        .expect("failed to execute cat");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let count: Vec<_> = stdout.chars().collect();
    format!("{}{}{}{}", count[0], count[1], count[2], count[3])
}

// sysinfo cpu measure is busted:
//pub fn cpuz() -> String {
//    let system = System::new_all();
//    let cpu_usage = system.cpus()
//        .iter()
//        .map(|p| p.cpu_usage())
//        .sum::<f32>() / system.cpus().len() as f32;
//
//    format!("{:.2}", cpu_usage)
//}
