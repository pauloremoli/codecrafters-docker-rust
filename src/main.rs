use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::os::unix::fs;
use std::fs::File;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];

    create_sandbox()?;

    
    let output = Command::new(command)
        .args(command_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;

        if !output.status.success()
        {
            match output.status.code() {
                Some(code) => { std::process::exit(code); },
                None       => println!("Process terminated by signal")
            }
        }

    Ok(())
}

fn create_sandbox() -> Result<(), _> {
    std::fs::create_dir("sandbox")?;
    fs::chroot("/sandbox")?;
    std::env::set_current_dir("/")?;
    File::create("/dev/null")?;
    Ok(())
}
