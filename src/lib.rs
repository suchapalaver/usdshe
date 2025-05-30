//! # usdshe
//!
//! `usdshe` is a utility crate for conveniently accessing USDC (USD Coin)
//! contract addresses on various blockchain networks.
//!
//! It provides a simple trait, [`Usdc`], which can be implemented for different
//! chain identifiers to retrieve the respective USDC contract address. Currently,
//! an implementation for [`alloy_chains::NamedChain`] is provided.
//!
//! ## Examples
//!
//! ```rust
//! use usdshe::{Usdc, UsdcError};
//! use alloy_chains::NamedChain;
//! use alloy_primitives::Address;
//! use std::str::FromStr;
//!
//! // Get Mainnet USDC address
//! match NamedChain::Mainnet.usdc_address() {
//!     Ok(address) => {
//!         assert_eq!(
//!             address,
//!             Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap()
//!         );
//!         println!("Mainnet USDC Address: {}", address);
//!     }
//!     Err(e) => eprintln!("Error fetching Mainnet USDC: {}", e),
//! }
//!
//! // Attempt to get USDC address for an unsupported chain
//! // Assuming Kovan is not in the supported list for this example.
//! // Replace with a NamedChain variant that is genuinely not in your match statement
//! // if Kovan (or another example) gets added.
//! // For instance, if `NamedChain::Gnosis` is unsupported:
//! let some_unsupported_chain = NamedChain::Gnosis; // Example
//! match some_unsupported_chain.usdc_address() {
//!     Ok(address) => panic!("Should not find address for {:?}", some_unsupported_chain),
//!     Err(UsdcError::UnsupportedChain(chain)) => {
//!         assert_eq!(chain, some_unsupported_chain);
//!         eprintln!("Correctly failed for unsupported chain: {:?}", chain);
//!     }
//!     Err(e) => panic!("Unexpected error: {}", e),
//! }
//! ```

mod address;

use address::*;
use alloy_chains::NamedChain;
use alloy_primitives::Address;
use std::str::FromStr;
use thiserror::Error;

/// Represents errors that can occur when retrieving a USDC address.
#[derive(Error, Debug)]
pub enum UsdcError {
    /// Indicates that a USDC address is not available or known for the specified chain.
    #[error("USDC address not available for chain: {0:?}")]
    UnsupportedChain(NamedChain),

    /// Indicates that a known address string failed to parse into a valid [`Address`].
    /// This should ideally not happen if the constants are well-formed.
    #[error("Failed to parse address string '{address_str}': {source}")]
    AddressParseError {
        /// The address string that failed to parse.
        address_str: String,
        /// The underlying parsing error.
        #[source]
        source: alloy_primitives::hex::FromHexError,
    },
}

/// A trait for types that can provide a USDC contract address.
pub trait Usdc {
    /// Returns the USDC contract address for the implementing context.
    ///
    /// # Errors
    ///
    /// Returns [`UsdcError::UnsupportedChain`] if the address is not known for the
    /// given context (e.g., an unsupported blockchain).
    /// Returns [`UsdcError::AddressParseError`] if a known address string is malformed
    /// and cannot be parsed.
    fn usdc_address(&self) -> Result<Address, UsdcError>;
}

/// Implementation of the [`Usdc`] trait for the [`alloy_chains::NamedChain`] enum.
///
/// This implementation provides USDC addresses for a predefined set of chains.
impl Usdc for NamedChain {
    /// Retrieves the USDC address for the given `NamedChain`.
    ///
    /// ## Supported Chains
    ///
    /// Refer to the crate's README for a list of currently supported chains.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use usdshe::{Usdc, UsdcError};
    /// use alloy_chains::NamedChain;
    /// use alloy_primitives::Address;
    /// use std::str::FromStr;
    ///
    /// // Get Polygon USDC address
    /// let polygon_address = NamedChain::Polygon.usdc_address().unwrap();
    /// assert_eq!(
    ///     polygon_address,
    ///     // Replace with actual Polygon USDC address string from your constants
    ///     Address::from_str("0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359").unwrap()
    /// );
    ///
    /// // Handle an unsupported chain
    /// // Assuming Gnosis is unsupported for this example.
    /// let result = NamedChain::Gnosis.usdc_address();
    /// match result {
    ///     Err(UsdcError::UnsupportedChain(chain)) => {
    ///         assert_eq!(chain, NamedChain::Gnosis);
    ///         // Handle the error appropriately
    ///     }
    ///     Ok(_) => panic!("Expected an error for Gnosis, but got an address."),
    ///     Err(e) => panic!("Unexpected error type: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// - [`UsdcError::UnsupportedChain`]: If the USDC address for the specified `NamedChain`
    ///   is not defined in this crate.
    /// - [`UsdcError::AddressParseError`]: If the predefined address string for a supported
    ///   chain is malformed (this should be a bug in the crate if it occurs).
    fn usdc_address(&self) -> Result<Address, UsdcError> {
        use NamedChain::*;

        // Note: The address strings (ARBITRUM_USDC, etc.) are expected to be
        // valid hexadecimal strings.
        let address_s = match self {
            Arbitrum => Ok(ARBITRUM_USDC),
            ArbitrumSepolia => Ok(ARBITRUM_SEPOLIA_USDC),
            Avalanche => Ok(AVALANCHE_USDC),
            Base => Ok(BASE_USDC),
            BaseSepolia => Ok(BASE_SEPOLIA_USDC),
            BinanceSmartChain => Ok(BSC_USDC),
            Fantom => Ok(FANTOM_USDC),
            Fraxtal => Ok(FRAXTAL_USDC),
            Sepolia => Ok(ETHEREUM_SEPOLIA_USDC),
            Linea => Ok(LINEA_USDC),
            Mainnet => Ok(ETHEREUM_USDC),
            Mantle => Ok(MANTLE_USDC),
            Mode => Ok(MODE_USDC),
            Optimism => Ok(OPTIMISM_USDC),
            Polygon => Ok(POLYGON_USDC),
            Scroll => Ok(SCROLL_USDC),
            Sonic => Ok(SONIC_USDC),
            ZkSync => Ok(ZKSYNC_USDC),
            unsupported_chain => Err(UsdcError::UnsupportedChain(*unsupported_chain)),
        }?;

        Address::from_str(address_s).map_err(|e| UsdcError::AddressParseError {
            address_str: address_s.to_string(),
            source: e,
        })
    }
}
