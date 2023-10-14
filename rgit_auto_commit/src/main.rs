use std::env::current_exe;
use log::info;
use structopt::StructOpt;
use crate::cli_args::CliArgs;
use crate::commit_helper::{process_repository, search_repositories};
use crate::logger::init_logger;
use crate::push_helper::push_to_remote;

mod logger;
pub mod auto_commit_errors;
mod commit_helper;
mod push_helper;
mod cli_args;

macro_rules! log_error_and_return {
    ($arg:ident) => {{
        info!("Fatal Error: {:?}",$arg);
        return ();
    }};
}

macro_rules! log_error {
    ($arg:ident) => {{
        info!("Error: {:?}",$arg);
    }};
}

const EXECUTEABLE_NAME:&str = "rgit_auto_commit";

fn print_help() {
    println!("{} Options:", get_exe_name());
    println!("\t--dir\tParent Directory to search git repos in");
    println!("\t--help\tPrint this Text");

}

#[inline]
fn get_exe_name() -> String {
    return  match current_exe() {
        Ok(p) => p.file_name().unwrap().to_str().unwrap().to_string(),
        Err(_) => EXECUTEABLE_NAME.to_string()
    }
}

fn print_usage() {
    println!("{} --dir <path>", get_exe_name());
    println!("run to auto commit all changes and push to remote");
    println!("{} --help for detailed help", get_exe_name());
}


fn main() -> () {
    init_logger();
    let args = CliArgs::from_args();
    if args.print_help {
        print_help()
    } else if let Some(directory) = args.base_dir {
        let repository_paths = match search_repositories(directory) {
            Ok(rs) => rs,
            Err(e) => log_error_and_return!(e)
        };
        for repo in repository_paths {
            match process_repository(&repo) {
                Ok(_) => match push_to_remote(&repo) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("{:?}", e);
                        log_error!(e)
                    }
                },
                Err(e) => log_error!(e)
            };
        }
    } else {
        print_usage()
    }
}

