use anyhow::Result;
use clap::{Arg, App};
use cmd_lib::*;
use pretty_env_logger;

#[rustfmt::skip::macros(run_cmd)]

fn main() -> Result<()> {
    pretty_env_logger::init();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(App::new("deploy")
                    .about(format!("Mounts a build of {0} via docker-compose, and runs tests. Requires running {0} as a submodule of dotfiles.", env!("CARGO_PKG_VERSION")).as_str())
                    .arg(Arg::with_name("dev")
                         .short("d")
                         .long("dev")
                         .help("Use dev build."))
                    .arg(Arg::with_name("release")
                         .short("r")
                         .long("release")
                         .help("Use release build.")
                         .conflicts_with("dev")))
        .subcommand(App::new("run")
                    .about(format!("Runs tests. Should be used where dotfiles are installed.").as_str()))
        .get_matches();

    use_builtin_cmd!(info, warn, error, die, cat);

    cmd_info!("hello");
    run_cmd! (
        info "yah";
        echo "echo test";
        which yadm;
    )?;
    Ok(())
}
