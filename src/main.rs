extern crate clap;

use clap::{Arg, App, SubCommand};

use orange::init;

fn main() {
    let matches = App::new("Orange CLI")
        .version("0.1.0")
        .author("othelarian")
        //
        //
        .subcommand(SubCommand::with_name("add")
            .about("TODO add a dependency to the project"))
        //
        .subcommand(SubCommand::with_name("build")
            .about("TODO build the project")
            //
            .arg(Arg::with_name("env")
                .short("e")
                .long("env")
                .value_name("ENV_FILE")
                .help("load the given env file")
                .takes_value(true))
            //
            //
            )
        //
        .subcommand(SubCommand::with_name("init")
            .about("TODO initialize a directory to become a new orange project")
            .arg(Arg::with_name("PROJECT_NAME")
                .help("name of the project")))
        //
        .subcommand(SubCommand::with_name("install")
            .about("TODO check and download dependencies"))
        //
        .subcommand(SubCommand::with_name("remove")
            .about("TODO remove a given dependency to the project"))
        //
        .subcommand(SubCommand::with_name("serve")
            .about("TODO start a dev server"))
        //
        .get_matches();
    //
    match matches.subcommand() {
        ("add", _) => println!("call add (TODO)"),
        ("build", _) => println!("call build (TODO)"),
        ("init", Some(sub_a)) => init(sub_a),
        ("install", _) => println!("call install (TODO)"),
        ("remove", _) => println!("call remove (TODO)"),
        ("serve", _) => println!("call serve (TODO)"),
        _ => {
            println!("---------------------------------------------------------------");
            println!("Welcome to orange CLI! Please use --help to see what you can do");
            println!("---------------------------------------------------------------");
        }
    }
}
