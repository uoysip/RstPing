use structopt::StructOpt;
use std::net::{IpAddr};

// Argument parsing with structopt
#[derive(StructOpt, Debug)]
#[structopt(name = "RstPing", author = "Dishon Merkhai", no_version, about = "This program is a Rust implementation of the UNIX ping command")]

pub struct Opt {
  #[structopt(required = true, help = "The IP address (IPv4, IPv6) to send packets towards")]
  ip: IpAddr,

  #[structopt(required = true, default_value = "255", help = "Set Time to live (TTL) and report packets that have exceeded the TTL")]
  ttl: u8,

  #[structopt(short = "c", long = "count", default_value = "-1", help = "Terminates after sending (and receiving) `count` ECHO_RESPONSE packets")]
  max_packets: i32,

  // TODO: should be u32, but the library implementation has i32
  #[structopt(short = "s", default_value = "56", help = "Specify the number of data bytes to be sent.  The default is 56,
             which translates into 64 ICMP data bytes when combined with the 8
             bytes of ICMP header data.")]
  packet_size: i32,

  #[structopt(short = "i", long = "rtt", default_value = "1000", help = "Wait `wait_time` milliseconds between sending each packet.")]
  wait_time: u64,
}

// calculate mean for a vector (f32)
fn mean(data: &Vec<f32>) -> Option<f32> {
    let sum = data.iter().sum::<f32>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

// calculate standard deviation for a vector (f32)
fn std_deviation(data: &Vec<f32>) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}

// used to provide statistics on the pinging session
pub fn summary(_opt: &Opt, _icmp_seq: u32, _failed_packets: u32, _rtt_vec: &Vec<f32>) {
  println!("\n--- {} ping statistics ---", _opt.ip);
  let packet_loss: f32 = (_failed_packets as f32/_icmp_seq as f32)*100.0; 
  println!("{} packets transmitted, {} packets received, {:.3?}% packet loss", _icmp_seq, (_icmp_seq - _failed_packets), packet_loss);
  println!("round-trip min/avg/max/stddev = {:.3?}/{:.3?}/{:.3?}/{:.3?} ms", _rtt_vec.first().unwrap(), mean(_rtt_vec).unwrap(), _rtt_vec.iter().last().unwrap(), std_deviation(_rtt_vec).unwrap());
}

fn main() {
  env_logger::init();

  // capture arguments
  let opt = Opt::from_args();
  log::debug!("{:#?}", opt);

  // initialize Pinger device
  let (pinger, results) = match rst_ping::Pinger::new(Some(opt.wait_time), Some(opt.packet_size), Some(opt.ttl), opt.ip.is_ipv4()) {
      Ok((pinger, results)) => (pinger, results),
      Err(e) => panic!("Error creating pinger: {}", e)
  };

  // set the ip address and run the pinger
  pinger.add_ipaddr(&opt.ip.to_string());
  pinger.run_pinger();

  let mut rtt_vec: Vec<f32> = vec![];
  let send_size: i32 = pinger.get_size() + 8; // add 8 for the ICMP header size (8 bytes)
  let mut icmp_seq: u32 = 0;
  let mut failed_packets: u32 = 0;

  println!("PING {} ({}): {} data bytes", opt.ip, opt.ip, pinger.get_size());

  // infinitely receive packets or stop when max_packets is reached (if --count argument is set)
  let mut x: i32 = 0;
  while opt.max_packets != x {
    match results.recv() {
        Ok(result) => {
            icmp_seq += 1;
            match result {
                // case: no response from the IP address
                rst_ping::PingResult::Idle{addr} => {
                    failed_packets += 1;
                    let packet_loss: f32 = (failed_packets as f32/icmp_seq as f32) * 100.0;
                    println!("Request Timeout: TTL exceeded for {}: icmp_seq={} packet_size={}B loss={:.3?}%", addr, icmp_seq, send_size, packet_loss);
                },
                // case: response received from the IP address
                rst_ping::PingResult::Receive{addr, rtt} => {
                    println!("{} bytes from {}: icmp_seq={} ttl={} rtt={:.3?} loss={:.3?}%", send_size, addr, icmp_seq, opt.ttl, rtt, ((failed_packets as f32/icmp_seq as f32)*100.0));
                    rtt_vec.push(rtt.as_secs_f32() * 1000 as f32);
                }
            }
        },
        Err(_) => panic!("Worker threads disconnected before the solution was found!"),
    }
    x += 1;
  }
  // stop the Pinger device
  pinger.stop_pinger();

  log::debug!("Final RTT vector: {:?}", rtt_vec);

  // ping session statistics
  rtt_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
  summary(&opt, icmp_seq, failed_packets, &rtt_vec);
}


#[test]
fn test_std_deviation() {
  let mut number_vec: Vec<f32> = vec![0.0,10.0];
  assert_eq!(std_deviation(&number_vec).unwrap(), 5 as f32);

  number_vec = vec![-50.0, -10.0, -5.0, 0.0, 5.0, 10.0, 50.0];
  assert_eq!(std_deviation(&number_vec).unwrap(), 27.386127875258);

  number_vec = vec![-51437.0, -10.0, -5.0, 0.0, 5.0, 25633.0, 50.0];
  assert_eq!(std_deviation(&number_vec).unwrap(), 21407.582320671);

  number_vec = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
  assert_eq!(std_deviation(&number_vec).unwrap(), 3.1622776601684);

  number_vec = vec![0.0, 3.0, 4.0, 5.0, 6.0, 9.0];
  assert_eq!(std_deviation(&number_vec).unwrap(), 2.7537852736431);
}

#[test]
fn test_mean() {
  let mut number_vec: Vec<f32> = vec![0.0,10.0];
  assert_eq!(mean(&number_vec).unwrap(), 5 as f32);

  number_vec = vec![-50.0, -10.0, -5.0, 0.0, 5.0, 10.0, 50.0];
  assert_eq!(mean(&number_vec).unwrap(), 0 as f32);

  number_vec = vec![-51437.0, -10.0, -5.0, 0.0, 5.0, 25633.0, 50.0];
  assert_eq!(mean(&number_vec).unwrap(), -3680.5714285714);

  number_vec = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
  assert_eq!(mean(&number_vec).unwrap(), 5 as f32);

  number_vec = vec![0.0, 3.0, 4.0, 5.0, 6.0, 9.0];
  assert_eq!(mean(&number_vec).unwrap(), 4.5 as f32);
}