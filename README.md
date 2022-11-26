# Name

Marketing blurb

## The goal
1. read all .wasm modules from a given directory
   check interface, load them, start as actors
2. connect to mqtt server
3. send an init message to wasm actors/modules
4. broadcast all received mqtt messages to wasm actors/modules
5. if actors respond, send to mqtt

## Quick start

```
sudo systemctl start mosquitto.service
rustup target add wasm32-wasi
cargo run
```

## References
* https://github.com/lunatic-solutions/lunatic-rs
* https://github.com/SquattingSocrates/mqtt_packet
* https://github.com/SquattingSocrates/m-cutie-tea

## Maybe
* reload
* web dashboard
* filtering/routing (instead of full broadcast)
