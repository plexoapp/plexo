use std::error::Error;

use git2::Repository;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./playground/self";
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

    let diff = repo.diff_index_to_workdir(None, None)?;
    let location = git2::ApplyLocation::WorkDir;

    // let options = git2::ApplyOptions::new();

    let res = repo.apply(&diff, location, None);

    println!("Apply result: {:?}", res);

    Ok(())
}
