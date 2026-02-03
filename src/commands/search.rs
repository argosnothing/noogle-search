use anyhow::Result;
use std::process::Command;
use std::env;

pub fn execute(initial_filter: Option<String>) -> Result<()> {
    let exe_path = env::current_exe()?;
    
    let initial_cmd = if let Some(filter) = &initial_filter {
        format!("{} print --filter {}", exe_path.display(), filter)
    } else {
        format!("{} print", exe_path.display())
    };
    
    let mut child = Command::new("fzf")
        .args(&[
            "--preview",
            &format!("{} preview {{}}", exe_path.display()),
            "--preview-window=wrap",
            "--scheme=history",
            "--layout=reverse",
            "--with-nth=1",
            "--delimiter=\t",
            "--bind",
            &format!("ctrl-o:execute({} open-source {{}})", exe_path.display()),
            "--bind",
            &format!("ctrl-n:execute({} open-noogle {{}})", exe_path.display()),
            "--bind",
            &format!("ctrl-l:reload({} print --filter lib)", exe_path.display()),
            "--bind",
            &format!("ctrl-b:reload({} print --filter builtins)", exe_path.display()),
            "--bind",
            &format!("ctrl-p:reload({} print --filter pkgs)", exe_path.display()),
            "--bind",
            &format!("ctrl-a:reload({} print)", exe_path.display()),
            "--header",
            "Ctrl-L: lib | Ctrl-B: builtins | Ctrl-P: pkgs | Ctrl-A: all\nCtrl-O: source | Ctrl-N: noogle | Ctrl-/: preview",
        ])
        .stdin(Command::new("sh")
            .arg("-c")
            .arg(&initial_cmd)
            .stdout(std::process::Stdio::piped())
            .spawn()?
            .stdout
            .take()
            .unwrap())
        .spawn()?;
    
    child.wait()?;
    Ok(())
}
