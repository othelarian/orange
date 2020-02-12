extern crate clap;

use clap::ArgMatches;
use micro_http_server::MicroHTTP;
use std::env;
use std::fs::{self, File};
use std::io::Write;

// PUBLIC ZONE ######################################################

pub fn init(args: &ArgMatches) {
    let c_pth = env::current_dir().unwrap();
    let is_empty = c_pth.read_dir().unwrap().next().is_none();
    if args.is_present("JUICE_NAME") {
        let p_name = args.value_of("JUICE_NAME").unwrap();
        if check_juice_name(p_name) {
            println!("Are you sure about the name of this juice?");
            println!("{}", args.usage());
        }
        else if c_pth.ends_with(p_name) {
            if is_empty { gen_juice(p_name); }
            else {error_output(OrangeError::InitDirNotEmpty(args.usage()));}
        } else {
            fs::create_dir(p_name).expect("Impossible to create the juice directory");
            env::set_current_dir(c_pth.join(p_name)).unwrap();
            gen_juice(p_name);
        }
    } else {
        let p_name = c_pth.file_name().unwrap().to_str().unwrap();
        if is_empty { gen_juice(p_name); }
        else {error_output(OrangeError::InitDirNotEmpty(args.usage()));}
    }
}

pub fn pour(_args: &ArgMatches) {
    //
    println!("trying to serve (TODO)");
    //
    let addr = "127.0.0.1:4213";
    //
    let _server = MicroHTTP::new(addr.into())
        .expect("Impossible to find a glass to pour in");
    //
    println!("Pouring on {}", addr);
    //
    //let _conn =
    //
}

pub fn press(_args: &ArgMatches) {
    //
    println!("Eh! let's go build something! (TODO)");
    //
    //
}

// PRIVATE ZONE #####################################################

enum OrangeError<'a> {
    InitDirNotEmpty(&'a str)
}



fn error_output(err: OrangeError) {
    match err {
        OrangeError::InitDirNotEmpty(e) => {
            println!("You're already in a directory named similarly to the juice,");
            println!("But it looks like this juice already exists.");
            println!("Maybe it's time for a real juice?");
            println!("{}", e);
        }
    }
}

// CROP =========================================

// INIT =========================================

fn check_juice_name(_name: &str) -> bool {
    //
    // TODO
    //
    return false;
}

fn gen_juice(name: &str) {
    println!("generate juice '{}', NOW!", name);
    println!("---------------------------------------------------------");
    println!("Adding shop list file...");
    //
    //let mut recipe = "";
    //
    //
    let mut json = File::create("orange.json")
        .expect("Failed to generate the shopping list");
    //
    json.write_all(b"aaaaa")
        .expect("Failed to fill the shopping list");
    //
    println!("Making a new basket directory...");
    fs::create_dir("src").expect("Failed to create the basket directory");
    env::set_current_dir(env::current_dir().unwrap().join("src")).unwrap();
    println!("Adding basic recipe file...");
    let mut main_oran = File::create("Main.juice")
        .expect("Failed to generate the basic recipe");
    //
    main_oran.write_all(b"aaa")
        .expect("Failed to fill basic recipe into the Main.juice file");
    //
    println!("\nDONE! You did it!\n");
}

// HARVEST ======================================

// POUR =========================================

// PRESS ========================================

// REMOVE =======================================
