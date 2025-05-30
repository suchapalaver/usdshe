mod arbitrum;
mod arbitrum_sepolia;
mod base;
mod bsc;
mod ethereum;
mod mode;
mod polygon;
mod scroll;
mod sonic;

pub use arbitrum::USDC as ARBITRUM_USDC;
pub use arbitrum_sepolia::USDC as ARBITRUM_SEPOLIA_USDC;
pub use base::USDC as BASE_USDC;
pub use bsc::USDC as BSC_USDC;
pub use ethereum::USDC as ETHEREUM_USDC;
pub use mode::USDC as MODE_USDC;
pub use polygon::USDC as POLYGON_USDC;
pub use scroll::USDC as SCROLL_USDC;
pub use sonic::USDC as SONIC_USDC;

/// <https://debank.com/token/avax/0xb97ef9ef8734c71904d8002f8b6bc66dd9c48a6e/overview>
pub const AVALANCHE_USDC: &str = "0xb97ef9ef8734c71904d8002f8b6bc66dd9c48a6e";

/// <https://sepolia.etherscan.io/address/0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238>
pub const ETHEREUM_SEPOLIA_USDC: &str = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";

/// <https://www.oklink.com/fantom/token/0x04068da6c83afcfa0e13ba15a6696662335d5b75>
pub const FANTOM_USDC: &str = "0x04068da6c83afcfa0e13ba15a6696662335d5b75";

/// <0xDcc0F2D8F90FDe85b10aC1c8Ab57dc0AE946A543>
pub const FRAXTAL_USDC: &str = "0xDcc0F2D8F90FDe85b10aC1c8Ab57dc0AE946A543";

pub const LINEA_USDC: &str = "0x176211869cA2b568f2A7D4EE941E073a821EE1ff";

/// <http://mantlescan.xyz/token/0x09bc4e0d864854c6afb6eb9a9cdf58ac190d0df9>
pub const MANTLE_USDC: &str = "0x09Bc4E0D864854c6aFB6eB9A9cdF58aC190D0dF9";

/// <https://base-sepolia.blockscout.com/address/0x036CbD53842c5426634e7929541eC2318f3dCF7e>
pub const BASE_SEPOLIA_USDC: &str = "0x036CbD53842c5426634e7929541eC2318f3dCF7e";

pub const OPTIMISM_USDC: &str = "0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85";

pub const ZKSYNC_USDC: &str = "0x1d17CBcF0D6D143135aE902365D2E5e2A16538D4";
