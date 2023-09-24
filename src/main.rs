mod initial_data;
mod server;

use crate::initial_data::get_initial_data;
use crate::server::initialize_server;

fn main() {
    let initial_data = get_initial_data();

    if initial_data.run_as_server {
        initialize_server(initial_data.address);
    } else {
        println!("Client mode is not implemented yet!");
    }
}
