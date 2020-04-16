use structopt::StructOpt;
use std::net::{IpAddr};

#[derive(StructOpt, Debug)]
#[structopt(name = "ping-util-rs", author = "Dishon Merkhai", no_version, about = "This program is a Rust implementation of the UNIX ping command")]
struct Opt {
  // Activate debug mode
  #[structopt(short, long, help ="Activate debug mode")]
  debug: bool,

  // The IP address to ping
  // ! need to implement this without requiring --ip
  #[structopt(required = true, long, help = "The IP address (IPv4, IPv6) to send packets towards")]
  ip: IpAddr,

  // Specify TTL (Time-To-Live), reports ICMP messages that have exceeded the set TTL
  // ! need to implement this without requiring --ttl
  #[structopt(long, default_value = "-1", help = "Set Time to live (TTL) and report packets that have exceeded the TTL")]
  ttl: i32,

  // Terminate after sending (and receiving) count ECHO_RESPONSE packets.
  #[structopt(short = "c", long = "count", default_value = "-1", help = "Terminates after sending (and receiving) count ECHO_RESPONSE packets")]
  max_packets: i32,
}


fn main() {
  let opt = Opt::from_args();
  println!("{:#?}", opt);

    // experimental implementation provided by fastping_rs documentation
    env_logger::init();
    let (pinger, results) = match Pinger::new(None, None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e)
    };


    pinger.add_ipaddr(&opt.ip.to_string());
    pinger.run_pinger();

    loop {
      println!("Looping...");
        match results.recv() {
            Ok(result) => {
                match result {
                    Idle{addr} => {
                        error!("Idle Address {}.", addr);
                    },
                    Receive{addr, rtt} => {
                        println!("Receive from Address {} in {:?}.", addr, rtt);
                    }
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }


}
