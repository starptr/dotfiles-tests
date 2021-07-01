use anyhow::{Result, anyhow};
use clap::{App, Arg, ArgMatches};
use cmd_lib::*;
use pretty_env_logger;
use std::env;
use dirs;

#[rustfmt::skip::macros(run_cmd)]

#[derive(strum_macros::Display, PartialEq, Clone, Copy)]
enum Mode {
    Deploy, Run,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    use_builtin_cmd!(echo, info, warn, error, die, cat);

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(App::new("deploy")
                    .about(format!("Mounts a build of {0} via docker-compose, and runs tests. Requires running {0} as a submodule of dotfiles.", env!("CARGO_PKG_NAME")).as_str())
                    .arg(Arg::with_name("skip_build")
                         .long("skip-build")
                         .help("Skip `docker-compose build`."))
                    .arg(Arg::with_name("interactive")
                         .short("i")
                         .long("interactive")
                         .help("Don't stop the container at the end.")))
        .subcommand(App::new("run")
                    .about(format!("Runs tests. Should be used where dotfiles are installed.").as_str()))
        .get_matches();

    let mut is_interactive = false;
    let mut mode: Option<Mode> = None;

    match matches.subcommand() {
        ("deploy", Some(deploy_matches)) => {
            is_interactive = deploy_matches.is_present("interactive");
            mode = Some(Mode::Deploy);
            deploy(deploy_matches)
        },
        ("run", Some(run_matches)) => {
            mode = Some(Mode::Run);
            run(run_matches)
        },
        _ => {
            Err(anyhow!("No command specified"))
        },
    }.map_err(|e| {
        eprintln!("Testing failed! Current mode: {}", mode.unwrap());
        if is_interactive {
            println!("Interactive mode enabled; skipping container stops including on test failure.");
        } else if run_cmd! (
            docker-compose down 2>&1;
        ).is_err() {
            cmd_error!("Failed to stop containers.");
        };
        e
    })
}

fn deploy(deploy_matches: &ArgMatches) -> Result<()> {
    // Set cwd to home
    env::set_current_dir(dirs::home_dir().unwrap()).unwrap();

    // Set polka dots env var to be mounted
    // This variable is used in docker-compose.yml
    let binary_path = env::current_exe();
    let binary_path = binary_path.unwrap();
    let binary_path = binary_path.to_str().unwrap();
    env::set_var("POLKA_DOTS_BIN", binary_path);

    // Skip build if flagged
    if !deploy_matches.is_present("skip_build") {
        // Build image
        run_cmd!(
            docker-compose build 2>&1;
                )?;
    };

    // Start container
    run_cmd! (
        docker-compose up -d 2>&1;
             )?;
    // Grab container id for executing in later
    let container_id = run_fun!(docker-compose ps -q)?;
    // Test and stop container
    let testing_command = format!("~/bin/{} run", env!("CARGO_BIN_NAME"));
    run_cmd! (
        docker exec -it $container_id bash -c $testing_command;
             )?;

    // Stop container if not interactive
    if deploy_matches.is_present("interactive") {
        println!("Interactive mode enabled; skipping container stops.");
    } else {
        run_cmd!(
            docker-compose down 2>&1;
                )?;
    }
    Ok(())
}

fn run(run_matches: &ArgMatches) -> Result<()> {
    // Set cwd to home
    env::set_current_dir(dirs::home_dir().unwrap()).unwrap();

    println!("inside :yeah:");

    Ok(())
}
