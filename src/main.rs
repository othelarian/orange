extern crate clap;

use clap::{Arg, App, SubCommand};

use orange::{init, pour, press};

fn main() {
    let matches = App::new("Orange CLI")
        .version("0.1.0")
        .author("othelarian")
        //
        .subcommand(SubCommand::with_name("crop")
            .about("TODO add a fruit to the juice"))
        //
        .subcommand(SubCommand::with_name("dilute")
            .about("TODO dilute your juice to remove some bad taste"))
        //
        .subcommand(SubCommand::with_name("init")
            .about("initialize a directory to become a new juice")
            .arg(Arg::with_name("JUICE_NAME")
                .help("name of the project")))
        //
        .subcommand(SubCommand::with_name("harvest")
            .about("TODO check and download fruits, do also an update if necessary"))
        //
        .subcommand(SubCommand::with_name("pour")
            .about("serving the juice to your web browser")
            //
            /*
            .arg(Arg::with_name("env")
                .short("e")
                .long("env")
                .value_name("ENV_FILE")
                .help("load the given env file")
                .takes_value(true))
            */
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("select a specific port (4213 by default)")
                .takes_value(true)))
        //
        .subcommand(SubCommand::with_name("press")
            .about("TODO pressing the juice from the recipe")
            //
            .arg(Arg::with_name("env")
                .short("e")
                .long("env")
                .value_name("ENV_FILE")
                .help("load the given env file")
                .number_of_values(1)
                .takes_value(true)))
        //
        .get_matches();
    //
    match matches.subcommand() {
        ("crop", _) => println!("call add (TODO)"),
        ("dilute", _) => println!("call remove (TODO)"),
        ("init", Some(sub_a)) => init(sub_a),
        ("harvest", _) => println!("call prepare (TODO)"),
        ("pour", Some(sub_a)) => pour(sub_a),
        ("press", Some(sub_a)) => press(sub_a),
        _ => {
            println!("---------------------------------------------------------------");
            println!("Welcome to orange CLI! Please use --help to see what you can do");
            println!("---------------------------------------------------------------");
        }
    }
}
