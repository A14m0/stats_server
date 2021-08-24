# Stats Server

This is a Rust-based web server which allows for fast push/get of config-defined
statistical data. 

## Getting started
Because the server utilizes TLS for its communications, you will need to 
generate your own certificate. Run the following to create one:

```
openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.pem -out cert.pem
```

This will generate your key and certificate files. Store these in the root of 
the crate (TODO: MAKE THE THING SMART ENOUGH TO FIND THE KEYS IN SAFER PLACES).

Then all you need to do is run the following to spool up the environment
```
cargo run --release
```