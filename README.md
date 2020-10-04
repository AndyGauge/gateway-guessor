# gateway-guessor
Displays the Broadcast address and Network address given a host's ip address and subnet mask

Attempts to predict the default gateway, given the gateway either one above the network address or one below the broadcast address.

# 2018 Edition
This project uses Rust 2018

```
cargo build
cargo install --path .
gateway 192.168.0.15/24
```
