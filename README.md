# RstPing

RstPing is a highly performant CLI ping utility meant to offer similar functionality to the UNIX `ping` command. This program was developed for the 2020 [Cloudflare Internship Application: Systems challenge](https://github.com/cloudflare-internship-2020/internship-application-systems).

## Installation

A release-optimized executable is avaiable on the [releases page](https://github.com/uoysip/ping-util-rs/releases), as well as source code for the latest version. If you would like to build the latest version yourself, follow the commands below:

```bash
git clone https://github.com/uoysip/RstPing.git
cd ./RstPing/
sudo cargo run -- --help
```

## Usage

```
USAGE:
    rst_ping [OPTIONS] <ip> <ttl>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <max-packets>    Terminates after sending (and receiving) `count` ECHO_RESPONSE packets [default: -1]
    -s <packet-size>             Specify the number of data bytes to be sent.  The default is 56,
                                              which translates into 64 ICMP data bytes when combined with the 8
                                              bytes of ICMP header data. [default: 56]
    -i, --rtt <wait-time>        Wait `wait_time` milliseconds between sending each packet. [default: 1000]

ARGS:
    <ip>     The IP address (IPv4, IPv6) to send packets towards
    <ttl>    Set Time to live (TTL) and report packets that have exceeded the TTL [default: 255]
```


## Features

- [x] Argument parsing
- [x] Support IPv4 and IPv6
- [x] Allow to set TTL as an argument
- [x] Support size of payload (in bytes) setting 
- [x] Send ICMP echo requests in an infinite loop to the target while receiving echo reply message (have a periodic delay)
- [x] Report RTT time for each sent message
- [x] Support TTL (IPv4, IPv6)
- [x] Report the ICMP messages that are exceeding the time set by the TTL argument
- [x] Implement `-s packet_size` argument (specify number of data bytes sent)
- [x] Report Loss for each sent message
- [x] Implement `-c count` argument(stop after sending (and receiving) count ECHO_RESPONSE packets.)
- [x] Implement `-i max_rtt` argument (duration between sending packets)
- [x] Implement ping summary (for --count argument)
- [ ] Implement ping summary for SIGINT signal

## Credits

Credits to [bparli](https://github.com/bparli) for his ICMP ping library.

## License

This project is released under the MIT license, see the LICENSE file for details.