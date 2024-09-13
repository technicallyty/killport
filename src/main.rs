use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide at least one number as an argument.");
        return;
    }

    let ports: Vec<u64> = args
        .iter()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    for port in ports {
        kill_process_on_port(port);
    }
}

fn kill_process_on_port(port: u64) {
    let mut binding = Command::new("lsof");
    let lsof = binding.arg("-i").arg(format!(":{}", port));
    let out = lsof.output().expect("failed to execute lsof command");
    let output = String::from_utf8_lossy(&*out.stdout);
    if output.is_empty() {
        println!("no process is running on port {}", port);
        return
    }

    let mut lines = output.lines();

    // the first line us usually the header. we want the next line.
    match lines.nth(1){
        None => {println!("no process found")}
        Some(p) => {
            let mut items = p.split_whitespace();
            let process_name = items.nth(0).unwrap();
            let process_id = items.nth(0).unwrap();
            println!("killing {}", process_name);
            let cmd = Command::new("kill").arg(format!("{}", process_id)).spawn();
            match cmd {
                Err(err) => {println!("failed to kill process: {}", err)}
                _ => {}
            }
        }
    }
}