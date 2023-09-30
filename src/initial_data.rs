use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};

pub(crate) struct InitialData {
    pub(crate) run_as_server: bool,
    pub(crate) address: SocketAddrV4,
}

const DEFAULT_ADDRESS: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 2137);
const RUN_AS_SERVER_FLAG: &str = "--run-as-server";

pub(crate) fn get_initial_data() -> InitialData {
    let process_arguments: Vec<String> = env::args().collect();

    let run_as_server = process_arguments.contains(&RUN_AS_SERVER_FLAG.to_string());

    let address = match process_arguments.get(1) {
        Some(address) => {
            if address == RUN_AS_SERVER_FLAG {
                DEFAULT_ADDRESS
            } else {
                address
                    .parse::<SocketAddrV4>()
                    .expect("Should properly parse the TCP server address");
            }
        }
        None => DEFAULT_ADDRESS,
    };

    return InitialData {
        run_as_server,
        address,
    };
}
