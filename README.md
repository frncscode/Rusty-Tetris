# The lovely game tetris built in rust

Note this is meant to be run on wasm in your browser although it does work with cargo run however the screen size is very off

## To run this

First run the command

```
rustup target add wasm32-unknown-unknown
```

This installs the wasm packages needed to run this thing
After that runs successfully run the following command

```
cargo build --target wasm32-unknown-unknown
```

This should build the package

Finally chose a live server runner of your choise and run the index.html file
