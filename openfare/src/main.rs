use env_logger;
use structopt::StructOpt;

mod command;
mod common;
mod extension;
mod price;
mod profile;
mod setup;

fn main() {
    let env = env_logger::Env::new().filter_or("OPENFARE_LOG", "off");
    env_logger::Builder::from_env(env).init();

    let args: Vec<String> = std::env::args().collect();
    let (openfare_args, extension_args) = split_extension_args(&args);
    let commands = command::Opts::from_iter(openfare_args.iter());

    match command::run_command(commands.command, &extension_args) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(-2)
        }
    }
}

/// Arguments after -- are passed to extensions.
fn split_extension_args(args: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let split_element = "--";
    let mut pre_split = vec![];
    let mut post_split = vec![];

    let mut split_point_found = false;
    for arg in args {
        if arg == split_element {
            split_point_found = true;
            continue;
        }
        if !split_point_found {
            pre_split.push(arg.clone());
        } else {
            post_split.push(arg.clone());
        }
    }
    (pre_split, post_split)
}
