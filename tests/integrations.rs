
extern crate rust_skeleton;

use rust_skeleton::{Server, Props};

#[test]
fn server_start_test() {
    Server::serve(Props {
        host: "127.0.0.1".to_string(),
        port: 1000
    });
}
