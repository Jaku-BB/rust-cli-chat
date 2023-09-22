mod server;

use crate::server::initialize_server;
use std::thread;

fn main() {
    thread::spawn(|| {
        initialize_server("127.0.0.1:2137");
    });
}
