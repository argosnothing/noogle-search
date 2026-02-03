use anyhow::Result;
use std::env;
use std::process::Command;

pub fn execute(initial_filter: Option<String>, initial_query: Option<String>) -> Result<()> {
    let exe_path = env::current_exe()?;

    let initial_cmd = if let Some(filter) = &initial_filter {
        format!("{} print --filter {}", exe_path.display(), filter)
    } else {
        format!("{} print", exe_path.display())
    };

    let mut fzf_args = vec![
        "--preview".to_string(),
        format!("{} preview {{}}", exe_path.display()),
        "--preview-window=wrap".to_string(),
        "--scheme=history".to_string(),
        "--layout=reverse".to_string(),
        "--with-nth=1".to_string(),
        "--delimiter=\t".to_string(),
        "--bind".to_string(),
        format!("ctrl-o:execute({} open-source {{}})", exe_path.display()),
        "--bind".to_string(),
        format!("ctrl-n:execute({} open-noogle {{}})", exe_path.display()),
        "--bind".to_string(),
        format!("ctrl-l:reload({} print --filter lib)", exe_path.display()),
        "--bind".to_string(),
        format!("ctrl-b:reload({} print --filter builtins)", exe_path.display()),
        "--bind".to_string(),
        format!("ctrl-p:reload({} print --filter pkgs)", exe_path.display()),
        "--bind".to_string(),
        format!("ctrl-a:reload({} print)", exe_path.display()),
        "--header".to_string(),
        "Ctrl-L: lib | Ctrl-B: builtins | Ctrl-P: pkgs | Ctrl-A: all\nCtrl-O: source | Ctrl-N: noogle | Ctrl-/: preview".to_string(),
    ];

    if let Some(query) = initial_query {
        fzf_args.push("--query".to_string());
        fzf_args.push(query);
    }

    let mut child = Command::new("fzf")
        .args(&fzf_args)
        .stdin(
            Command::new("sh")
                .arg("-c")
                .arg(&initial_cmd)
                .stdout(std::process::Stdio::piped())
                .spawn()?
                .stdout
                .take()
                .unwrap(),
        )
        .spawn()?;

    child.wait()?;
    Ok(())
}
