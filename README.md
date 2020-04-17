# ping-util-rs

This program is a Rust implementation of the ping command found in UNIX.

## Roadmap

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
- [ ] Implement ping summary
- [ ] Implement any interesting arguments in the UNIX ping command

