use std::env;
use std::process;

use gawain::Target;
mod backends;
use backends::git;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut is_list = false;
    if args[2] == "-l"{
        println!("list!");
        is_list = true;
    } else {
        println!("not list!");
    }

    if !is_list {
        let target = Target::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    
        println!("Looking for project {}", target.vcs_link);
    
        if target.vcs_type != "git"{
            println!("Sorry! Only git right now.");
            process::exit(1);
        }
    
        let git_results = git::GitRepository::new(target).unwrap_or_else(|err| {
            println!("Problem with git data: {}", err);
            process::exit(1);
        });
        println!("{:?}",git_results);
    }

}
