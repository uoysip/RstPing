# RstPing

RstPing is a compact, fast CLI ping utility meant to offer similar functionality to the UNIX `ping` command.

![RstPing CLI](https://i.imgur.com/qNBiKbA.png)

## Installation

A release-optimized executable is available on the [releases page](https://github.com/uoysip/ping-util-rs/releases), as well as source code for the latest version. If you would like to build the latest version yourself, follow the commands below:

```bash
git clone https://github.com/uoysip/RstPing.git
cd ./RstPing/
sudo cargo run -- --help
```

## Usage
`sudo` is required due to the nature of the program (unless special permissions for the executable is set).

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

```
>sudo ./rst_ping 1.1.1.1 12 -c 10 -s 26 -i 324                                                                                  

PING 1.1.1.1 (1.1.1.1): 26 data bytes
34 bytes from 1.1.1.1: icmp_seq=1 ttl=12 rtt=22.60142ms loss=0%
34 bytes from 1.1.1.1: icmp_seq=2 ttl=12 rtt=22.44628ms loss=0%
34 bytes from 1.1.1.1: icmp_seq=3 ttl=12 rtt=19.09291ms loss=0%
34 bytes from 1.1.1.1: icmp_seq=4 ttl=12 rtt=20.74554ms loss=0%
34 bytes from 1.1.1.1: icmp_seq=5 ttl=12 rtt=17.20550ms loss=0%
34 bytes from 1.1.1.1: icmp_seq=6 ttl=12 rtt=19.82068ms loss=0%
34 bytes from 1.1.1.1: icmp_seq=7 ttl=12 rtt=25.86904ms loss=0%
34 bytes from 1.1.1.1: icmp_seq=8 ttl=12 rtt=20.63334ms loss=0%
34 bytes from 1.1.1.1: icmp_seq=9 ttl=12 rtt=23.52475ms loss=0%
34 bytes from 1.1.1.1: icmp_seq=10 ttl=12 rtt=14.39621ms loss=0%

--- 1.1.1.1 ping statistics ---
10 packets transmitted, 10 packets received, 0% packet loss
round-trip min/avg/max/stddev = 14.396/20.634/25.869/3.109 ms
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
