use std::error::Error;

use git2::Repository;

fn main() -> Result<(), Box<dyn Error>> {
    let path = ".";
    let repo = Repository::open(path)?;

    let branches = repo.branches(None).unwrap();

    println!(
        "Branches: \n{}\n",
        branches
            .into_iter()
            .map(|b| b.unwrap().0.name().unwrap().unwrap().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(())
}
