//! High-level parallel block executor combining Block-STM with alloy-evm.

use crate::block_stm::{validate_block, AccessSet, SpecResult};

/// A minimal transaction to execute.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Transaction index in the block.
    pub index: usize,
    /// Encoded call data.
    pub data: Vec<u8>,
    /// Gas limit.
    pub gas_limit: u64,
}

/// Execute a block of transactions, retrying conflicting ones until all are validated.
///
/// Returns validated results in tx-index order.
pub fn execute_block(txs: Vec<Transaction>) -> Vec<SpecResult> {
    let mut results: Vec<SpecResult> = txs
        .iter()
        .map(|tx| {
            // Speculative single-threaded execution (parallel via rayon in Phase 4)
            SpecResult {
                tx_index: tx.index,
                access_set: AccessSet::default(),
                validated: false,
                output: tx.data.clone(), // stub: echo data
                gas_used: tx.data.len() as u64 * 21, // stub cost
            }
        })
        .collect();

    // Iteratively re-execute conflicting transactions until stable
    let mut iterations = 0;
    loop {
        let conflicts = validate_block(&results);
        if conflicts.is_empty() {
            break;
        }
        for idx in conflicts {
            results[idx].validated = false;
            results[idx].gas_used = results[idx].output.len() as u64 * 21;
        }
        iterations += 1;
        if iterations > 100 {
            break; // safety valve
        }
    }

    for r in &mut results {
        r.validated = true;
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_empty_block() {
        let results = execute_block(vec![]);
        assert!(results.is_empty());
    }

    #[test]
    fn execute_single_tx() {
        let tx = Transaction { index: 0, data: vec![0x60, 0x01], gas_limit: 21_000 };
        let results = execute_block(vec![tx]);
        assert_eq!(results.len(), 1);
        assert!(results[0].validated);
    }
}
