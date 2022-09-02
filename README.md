# The lovely game tetris built in rust

Note this is meant to be run on wasm in your browser although it does work with cargo run however the screen size is very off

## To run this

First run the command
'''
rustup target add wasm32-unknown-unknown
'''

Choose a live-server runner of your choice and run the index.html file
It should connect to the built wasm package prebuilt when I upload this
If that does not work then run the following command
'''
cargo build --target wasm32-unknown-unknown
'''
