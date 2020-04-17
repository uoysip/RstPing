# ping-util-rs

This program is a Rust implementation of the ping command found in UNIX.

## Roadmap

- [x] Argument parsing
- [x] Add support for both IPv4 and IPv6
- [x] Allow to set TTL as an argument
- [x] Support size of payload (in bytes) setting 
- [x] Send ICMP echo requests in an infinite loop to the target while receiving echo reply message (have a periodic delay)
- [x] Report RTT time for each sent message
- [x] Support TTL (IPv4, IPv6)
- [x] Report the ICMP messages that are exceeding the time set by the TTL argument
- [x] Implement -s argument (specify number of data bytes sent)
= [ ] Report Loss for each sent message
- [ ] Implement any interesting arguments in the UNIX ping command
    - [ ] Implement -c count (stop after sending (and receiving) count ECHO_RESPONSE packets.)
