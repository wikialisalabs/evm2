//! Block-STM: Software Transactional Memory scheduler for parallel tx execution.
//!
//! # Algorithm Overview
//!
//! ```text
//! Block of N transactions
//!      │
//!      ▼
//! ┌────────────────────────────────────┐
//! │  Speculation Phase                 │
//! │  Execute txs in parallel threads   │
//! │  Record read/write sets            │
//! └──────────────┬─────────────────────┘
//!                │
//!      ┌─────────▼──────────┐
//!      │  Validation Phase  │
//!      │  Detect conflicts  │
//!      └─────────┬──────────┘
//!                │ conflict?
//!           Yes  │        No
//!     ┌──────────┘         └────────────┐
//!     ▼                                 ▼
//!  Re-execute tx                   Commit result
//! ```

/// A storage key (contract address + slot).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StorageKey {
    /// Contract address (20 bytes).
    pub address: [u8; 20],
    /// Storage slot (32 bytes).
    pub slot: [u8; 32],
}

/// Read/write access set recorded during speculative execution.
#[derive(Debug, Default, Clone)]
pub struct AccessSet {
    /// Storage keys read during execution.
    pub reads: Vec<StorageKey>,
    /// Storage keys written during execution.
    pub writes: Vec<StorageKey>,
}

/// A speculative transaction execution result.
#[derive(Debug)]
pub struct SpecResult {
    /// Transaction index in the block.
    pub tx_index: usize,
    /// Recorded accesses.
    pub access_set: AccessSet,
    /// Whether this result has been validated.
    pub validated: bool,
    /// Output bytes.
    pub output: Vec<u8>,
    /// Gas used.
    pub gas_used: u64,
}

/// Detect a conflict between two access sets.
///
/// A conflict occurs when tx `j > i` reads a key that tx `i` writes.
pub fn has_conflict(earlier: &AccessSet, later: &AccessSet) -> bool {
    for write in &earlier.writes {
        if later.reads.contains(write) {
            return true;
        }
    }
    false
}

/// Validate all speculative results in a block, returning indices that must be re-executed.
pub fn validate_block(results: &[SpecResult]) -> Vec<usize> {
    let mut reexecute = Vec::new();
    for j in 1..results.len() {
        for i in 0..j {
            if has_conflict(&results[i].access_set, &results[j].access_set) {
                reexecute.push(j);
                break;
            }
        }
    }
    reexecute
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(addr: u8, slot: u8) -> StorageKey {
        let mut address = [0u8; 20];
        address[19] = addr;
        let mut s = [0u8; 32];
        s[31] = slot;
        StorageKey { address, slot: s }
    }

    #[test]
    fn no_conflict_disjoint_sets() {
        let a = AccessSet { reads: vec![], writes: vec![key(1, 0)] };
        let b = AccessSet { reads: vec![key(2, 0)], writes: vec![] };
        assert!(!has_conflict(&a, &b));
    }

    #[test]
    fn conflict_write_then_read() {
        let a = AccessSet { reads: vec![], writes: vec![key(1, 0)] };
        let b = AccessSet { reads: vec![key(1, 0)], writes: vec![] };
        assert!(has_conflict(&a, &b));
    }
}
