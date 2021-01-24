use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::process::Command;
use std::time::Duration;
use std::fmt::Debug;

fn initialize_watcher() {
    println!("INITIALIZING WATCHER");
    let path = "/home/remotefive/Documents/repo/local/test";
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();
    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

/// supersync
/// Allows multiple computers connected to watch, diff, and update each other's project files.
///
/// * Watch a file directory
/// * If a change is detected on this file dir, make the other file directory fetch the file from
///     the other.
/// * Run diff on file directory with another
/// * Sync file directories based on diff
fn main() {
    let client1 = "../mock/client1/test";
    let client2 = "../mock/client2/test";
    let arg = "-qr";
    let args = [arg, client1, client2].join(" ");
    let output = Command::new("diff")
        .arg(&args)
        .output()
        .expect("Failed to execute command");

    println!("{}", output.status);
}
