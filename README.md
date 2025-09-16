# prime_sdk_rs

A Rust SDK for interacting with the Coinbase Prime REST APIs, built on top of the `core_rs` library.

## Overview

The `prime_sdk_rs` crate provides a high-level, type-safe, and async interface for Coinbase Prime APIs. It includes generated types and services for all major API endpoints, as well as example programs to help you get started.

## Features

- Async client for Coinbase Prime REST APIs
- Strongly-typed request and response models
- Credential and authentication management
- Example programs for common API operations

## Installation

### From crates.io (Recommended)

Add the library to your project using Cargo:

```sh
cargo add prime_sdk_rs
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
prime_sdk_rs = "0.1.0"
```

## Setup

### 1. Clone the Repository

```sh
git clone git@github.com:coinbase-samples/prime_sdk_rs.git
cd prime_sdk_rs
```

### 2. Environment Variables

Create a `.env` file in the project root to store your API credentials and configuration. Example:

```env
PRIME_CREDENTIALS='{"AccessKey": "", "SecretKey": "", "Passphrase": ""}'
PORTFOLIO_ID='d7a7abc5-xxxx-xxxx-xxxx-9252a740a3c8'
ENTITY_ID='ba65079e-xxxx-xxxx-xxxx-89041fa232b1'
```

> **Note:** The actual variable names and usage may depend on your integration. Refer to your application or integration code for the required variables.

### 3. Build the SDK

To build the SDK, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed (Rust 1.61+ recommended):

```sh
cargo build
```

### 4. Run Examples

A variety of example programs are provided in the `examples/` directory. All examples can be run using the provided shell script:

```sh
./run_example.sh <example_path>
```

For example, to run the portfolio listing example:

```sh
./run_example.sh portfolios/list_portfolios
```

## License

This project is licensed under the [Apache-2.0 license](LICENSE). 