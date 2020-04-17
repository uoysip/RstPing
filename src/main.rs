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
  #[structopt(short, long, default_value = "255", help = "Set Time to live (TTL) and report packets that have exceeded the TTL")]
  ttl: u8,

  // Terminate after sending (and receiving) count ECHO_RESPONSE packets.
  #[structopt(short = "c", long = "count", default_value = "-1", help = "Terminates after sending (and receiving) count ECHO_RESPONSE packets")]
  max_packets: i32,

  #[structopt(short = "s", default_value = "56", help = "Specify the number of data bytes to be sent.  The default is 56,
             which translates into 64 ICMP data bytes when combined with the 8
             bytes of ICMP header data.  This option cannot be used with ping
             sweeps.")]
  packet_size: i32, // TODO: should be u32, but the library implementation has i32

}


fn main() {
  let opt = Opt::from_args();
  println!("{:#?}", opt);

  // experimental implementation provided by fastping_rs documentation
  env_logger::init();
  let (pinger, results) = match ping_util_rs::Pinger::new(None, Some(opt.packet_size), Some(opt.ttl), opt.ip.is_ipv4()) {
      Ok((pinger, results)) => (pinger, results),
      Err(e) => panic!("Error creating pinger: {}", e)
  };


  pinger.add_ipaddr(&opt.ip.to_string());
  pinger.run_pinger();
  // add 8 for the ICMP header size (8 bytes)
  let send_size: i32 = pinger.get_size() + 8;

  let mut icmp_seq: i32 = 0;

  println!("PING {} ({}): {} data bytes", opt.ip, opt.ip, pinger.get_size());

  loop {
    icmp_seq += 1;
      match results.recv() {
          Ok(result) => {
              match result {
                  ping_util_rs::PingResult::Idle{addr} => {
                      log::error!("TTL Time Exceeded from {}: icmp_seq={} payload={}B", addr, icmp_seq, send_size);
                  },
                  ping_util_rs::PingResult::Receive{addr, rtt} => {
                      println!("{} bytes from {}: icmp_seq={} ttl={} rtt={:?}.", send_size, addr, icmp_seq, opt.ttl, rtt);
                  }
              }
          },
          Err(_) => panic!("Worker threads disconnected before the solution was found!"),
      }
  }


}
