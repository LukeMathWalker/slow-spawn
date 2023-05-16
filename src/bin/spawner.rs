use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello before spawning!");

    let mut process = std::process::Command::new("./target/release/spawned")
        .arg("4")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    // Loop and read from the output of the child process, printing it to stdout
    // as we go.
    let mut stdout = process.stdout.take().unwrap();
    let mut parent_stdout = std::io::stdout().lock();
    let mut stderr = process.stderr.take().unwrap();
    let mut parent_stderr = std::io::stderr().lock();
    let mut buffer = [0; 1024];
    let _status = loop {
        let mut check_status = false;

        match stdout.read(&mut buffer) {
            Ok(0) | Err(_) => {
                check_status = true;
            },
            Ok(n) => {
                parent_stdout.write_all(&buffer[..n])?;
            }
        }

        match stderr.read(&mut buffer) {
            Ok(0) | Err(_) => {
                check_status = true;
            },
            Ok(n) => {
                parent_stderr.write_all(&buffer[..n])?;
            }
        }

        if check_status {
            if let Some(s) = process.try_wait()? {
                break s;
            }
            check_status = false;
        }
    };

    println!("Hello after spawning!");
    Ok(())
}
