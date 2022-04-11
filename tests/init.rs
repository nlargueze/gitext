use gitt::commands::init;

/// Initializes a new git repository
#[test]
fn init_reset() {
    init::run(&init::Args {
        cwd: None,
        reset: true,
    })
}
