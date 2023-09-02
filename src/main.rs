use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, SystemExt};

fn main() {
    let virus_dir = std::path::PathBuf::from("C:\\Program Files (x86)\\Microsoft\\Edge\\");
    //let virus_exe = std::path::PathBuf::from("Application\\msedge.exe");
    let mut virus_exe = virus_dir.clone();
    virus_exe.push("Application\\");
    virus_exe.push("msedge.exe");
    loop {
        if virus_exe.exists() {
            println!("msedge.exe has been found. Destroying...");
            let sys = sysinfo::System::new_with_specifics(
                RefreshKind::new().with_processes(ProcessRefreshKind::new()),
            );
            for p in sys.processes_by_exact_name("msedge.exe") {
                println!("Killing msedge.exe instance: {}", p.pid());
                p.kill();
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
            match std::fs::remove_dir_all(&virus_dir) {
                Ok(_) => println!("msedge.exe has been removed from the system"),
                Err(e) => eprintln!("Failed to delete msedge.exe: {}", e),
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(120));
    }
}
