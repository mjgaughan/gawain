//use backends::git;

pub struct Target {
    pub vcs_type: String,
    pub vcs_link: String,
    pub tmp_path: String
}

impl Target{
    pub fn new(args: &[String]) -> Result<Target, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else if  args.len() > 4 {
            return Err("too many arguments");
        }
        let vcs_type = args[1].clone();
        let vcs_link = args[2].clone();
        let tmp_path = args[3].clone();

        Ok(Target { vcs_type, vcs_link, tmp_path })
    }
}