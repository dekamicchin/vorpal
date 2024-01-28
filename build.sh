rm /usr/bin/vorpal
cargo build --release
cp target/release/vorpal /usr/bin
