<h1 align="center">
  Rust Academic Project
</h1>

<p align="center">
  This project developed in Rust aims to manage basic information about students, such as name, identification, and status, implementing full CRUD (Create, Read, Update, Delete) functionalities. Additionally, the system integrates Smart Contracts and Pallets, utilizing the Ink! and Substrate frameworks from Polkadot, ensuring a decentralized and secure solution for data management.
</p>

<p align="center">
  <a href="#usage">Usage</a> •
  <a href="#license">License</a>
</p>

## Usage

```sh
git clone https://github.com/gustavopettine/rust-academic-project

cd rust-academic-project
```

### Rust CRUD

```sh
cargo run
```

### Smart Contract

```sh
docker compose run rust-academic-project bash

cd student

cargo +nightly contract build
```

### Pallets

```sh
docker compose run rust-academic-project bash

cd solochain-template

cargo build --release
```

## License

MIT
