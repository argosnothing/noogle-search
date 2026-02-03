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
            "--bind",
            &format!("ctrl-o:execute({} open-source {{}})", exe_path.display()),
            "--bind",
            &format!("ctrl-n:execute({} open-noogle {{}})", exe_path.display()),
            "--header",
            "Ctrl-O: Open source | Ctrl-N: Open noogle | Ctrl-/: Toggle preview",
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
