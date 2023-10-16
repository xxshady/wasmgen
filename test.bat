@REM set RUST_BACKTRACE=full
cd test-guest && cargo build && cd ../test-host && cargo run
