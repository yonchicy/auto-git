
use std::env;
use clap::{command, Arg, ArgAction};

pub mod gh_helpers;

fn main() {
    let matches =
        command!()
            .propagate_version(true)
            .arg_required_else_help(true)
            .arg(
                Arg::new("public")
                    .long("public")
                    .help("create a public remote repository")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("private")
                    .long("private")
                    .help("create a private remote repository")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("push")
                    .short('p')
                    .long("push")
                    .help("create a new remote repository and link it with the current directory")
                    .action(ArgAction::SetTrue),
            )
            .arg(Arg::new("init").short('i').long("init").help(
                "init a local git project and link it with a newly created remote repository",
            ))
            .get_matches();

    let is_public_repo = matches.get_flag("public");
    let is_push_mode = matches.get_flag("push");
    
    if is_push_mode {
        let dir_path = env::current_dir().unwrap();
        let dir_name = dir_path.file_name();

        //todo
        println!("pushing {:?} ,is a is_public_repo{{{}}}",dir_name,is_public_repo);
        
    }
    else {
        //todo
        let dir_name  = matches.get_one::<String>("init");
        println!("pushing {:?} ,is a is_public_repo{}",dir_name,is_public_repo);
    }
}
