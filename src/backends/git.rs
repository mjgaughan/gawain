use std::env;
use std::error::Error;
use std::process::Command;
use std::str::FromStr;

use git2::Repository;
use gawain::Target;

// GitRepository object which will also act as the blueprint for other 
// 'Repository' objects when more backends are implemented
#[derive(Debug)]
pub struct GitRepository {
    pub vcs_link: String,
    pub tmp_path: String,
    pub roster_list: Vec<String>,
    pub roster_size: i32,
    pub commit_count: i32
}
// need to also implement a function to return a standardized object
impl GitRepository{ 
    // creating the new GitRepository struct
    pub fn new(target: Target) -> Result<GitRepository, &'static str> {
        let vcs_link = target.vcs_link.clone();
        let tmp_path = target.tmp_path.clone();
        let _cloned_repo = clone_repo(target);
        let commit_count = get_commit_info().unwrap();
        let roster_tuple = get_roster_info().unwrap();
        let roster_size = roster_tuple.0;
        let roster_list = roster_tuple.1;
        //gardening
        let _deletion = Command::new("rm").args(["-r", "-f", &tmp_path]).output().expect("failed to delete wd");
        let _newdir = Command::new("mkdir").args([&tmp_path]).output().expect("replacing wd");
        Ok(GitRepository{vcs_link, tmp_path, roster_list, roster_size, commit_count})
    }
}

// Function for cloning the git repo into the temp directory
pub fn clone_repo(target: Target) -> Result<Repository, Box<dyn Error>>  {
    let url = target.vcs_link;
    let path = target.tmp_path;
    // clone target repository into the target path
    let repository = match Repository::clone(&url, path) {
        Ok(repository) => repository,
        Err(e) => panic!("Git failed to clone!: {}", e),
    };
    // set working directory to the repo
    let wd = Repository::workdir(&repository);
    let unwrapped_wd = wd.unwrap();
    assert!(env::set_current_dir(unwrapped_wd).is_ok());
    Ok(repository)
}

// Function for getting general, cross-sectional commit information
pub fn get_commit_info() -> Result<i32, Box<dyn Error>>  {
    let count_output = Command::new("git").args(["rev-list", "HEAD", "--count"]).output().expect("failed to get commit count");
    let commit_count = &String::from_utf8(count_output.stdout).unwrap();
    let commit_count_clean = &commit_count.replace("\n", "");
    let numeric_count = FromStr::from_str(commit_count_clean).unwrap();
    Ok(numeric_count)
}
// Function for getting contributor roster information (general, cross-sectional)
pub fn get_roster_info() -> Result<(i32, Vec<String>), Box<dyn Error>>  {
    let roster_output = Command::new("git").args(["shortlog", "-sne", "--all"]).output().expect("failed to get commit count");
    let commit_roster_string = String::from_utf8(roster_output.stdout).unwrap();
    let commit_roster_list: Vec<String>  = commit_roster_string.split("\n").map(|s| s.to_string()).collect();
    //let roster_size = commit_roster_list.len().try_into().unwrap() - 1;
    Ok((commit_roster_list.len().try_into().unwrap(), commit_roster_list))
}