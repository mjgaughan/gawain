use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::str::Split;
use std::error::Error;

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
        if args[1].clone() != "git"{
            println!("Sorry! Only git right now.");
            process::exit(1);
        }

        let _temp = get_project_git_info(args);
    } else {
        let file_path = args[3].clone();
        let projects = list_handling(file_path);
        let tmp_path = args[4].clone();
        let vcs_type = args[1].clone();
        if vcs_type != "git"{
            println!("Sorry! Only git right now.");
            process::exit(1);
        }
        let first_arg = args[0].clone();
        for project in projects.unwrap(){
            let psuedo_args:Vec<String> = vec![first_arg.clone(), vcs_type.clone(),  project.clone(), tmp_path.clone()];
            let _temp = get_project_git_info(psuedo_args);
        }
    }

}

fn get_project_git_info(args: Vec<String>) -> Result<git::GitRepository, Box<dyn Error>>  {
    let target = Target::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let git_results = git::GitRepository::new(target).unwrap_or_else(|err| {
        println!("Problem with git data: {}", err);
        process::exit(1);
    });
    println!("{:?}",git_results);
    Ok(git_results)
}


fn list_handling (filename: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut f = File::open(filename).expect("list file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let projects: Vec<String> = contents.split(',')
        .map(|s| s.trim().to_string()) 
        .collect();
    Ok(projects)
}