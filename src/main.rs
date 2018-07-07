

extern crate failure;
extern crate git2;

use failure::Error;
use git2::{BranchType, Repository};

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let repo = Repository::open(".")?;
    println!("... got repo");
    let master = repo.find_branch("master", BranchType::Local)?;
    println!("... got branch");
    let commit = master.get().peel_to_commit()?;
    println!("... got commit");
    let msg = commit.message().unwrap();
    println!("Latest commit message: {}", msg);

    Ok(())
}
