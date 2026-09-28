# Memory Safety in Rust

A course study of how Rust prevents the classes of memory-corruption vulnerabilities seen in C/C++, what it costs, and where its guarantees stop.

Coursework for *Tecniche di Protezione del Software* (Software Protection Techniques), MSc in Computer Science / Cybersecurity, University of Milan, academic year 2025/2026.

**Scope, stated plainly:** this is a literature review plus a small experiment (five vulnerability classes, one Juliet test case each) and a single microbenchmark. It is a student project, not a research paper, and it says nothing about post-quantum cryptography or AI security.

## Repository layout

```
.
├── docs/
│   ├── paper_it.pdf            # Full write-up (Italian)
│   └── slides_it.pdf           # 20-minute presentation (Italian)
├── Rust/                       # Standalone Rust programs (see below)
├── testcases/                  # The five Juliet C test cases used
├── testcasesupport/            # Juliet support files needed to build the C cases
├── test_con_miri
└── README.md
```

The documents are in Italian; this README summarises them in English.

<!-- TODO: add one line describing test_con_miri -->

### `Rust/`

Each file is a standalone program (no Cargo project).

| Files | Purpose |
|---|---|
| `CWE121.rs`, `CWE122.rs`, `CWE415.rs`, `CWE416.rs`, `CWE562.rs` | Safe-Rust ports of the five Juliet cases (see the experiment below) |
| `Non_copy_types.rs`, `lifetimes.rs` | Ownership, moves and lifetimes examples |
| `double_free.rs`, `Use_After_Free.rs`, `Another_Use_After_Free.rs` | Temporal-safety examples |
| `write_beyond_array_limits*.rs` (five variants) | Out-of-bounds write examples |
| `unsafe_test.rs` | `unsafe` examples |
| `costo_bounds_checker.rs` | Bounds-checking microbenchmark |

<!-- TODO: check that each description matches what the file actually does -->

### Running the examples

Compile a file with `rustc`, for example `rustc Rust/CWE121.rs`, then run the resulting binary.

**Some files are meant to fail.** `CWE415.rs`, `CWE416.rs` and `CWE562.rs` are expected to be rejected by the compiler: that rejection is the result being demonstrated. `CWE121.rs` and `CWE122.rs` compile and then panic at run time.

The C originals in `testcases/` need the files in `testcasesupport/` to build.

The C test cases and support files come from the NIST Juliet Test Suite for C/C++ v1.3; see NIST's SARD pages for their terms.

## Question

Memory-safety bugs (buffer overflow, use-after-free, double free, dangling pointers) remain a leading source of serious vulnerabilities. Garbage-collected languages avoid many of them at a runtime cost. Rust aims to give memory safety without a garbage collector by moving checks to compile time. The study asks:

1. Which of these vulnerability classes can be expressed at all in safe Rust?
2. For those that can, does Rust stop them at compile time or at run time?
3. What does the mechanism cost, and what does it not cover?

## Background covered

- **Temporal safety:** ownership and moves, `Copy` vs non-`Copy` types, borrowing, the three borrow-checker rules (references must not outlive their referents, no dangerous aliasing, moves must not leave invalid accesses), `Rc`/`Arc`, and interior mutability (`Cell`, `RefCell`, `Mutex`).
- **Spatial safety:** no pointer arithmetic or raw-pointer dereferencing in safe code, slices and wide pointers, static and dynamic bounds checking.
- **Limits:** `unsafe` blocks and FFI, reference cycles with `Rc`/`RefCell` (a leak, not a memory-safety violation), and compiler mitigation options (some, such as stack protector and CFI, were nightly-only and off by default at the time of writing).

## Experiment: five CWEs from the Juliet Test Suite

For each class, one C test case from the NIST Juliet Test Suite was analysed and then translated conceptually to safe Rust, keeping the logic as close as possible. This is one file per CWE, **not** a run of the full suite.

| CWE | Class | C test case (`testcases/`) | Rust port | Compiles? | Panics at run time? | Outcome in safe Rust |
|---|---|---|---|---|---|---|
| 415 | Double free | `CWE415_Double_Free__malloc_free_char_01.c` | `CWE415.rs` | No | – | Second `drop(p)` rejected: `p` was moved |
| 416 | Use after free | `CWE416_Use_After_Free__malloc_free_char_01.c` | `CWE416.rs` | No | – | Use after `drop` rejected by the borrow checker |
| 562 | Return of stack variable address | `CWE562_Return_of_Stack_Variable_Address__return_buf_01.c` | `CWE562.rs` | No | – | Returning a reference to a local is rejected |
| 121 | Stack-based buffer overflow | `CWE121_Stack_Based_Buffer_Overflow__char_type_overrun_memcpy_01.c` | `CWE121.rs` | Yes | Yes | `copy_from_slice` fails the length check and panics |
| 122 | Heap-based buffer overflow | `CWE122_Heap_Based_Buffer_Overflow__char_type_overrun_memcpy_01.c` | `CWE122.rs` | Yes | Yes | `copy_from_slice` fails the length check and panics |

**Observed pattern:** in this sample, temporal-safety violations were rejected at compile time, while spatial-safety violations compiled when the error was not statically determinable and were then caught by run-time bounds checks, which end in a controlled panic instead of silent corruption.

**Limitation:** five cases are a representative but small sample. The results say nothing about logic errors, `unsafe` code, or FFI.

## Microbenchmark: cost of bounds checking

Source: `Rust/costo_bounds_checker.rs`. Sum of an array of 100 million `u64` values, three ways:

- **standard:** indexed access `data[i]` (with bounds check)
- **unsafe:** unchecked access in an `unsafe` block
- **iter:** idiomatic iterator

Observed on the author's machine (mean ± standard deviation):

| Variant | Time (ms) |
|---|---|
| standard | 39.64 ± 0.96 |
| unsafe | 42.84 ± 2.47 |
| iter | 42.88 ± 3.58 |

The three variants are indistinguishable given the spread. The write-up interprets this as the compiler removing redundant bounds checks.

**Caveats:** this is a single microbenchmark on one machine. A memory-bound sum over a large array can hide the per-element cost of a check, and the machine code was not inspected in this study, so the explanation is a hypothesis, not a demonstrated result.

<!-- TODO: add build flags, CPU, OS, rustc version and number of runs so the benchmark is reproducible -->

## What the write-up also reviews

- Performance literature: *Is Rust C++-fast?* (2022), NAS Parallel Benchmarks in Rust (2025), and a user study on C→Rust translation (NDSS 2025).
- Industry adoption, as reported by the cited sources: Android's shift to memory-safe languages (Google reported the memory-safety share of vulnerabilities falling from 76% in 2019 to 24% in 2024), Microsoft's Rust rewrite of parts of SymCrypt, and Rust in the Linux kernel.

## Conclusions

- Rust turns the temporal-safety bugs studied here into compile-time errors and the spatial-safety bugs into controlled run-time failures.
- The guarantees are not absolute: `unsafe` code, FFI, `Rc` cycles and logic errors sit outside them, and the trusted computing base of the language is its `unsafe` code.
- Performance evidence from the literature is broadly favourable, but no single benchmark, including the one here, supports strong general claims.

## Main references

1. Klabnik, Nichols et al., *The Rust Programming Language*.
2. *The Rustonomicon*.
3. Ivanov, *Is Rust C++-fast? Benchmarking System Languages on Everyday Routines* (2022).
4. Martins et al., *NPB-Rust: NAS Parallel Benchmarks in Rust* (2025).
5. Li et al., *Translating C To Rust: Lessons from a User Study* (NDSS 2025).
6. Google Security Blog, *Eliminating Memory Safety Vulnerabilities at the Source* (2024).
7. NIST, *Juliet Test Suite*.

The full bibliography is in the paper.

## Author

Marco Agnoli, [LinkedIn](https://www.linkedin.com/in/marcoagnoli/)
