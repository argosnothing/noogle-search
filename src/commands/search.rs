use anyhow::Result;
use std::process::Command;
use std::env;

pub fn execute() -> Result<()> {
    let exe_path = env::current_exe()?;
    
    let mut child = Command::new("fzf")
        .args(&[
            "--preview",
            &format!("{} preview {{}}", exe_path.display()),
            "--preview-window=wrap",
            "--scheme=history",
        ])
        .stdin(Command::new(&exe_path)
            .arg("print")
            .stdout(std::process::Stdio::piped())
            .spawn()?
            .stdout
            .take()
            .unwrap())
        .spawn()?;
    
    child.wait()?;
    Ok(())
}
