use std::{io, process, process::Command};

enum Process {
    DownloadingJava,
    RunGame,
}

fn main() {
    let mut command = String::new();
    println!("Minecraft Setup");
    println!("D: For Downloading java.");
    println!("R: For Running The game.");

    io::stdin().read_line(&mut command).expect("\nBruh\n");
    let process = match command.trim() {
        "R" | "r" => Process::RunGame,
        "D" | "d" => Process::DownloadingJava,
        _ => {
            println!("Bruh: Try to type a valid command");
            process::exit(-1);
        }
    };

    match process {
        Process::DownloadingJava => {
            println!("\nDownloading!\n");
            match Command::new("sh").arg("install.sh").spawn() {
                Ok(a) => a,
                Err(_) => {
                    println!("maybe the file \"install.sh\" doesn't exist");
                    process::exit(-1)
                }
            };
        }
        Process::RunGame => {
            println!("Running");
            match Command::new("sh").arg("rungame.sh").spawn() {
                Ok(a) => a,
                Err(_) => {
                    println!("maybe the file \"rungame.sh\" doesn't exist");
                    process::exit(-1)
                }
            };
        }
    }
}
