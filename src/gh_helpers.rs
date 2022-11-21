use std::process::{Command, Stdio};
fn create_repos(repo_name:&str,is_public_repo:bool) {
    let private_flag = if is_public_repo { "--public"} else {"--private"};
     Command::new("gh")
        .arg("repo")
        .arg("create")
        .arg(repo_name)
        .arg(private_flag);

}

fn get_user_name() -> String{

    let child = Command::new("git").arg("config").arg("--list").output().expect("failed to run cmd");


    let output =  String::from_utf8( child.stdout).expect("error");

    for line in output.lines() {
        if line.contains("user.name="){
            let (head,name) = line.split_at(10);
            return name.to_string();
        }
    }
    return "".to_string();
    
}

fn link_repos(){
    let user_name = get_user_name();
    let remote_link  = format!("git@github.com:{}/",user_name);
    Command::new("git")
    .arg("remote")
    .arg("add")
    .arg("orgin")
    .arg(remote_link);
    
}
