# Crypto-cli

Crypto-cli is a command-line tool that interacts with a Coinmarketcap's API to fetch and display cryptocurrency data. It
provides users with a convenient way to access real-time information about various cryptocurrencies.

### Technologies Used

Crypto-cli is developed using the following technologies:

1. [Rust](https://www.rust-lang.org/): The programming language used for the project development, providing a safe and
   efficient environment for building command-line applications.
2. [reqwest crate](https://crates.io/crates/reqwest): A Rust HTTP client library used for making requests to the
   cryptocurrency API and handling responses.
3. [serde crate](https://crates.io/crates/serde) and [serde_json crate](https://crates.io/crates/serde_json): Rust
   crates used for serialization and deserialization of JSON data, enabling parsing and handling of API responses.
4. [tokio crate](https://crates.io/crates/tokio): A runtime for asynchronous programming in Rust, used for running the
   async functions in Crypto-cli.

## Installation and Setup

To install and set up Crypto-cli, follow these steps:

1. Clone the repository:

   ```shell
   git clone https://github.com/Goodnessuc/crypto-cli.git
   ```

2. Change to the project directory:

   ```shell
   cd crypto-cli
   ```

3. Install the dependencies:

   ```shell
   cargo build
   ```

## Usage

To use Crypto-cli, execute the command below to retrieve cryptocurrency data:

   ```shell
   cargo run crypto
   ```

The command will fetch data from the API and display the results in the terminal.

That's it! You can now use Crypto-cli to fetch cryptocurrency data from the API.

Please note that the usage instructions assume you have Rust and Cargo installed on your system. If you haven't
installed them, please visit the official Rust website (https://www.rust-lang.org/) for instructions on how to install
Rust and Cargo.

## Contributing

Contributions to the Crypto-cli project are welcome and encouraged. To contribute, follow these steps:

1. Fork the project repository.
2. Create a new branch.
3. Make your contributions.
4. Push your branch to your forked repository.
5. Submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).










