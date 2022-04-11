//! Tests git tags

#[test]
fn get_tags() {
    let _tags = gitt::git::tag::get_tags().unwrap();
    // println!("--- TAGS ---");
    // for tag in tags {
    //     println!("{}", tag);
    // }
    // println!("------------");
}

#[test]
fn get_latest_version() {
    let _version = gitt::git::tag::get_latest_version().unwrap();
    // println!("{:?}", version);
}
