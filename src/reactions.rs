use uuid::Uuid;
use std::process::Command;
use chrono::prelude::*;

// these are examples
// customize the functions here to be refined incident response playbooks and systems recovery
// actions to take when issues happen
// put all the logic on WHEN and WHY back in main.rs, this file is purely functions to execute
// actions

// customize a1.sh b1.sh c1.sh and d1.sh if you like
// to be the shell executed on the remote system when the first threshold is triggered
// or redesign the actions and thresholds to your liking :) this is a template of sorts anyway
pub fn local1(uid: &Uuid) {
    let output = Command::new("sh")
      .arg("-c")
      .arg("journalctl --vacuum-time=1d; find /var/cache/apt/archives/*.deb  -mtime +7 -exec rm -f {} \\; pkill -9 netcat 2>/dev/null; pkill -9 telnet; systemctl restart rsyslog; systemctl restart logrotate; dmesg; ps auxwwwww; df -i; w; docker sysem prune -af")
      .output()
      .expect("Failed to execute command");
    let readi: DateTime<Utc> = Utc::now();
    println!("{} {} {:?}", readi, uid, output);
}

pub fn a1(uid: &Uuid) {
    let output = Command::new("sh")
      .arg("-c")
      .arg("ssh root@api1<a1.sh")
      .output()
      .expect("Failed to execute command");
    let readi: DateTime<Utc> = Utc::now();
    println!("{} {} {:?}", readi, uid, output);
}

pub fn b1(uid: &Uuid) {
    let output = Command::new("sh")
      .arg("-c")
      .arg("ssh -o StrictHostKeyChecking=no root@api2<b1.sh")
      .output()
      .expect("Failed to execute command");
    let readi: DateTime<Utc> = Utc::now();
    println!("{} {} {:?}", readi, uid, output);
}

pub fn c1(uid: &Uuid) {
    let output = Command::new("sh")
      .arg("-c")
      .arg("ssh -o StrictHostKeyChecking=no root@api3<c1.sh")
      .output()
      .expect("Failed to execute command");
    let readi: DateTime<Utc> = Utc::now();
    println!("{} {} {:?}", readi, uid, output);
}

pub fn d1(uid: &Uuid) {
    let output = Command::new("sh")
      .arg("-c")
      .arg("ssh -o StrictHostKeyChecking=no root@api4<d1.sh")
      .output()
      .expect("Failed to execute command");
    let readi: DateTime<Utc> = Utc::now();
    println!("{} {} {:?}", readi, uid, output);
}
