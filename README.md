# Gawain 
A crate to gather data from software repositories, quickly.

Re: 'gawain', this project is heavily inspired by CHAOSS's [Perceval](https://github.com/chaoss/grimoirelab-perceval) tool. 

## Usage
```
usage: gawain <backend>  <-l> <repository/repositories> <temporary directory>

-l: denotes passing gawain a comma-separated list of VCS links
repository/repositories: either a VCS link or a comma-separated list of VCS link

Repositories are reached through specific backends. Right now, Gawain supports:
    git     Fetch commit data from Git
```
