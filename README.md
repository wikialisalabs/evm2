# evm2

> **Tầng 2 — Experimental EVM** · Block-STM parallel scheduler · ZK-IR output

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

---

## Tổng quan

`alloy-evm2` là lớp thực nghiệm phía trên `alloy-evm`, bổ sung:

| Tính năng | Mô tả |
|-----------|-------|
| **Block-STM** | Thực thi giao dịch song song theo thuật toán STM, phát hiện & giải quyết xung đột write-read |
| **ZK-IR** | Xuất IR (Intermediate Representation) tương thích zkEVM (Halo2 / Plonky3) |
| **Parallel Executor** | Orchestrator cấp cao kết hợp Block-STM + evm interpreter |

---

## Cài đặt

```toml
[dependencies]
alloy-evm2 = { git = "https://github.com/wikialisalabs/evm2", features = ["parallel", "zk-ir"] }
```

## Sử dụng nhanh

```rust
use alloy_evm2::parallel_executor::{execute_block, Transaction};

let txs = vec![
    Transaction { index: 0, data: vec![0x60, 0x01], gas_limit: 21_000 },
    Transaction { index: 1, data: vec![0x60, 0x02], gas_limit: 21_000 },
];
let results = execute_block(txs);
assert!(results.iter().all(|r| r.validated));
```

```rust
use alloy_evm2::zk_ir::ZkProgram;

let mut prog = ZkProgram::default();
let a = prog.assign([1u8; 32]);
let b = prog.assign([2u8; 32]);
let _c = prog.add(a, b);
println!("Wire count: {}", prog.wire_count());
```

---

## Feature Flags

| Flag | Mặc định | Mô tả |
|------|----------|-------|
| `std` | ✅ | Standard library |
| `parallel` | ❌ | Bật Block-STM scheduler |
| `zk-ir` | ❌ | Bật ZK-IR output backend |

---

## Kiểm thử

```bash
cargo test
cargo test --all-features
```

---

## Đóng góp

Xem [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Giấy phép

Apache-2.0 © 2026 Wikialisalabs Foundation
