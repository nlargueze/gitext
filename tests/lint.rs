use gitt::{config::Config, conventional::ConventionalCommit};
use indoc::indoc;

#[test]
fn simple() {
    let msg = indoc!(
        "
    fix: commit description
    "
    );

    ConventionalCommit::parse(&msg, &Config::default()).unwrap();
}

#[test]
fn with_scope() {
    let msg = indoc!(
        "
    fix(myscope): commit description
    "
    );

    ConventionalCommit::parse(&msg, &Config::default()).unwrap();
}

#[test]
fn with_breaking_change() {
    let msg = indoc!(
        "
    fix!: commit description
    "
    );

    ConventionalCommit::parse(&msg, &Config::default()).unwrap();
}

#[test]
fn with_scope_breaking() {
    let msg = indoc!(
        "
    fix(myscope)!: commit description
    "
    );

    ConventionalCommit::parse(&msg, &Config::default()).unwrap();
}

#[test]
#[should_panic]
fn missing_description() {
    let msg = indoc!(
        "
    fix
    "
    );

    ConventionalCommit::parse(&msg, &Config::default()).unwrap();
}

#[test]
fn empty_scope() {
    let msg = indoc!(
        "
    fix(): description
    "
    );

    ConventionalCommit::parse(&msg, &Config::default()).unwrap();
}

#[test]
#[should_panic]
fn no_empty_after_scope() {
    let msg = indoc!(
        "
    fix(myscope)err_here: description
    "
    );

    ConventionalCommit::parse(&msg, &Config::default()).unwrap();
}
