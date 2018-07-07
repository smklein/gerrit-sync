extern crate clap;
#[macro_use]
extern crate failure;
extern crate git2;

use clap::{Arg, App};
use failure::Error;
use git2::{BranchType, Repository};

#[derive(Debug)]
enum Command {
    Synchronize,
}

fn main() -> Result<(), Error> {
    let matches = App::new("gerrit-sync")
        .version("0.1.0")
        .author("Sean Klein")
        .about("Git commit synchronization tool")
        .arg(Arg::with_name("command")
            .required(false)
            .takes_value(true)
            .index(1)
            .help("What to do; defaults to ____"))
        .arg(Arg::with_name("repo")
            .short("r")
            .long("repo")
            .takes_value(true)
            .value_name("PATH")
            .help("Provides a path to a repository"))
        .get_matches();

    let command = match matches.value_of("command").unwrap_or("sync") {
        "sync" => Command::Synchronize,
        _ => return Err(format_err!("Unknown command")),
    };

    let repo_path = matches.value_of("repo").unwrap_or(".");

    println!("{:?}", command);

    let repo = Repository::open(repo_path)?;
    println!("... got repo");
    let master = repo.find_branch("master", BranchType::Local)?;
    println!("... got branch");
    let commit = master.get().peel_to_commit()?;
    println!("... got commit");
    let msg = commit.message().unwrap();
    println!("Latest commit message: {}", msg);
    Ok(())
}
