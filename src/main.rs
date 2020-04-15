use structopt::StructOpt;
use std::net::{IpAddr};

#[derive(StructOpt, Debug)]
#[structopt(name = "Ping", author = "Dishon Merkhai", no_version, about = "This program is a Rust implementation of the UNIX ping command")]
struct Opt {
  // Activate debug mode
  #[structopt(short, long, help ="Activate debug mode")]
  debug: bool,

  // The IP address to ping
  // ! need to implement this without requiring that --ip
  #[structopt(required = true, long, help = "The IP address (IPv4, IPv6) to send packets towards")]
  ip: IpAddr,

  // Specify TTL (Time-To-Live), reports ICMP messages that have exceeded the set TTL
  // ! need to implement this without requiring that --ttl
  #[structopt(long, default_value = "-1", help = "Set Time to live (TTL) and report packets that have exceeded the TTL")]
  ttl: i32,

  // Terminate after sending (and receiving) count ECHO_RESPONSE packets.
  #[structopt(short = "c", long = "count", default_value = "-1", help = "Terminates after sending (and receiving) count ECHO_RESPONSE packets")]
  max_packets: i32,
}


fn main() {
  let opt = Opt::from_args();
  println!("{:#?}", opt);

}
