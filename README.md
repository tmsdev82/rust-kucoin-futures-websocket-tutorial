# KuCoin Futures WebSocket API Example

This code repository represents an example of how one can connect to the KuCoin Futures WebSocket API to receive Symbol Ticker data. This is real-time price data for Cryptocurrency futures contracts.

On my blog there is a tutorial explaining the code: [Kucoin API with Rust how to get symbol ticker data](https://tms-dev-blog.com/kucoin-api-with-rust-how-to-get-symbol-ticker-data/)

## Running the program

Simply run `cargo run` to start the program. No configuration is necessary.

The program will retrieve connection information from the KuCoin REST API, then connect via WebSocket, subscribe to all available cryptocurrency futures ticker channels, and log the received data to stdout.
