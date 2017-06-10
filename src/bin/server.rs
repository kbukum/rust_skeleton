extern crate argparse;
extern crate rust_skeleton;

use rust_skeleton::Server;
use rust_skeleton::Props;
use argparse::{ArgumentParser, Store, Print};

fn main() {
    let props = get_options();
    println!("{:?}", props);
    Server::serve(props);
}

fn get_options() -> Props {
    let mut port: i32 = 8080;
    let mut host: String = "0.0.0.0".to_string();
    ;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.add_option(&["-V", "--version"],
                      Print(env!("CARGO_PKG_VERSION").to_string()), "Show version");
        // set host argument
        ap.refer(&mut host).add_option(&["-h", "--host"], Store, "host");

        // set port argument
        ap.refer(&mut port).add_option(&["-p", "--port"], Store, "Port for {}");

        // parse arguments
        ap.parse_args_or_exit();
    }

    Props {
        host: host,
        port: port
    }
}


#[test]
fn get_options_test() {
    let props = get_options();
    // TODO how can I do deep equal
    assert_eq!("0.0.0.0".to_string(), props.host);
    assert_eq!(8080, props.port);
}

