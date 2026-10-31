# Hướng dẫn Đóng góp — `evm2`

**`alloy-evm2`** là crate thực nghiệm — đóng góp ở đây thường liên quan đến thuật toán phức tạp (Block-STM, ZK circuits). Hãy mở Issue thảo luận **trước** khi bắt đầu implement.

---

## Mục lục

1. [Môi trường Phát triển](#1-môi-trường-phát-triển)
2. [Cấu trúc Repository](#2-cấu-trúc-repository)
3. [Quy trình Đóng góp](#3-quy-trình-đóng-góp)
4. [Tiêu chuẩn cho Block-STM](#4-tiêu-chuẩn-cho-block-stm)
5. [Tiêu chuẩn cho ZK-IR](#5-tiêu-chuẩn-cho-zk-ir)
6. [Viết Tests](#6-viết-tests)
7. [Commit Convention](#7-commit-convention)
8. [Pull Request Checklist](#8-pull-request-checklist)

---

## 1. Môi trường Phát triển

```bash
git clone https://github.com/wikialisalabs/evm2.git
cd evm2
rustup show
cargo build --all-features
cargo test --all-features
```

---

## 2. Cấu trúc Repository

```
evm2/
├── src/
│   ├── lib.rs                # Crate root, feature gates
│   ├── block_stm.rs          # Block-STM scheduler, AccessSet, conflict detection
│   ├── zk_ir.rs              # ZK-IR instruction set, ZkProgram builder
│   └── parallel_executor.rs  # High-level block executor
├── Cargo.toml
├── README.md
└── CONTRIBUTING.md
```

---

## 3. Quy trình Đóng góp

> ⚠️ Với mọi thay đổi thuật toán Block-STM hoặc ZK-IR, **bắt buộc** mở Issue mô tả design trước. Maintainer sẽ review design trước khi bạn code.

1. Mở Issue → thảo luận design.
2. Nhận approval từ maintainer.
3. Fork → branch `feat/<tên>`.
4. Implement + tests.
5. CI local pass (xem checklist).
6. Mở PR với link tới Issue.

---

## 4. Tiêu chuẩn cho Block-STM

- **Correctness first:** Tất cả kết quả sau Block-STM phải bằng kết quả thực thi tuần tự.
- **No data races:** Mọi truy cập shared state phải qua `AccessSet` — không dùng global mutables.
- **Determinism:** Với cùng một block, kết quả phải luôn giống nhau bất kể số luồng.
- Hàm `has_conflict()` phải có complexity **O(R × W)** hoặc tốt hơn (dùng HashSet).

---

## 5. Tiêu chuẩn cho ZK-IR

- **Sound:** Mọi `ZkInstruction` phải có semantic rõ ràng và không mâu thuẫn.
- **Completeness:** Mọi opcode EVM phải có thể biểu diễn qua tổ hợp các `ZkInstruction`.
- Không add instruction mới mà không có proof-of-concept circuit sketch trong PR description.
- Wire indices phải monotonically increasing (đảm bảo không tái dùng nhầm).

---

## 6. Viết Tests

```bash
# Chạy unit tests
cargo test

# Với tất cả features
cargo test --all-features

# Chạy một module cụ thể
cargo test block_stm
cargo test zk_ir
```

### Yêu cầu tối thiểu

| Module | Test yêu cầu |
|--------|-------------|
| `block_stm` | No-conflict case, conflict detection, re-execution convergence |
| `zk_ir` | Build program, wire count, instruction sequence |
| `parallel_executor` | Empty block, single tx, multi-tx no conflict |

---

## 7. Commit Convention

```
feat(block-stm): implement HashSet-based conflict detection O(R+W)
feat(zk-ir): add Select (conditional) instruction
fix(executor): fix safety valve iteration count
test(block-stm): add multi-tx conflict scenario
```

---

## 8. Pull Request Checklist

```
[ ] cargo fmt --all                        → no diff
[ ] cargo clippy --all-features -- -D warnings → 0 warnings
[ ] cargo test --all-features              → all pass
[ ] Design discussed in Issue first        ✓
[ ] Block-STM: sequential == parallel result verified ✓
[ ] ZK-IR: new instruction has clear semantic ✓
[ ] PR linked to Issue                     ✓
```

---

## Liên hệ

- **Email:** root@wikialisalabs.axvn.vn
- **Issues:** https://github.com/wikialisalabs/evm2/issues

*Cảm ơn bạn đã đóng góp! 🦀⚡🔐*
