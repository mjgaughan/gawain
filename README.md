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
Collected data will be returned in the terminal in the format of a GitRepository object:
```
GitRepository {vcs_link : <VCS link>, tmp_path: <temporary path>, roster_list: [<<commits> <git user>> ...], roster_size: <contributor count>, commit_count: <# of commits>}
```
## Planned Work 
This project is still in its early stages, with many features yet to be implemented. Proposed features are [here](https://github.com/mjgaughan/gawain/issues). 
