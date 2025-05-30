# usdshe

[![crates.io](https://img.shields.io/crates/v/usdshe.svg)](https://crates.io/crates/usdshe)
[![crates.io downloads](https://img.shields.io/crates/d/usdshe.svg)](https://crates.io/crates/usdshe)
[![docs.rs](https://img.shields.io/docsrs/usdshe)](https://docs.rs/usdshe)
[![License: MIT](https://img.shields.io/crates/l/usdshe.svg)](https://opensource.org/licenses/MIT)

A Rust crate for conveniently accessing USDC contract addresses on various blockchain networks.

`usdshe` provides a simple trait, `Usdc`, which can be implemented for different chain identifiers to retrieve the respective USDC contract address. Currently, it offers an implementation for `alloy_chains::NamedChain`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
usdshe = "0.1.0" # Or the latest version
alloy-chains = "0.2.2" # Or your desired version
alloy-primitives = "1.1.2" # Or your desired version
thiserror = "1.0" # For error handling if you interact with UsdcError
```

*(Note: Added `thiserror` to the example dependencies as users might want to match on `UsdcError` specifically, though it's not strictly required just to use the `usdc_address` method and handle its `Result`.)*

## Usage

```rust
use usdshe::{Usdc, UsdcError}; // Import UsdcError if you want to match specific errors
use alloy_chains::NamedChain;
use alloy_primitives::Address;

fn main() {
    // Get Mainnet USDC address
    match NamedChain::Mainnet.usdc_address() {
        Ok(address) => println!("Mainnet USDC Address: {}", address),
        Err(e) => eprintln!("Error fetching Mainnet USDC: {}", e),
    }

    // Get Polygon USDC address
    match NamedChain::Polygon.usdc_address() {
        Ok(address) => println!("Polygon USDC Address: {}", address),
        Err(e) => eprintln!("Error fetching Polygon USDC: {}", e),
    }

    // Example for a chain that is not supported
    // This will now return Err(UsdcError::UnsupportedChain(_))
    let some_other_chain = NamedChain::from_chain_id(12345); // An arbitrary unsupported chain ID
    match some_other_chain.usdc_address() {
        Ok(address) => println!("{:?} USDC Address: {}", some_other_chain, address),
        Err(UsdcError::UnsupportedChain(chain_name)) => {
            eprintln!(
                "USDC address lookup failed for {:?}: {}",
                chain_name,
                UsdcError::UnsupportedChain(chain_name) // Re-create error for display or use original 'e'
            );
        }
        Err(e) => eprintln!("An unexpected error occurred for {:?}: {}", some_other_chain, e),
    }
}
```

## Supported Chains

The `Usdc` trait is implemented for the following `NamedChain` variants:

* Arbitrum
* ArbitrumSepolia
* Avalanche
* Base
* BaseSepolia
* BinanceSmartChain (BSC)
* Ethereum (Mainnet)
* EthereumSepolia (Sepolia)
* Fantom
* Fraxtal
* Linea
* Mantle
* Mode
* Optimism
* Polygon
* Scroll
* Sonic
* ZkSync

If you attempt to get the USDC address for an unsupported chain using the `NamedChain` implementation, the `usdc_address` method will return an `Err(UsdcError::UnsupportedChain(chain_name))`. If a known address string fails to parse (which should be rare for correctly defined constants), it will return `Err(UsdcError::AddressParseError { .. })`.

## Contributing

Contributions are welcome! If you'd like to add support for a new chain or improve existing functionality, please feel free to open an issue or submit a pull request.

When adding a new chain, please ensure you:

1. Add the USDC contract address constant in the appropriate `src/address/` module (either a new file or `src/address/mod.rs`). Ensure it's a valid address string.
2. Export it with a clear name (e.g., `CHAIN_NAME_USDC`).
3. Add a match arm for the `NamedChain` variant in `src/lib.rs` to return `Ok(YOUR_CHAIN_USDC_CONSTANT)`.
4. Update this README with the newly supported chain.

## License

This crate is licensed under the MIT License.
