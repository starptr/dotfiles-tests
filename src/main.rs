use anyhow::{Result, anyhow};
use clap::{App, Arg, ArgMatches};
use cmd_lib::*;
use pretty_env_logger;
use std::env;
use dirs;

#[rustfmt::skip::macros(run_cmd)]

fn main() -> Result<()> {
    pretty_env_logger::init();
    use_builtin_cmd!(info, warn, error, die, cat);

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(App::new("deploy")
                    .about(format!("Mounts a build of {0} via docker-compose, and runs tests. Requires running {0} as a submodule of dotfiles.", env!("CARGO_PKG_NAME")).as_str()))
        .subcommand(App::new("run")
                    .about(format!("Runs tests. Should be used where dotfiles are installed.").as_str()))
        .get_matches();

    match matches.subcommand() {
        ("deploy", Some(deploy_matches)) => {
            deploy(deploy_matches)
        },
        ("run", Some(run_matches)) => {
            run(run_matches)
        },
        _ => {
            Err(anyhow!("No command specified"))
        },
    }.map_err(|e| {
        if run_cmd! (
            docker-compose down 2>&1;
        ).is_err() {
            cmd_error!("Failed to stop containers.");
        };
        e
    })
}

fn deploy(deploy_matches: &ArgMatches) -> Result<()> {
    let binary_path = env::current_exe();
    let binary_path = binary_path.unwrap();
    let binary_path = binary_path.to_str().unwrap();

    let home_dir = dirs::home_dir().unwrap();
    let home_dir = home_dir.to_str().unwrap();

    env::set_var("POLKA_DOTS_BIN", binary_path);

    run_cmd! (
        cd $home_dir;
        docker-compose build 2>&1;
        docker-compose up -d 2>&1;
    )?;
    let container_id = run_fun!(docker-compose ps -q)?;
    run_cmd! (
        docker exec -it $container_id bash -c "expr 1 + 1";
        docker-compose down 2>&1;
    )?;
    Ok(())
}

fn run(run_matches: &ArgMatches) -> Result<()> {
    Ok(())
}
