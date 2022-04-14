use gitext::commands::init;

/// Initializes a new git repository
#[test]
fn reset_config() {
    init::run(&init::Args {
        cwd: None,
        reset: true,
    })
}
