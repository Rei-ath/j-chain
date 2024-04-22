use std::process;
use sysinfo::System;

/// .
///
/// # Panics
///
/// Panics if .
#[allow(dead_code)]
pub fn health_check() {
    let mut system = System::new_all();
    system.refresh_all();

    if let Some(process) = system.process(sysinfo::get_current_pid().unwrap()) {
        let memory = process.memory() / 1024; // Memory usage in KB
        let cpu_usage = process.cpu_usage();
        let disk_usage = process.disk_usage();

        println!(
            "Memory usage: {} KB | CPU usage: {}% | DISK usage: {}/{}",
            memory, cpu_usage, disk_usage.written_bytes, disk_usage.total_written_bytes
        );
    }
    println!("total memory: {} bytes", system.total_memory());
    println!("used memory : {} bytes", system.used_memory());
    println!("total swap  : {} bytes", system.total_swap());
    println!("used swap   : {} bytes", system.used_swap());
    println!("pid : {}", process::id());

    for cpu in system.cpus() {
        println!("CPU USAGE: {}% ", cpu.cpu_usage());
    }
}
