use std::process::{Command, Stdio};
pub fn create_respo(repo_name:&str,is_public_repo:bool) {
    let private_flag = if is_public_repo { "--public"} else {"--private"};
    let output = Command::new("gh")
        .arg("repo")
        .arg("create")
        .arg(repo_name)
        .arg(private_flag);

}
