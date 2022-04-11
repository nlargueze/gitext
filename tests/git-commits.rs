use gitt::git::commit::get_commits;

#[test]
fn read_commits() {
    let commits = get_commits().unwrap();
    for commit in commits {
        println!("{}", commit);
    }
}
