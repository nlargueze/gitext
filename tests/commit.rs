use gitt::commands::commit;

#[test]
fn commit() {
    commit::run(&commit::Args {
        cwd: None,
        dry_run: true,
        push: false,
        r#type: Some("fix".to_string()),
        scope: todo!(),
        desc: todo!(),
        body: todo!(),
        breaking_change: todo!(),
    })
}
