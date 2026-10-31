//! # alloy-evm2
//!
//! Experimental EVM with two key capabilities:
//!
//! - **Block-STM** parallel transaction scheduling (Software Transactional Memory)
//! - **ZK-IR** intermediate representation output for zero-knowledge proof generation
//!
//! ## Feature Flags
//!
//! | Flag | Default | Description |
//! |------|---------|-------------|
//! | `std` | ✅ | Standard library support |
//! | `parallel` | ❌ | Block-STM parallel scheduler |
//! | `zk-ir` | ❌ | ZK-IR output backend |

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod block_stm;
pub mod zk_ir;
pub mod parallel_executor;
