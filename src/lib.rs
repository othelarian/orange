extern crate clap;

use clap::ArgMatches;
use micro_http_server::MicroHTTP;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::{thread, time};

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

pub fn pour(args: &ArgMatches) {
    let port = if args.is_present("port") {
        match args.value_of("port").unwrap().parse::<u32>() {
            Ok(v) => v,
            Err(_) => {
                println!("Please specify a real port for the server");
                return;
            }
        }
    } else { 4213 };
    let addr = &format!("127.0.0.1:{}", port);
    let server = MicroHTTP::new(addr)
        .expect("Impossible to find a glass to pour in");
    println!("Pouring on {}", addr);
    thread::spawn(move || loop {
        let res = server.next_client();
        if res.is_err() {
            println!("Server failed: {:?}", res);
            break;
        }
        match res.unwrap() {
            None => thread::sleep(time::Duration::from_millis(200)),
            Some(mut client) => {
                if client.request().is_none() {
                    client.respond("400", "No request :(".as_bytes(), &vec!()).unwrap_err();
                } else {
                    let req_copy = client.request().as_ref().unwrap();
                    let url_split: Vec<&str> = req_copy.split('?').collect();
                    let sending = match url_split.get(0) {
                        Some(path) => match path {
                            &"/" => {
                                //
                                // TODO : create a home page
                                //
                                // TODO : list the files inside the directory
                                //
                                client.respond_ok("WIP".as_bytes())
                                //
                            }
                            //
                            //
                            &"/logo" => {
                                //
                                if let Ok(logo) = fs::read_to_string(Path::new("./resources/logo/logo.svg")) {
                                    client.respond("200 OK", logo.as_bytes(), &vec!(String::from("Content-Type: image/svg+xml")))
                                } else {
                                    client.respond("500 Server Error", "Error delivering juice".as_bytes(), &vec!())
                                }
                                //
                                //
                            }
                            //
                            //
                            //
                            _ => client.respond("404 Not Found", "No juice found".as_bytes(), &vec!())
                        }
                        None => client.respond("404 Not Found", "No juice found".as_bytes(), &vec!())
                    };
                    sending.expect("Couldn't deliver some juice");
                }
            }
        }
    });
    //
    // TODO : launch the watcher here
    //
    let mut w = String::new();
    io::stdin().read_line(&mut w).unwrap();
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
