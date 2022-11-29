use std::process::Command;
fn create_repos(repo_name:&str,is_public_repo:bool) {
    println!("creating remote repository");
    let private_flag = if is_public_repo { "--public"} else {"--private"};
     let mut child  = Command::new("gh")
        .arg("repo")
        .arg("create")
        .arg(repo_name)
        .arg(private_flag).spawn().expect("failed to run [gh create] ");
    let _res = child.wait();
    println!("finish creating remote repository");

}

fn get_user_name() -> String{

    let child = Command::new("git").arg("config").arg("--list").output().expect("failed to run cmd");


    let output =  String::from_utf8( child.stdout).expect("error");

    for line in output.lines() {
        if line.contains("user.name="){
            let (_,name) = line.split_at(10);
            return name.to_string();
        }
    }
    return "".to_string();
    
}

fn link_repos(dir_name:&str){
    let user_name = get_user_name();
    let remote_link  = format!("git@github.com:{}/{}.git",user_name,dir_name);
    println!("linking with {}",remote_link);
   let mut child1 =  Command::new("git")
    .arg("remote")
    .arg("add")
    .arg("origin")
    .arg(remote_link).spawn().expect("failed to run [git remote add]");
    let _res = child1.wait();
    println!("setting push up-stream");
   let mut child2 =  Command::new("git")
    .arg("push")
    .arg("--set-upstream")
    .arg("origin")
    .arg("main").spawn().expect("failed to run [git remote add]");
    let _res = child2.wait();
    
}
pub fn push_mode(dir_name:&str,is_public_repo:bool) {
    create_repos(dir_name, is_public_repo);
    link_repos(dir_name);
}
