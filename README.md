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

Install [`lunatic`-runtime](https://github.com/lunatic-solutions/lunatic) and [mosquitto](https://github.com/eclipse/mosquitto/),
```
sudo systemctl start mosquitto.service
rustup target add wasm32-wasi
cargo run
```


## Maybe
* automatic reloads
* web dashboard
* filtering/routing (instead of full broadcast)
