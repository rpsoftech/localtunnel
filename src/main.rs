use std::{env, fs, io, path::PathBuf};

use anyhow::Error;
use serde::{Deserialize, Serialize};

use bore_cli::client::Client;

#[derive(Serialize, Deserialize, Debug)]
struct LocalRemotePortMapping {
    local: u16,
    remote: u16,
}

#[derive(Serialize, Deserialize)]
struct LocalTunnelConfig {
    remote: String,
    secret: Option<String>,
    // Map<local port,remote port>
    ports: Vec<LocalRemotePortMapping>,
}

fn inner_main() -> io::Result<PathBuf> {
    let mut dir = env::current_dir()?;
    dir.pop();
    dir.push("config.json");
    Ok(dir)
}

#[tokio::main]
async fn run() {
    // println!("In file {}", file_path);
    let path = inner_main().expect("Couldn't");
    println!("{}", path.display());
    let contents =
        fs::read_to_string("config.json").expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    let config: LocalTunnelConfig = serde_json::from_str(&contents).unwrap();

    // Client::new(local_host, local_port, to, port, secret)
    let mut handles:Vec<tokio::task::JoinHandle<Result<(), Error>>> = vec![];

    for LocalRemotePortMapping { local, remote } in config.ports.into_iter() {
        let remote_host = config.remote.clone();
        let secrate = config.secret.clone();

        handles.push(tokio::spawn(async move {
            // println!("this is thread number {}", i);
            println!("local port = {:?}", local);
            let client =
                Client::new("127.0.0.1", local, &remote_host, remote, secrate.as_deref()).await?;
            // println!("HERE");
            client.listen().await?;
            // println!("HERE 2");
            return Ok(());
        }));
    }
    for handle in handles {
         let _ =handle.await.expect("Panic in task");
    }
    //  handles[0].join().unwrap();
    // join(reader, writer)
}

fn main() {
    tracing_subscriber::fmt::init();
    run()
}