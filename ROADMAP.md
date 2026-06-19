# db-labs — DBMS Roadmap

> **Implement in Rust. Every lecture ends with code you can run.**

This roadmap follows the CMU database curriculum lecture-by-lecture, from a single-node relational DBMS all the way to distributed, columnar, vector, and graph databases. Every lecture section includes key concepts to master, concrete coding exercises (all in Rust), and the exact papers to read. The roadmap is sequenced so each phase's knowledge is a strict prerequisite for the next: you cannot reason about distributed consistency without first understanding local transaction isolation, and you cannot optimize a column store without first implementing a row store.

---

## ⏱️ Self-Paced Timeline — Phase 1 (anchored to 2026-06-16)

> CMU 15-445 Spring 2026 already ended (last lecture Apr 22), so these are **self-paced targets**, not live Gradescope deadlines. **P0 (primer / Count-Min Sketch) is ✅ complete** — remaining work is Projects 1–4. Targets below are calibrated for a **Rust port** (no C++ scaffolding, port the tests yourself, borrow-checker tax) — not CMU's full-time C++ cadence.

**Primary target — Rust-port pace** (~1.5× CMU per project, extra slack on B+Tree, ~10–15 hrs/wk):

| Project | Window | Length | Notes |
|---|---|---|---|
| ✅ **P0** Primer (Count-Min Sketch) | done | — | |
| **P1** Buffer Pool (ARC · Disk Scheduler · BPM · Page Guards) | Jun 16 → **Jul 11** | ~3.5 wk | running start — concurrency stack already learned |
| **P2** Database Index (B+Tree) — *the beast* | Jul 13 → **Aug 28** | ~6.5 wk | most slack; tree + latch-crabbing fights the borrow checker |
| **P3** Query Execution | Aug 31 → **Sep 25** | ~3.5 wk | traits map cleanly to operators |
| **P4** Concurrency Control | Sep 28 → **Oct 30** | ~4.5 wk | lock manager / MVCC |
| **Buffer / slack** | Nov | ~2 wk | absorb slippage + lectures/HWs |

➡️ **Target finish end-Oct, realistic mid-Nov 2026** (~5 months, sustainable).

**Stretch goal — CMU full-time cadence** (~3 wk/project, back-to-back): P1 Jul 6 · P2 Jul 31 · P3 Aug 21 · P4 Sep 12 → finish ~Sep 12. Only realistic at full-time hours with no port friction.

**Pacing rules (matter more than the dates):**
1. **Re-anchor after each project** — reset remaining rows from your real finish date; don't chase stale absolute dates.
2. **Track by milestone, not calendar** — "P1 all tests green" is the signal, not "it's Jul 11."
3. **Protect P2's slack** — it's the one project likely to overrun; the Nov buffer exists mostly for it.
4. **Weave lectures/homeworks into the gaps** — watch each lecture before its project; do SQL/written HWs in the seams.
5. **Don't pack to 100%** — the 2-week buffer is the difference between "finished" and "abandoned."

---

## Five Phases

| Phase | Focus |
|-------|-------|
| **Phase 1** | CMU 15-445: Single-node relational DBMS. Storage, indexes, query execution, transactions, recovery. Implement BusTub components in Rust. |
| **Phase 2** | CMU 15-721: Advanced OLAP. Columnar storage, vectorized execution, query compilation, SIMD, NUMA-aware scheduling, hash joins. Paper-reading-heavy. |
| **Phase 3** | Storage engines deep dive. B-Tree vs LSM-Tree, row-oriented vs columnar, WAL designs, compaction strategies. Build a KV store and a column store. |
| **Phase 4** | Distributed systems. Consensus (Raft, Paxos), distributed transactions (2PC, Percolator), sharding, replication, geo-distribution. |
| **Phase 5** | Specialized databases. Vector databases (HNSW, IVF, FAISS), graph databases (property graphs, Cypher, GNN storage), time-series, stream processing. |

---

# Phase 1 — CMU 15-445: Intro to Database Systems

**Single-node relational DBMS internals · 26 lectures · 5 projects · Implement BusTub in Rust**

**Goal:** Build every major component of a disk-oriented relational DBMS: storage manager, buffer pool, B+ tree index, query executors, and concurrency control. BusTub is the course's reference DBMS — implement all five projects in Rust.

---

## 📅 Synced to Spring 2026 (authoritative)

> Source: <https://15445.courses.cs.cmu.edu/spring2026> (schedule + assignments, fetched 2026-06-16). The detailed per-lecture sections further down are study material; **this table is the source of truth for ordering, projects, and dates.** The detailed sections use older numbering — map them via the module names.

### Lecture schedule (26 lectures)

| # | Date | Lecture | Module |
|---|------|---------|--------|
| 01 | Jan 12 | Relational Model & Algebra | A · Relational Model & SQL |
| 02 | Jan 14 | Modern SQL | A |
| 03 | Jan 21 | Database Storage I | B · Storage |
| 04 | Jan 26 | Memory Management (Buffer Pool) | B |
| 05 | Jan 28 | Database Storage II | B |
| 06 | Feb 02 | Storage Models & Compression | B |
| 07 | Feb 04 | Hash Tables | C · Indexes & Filters |
| 08 | Feb 09 | Indexes & Filters I | C |
| 09 | Feb 11 | Indexes & Filters II | C |
| 10 | Feb 16 | Index Concurrency Control | C |
| 11 | Feb 18 | Sorting & Aggregations Algorithms | D · Query Execution |
| 12 | Feb 23 | Joins Algorithms | D |
| 13 | Mar 09 | Query Execution I | D |
| 14 | Mar 11 | Query Execution II | D |
| 15 | Mar 16 | Query Planning & Optimization I | D |
| 16 | Mar 18 | Query Planning & Optimization II | D |
| 17 | Mar 23 | Concurrency Control Theory | E · Transactions |
| 18 | Mar 25 | Two-Phase Locking | E |
| 19 | Mar 30 | Timestamp Ordering | E |
| 20 | Apr 01 | Multi-Version Concurrency Control I | E |
| 21 | Apr 06 | Multi-Version Concurrency Control II | E |
| 22 | Apr 08 | Database Logging | F · Recovery |
| 23 | Apr 13 | Database Recovery | F |
| 24 | Apr 15 | Distributed Databases I | G · Distributed |
| 25 | Apr 20 | Distributed Databases II | G |
| 26 | Apr 22 | Final Review + Systems Potpourri | — |

### Projects (5) — implement in Rust

| Project | Released | Topic | Components to implement |
|---------|----------|-------|-------------------------|
| **P0** | Jan 12 | C++ Primer | *(our track: Rust primer — **Count-Min Sketch ✅ done**)* |
| **P1** | Jan 26 | **Buffer Pool Manager** *(due Feb 15)* | ARC Replacer · Disk Scheduler (bg worker + promise/future) · Buffer Pool Manager · Page Guards (RAII). **Pages are 8 KB.** Must be thread-safe. |
| **P2** | Feb 16 | Database Index | B+Tree index (structure, ops, latch-crabbing concurrency) |
| **P3** | Mar 09 | Query Execution | Executor operators (scan/filter/join/aggregate), query plan |
| **P4** | Mar 30 | Concurrency Control | Transactions, locking / MVCC |

### Homeworks (6, written/SQL)

HW1 SQL (Jan 14) · HW2 Storage (Jan 28) · HW3 Indexes & Filters (Feb 11) · HW4 Execution & Planning (Mar 11) · HW5 Transactions (Mar 25) · HW6 Recovery (Apr 13)

### Key dates

Jan 19 MLK (no class) · **Feb 25 Mid-Term Exam** · Mar 02 & 04 Spring Break

> **Notable changes vs the older roadmap below:** replacement policy is now **ARC** (LRU-K/LRU/Clock are optional stubs); BusTub pages are **8 KB** (not 4 KB); buffer pool is its own lecture (**#04 Memory Management**) split from storage; indexing is reframed as **"Indexes & Filters"** (adds Bloom/cuckoo filters) with a dedicated **Index Concurrency Control** lecture; MVCC and Logging/Recovery each span two lectures; **Distributed Databases** (#24–25) is now in 15-445.

---

## Module A — Relational Model & SQL

### L01 — Relational Model & Relational Algebra

**What you learn:** The theoretical foundation of every relational DBMS: relations, tuples, attributes, keys, and the six relational algebra operators that all SQL compiles to.

#### Key Concepts

- Relation = unordered set of tuples with a fixed schema. Table ≠ relation (tables have row order, relations do not).
- Relational algebra: Select (σ), Project (π), Union (∪), Difference (−), Product (×), Rename (ρ). Every SQL query compiles to a tree of these operators.
- Keys: superkey, candidate key, primary key, foreign key. Referential integrity constraints.
- Functional dependencies: X → Y means knowing X uniquely determines Y. Foundation for normal forms.
- Normal forms: 1NF (atomic attributes), 2NF (no partial dependency), 3NF (no transitive dependency), BCNF (every determinant is a candidate key).

#### Coding Exercises

1. Implement a Rust struct `Relation` with a `Vec` and a `Schema`. Write select, project, join, and union as methods. Verify they are correct on a 3-table university schema.
2. Write a SQL-to-relational-algebra translator for simple queries (SELECT/WHERE/JOIN/GROUP BY). Output the algebra tree as a formatted string.
3. Implement BCNF decomposition: given a relation and a set of FDs, decompose it into BCNF relations. Test on the canonical examples from the textbook.

#### Papers & Resources

| Type | Resource |
|------|----------|
| BOOK | *Database System Concepts* — Silberschatz, Korth, Sudarshan ch 1–6. The standard textbook. Chapters 1–6 cover the relational model, SQL, and normal forms. |
| VIDEO | [CMU 15-445 Lecture 01](https://youtube.com/@CMUDatabaseGroup) — Andy Pavlo. All 15-445 lectures are free. Watch before reading the textbook chapter. |
| PAPER | *A Relational Model of Data for Large Shared Data Banks* — Codd 1970. The original paper that defined the relational model. ACM DL. Read the first 10 pages. |

---

### L02 — Modern SQL

**What you learn:** SQL beyond basic SELECT/WHERE: window functions, CTEs, lateral joins, GROUPING SETS, ROLLUP, CUBE, and string/date functions that appear in every real workload.

#### Key Concepts

- Window functions: `ROW_NUMBER()`, `RANK()`, `DENSE_RANK()`, `LAG()`, `LEAD()`, `NTILE()`, `FIRST_VALUE()` over `PARTITION BY` / `ORDER BY` / frame clauses.
- Common Table Expressions (WITH): recursive CTEs for hierarchical queries (org charts, bill of materials). Know when a recursive CTE is equivalent to a fixed-point computation.
- GROUPING SETS, ROLLUP, CUBE: generate multiple GROUP BY results in a single pass. Used heavily in OLAP workloads.
- Lateral joins: correlate a subquery with the outer query's columns. Equivalent to a for-loop over the outer relation.
- NULL semantics: SQL uses three-valued logic (TRUE/FALSE/UNKNOWN). NULL in WHERE produces UNKNOWN, which filters the row. Know every NULL propagation rule.

#### Coding Exercises

1. Solve all 50 problems on [SQLZoo](https://sqlzoo.net). Time yourself — you should be able to write any of them in under 3 minutes.
2. Implement a query executor in Rust for: SELECT, WHERE, GROUP BY with aggregates, ORDER BY, LIMIT, and LEFT/INNER JOIN over in-memory `Vec`. No SQL parser — just a typed query builder API.
3. Write a recursive CTE that computes the transitive closure of a directed graph stored in an `edges(src, dst)` table. Verify on a graph with a cycle.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 02 — Modern SQL](https://youtube.com/@CMUDatabaseGroup) — Pavlo's SQL lecture covers every feature listed above with live examples. |
| SITE | [SQLZoo](https://sqlzoo.net) — complete all sections including window functions and recursive CTEs before moving on. |
| BLOG | [Use The Index, Luke](https://use-the-index-luke.com) — Markus Winand. How SQL queries translate to index accesses. Read the 'Sorting and Grouping' chapter now. |

---

## Module B — Storage

### L03 — Database Storage I — Disk-Oriented Architecture

**What you learn:** How a DBMS manages files on disk: the storage hierarchy, page layout, heap files, and why the DBMS must manage its own I/O rather than relying on the OS.

#### Key Concepts

- Storage hierarchy: NVM/DRAM (ns latency) → NVMe SSD (10µs) → HDD (10ms). The DBMS's job is to minimize I/O by keeping hot pages in memory.
- Pages: fixed-size blocks (typically 4KB–16KB). Every read/write is at page granularity. The page ID is the unit of addressing throughout the DBMS.
- Heap file: an unordered collection of pages. Maintained with a free-space map or a header page directory. Simple but requires full scan for most queries.
- Page layout — slotted pages: a header with slot array pointing to variable-length tuples packed from the page tail. Deletions mark slots as free without moving data.
- Tuple layout: fixed-length attributes stored contiguously. Variable-length stored with offset+length header. NULLs encoded in a null bitmap in the tuple header.
- Why not use mmap? The OS cannot make DBMS-aware decisions about which pages to evict, cannot handle page-level latches, and mmap's page fault handling is unacceptable for write-ahead logging.

#### Coding Exercises

1. Implement a `DiskManager` in Rust: read/write fixed **8 KB** pages (BusTub's page size) to one file by `page_id`, using `std::os::unix::fs::FileExt` (`read_exact_at`/`write_all_at`). This is the storage backend the Project 1 buffer pool sits on.
2. Implement a `SlottedPage` in Rust: insert variable-length tuples, delete by slot ID (mark free), compact (garbage collect) the page. Write a test with 100 inserts, 50 deletes, verify space accounting. *(Used later by the table heap in Project 3, not Project 1.)*
3. Implement a `HeapFile` in Rust: a sequence of `SlottedPage`s, with a header page maintaining a free-space directory. Support insert (find a page with space), delete by `(page_id, slot_id)`, and full scan.

> **Implementation note:** The `DiskManager` is the persistent backend for **Project 1** (the buffer pool drives it via the disk scheduler). Slotted-page / tuple layout (exercises 2–3) is foundational but is exercised later by the table heap, not Project 1.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 03 — Database Storage I](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *The Design and Implementation of Modern Column-Oriented Database Systems* — Abadi et al. 2013. Sections 1–2 on row-store architecture to contrast with column stores later. VLDB. |
| BLOG | ['Why mmap is bad for databases'](https://db.cs.cmu.edu) — Andy Pavlo. Pavlo's blog post on why mmap-based DBs are fundamentally broken for correctness. |

---

### L04 — Database Storage II — Log-Structured Storage & Encoding

**What you learn:** LSM-trees, SSTables, WAL-only designs (LSMT-based stores like LevelDB/RocksDB), and how data encoding affects compression and scan performance.

#### Key Concepts

- LSM-Tree (Log-Structured Merge Tree): all writes go to an in-memory MemTable. When full, flush to an immutable SSTable on disk. Background compaction merges SSTables.
- SSTable format: sorted key-value pairs with a block index at the end. Block-level compression (Snappy, LZ4, Zstd). Bloom filters per SSTable to skip files on point lookups.
- Compaction strategies: size-tiered (group SSTables by size, merge when a level has too many), leveled (each level is a sorted run, merge down when a level exceeds its size budget), FIFO (time-based, for time-series).
- Write amplification (B-tree ≈ 10x, LSM leveled ≈ 30x), read amplification (B-tree ≈ 3, LSM ≈ level count), space amplification (B-tree ≈ 1.3x, LSM ≈ 1.1x). Know the tradeoff triangle.
- Encoding: fixed-width integers (no overhead), variable-length integers (protobuf varint), dictionary encoding (map repeated strings to small integers), RLE (run-length encoding for sorted columns), bit-packing.

#### Coding Exercises

1. Implement a MemTable in Rust as a `BTreeMap<Vec<u8>, Vec<u8>>` with a configurable size limit. When full, sort and flush to an SSTable file.
2. Implement SSTable: sorted blocks of key-value pairs, with a block index (`Vec<(key, file_offset)>`) appended at the end. Implement point lookup and range scan.
3. Implement leveled compaction: maintain L0 through L3 as `Vec<SSTable>`. When L0 has 4 SSTables, merge them into L1. When L1 exceeds 10MB, push the overlap down to L2. Implement the merge-sort-based compaction.
4. Implement bloom filters (crate: `bloomfilter`) per SSTable. Measure false positive rate and the reduction in disk reads for point lookups on a dataset with 10% key presence.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *The Log-Structured Merge-Tree* — O'Neil et al. 1996. The original LSM-Tree paper. Short and foundational. ACM DL. |
| PAPER | *WiscKey: Separating Keys from Values in SSD-Conscious Storage* — Lu et al. 2016. USENIX FAST. Key-value separation optimization for SSDs. Read after understanding basic LSM. |
| IMPL | [MiniLSM tutorial](https://skyzh.github.io/mini-lsm) — skyzh. A guided Rust implementation of an LSM-Tree storage engine. The best open-source Rust DBMS tutorial available. Follow all 7 weeks. |

---

### L05 — Buffer Pool Management *(Spring 2026 Lecture #04 — Memory Management; this is **Project 1**)*

**What you learn:** The buffer pool manager: mapping disk pages into memory frames, replacement policies, dirty-page tracking, thread-safe page access, and why the OS page cache is insufficient. **Spring 2026 Project 1 has three deliverables — ARC Replacer, Disk Scheduler, Buffer Pool Manager (+ Page Guards) — and pages are 8 KB.**

#### Key Concepts

- Page vs frame: a **page** is 8 KB of logical data (in memory, on disk, or both); a **frame** is a fixed 8 KB block of memory that holds one page. The buffer pool stores pages inside frames.
- Buffer pool: a fixed-size array of frames. Each frame holds one page. A page table maps `page_id → frame_id`. A free list tracks available frames. Must be **thread-safe** (latched).
- Page pins: a pinned page cannot be evicted. A reader/writer pins before accessing, unpins when done. `pin_count` is an atomic counter on the `FrameHeader`; keep it in sync with the replacer's evictable state.
- **ARC (Adaptive Replacement Cache)** — the required Spring 2026 policy. Two cache lists (MRU = seen once, MFU = seen >once) + two **ghost lists** of recently-evicted pages, plus an adaptive `mru_target_size` that self-tunes on ghost-list hits. Generally beats LRU. (LRU, LRU-K, Clock are optional stubs.)
- **Disk Scheduler**: a background worker thread consumes a thread-safe queue (channel) of read/write `DiskRequest`s and dispatches them to the `DiskManager`; each request carries a promise/future the caller waits on. Decouples I/O from callers and enables batching/prefetch.
- **Page Guards** (`ReadPageGuard`/`WritePageGuard`): RAII handles giving thread-safe shared/exclusive access; on drop they unpin (and flush if dirty). In Rust this is the `Drop` trait + native move semantics — far less boilerplate than C++'s move ctors.
- Dirty pages: a modified page must be flushed to disk before its frame is reused (WAL requirement). Track via `is_dirty_` on the `FrameHeader`.
- Sequential flooding: a full scan pollutes the pool with pages never reused — part of why ARC (frequency-aware) helps over plain LRU.

#### Coding Exercises (Project 1, in Rust)

1. **ARC Replacer**: `record_access`, `set_evictable`, `evict`, `remove`, `size`. Implement the four `record_access` cases (MRU/MFU hit, MRU-ghost, MFU-ghost, miss) and the adaptive target. Use `VecDeque` + `HashMap` for O(1) move-to-front + membership. Thread-safe.
2. *(Optional, for learning)* Also implement **Clock** and **LRU-K** behind the same `Replacer` trait; benchmark all three on a Zipfian access pattern (80% of accesses to 20% of pages).
3. **Disk Manager**: read/write fixed **8 KB** pages to one file by `page_id` via `std::os::unix::fs::FileExt` (`read_exact_at`/`write_all_at`).
4. **Disk Scheduler**: `std::sync::mpsc` channel + a background worker thread; complete each request via a oneshot/`mpsc` callback (Rust's promise/future analogue); join the worker on drop.
5. **Buffer Pool Manager**: frames + `HashMap` page table + free list, `new_page`/`delete_page`/`checked_read_page`/`checked_write_page`/`flush_page`/`get_pin_count`, wired to the ARC replacer and disk scheduler. Return `ReadPageGuard`/`WritePageGuard` (RAII via `Drop`).

> **Implementation note:** This is **BusTub Project #1 (Spring 2026)** — the most foundational component; every later project depends on it. Build bottom-up: ARC replacer → disk manager → disk scheduler → buffer pool + page guards.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture #04 — Memory Management](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *ARC: A Self-Tuning, Low Overhead Replacement Cache* — Megiddo & Modha, USENIX FAST 2003. The ARC paper. Required reading for Project 1. |
| PAPER | *LRU-K Buffer Replacement* — O'Neil et al., SIGMOD 1993. The classic LRU-K policy (optional alternative). |
| SPEC | [BusTub Project #1 — Buffer Pool Manager](https://15445.courses.cs.cmu.edu/spring2026) — read the `arc_replacer.h`, `disk_scheduler.h`, `page_guard.h`, and `buffer_pool_manager.h` headers before implementing. |

---

## Module C — Indexes

### L06 — Hash Indexes

**What you learn:** Static and dynamic hash tables for point lookups: linear probing, chained hashing, extendible hashing, and cuckoo hashing. Understand when hash indexes outperform B-trees and when they fail.

#### Key Concepts

- Static hashing: fixed number of buckets. Overflow with chaining or open addressing. Fast for equality lookups, useless for range queries.
- Extendible hashing: maintain a global directory of 2^d pointers to local buckets. When a bucket overflows, split it and double the directory if needed. Amortized O(1) insert.
- Linear hashing: split buckets in round-robin order independent of which bucket overflows. No directory — uses a split pointer. Better for concurrent access than extendible hashing.
- Cuckoo hashing: maintain two hash tables with two hash functions. On collision, evict the existing key to its alternate position (recursively). O(1) worst-case lookup, O(1) amortized insert.
- Robin Hood hashing: on collision during linear probing, displace the element with the smaller probe distance. Reduces variance in probe lengths. Used in high-performance hash maps (abseil, Rust's hashbrown).

#### Coding Exercises

1. Implement an extendible hash table in Rust: global directory as `Vec<Box<Bucket>>`, split on overflow, double directory on global depth exhaustion. Write a test with 10,000 random inserts.
2. Implement Robin Hood open-addressing hash map in Rust from scratch. Benchmark it against `std::collections::HashMap` on 1M string lookups.
3. Implement cuckoo hashing with two tables and two hash functions. Implement stash for eviction cycles. Measure the maximum load factor before stash kicks in.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 07 — Hash Tables](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *Cuckoo Hashing* — Pagh & Rodler 2004. The original cuckoo hashing paper. Short. Read before implementing. |
| BLOG | [Robin Hood Hashing](https://sebastiansylvan.com) — The best blog post explaining Robin Hood displacement with diagrams. |

---

### L07 — B+ Tree Indexes I — Structure & Operations

**What you learn:** The B+ tree: the most important data structure in database systems. Internal nodes store only keys for routing; leaf nodes store key-value pairs and are linked in a doubly linked list.

#### Key Concepts

- B+ tree vs B-tree: in a B+ tree all data lives in leaf nodes; internal nodes are navigation-only. The leaf linked list enables efficient range scans with no tree traversal after the first lookup.
- Node structure: each node holds between ⌈n/2⌉ and n keys (order-n B+ tree). Leaf nodes hold actual values (or record IDs for secondary indexes). Internal nodes hold separating keys and child pointers.
- Search: O(log_n N) — traverse from root, binary search within each node, follow child pointer. For a 4KB page holding 200 4-byte keys, a tree of 1B records needs only 3 levels.
- Insert: search for leaf, insert key. If leaf overflows, split — push the middle key up to the parent. Splits propagate up until a non-full node or a new root is created.
- Delete: search for leaf, remove key. If leaf underflows, try to borrow from a sibling (redistribute). If not possible, merge with sibling and pull separator down from parent. Merges propagate up.
- Bulk loading: sort the data, then build the B+ tree bottom-up filling pages to 70% to leave room for future inserts. Orders of magnitude faster than individual inserts.

#### Coding Exercises

1. Implement a B+ tree in Rust with configurable order. Implement search, insert, delete, and range scan (return all keys in `[lo, hi]`). Write 500 randomized tests.
2. Implement bulk loading: given a sorted `Vec<(Key,Value)>`, build the B+ tree bottom-up without using the insert path. Verify the result is identical to building it via inserts.
3. Benchmark your B+ tree vs `std::collections::BTreeMap` on 1M random inserts and 1M random lookups. Measure both throughput and the number of cache misses using `perf stat`.

> **Implementation note:** BusTub Project #2: implement the B+ tree index. This is the most complex project. Budget 3 weeks. Implement concurrent latch-crabbing in the next lecture's exercises.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 08 — Trees I](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *Organization and Maintenance of Large Ordered Indices* — Bayer & McCreight 1972. The B-tree paper. Read alongside the CMU lecture. |
| BLOG | [B-Trees: More Than I Thought I'd Want to Know](https://benjamindicken.com) — Ben Dicken. Excellent visual walkthrough of B+ tree operations with animations. |

---

### L08 — B+ Tree Indexes II — Concurrency & Optimizations

**What you learn:** Concurrent B+ tree access via latch crabbing, Blink trees, and optimistic latch coupling. Plus optimizations: prefix compression, suffix truncation, deferred merges, and fractional cascading.

#### Key Concepts

- Latch crabbing (lock coupling): acquire child latch before releasing parent latch. Read: shared latches, release parent immediately. Write: exclusive latches, release parent only when child is confirmed safe (not full/underflow).
- Blink tree: add a 'high key' and sibling pointer to each node. A thread that misses a split can follow the sibling pointer rather than restarting from the root. Enables fine-grained locking.
- Optimistic latch coupling (OLC): assume no structural modification. Descend with shared latches only, validate at the leaf. On failure, restart. Reduces contention 10x on read-heavy workloads.
- Prefix compression: if all keys in a node share a common prefix, store the prefix once. Reduces node size, fits more keys per page.
- Suffix truncation: internal node separator keys only need enough bytes to distinguish left and right subtrees — not the full key. Saves space in internal nodes.
- Deferred merges: instead of merging on every underflow, mark nodes as 'half-full' and merge in the background. Reduces write amplification at the cost of slightly more wasted space.

#### Coding Exercises

1. Add latch crabbing to your B+ tree: use `RwLock` per node. Implement the read path (shared latches, release parent on descent) and the write path (exclusive latches, safe node check).
2. Implement OLC using `std::sync::atomic::AtomicU64` version counters per node. Descend with optimistic reads, validate version at the leaf. Benchmark vs latch crabbing under 8-thread concurrent load.
3. Implement prefix compression: store a per-node prefix string, strip it from all keys before insertion into the node. Measure the reduction in node count for a dataset of URL strings.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 09 — Trees II](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *Efficient Locking for Concurrent Operations on B-Trees* — Lehman & Yao 1981. The Blink tree paper. Defines the foundation of concurrent B+ tree access. |
| PAPER | *Optimistic Lock Coupling: A Scalable and Efficient General-Purpose Synchronization Method* — Leis et al. 2019. Modern OLC applied to B+ trees. VLDB. The technique used in HyPer and LeanStore. |

---

## Module D — Query Execution

### L09 — Query Processing & Execution Models

**What you learn:** Iterator (Volcano) model, materialization model, and vectorized (batch) model. How the query executor processes operators: sequential scan, index scan, aggregation, joins.

#### Key Concepts

- Volcano (iterator) model: each operator is an iterator with `open()`, `next()`, `close()`. `next()` returns one tuple at a time. Simple to implement but has high function call overhead — one virtual dispatch per tuple per operator.
- Materialization model: each operator processes all its input and produces all its output before handing it to the parent. High memory, but eliminates per-tuple overhead. Used in systems like VoltDB.
- Vectorized model: `next()` returns a batch of N tuples (typically 1024–4096). Amortizes function call overhead across a batch. Enables SIMD processing. Used in DuckDB, Velox, ClickHouse.
- Sequential scan: iterate over every page in the heap file, apply predicates per tuple. The simplest executor but the most important to optimize (prefetching, filter pushdown, SIMD predicate evaluation).
- Aggregation: hash aggregation (hash on group-by keys, accumulate aggregates in hash table) vs sort-based aggregation (sort on group-by keys, then scan). Hash is faster; sort handles larger-than-memory data.
- External merge sort: sort runs that exceed available memory. Phase 1: create sorted runs of B pages each. Phase 2: merge B-1 runs at a time using a priority queue.

#### Coding Exercises

1. Implement the Volcano executor model in Rust: trait `Executor { fn next(&mut self) -> Option<Tuple> }`. Implement SeqScan, Filter, Projection, HashAggregate, and NestedLoopJoin.
2. Extend to the vectorized model: `next()` returns `Vec<Tuple>` with capacity 1024. Benchmark the per-tuple overhead reduction on a 10M row aggregation query.
3. Implement external merge sort for a `Vec` that exceeds memory: create sorted runs, then k-way merge using a `BinaryHeap<(Key, RunId)>`. Verify on a dataset 10x larger than the configured memory budget.

> **Implementation note:** BusTub Project #3: implement all executor operators. The project tests include aggregation, hash join, and index scan. Implement them top-down from the trait.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 11 — Query Execution I](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *MonetDB/X100: Hyper-Pipelining Query Execution* — Boncz et al. CIDR 2005. The paper that introduced vectorized execution. Read before implementing the batch model. |
| BLOG | [How DuckDB's Vectorized Execution Works](https://blog.duckdb.org) — explains DuckDB's push-based vectorized model with code examples. |

---

### L10 — Join Algorithms

**What you learn:** Nested loop join, sort-merge join, hash join, and their variants for OLTP vs OLAP workloads. Cost models for join ordering.

#### Key Concepts

- Simple nested loop join: O(N*M) — for each outer tuple, scan all inner tuples. Only correct approach for theta joins (non-equi). Terrible for large inputs.
- Block nested loop join: load a block of outer tuples, scan inner once per block. Reduces I/O from O(N*M) to O(N/B * M) where B is buffer pool size. Use B-2 frames for the outer block.
- Index nested loop join: if the inner relation has an index on the join key, use it. O(N * log M). Best algorithm when the inner relation is small or indexed.
- Sort-merge join: sort both relations on the join key, then merge. O(N log N + M log M). Excellent when inputs arrive sorted (e.g., from an ORDER BY earlier in the plan). Handles duplicate keys correctly.
- Hash join: partition both relations by hash(join_key) into B-1 buckets. Then probe each partition pair. O(N+M) average case. Build phase loads the smaller relation's partition into memory.
- Hybrid hash join: if the build relation fits in memory, skip partitioning entirely (classic hash join). If not, partition first. The DBMS chooses based on estimated relation size from statistics.

#### Coding Exercises

1. Implement all four join algorithms in Rust as `Executor` trait implementations: SimpleNLJ, BlockNLJ, SortMergeJoin, HashJoin. Test each produces identical output.
2. Benchmark the four algorithms on: (1) 100K x 100K row join, (2) 100K x 10 row join (inner is small), (3) pre-sorted input. Measure wall time and I/O for each scenario.
3. Implement a simple join-order optimizer: given 3 relations and their estimated cardinalities, enumerate all left-deep join trees and pick the one with minimum estimated intermediate result size using a cost model.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 13 — Join Algorithms](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *An Experimental Comparison of Thirteen Relational Equi-Joins* — Schuh et al. SIGMOD 2016. Empirically tests which join algorithm wins under which conditions. Required reading. |
| PAPER | *An Evaluation of Adaptive Partitioning for In-Memory Hash Joins* — Balkesen et al. ICDE 2013. NUMA-aware hash join partitioning. Relevant when you reach Phase 2 scheduling. |

---

### L11 — Query Planning & Optimization

**What you learn:** Query plan generation from SQL: parsing, binding, logical planning, cost-based optimization (Selinger algorithm), rule-based rewrites, and statistics-based cardinality estimation.

#### Key Concepts

- Query compilation pipeline: SQL text → parse tree → logical plan (relational algebra tree) → physical plan (concrete operators with access methods) → bytecode/interpreted execution.
- Heuristic (rule-based) optimizations: always apply first. Push predicates below joins (predicate pushdown). Project out unused columns early (projection pushdown). Convert subqueries to joins.
- Selinger algorithm (System R join ordering): dynamic programming over all subsets of relations. For n relations, enumerate all 2^n subsets (feasible for n ≤ 15). For n > 15, use genetic algorithms or greedy heuristics.
- Statistics: for each column, the optimizer maintains a histogram of value distribution, number of distinct values (NDV), and null fraction. Used to estimate selectivity of predicates.
- Cardinality estimation: the hardest problem in query optimization. Errors compound multiplicatively across joins — a 10x error on each of 5 joins becomes a 100,000x error in the final plan.
- Adaptive query execution: re-estimate cardinalities at runtime using actual row counts, then re-optimize mid-query. Used in Apache Spark (AQE), Greenplum, and Orca.

#### Coding Exercises

1. Implement a simple query optimizer in Rust: take a logical plan tree (relational algebra), apply predicate pushdown and projection pushdown, then enumerate left-deep join orders using dynamic programming.
2. Implement equi-depth histograms for a column: given a sorted `Vec`, create B buckets each containing approximately equal number of values. Implement selectivity estimation for range predicates.
3. Implement join cardinality estimation using the independence assumption: `est(A ⋈ B) = |A| * |B| / max(NDV(A.key), NDV(B.key))`. Measure the estimation error on the TPC-H benchmark.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lectures 15–16 — Query Planning](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *Access Path Selection in a Relational Database Management System* — Selby et al. IBM 1979. The System R optimizer paper. Defines the DP join ordering algorithm still used today. |
| PAPER | *How Good Are Query Optimizers, Really?* — Leis et al. VLDB 2015. Benchmark of cardinality estimation errors in real systems. Motivates why this is hard. |

---

## Module E — Transactions & Concurrency

### L12 — Transactions & ACID

**What you learn:** What transactions are, ACID properties, the formal definition of serializability, and why isolation levels exist — the theoretical core of all concurrency control.

#### Key Concepts

- Transaction: a sequence of read/write operations that executes atomically and in isolation. Either all operations commit or all are rolled back.
- ACID: Atomicity (all-or-nothing), Consistency (constraints preserved), Isolation (concurrent transactions appear serial), Durability (committed data survives crashes).
- Conflict serializability: a schedule is conflict-serializable if it is equivalent to some serial schedule. Detect using a precedence graph (conflict graph): add edge Ti→Tj if Ti has a conflicting operation before Tj. Cycle = not serializable.
- Isolation levels (SQL standard): READ UNCOMMITTED (dirty reads allowed), READ COMMITTED (no dirty reads), REPEATABLE READ (no non-repeatable reads), SERIALIZABLE (no phantoms). Lower levels have higher concurrency but more anomalies.
- Anomalies: dirty read (read uncommitted data that gets rolled back), non-repeatable read (two reads of same row return different values), phantom read (a re-executed predicate returns different rows due to concurrent INSERT).

#### Coding Exercises

1. Implement a conflict graph builder in Rust: given a schedule (sequence of `(TxnId, Operation, Key)`), build the precedence graph and detect cycles using DFS. Return whether the schedule is conflict-serializable.
2. Write a test suite of 20 schedules: 10 serializable, 10 not. Verify your conflict graph builder correctly classifies all of them.
3. Implement a simple lock-free in-memory database using a `HashMap` and simulate READ COMMITTED isolation using a copy-on-write scheme.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 17 — Transactions & ACID](https://youtube.com/@CMUDatabaseGroup) |
| BOOK | *Transaction Processing: Concepts and Techniques* — Gray & Reuter 1992. The definitive book on transactions. Read chapters 1–4 this week. |
| PAPER | *ACID Rain: Lessons from Two Decades of ACID* — Hellerstein & Stonebraker 2009. Retrospective on what ACID means in practice. Good motivation before diving into implementation. |

---

### L13 — Two-Phase Locking (2PL) & Deadlock Handling

**What you learn:** The 2PL protocol for conflict-serializable schedules: growing phase, shrinking phase, strict 2PL. Lock granularity, intention locks, and deadlock detection vs prevention.

#### Key Concepts

- 2PL protocol: growing phase — acquire locks, release none. Shrinking phase — release locks, acquire none. Any schedule produced by 2PL is conflict-serializable.
- Strict 2PL: hold all exclusive (write) locks until commit or abort. Prevents cascading aborts. Used by every mainstream DBMS.
- Lock granularity hierarchy: database → table → page → tuple. Coarse = less overhead, more contention. Fine = more overhead, less contention.
- Intention locks: IX (intention exclusive), IS (intention shared), SIX (shared + intention exclusive). Allow a transaction to signal its intent to acquire fine-grained locks without blocking the entire table.
- Deadlock detection: build a waits-for graph. Edge Ti→Tj if Ti waits for a lock held by Tj. Detect cycles periodically (every 100ms). Victim selection: youngest transaction, fewest locks held, least work done.
- Deadlock prevention: wound-wait (older transaction wounds younger) or wait-die (younger waits, older dies). Timestamps determine priority. No cycle detection needed.

#### Coding Exercises

1. Implement a lock manager in Rust: `LockTable` as `HashMap`. Support shared and exclusive locks with upgrade. Implement lock acquisition and release.
2. Implement the waits-for graph and deadlock detection: build the graph from the `LockTable` state and run DFS to detect cycles. Select and abort the victim transaction.
3. Implement strict 2PL on top of your lock manager: wrap a transaction's read/write operations to automatically acquire locks, release all on commit/abort. Write a test with 3 concurrent transactions that would deadlock without detection.

> **Implementation note:** BusTub Project #4: implement the lock manager with shared/exclusive locks, table/row granularity, and deadlock detection. This is the last BusTub project — it combines everything.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 18 — Two-Phase Locking](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *Granularity of Locks and Degrees of Consistency* — Gray et al. 1976. The paper defining intention locks and isolation levels. VLDB. Read sections 1–3. |
| BLOG | [Designing a Lock Manager](https://codingconfessions.com) — Practical walkthrough of implementing a lock manager with real concurrency considerations. |

---

### L14 — Timestamp Ordering & Optimistic Concurrency Control

**What you learn:** Lock-free concurrency control via timestamp ordering (T/O), Thomas Write Rule, and optimistic concurrency control (OCC/BOCC/FOCC) — the basis of Silo, TicToc, and many modern systems.

#### Key Concepts

- Basic T/O: assign each transaction a timestamp TS(Ti). Read: check TS(Ti) >= W-TS(X). Write: check TS(Ti) >= both R-TS(X) and W-TS(X). Violation → abort and restart with new timestamp.
- Thomas Write Rule: a write to X by Ti is ignored (not aborted) if a later transaction Tj has already written X (TS(Ti) < W-TS(X)). Produces a non-strict but correct schedule.
- OCC (Optimistic Concurrency Control): read phase (execute speculatively in private workspace), validation phase (check for conflicts with committed transactions), write phase (if valid, install changes).
- Validation in OCC: for transaction Ti, check all transactions Tj that committed during Ti's read phase. If Tj's write set intersects Ti's read set, abort Ti.
- BOCC vs FOCC: backward OCC validates against already-committed transactions (simple, conservative). Forward OCC validates against transactions that will commit later (more complex, less aborts).
- TicToc: timestamp-free OCC — compute a valid commit timestamp range at validation time, rather than assigning timestamps at start. Reduces false aborts significantly.

#### Coding Exercises

1. Implement basic Timestamp Ordering in Rust: each tuple has R-TS and W-TS. Implement read and write with the T/O check and abort on violation.
2. Implement OCC: read phase builds a read set and write set in a private `HashMap`. Validation phase iterates committed transactions (stored in a `Vec` with their write sets) and checks for intersection. Write phase installs changes.
3. Benchmark 2PL vs OCC on: (1) high-contention workload (all transactions touch the same 10 rows), (2) low-contention workload (transactions touch disjoint rows). OCC should win on low contention.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 19 — Timestamp Ordering Concurrency Control](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *Optimistic Concurrency Control Revisited* — Kung & Robinson 1981. The original OCC paper. Essential reading. Read before implementing. |
| PAPER | *Silo: Speedy Transactions in Multicore In-Memory Databases* — Tu et al. SOSP 2013. Modern OCC at scale — the basis of many in-memory DBMS designs. |

---

### L15 — Multi-Version Concurrency Control (MVCC)

**What you learn:** MVCC: maintain multiple physical versions of each tuple, allowing readers to never block writers. The foundation of PostgreSQL, MySQL InnoDB, Oracle, and most modern DBMSs.

#### Key Concepts

- MVCC model: each write creates a new version of a tuple. Readers access the latest version visible to their snapshot timestamp. Readers never block writers; writers never block readers.
- Version storage: append-only storage (new versions appended to the table's heap — PostgreSQL), delta storage (store only changed fields — DB2), time-travel storage (move old versions to a separate table).
- Snapshot isolation: a transaction sees the database as it existed at its start timestamp. Achieves repeatable reads without blocking. Does not provide full serializability (write skew anomaly).
- Write skew: T1 reads A and B, T2 reads A and B, both decide to write based on both values. Neither write conflicts, but the combined result is inconsistent. Requires serializable snapshot isolation (SSI) to prevent.
- MVCC garbage collection: old versions must be reclaimed when no active transaction can see them. Epoch-based GC (wait for all transactions in an epoch to finish) or vacuum (PostgreSQL's background process).
- MVCC and indexes: secondary indexes point to the latest visible version or to all versions. PostgreSQL stores heap TIDs; InnoDB stores primary key values and follows them to the clustered index.

#### Coding Exercises

1. Implement a basic MVCC store in Rust: each key maps to a `Vec<(Version, Value)>` sorted by version descending. Reads return the latest version <= transaction's read timestamp. Writes append a new version.
2. Implement snapshot isolation: assign each transaction a `start_ts` at begin and a `commit_ts` at commit. A transaction can only commit if its write set doesn't conflict with any transaction that committed after its `start_ts`.
3. Implement GC: after a global watermark timestamp (the minimum active transaction `start_ts`), delete all versions older than the watermark from all keys. Measure memory reclaimed per GC cycle.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lecture 20 — Multi-Version Concurrency Control](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *An Evaluation of Concurrency Control with One Thousand Cores* — Yu et al. VLDB 2014. Compares 2PL, T/O, OCC, MVCC at scale. Essential benchmark paper. |
| PAPER | *A Critique of ANSI SQL Isolation Levels* — Berenson et al. SIGMOD 1995. Defines isolation anomalies precisely and shows the SQL standard is ambiguous. Read after understanding MVCC. |

---

## Module F — Recovery

### L16 — Crash Recovery — Logging & ARIES

**What you learn:** Write-ahead logging (WAL), the ARIES recovery algorithm (Analysis, Redo, Undo), log sequence numbers (LSNs), and checkpointing. The correctness guarantee that makes DBMS crash-safety possible.

#### Key Concepts

- WAL rule: before any page can be written to disk, every log record that modifies that page must first be flushed to the log. Guarantees durability without syncing data pages immediately.
- Log record types: BEGIN, COMMIT, ABORT, UPDATE (contains before-image and after-image of modified bytes), COMPENSATION LOG RECORD (CLR — logged undo operations, used during recovery).
- Log sequence numbers (LSN): monotonically increasing identifier for each log record. Each page's header stores the pageLSN of the most recent update. The log's flushedLSN tracks the last synced log record.
- ARIES Analysis phase: scan forward from the last checkpoint. Reconstruct which transactions were active at crash time (ATT — active transaction table) and which pages were dirty (DPT — dirty page table).
- ARIES Redo phase: scan forward from the oldest dirty page in the DPT. Redo every update whose LSN is greater than the page's stored pageLSN. Redo even aborted transactions (the undo phase handles rollback).
- ARIES Undo phase: process the ATT (all transactions that were active at crash time). Undo their operations in reverse LSN order. Write CLRs for each undone operation so undo itself is idempotent.

#### Coding Exercises

1. Implement a WAL in Rust: log records as an append-only `Vec`. Implement a `LogManager` that writes records, assigns LSNs, and flushes to a file. Implement the WAL rule in your `BufferPoolManager`.
2. Implement ARIES: given a log file and a dirty page table from the last checkpoint, run Analysis, Redo, and Undo phases. Write a test that crashes mid-transaction and verifies the recovered state is consistent.
3. Implement fuzzy (ARIES) checkpointing: write a BEGIN_CHECKPOINT record, then an END_CHECKPOINT containing the ATT and DPT at checkpoint time, without halting ongoing transactions. Verify recovery still works correctly.

#### Papers & Resources

| Type | Resource |
|------|----------|
| VIDEO | [CMU 15-445 Lectures 21–22 — Logging & Recovery](https://youtube.com/@CMUDatabaseGroup) |
| PAPER | *ARIES: A Transaction Recovery Method Supporting Fine-Granularity Locking and Partial Rollbacks* — Mohan et al. ACM TODS 1992. The ARIES paper. Foundational. Read sections 1–5 and the algorithm itself. |
| BLOG | [How PostgreSQL Handles Crash Recovery](https://brandur.org) — Excellent practical explanation of WAL and recovery in a real system. |

---

# Phase 2 — CMU 15-721: Advanced Database Systems (OLAP Focus)

**Columnar storage · Vectorized execution · Query compilation · SIMD · NUMA · Paper-per-lecture**

**Goal:** Build a mental model of production-grade OLAP systems: DuckDB, ClickHouse, Snowflake, Redshift. Every lecture is anchored to 1–2 mandatory research papers. Read them before watching.

---

### L17 — Modern OLAP Architecture & Lakehouses

**What you learn:** The evolution from data warehouses to data lakes to lakehouses. Compute-storage separation, open table formats (Delta Lake, Apache Iceberg, Apache Hudi), and object store-backed OLAP.

#### Key Concepts

- 2000s data warehouses: monolithic OLAP DBMSs (Teradata, Netezza). Shared-nothing architecture, columnar storage. ETL pipelines copy data from OLTP systems.
- 2010s data lakes: raw files on HDFS/S3. Schema-on-read. Cheap storage but no ACID, poor performance, no schema enforcement.
- Lakehouses: metadata layer over object store adding ACID transactions, versioning, and schema enforcement. Delta Lake (Databricks), Apache Iceberg (Netflix/Apple), Apache Hudi (Uber). Row groups = PAX storage.
- Compute-storage separation: storage in S3, compute in ephemeral clusters. Enables independent scaling. But cold start latency, caching becomes critical.
- Open file formats: Parquet (columnar, row-group-based, open). ORC (columnar, Hive origin). Arrow (in-memory, columnar, IPC format). Know the difference between on-disk and in-memory columnar formats.
- PAX storage model (Partition Attributes Across): tables partitioned into row groups. Within each row group, data stored column-by-column. Balances scan performance (columnar) with tuple reconstruction (row groups).

#### Coding Exercises

1. Implement a Parquet reader in Rust using the `parquet` crate: read a Parquet file, decode row groups, apply a predicate pushdown, and return matching rows as `Vec`.
2. Implement a simple Iceberg-style table format: metadata JSON file tracking a list of manifest files, each manifest listing data files (Parquet) with min/max statistics. Implement snapshot read and append.
3. Benchmark PAX vs pure row storage vs pure columnar storage on: (1) full table scan with one column, (2) point lookup, (3) aggregation over 3 columns. Use a 10M row dataset.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *Lakehouse: A New Generation of Open Platforms* — Armbrust et al. CIDR 2021. The paper defining the lakehouse concept. Read before the lecture. |
| PAPER | *An Empirical Evaluation of Columnar Storage Formats* — Zeng et al. VLDB 2023. Empirical comparison of Parquet, ORC, and other formats on real workloads. Essential. |
| VIDEO | [CMU 15-721 Lecture 01 — Modern Analytical Database Systems](https://youtube.com/playlist?list=PLSE8ODhjZXjYa_zX-KeMJui7pcN1rIaIJ) |

---

### L18 — Columnar Data Formats & Compression

**What you learn:** Run-length encoding, delta encoding, dictionary encoding, bit-packing, frame-of-reference, FSST, and how modern columnar compression achieves both high compression ratios and fast decompression for query processing.

#### Key Concepts

- Dictionary encoding: map repeated values (strings, categories) to integer codes. Store the dictionary separately. Enables operating on codes directly without decompression.
- Run-length encoding (RLE): store (value, count) pairs for runs of identical values. Extremely effective for sorted columns or low-cardinality columns. Used heavily in column stores.
- Delta encoding: store differences between consecutive values instead of absolute values. Effective for timestamps, monotonically increasing IDs, and sorted numeric columns.
- Bit-packing: instead of 32 bits per integer, use only `ceil(log2(max_value))` bits. Pack multiple values into each 64-bit word. Reduces storage by 4–8x for small integers.
- Frame of Reference (FOR): subtract a base value (e.g., minimum in a block) from all values, then bit-pack the residuals. Effective for dense integer ranges.
- FastLanes: a SIMD-friendly bit-packing layout that enables decoding >100 billion integers per second. Reorders bit planes to maximize SIMD utilization.

#### Coding Exercises

1. Implement dictionary encoding in Rust: encode a `Vec` into `(Vec<u32> codes, HashMap dict)`. Implement predicate evaluation directly on codes (equality, IN list) without decoding.
2. Implement RLE encoding and decoding in Rust. Implement predicate pushdown: for a range predicate on a RLE-encoded column, skip entire runs without decompressing.
3. Implement bit-packing: pack N integers each using B bits into a `u64` array. Implement both scalar and SIMD (using `std::simd`) decode. Benchmark decode throughput for B=4,8,16,32.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *The FastLanes Compression Layout* — Afroozeh et al. VLDB 2023. The state-of-the-art SIMD-friendly bit-packing layout. Read before implementing. |
| PAPER | *BtrBlocks: Efficient Columnar Compression for Data Lakes* — Kuschewski et al. SIGMOD 2023. Nested cascading compression scheme. Covers FSST for string compression. |
| VIDEO | [CMU 15-721 Lectures 02–03 — Data Formats](https://youtube.com/playlist?list=PLSE8ODhjZXjYa_zX-KeMJui7pcN1rIaIJ) |

---

### L19 — Vectorized Query Execution & SIMD

**What you learn:** Processing queries in batches of 1024–4096 tuples using SIMD instructions. Selection vectors, late materialization, and the MonetDB/X100 execution model that underlies DuckDB, Velox, and ClickHouse.

#### Key Concepts

- Vectorized execution: instead of one tuple at a time (Volcano), process a vector (batch) of 1024–4096 tuples per operator call. Amortizes function call overhead and enables SIMD parallelism.
- Selection vectors: instead of copying matching tuples, maintain a list of row indices that pass the predicate. Downstream operators process only selected indices. Used in DuckDB and ClickHouse.
- Late materialization: don't reconstruct full tuples until the last possible moment. Apply predicates on individual columns, then stitch together matching rows at the final projection. Reduces bandwidth.
- SIMD (Single Instruction Multiple Data): AVX2 processes 8 floats or 4 doubles simultaneously. AVX-512 processes 16 floats. Key primitives: gather, scatter, comparison, blend, prefix sum.
- Branchy vs branch-free code: mispredicted branches stall the CPU pipeline for 15–20 cycles. Convert if-then-else to blend operations (branchless selection). Critical for vectorized predicates.
- Filter representation: selection vector (list of selected indices) vs bitmask (one bit per row). Selection vector is faster when selectivity is high (many rows pass). Bitmask is better when selectivity is low.

#### Coding Exercises

1. Implement a vectorized filter in Rust using `std::simd`: given a `Vec<f64>` and a threshold, return a `Vec<usize>` of indices where value > threshold. Compare throughput to a scalar loop.
2. Implement a vectorized hash aggregation: process tuples in batches of 1024. Compute group keys in batch, hash in batch (SIMD hash), accumulate aggregates. Benchmark vs your scalar hash aggregation from Phase 1.
3. Implement a selection vector-based pipeline: SeqScan produces `(column_arrays, selection_vector)`, Filter refines the selection vector in-place without copying, Projection materializes only selected rows.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *MonetDB/X100: Hyper-Pipelining Query Execution* — Boncz et al. CIDR 2005. The paper that started vectorized execution. Read before this lecture. |
| PAPER | *Rethinking SIMD Vectorization for In-Memory Databases* — Polychroniou et al. SIGMOD 2015. Best paper on applying SIMD to database operators: selection, partitioning, sorting. |
| VIDEO | [CMU 15-721 Lecture 06 — Vectorized Execution](https://youtube.com/playlist?list=PLSE8ODhjZXjYa_zX-KeMJui7pcN1rIaIJ) |

---

### L20 — Query Compilation & Code Generation

**What you learn:** Compiling query plans to native code: the Neumann pipeline model (HyPer), LLVM-based compilation, and why compiled queries outperform interpreted vectorized execution by 2–10x on CPU-bound workloads.

#### Key Concepts

- Push-based execution (producer-consumer model): instead of a parent pulling from a child (Volcano), a child pushes tuples to its parent. Enables tight inner loops without virtual dispatch.
- Tighten the loop: the compiled code for a query like `SELECT SUM(a) FROM t WHERE b > 5` is a single tight loop over the data with no function calls. Achieves near-peak memory bandwidth utilization.
- LLVM IR generation: translate each query plan operator to LLVM IR. The LLVM compiler applies standard optimizations (loop unrolling, auto-vectorization, register allocation). JIT compile to native code per query.
- Compilation latency vs execution speedup: compilation takes 50–200ms. For short OLTP queries (< 1ms), compilation overhead is not worth it. For long OLAP queries (> 1s), the 10x speedup pays off.
- Adaptive execution: start interpreting the query. If it runs long enough to justify compilation, compile in the background and switch to compiled execution mid-query. Used in Umbra (HyPer successor).
- WASM as a compilation target: some systems compile queries to WebAssembly for portability. Slower than native LLVM but enables sandboxed UDF execution.

#### Coding Exercises

1. Implement a simple push-based query engine in Rust: define trait `Producer { fn produce(&self, consumer: &dyn Consumer); }`. Implement SeqScan (producer) → Filter → Aggregate (consumers).
2. Generate Rust closures for predicates at query build time rather than evaluating a predicate tree at runtime: instead of `eval_expr(expr, tuple)`, generate `|tuple| tuple.a > 5 && tuple.b < 10`. Benchmark.
3. Read the HyPer paper and sketch the LLVM IR for the query `SELECT SUM(a) FROM t WHERE b > 5` on paper. Identify which loops the compiler can vectorize and which it cannot.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *Efficiently Compiling Efficient Query Plans for Modern Hardware* — Neumann VLDB 2011. The HyPer compilation paper. The most important paper in modern query execution. Read carefully. |
| PAPER | *Everything You Always Wanted to Know About Compiled and Vectorized Queries* — Kersten et al. VLDB 2018. Side-by-side comparison of compilation vs vectorization. When does each win? |
| VIDEO | [CMU 15-721 Lecture 07 — Query Compilation](https://youtube.com/playlist?list=PLSE8ODhjZXjYa_zX-KeMJui7pcN1rIaIJ) |

---

### L21 — NUMA-Aware Scheduling & Morsel-Driven Parallelism

**What you learn:** Modern CPUs are NUMA (Non-Uniform Memory Access) machines. The Morsel-Driven Parallelism framework (HyPer) distributes query work across NUMA nodes and cores to achieve near-linear scaling.

#### Key Concepts

- NUMA architecture: a multi-socket server has local memory (1x latency) and remote memory (3–5x latency). The OS's default thread scheduler is NUMA-unaware and causes significant remote access.
- Morsel: a chunk of ≈ 10,000 tuples from a relation. The dispatcher assigns morsels to worker threads pinned to specific NUMA nodes. Workers consume local data whenever possible.
- Work-stealing for load balancing: if a thread finishes its morsels early, it steals morsels from a sibling thread's queue. Steals from same-NUMA-node threads first.
- Pipeline parallelism: different stages of a query pipeline can run simultaneously. The dispatcher must coordinate so pipeline stages don't starve each other.
- Elastic parallelism: reduce the number of threads assigned to a query if the system is under light load (fewer NUMA nodes needed). Adjust dynamically based on system-wide load.
- Thread-local hash tables for aggregation: each thread builds its own partial hash table, then merge at the end. Avoids false sharing in the hash table during the build phase.

#### Coding Exercises

1. Implement a morsel-driven parallel table scan in Rust using rayon: split the table into chunks of 10,000 rows, process each chunk on a Rayon thread, aggregate partial results with a lock-free combiner.
2. Implement thread-local hash aggregation: each thread builds a `HashMap`, then merge all thread-local maps into a global result. Benchmark vs a single-threaded hash aggregation.
3. Simulate NUMA effects: pin threads to CPU cores using `nix::sched::sched_setaffinity`. Measure the latency difference between accessing memory allocated on the local NUMA node vs a remote node (use `numactl` on Linux).

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *Morsel-Driven Parallelism* — Leis et al. SIGMOD 2014. The HyPer parallelism paper. Defines morsels and the dispatching model. Essential reading. |
| PAPER | *Scaling Up Concurrent Main-Memory Column-Store Scans* — Psaroudakis et al. VLDB 2015. NUMA-aware data placement for columnar scans. |
| VIDEO | [CMU 15-721 Lecture 08 — Scheduling & Coordination](https://youtube.com/playlist?list=PLSE8ODhjZXjYa_zX-KeMJui7pcN1rIaIJ) |

---

# Phase 3 — Storage Engine Deep Dive

**B-Tree vs LSM-Tree · Row vs Columnar · WAL designs · Build a KV store and a column store**

**Goal:** Build two complete storage engines from scratch in Rust: (1) a B-tree-based OLTP KV store similar to SQLite/LevelDB, and (2) a columnar OLAP store similar to DuckDB. Benchmark both on TPC-H and TPC-C workloads.

---

### L22 — B-Tree Storage Engines — LeanStore, LMDB, SQLite

**What you learn:** How production B-tree databases manage storage: buffer pool replacement policies, the LMDB MVCC-over-B-tree design, SQLite's WAL mode, and LeanStore's novel virtual memory-based buffer manager.

#### Key Concepts

- LMDB design: B-tree with copy-on-write MVCC. No write-ahead log needed — modified pages are always written to new locations. Readers never blocked, single-writer at a time. Extremely simple recovery.
- SQLite WAL mode: all writes go to a write-ahead log file. Readers read from the database file (or the WAL if their snapshot includes recent writes). Checkpointing periodically merges WAL into the main file.
- LeanStore: buffer manager using virtual memory and OS page faults to track page access. A background thread evicts cold pages. No page table — uses the OS TLB as the 'hot page directory'.
- Buffer pool-free designs: DuckDB and some analytical systems mmap the entire database file. No buffer pool management — the OS handles eviction. Works for read-mostly workloads, poor for write-heavy ones.
- Page eviction granularity: whole-page eviction (standard), sub-page eviction (WiredTiger), or record-level eviction (MongoDB). Finer granularity reduces memory waste, increases complexity.
- Write coalescing: instead of writing each dirty page immediately, batch dirty pages and flush together. Reduces random I/O. WAL enables this by deferring data page writes.

#### Coding Exercises

1. Implement a copy-on-write B-tree in Rust (LMDB-style): every write creates new node versions, never modifying nodes in-place. Implement MVCC read consistency by retaining old root pointers per snapshot.
2. Implement SQLite WAL mode: write transactions append to a `wal.bin` file. Reads check the WAL first, then fall back to the main database file. Implement checkpointing (copy WAL pages into main file, then truncate WAL).
3. Benchmark your copy-on-write B-tree vs your standard B-tree from Phase 1 on: write throughput, read throughput under concurrent readers, and recovery time after a simulated crash.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *LMDB: The Lighting Memory-Mapped Database* — Chu 2012. The LMDB technical description. Short and clear. Read the MVCC section carefully. |
| PAPER | *LeanStore: In-Memory Data Management Beyond Main Memory* — Leis et al. ICDE 2018. The LeanStore buffer manager. Novel approach using virtual memory for page tracking. |
| IMPL | [SQLite WAL documentation](https://sqlite.org/wal.html) — the definitive WAL mode description with diagrams. Read before implementing. |

---

### L23 — LSM-Tree Storage Engines — RocksDB, Cassandra, InfluxDB

**What you learn:** Production LSM-tree designs: RocksDB's column families, compaction tuning, bloom filters, and the tradeoff between write amplification (WA), read amplification (RA), and space amplification (SA).

#### Key Concepts

- RocksDB architecture: MemTable (skip list or hash map), L0 SSTables (overlapping), L1+ (non-overlapping, sorted). Background compaction threads maintain level sizes.
- Column families in RocksDB: separate MemTable, SSTable hierarchy, and compaction policy per column family. Allows mixing FIFO compaction (for time-series) and leveled compaction (for indexes) in the same instance.
- Bloom filter tuning: a 10-bit Bloom filter has 1% false positive rate. For a 1B key DB with 64KB SSTables, Bloom filters eliminate 99% of SSTable reads for missing keys. Always enable for point lookup workloads.
- Compaction tuning: size-tiered minimizes WA (≈10x) but has high SA. Leveled minimizes SA (≈1.1x) and RA but has high WA (≈30x). Tiered+leveled hybrid (STCS with leveled compaction) balances all three.
- Tombstones: deletes in LSM-trees write a tombstone (special delete marker). The key is hidden from reads until compaction physically removes both the key and its tombstone. Tombstone accumulation degrades range scan performance.
- RocksDB write stalls: when L0 SSTable count exceeds the stall threshold (default 20), writes are throttled. When it exceeds the stop threshold (default 36), writes are halted. Tune compaction to avoid stalls.

#### Coding Exercises

1. Add RocksDB-style column families to your LSM-Tree from Phase 1: separate MemTable and SSTable hierarchies, configurable compaction policy per family. Test with two families using different compaction strategies.
2. Implement tombstone compaction: during a compaction, if a key has a tombstone and no older version exists in higher levels, physically remove both. Verify with a test that deletes 50% of keys and then compacts.
3. Benchmark your LSM-Tree on the YCSB workload A (50% read, 50% update) and workload D (95% read, 5% insert) at 1M operations. Measure write amplification by counting total bytes written to disk.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *RocksDB: Evolution of Development Priorities in a Key-Value Store Serving Large-Scale Applications* — Cao et al. FAST 2020. How RocksDB evolved under real production workloads at Facebook/Meta. |
| PAPER | *Monkey: Optimal Bloom Filters for LSM-Trees* — Dayan et al. SIGMOD 2017. Optimal Bloom filter allocation across levels. Read before tuning bloom filters. |
| IMPL | [MiniLSM](https://skyzh.github.io/mini-lsm) — complete all 7 weeks of this Rust tutorial before implementing your own. |

---

### L24 — Column Store Architecture — DuckDB, ClickHouse, MonetDB

**What you learn:** Implementing a columnar OLAP storage engine: PAX layout, zone maps, late materialization, dictionary-encoded string columns, and vectorized scan operators.

#### Key Concepts

- DuckDB architecture: single-file embedded database, PAX storage (row groups of 122,880 rows each), per-column compression, zone maps for scan pruning, vectorized execution engine.
- ClickHouse storage: MergeTree family. Data sorted by primary key. Parts (immutable sorted files) merged in the background. Per-column files. Excellent for time-series and log analytics.
- Zone maps (min/max indexes): per-row-group metadata storing the min and max value of each column. A scan predicate can skip entire row groups if the predicate is impossible given the min/max. Zero cost at query time.
- Late materialization: apply predicates on individual column arrays, produce a row bitmap, then fetch only the selected rows' other columns. Reduces bandwidth by `(1 - selectivity) * num_columns`.
- Row group size tradeoff: small row groups (1K rows) enable fine-grained zone map pruning but high metadata overhead. Large row groups (1M rows) have low overhead but coarse pruning. DuckDB uses 122,880 rows.
- String dictionary per column: in a columnar store, string columns are dictionary-encoded globally per row group. All operations on string predicates operate on integer codes, dramatically reducing memory bandwidth.

#### Coding Exercises

1. Implement a PAX storage format in Rust: a row group is a struct of column arrays. Implement serialization/deserialization to binary files. Implement zone map metadata (min/max per column per row group).
2. Implement late materialization: given a `Vec<f64>` and a predicate, return a `Vec<usize>` selection vector. Then reconstruct full rows by indexing into each column at the selected positions.
3. Implement zone map pruning: for a query with a predicate on column C, skip all row groups where predicate is impossible given min/max(C). Measure the skip rate on the TPC-H lineitem table with date range predicates.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *C-Store: A Column-Oriented DBMS* — Stonebraker et al. VLDB 2005. The paper that launched modern columnar databases. Read before implementing your column store. |
| PAPER | *Column Stores vs Row Stores* — Abadi et al. SIGMOD 2008. Empirical analysis of why column stores win for analytical workloads. Essential. |
| VIDEO | DuckDB Internals — Mark Raasveldt — VLDB 2023 Talk. Search 'DuckDB internals VLDB' on YouTube. The best 30-minute explanation of DuckDB's storage engine. |

---

# Phase 4 — Distributed Database Systems

**Consensus · Distributed transactions · Sharding · Replication · Geo-distribution**

**Goal:** Understand and implement the core distributed systems primitives underlying systems like CockroachDB, TiDB, Spanner, and Cassandra. Start with Raft consensus, then layer distributed transactions, then distributed SQL.

---

### L25 — Distributed Systems Fundamentals — CAP, FLP, Consistency Models

**What you learn:** The theoretical limits of distributed systems: CAP theorem, FLP impossibility, linearizability vs serializability vs eventual consistency, and the practical tradeoffs in real systems.

#### Key Concepts

- CAP theorem: a distributed system can provide at most two of: Consistency (linearizable reads), Availability (every request gets a response), Partition tolerance (continues operating despite network partition). In practice, partitions happen — choose C or A.
- FLP impossibility: in an asynchronous network, there is no deterministic algorithm that can solve consensus if even one process may fail. This is why consensus protocols require timeouts and randomization.
- Linearizability: the strongest consistency model. Operations appear instantaneous, as if executed on a single machine at some point between their invocation and response. Expensive — requires coordination.
- Serializability vs linearizability: serializability (database isolation) says transactions appear to execute in some serial order. Linearizability (distributed) says operations respect real-time ordering. Strict serializability = both.
- Eventual consistency: if no new updates are made, all replicas will eventually converge to the same state. Allows high availability but requires application-level conflict resolution (CRDTs, last-write-wins).
- Consistency models spectrum: linearizability > sequential consistency > causal consistency > PRAM consistency > eventual consistency. Each step weaker = higher availability and lower latency.

#### Coding Exercises

1. Implement a linearizable key-value store in Rust using tokio: a single server with a `Mutex`-protected `HashMap`. Show that it is linearizable by construction (the mutex ensures operations are totally ordered).
2. Implement an eventually consistent key-value store: multiple replicas with gossip-based replication. Implement last-write-wins conflict resolution using vector clocks. Show a scenario where two clients see different states.
3. Read the Jepsen test reports ([jepsen.io](https://jepsen.io)) for CockroachDB, MongoDB, and Cassandra. For each, identify which consistency model was violated and which operations triggered the bug.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *Brewer's Conjecture and the Feasibility of Consistent, Available, Partition-Tolerant Web Services* — Gilbert & Lynch 2002. The formal CAP proof. Read the statement and the main theorem. |
| PAPER | *Spanner: Google's Globally Distributed Database* — Corbett et al. OSDI 2012. The most important distributed database paper. Read sections 1–4 before distributed transactions. |
| BOOK | *Designing Data-Intensive Applications* — Kleppmann ch 8–9. The clearest explanation of distributed systems fundamentals written for practitioners. |

---

### L26 — Consensus — Paxos, Raft, Multi-Paxos

**What you learn:** Implementing fault-tolerant consensus: Paxos (single-decree), Multi-Paxos (repeated consensus for a log), and Raft (understandable Multi-Paxos). The foundation of every fault-tolerant distributed system.

#### Key Concepts

- Paxos single-decree: three roles (proposer, acceptor, learner). Phase 1 (Prepare/Promise): proposer sends Prepare(n), acceptors promise not to accept lower-numbered proposals. Phase 2 (Accept/Accepted): proposer sends Accept(n,v), acceptors accept if n is highest seen.
- Safety in Paxos: two conflicting values cannot both be chosen. If a value is chosen, any future chosen value is the same. Proven by the quorum intersection property: any two majorities share at least one acceptor.
- Multi-Paxos: skip Phase 1 for subsequent log entries once a leader is established. Leader leases: the leader promises not to be replaced for a lease period, allowing it to serve reads without Phase 1.
- Raft: leader election (election timeout, vote request, majority wins), log replication (leader appends to log, replicates to followers, commits when a majority acknowledges), and leader change (new leader reconciles log).
- Raft log consistency: the Log Matching Property: if two logs have an entry with the same index and term, all previous entries are identical. Enforced by the AppendEntries consistency check.
- Reconfiguration: adding or removing servers from a Raft cluster. Joint consensus: the cluster transitions through a state where both the old and new configurations must agree before the new configuration takes effect.

#### Coding Exercises

1. Implement single-decree Paxos in Rust using tokio channels to simulate message passing between acceptors. Verify safety: run 100 concurrent proposers proposing different values — only one should be chosen.
2. Implement Raft using the `raft-rs` crate (Rust) or implement from scratch following the Raft paper. Implement leader election, log replication, and log commitment. Write a 3-node cluster test.
3. Implement Raft log compaction (snapshotting): when the log grows beyond a threshold, take a snapshot of the state machine and truncate the log. Implement snapshot transfer to a lagging follower.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *In Search of an Understandable Consensus Algorithm (Raft)* — Ongaro & Ousterhout USENIX ATC 2014. The Raft paper. Read with the extended version from Stanford. Implement your consensus from this. |
| PAPER | *Paxos Made Simple* — Lamport 2001. The clearest Paxos explanation by Lamport himself. 11 pages. Read before or after Raft for contrast. |
| IMPL | [PingCAP TinyKV](https://github.com/pingcap/tinykv) — A guided Rust/Go implementation of a Raft-based KV store. Follows the TiKV architecture. |

---

### L27 — Distributed Transactions — 2PC, Percolator, Spanner

**What you learn:** Committing transactions that span multiple shards: Two-Phase Commit (2PC), Percolator's MVCC-over-Bigtable, Spanner's TrueTime, and CockroachDB's hybrid logical clocks.

#### Key Concepts

- Two-Phase Commit (2PC): coordinator sends Prepare to all participants. If all vote YES, coordinator sends Commit. If any votes NO or times out, coordinator sends Abort. Participants are blocked during the coordinator's failure.
- 2PC blocking problem: if the coordinator crashes after participants vote YES but before they receive Commit/Abort, participants are blocked indefinitely. Three-Phase Commit (3PC) avoids this but has higher latency.
- Percolator (Google): MVCC-based distributed transactions over Bigtable. Writes are staged in a 'write intent' column. Commit is a two-phase lock (primary lock first, then secondary). Reads check for write intents and may block.
- Spanner TrueTime: GPS and atomic clocks provide a global timestamp with bounded uncertainty (typically ±7ms). Spanner waits out the uncertainty window before committing to ensure external consistency.
- Hybrid Logical Clocks (HLC): combine physical clocks (for low latency) with logical clocks (for causal ordering). Used in CockroachDB. HLC timestamps are always >= wall clock time and causally ordered.
- Coordinator placement: co-locate the 2PC coordinator with the transaction's primary shard to reduce one round-trip in the common case. Used in CockroachDB and TiDB.

#### Coding Exercises

1. Implement 2PC over your Raft-based KV store: the coordinator is a separate process. Shard participants expose Prepare, Commit, and Abort RPCs. Test with 3 shards and a coordinator crash recovery scenario.
2. Implement Percolator-style MVCC distributed transactions: each key has a data column, a lock column, and a write column. Implement the transaction commit protocol (lock primary, lock secondaries, commit primary, async commit secondaries).
3. Implement HLC in Rust: a timestamp is `(physical_ms, logical)`. On send: `timestamp = max(local_HLC, msg_HLC) + 1`. On receive: update local HLC. Verify causality is preserved in a 3-node network simulation.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *Large-Scale Incremental Processing Using Distributed Transactions and Notifications (Percolator)* — Peng & Dabek OSDI 2010. The Percolator paper. Defines the MVCC distributed transaction protocol used by TiDB. |
| PAPER | *Spanner: Google's Globally Distributed Database* — Corbett et al. OSDI 2012. Read sections 3–5 on TrueTime and external consistency. |
| PAPER | *CockroachDB: The Resilient Geo-Distributed SQL Database* — Taft et al. SIGMOD 2020. How HLC-based distributed transactions work in a real production system. |

---

### L28 — Sharding, Replication & Geo-Distribution

**What you learn:** Horizontal partitioning strategies (range, hash, consistent hashing), replication topologies (primary-backup, multi-primary, Paxos groups), and geo-distributed architectures (Spanner, CockroachDB, Vitess).

#### Key Concepts

- Range sharding: partition keys into contiguous ranges. Simple and enables efficient range scans. Hotspots on sequential inserts (e.g., auto-increment IDs). Used in HBase, Bigtable, TiKV.
- Hash sharding: `hash(key) mod N` determines the shard. Uniform distribution, no hotspots on sequential inserts. Range scans require scatter-gather across all shards. Used in Cassandra, DynamoDB.
- Consistent hashing: place shards on a ring. A key maps to the next clockwise shard. Adding/removing a shard rebalances only the adjacent keys. Used in Cassandra and distributed caches.
- Virtual nodes (vnodes): each physical server owns multiple virtual nodes on the consistent hash ring. Adding a server splits its vnodes across the new server. Enables granular load balancing.
- Primary-backup replication: one primary processes writes, one or more backups replicate asynchronously or synchronously. Synchronous replication adds latency but guarantees no data loss on failover.
- Geo-distribution challenges: round-trip latency between data centers is 50–150ms. Use Paxos with multi-region quorums (Spanner), follower reads from a local region (CockroachDB), or async replication with conflict resolution (DynamoDB).

#### Coding Exercises

1. Implement consistent hashing in Rust: a `BTreeMap` ring with virtual nodes. Implement `add_server` (add K vnodes), `remove_server` (remove vnodes), and `get_server(key)`. Verify even distribution.
2. Implement a sharded key-value store in Rust: 3 shards, each a separate tokio task with an `mpsc` channel. A router hashes keys to shards. Implement cross-shard reads and scatter-gather range scans.
3. Implement primary-backup replication: the primary applies writes and sends them to backups. The backup responds ACK. The primary commits only after a quorum of backups ACK (synchronous replication). Measure latency overhead.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *Dynamo: Amazon's Highly Available Key-Value Store* — DeCandia et al. SOSP 2007. The consistent hashing paper. Defines the architecture that Cassandra is based on. |
| PAPER | *Bigtable: A Distributed Storage System for Structured Data* — Chang et al. OSDI 2006. Range-sharded storage. The foundation for HBase and TiKV's storage model. |
| BOOK | *Designing Data-Intensive Applications* — Kleppmann ch 5–6. Replication and partitioning chapters. The clearest treatment of these topics. |

---

# Phase 5 — Specialized Database Systems

**Vector DBs · Graph DBs · Time-Series · Stream Processing · In-Memory OLTP**

**Goal:** Understand the storage and query engine design decisions that distinguish specialized databases from general-purpose relational systems. Each section ends with a concrete implementation project.

---

### L29 — Vector Databases — HNSW, IVF, PQ, FAISS

**What you learn:** Approximate nearest-neighbor (ANN) search over high-dimensional embedding vectors: the algorithms, data structures, and tradeoffs that underlie Pinecone, Weaviate, pgvector, and Milvus.

#### Key Concepts

- Why vector databases: LLM embeddings (text, images, code) are high-dimensional vectors (768–4096 dimensions). Exact nearest-neighbor search is O(N*d) — infeasible for billions of vectors. Need approximate algorithms.
- IVF (Inverted File Index): cluster vectors into K clusters using k-means. At query time, search only the nearest nprobe clusters. Reduces search space from N to N/K * nprobe. Simple, predictable performance.
- HNSW (Hierarchical Navigable Small World): a layered proximity graph. Layer 0 has all nodes. Higher layers have fewer nodes (sampled probabilistically). Search starts at the top layer and greedily descends. O(log N) search.
- Product Quantization (PQ): compress each vector by dividing it into M subspaces, quantizing each subspace to its nearest centroid (codebook of 256 entries). 4-byte vector requires only M bytes. Enables billion-scale search in RAM.
- Recall vs latency tradeoff: increasing ef_search (HNSW) or nprobe (IVF) improves recall at the cost of search time. Production systems target recall@10 ≥ 0.95 at 1–10ms latency.
- Filtered ANN search: search for nearest neighbors that satisfy a predicate (e.g., WHERE category = 'sports'). Approach 1: post-filter (search, then filter — misses recall targets on low-selectivity predicates). Approach 2: pre-filter (filter first, build a sub-index — expensive). Approach 3: graph-based filtered search.

#### Coding Exercises

1. Implement a flat (brute-force) vector index in Rust: given a `Vec<Vec<f32>>` and a query `Vec<f32>`, return the top-K nearest neighbors by cosine similarity using SIMD dot products.
2. Implement IVF: train K=100 centroids using mini-batch k-means on 1M vectors. Build inverted lists. Implement query with nprobe=10. Measure recall@10 vs the flat index and search latency.
3. Implement HNSW from scratch in Rust following the paper: layered graph construction, greedy search, and ef_construction parameter. Benchmark against the hnswlib C++ library's Rust bindings on 1M 128-dim vectors.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *Efficient and Robust Approximate Nearest Neighbor Search Using Hierarchical Navigable Small World Graphs* — Malkov & Yashunin 2018. The HNSW paper. Read and implement from this. |
| PAPER | *Billion-Scale Similarity Search with GPUs* — Johnson et al. 2019. The FAISS paper. Covers IVF, PQ, and GPU-accelerated search. |
| IMPL | [usearch](https://github.com/unum-cloud/usearch) — fastest Rust/C++ ANN library. Study the HNSW implementation. |

---

### L30 — Graph Databases — Property Graphs, Cypher, Storage

**What you learn:** Graph database data models (property graphs vs RDF), storage strategies (adjacency lists, CSR, edge-partitioned), Cypher query language, and graph-native query engines.

#### Key Concepts

- Property graph model: nodes and edges each have a type (label) and a key-value property map. Edges are directed with a type. Example: `(Person)-[:KNOWS {since: 2020}]->(Person)`.
- RDF and SPARQL: triples (subject, predicate, object). Subject and object are URIs or literals. SPARQL matches patterns over triples. Used in knowledge graphs (Wikidata, DBpedia).
- Adjacency list storage: each node stores a list of outgoing edge IDs. Simple but poor cache behavior for traversal (following edges requires random memory access).
- Compressed Sparse Row (CSR): store all edges sorted by source node. The row offset array gives the start of each node's edge list. Excellent cache behavior for graph traversal — used in GraphBLAS and high-performance graph analytics.
- Edge-partitioned storage: partition the graph across machines by edge. Each machine stores a subset of edges. Used in PowerGraph (GAS model) for distributed graph analytics.
- Index-free adjacency: each node object directly stores pointers to its adjacent nodes (no index lookup needed). Used in Neo4j. Makes traversal O(1) per hop but makes full scans expensive.

#### Coding Exercises

1. Implement a property graph store in Rust: `Node { id: u64, labels: Vec<String>, properties: HashMap<String, Value> }` and `Edge { id: u64, src: u64, dst: u64, label: String, properties: HashMap<String, Value> }`. Support traversal from a node.
2. Implement CSR format for a directed graph: convert an edge list to CSR (`row_offsets: Vec<usize>`, `col_indices: Vec<usize>`). Implement BFS and PageRank on the CSR representation. Benchmark on the SNAP Twitter graph (80M edges).
3. Implement a simple Cypher-like query engine: parse `MATCH (a)-[r:KNOWS]->(b) WHERE a.name = 'Alice'` and execute it over your property graph store using DFS with predicate filtering.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *The Property Graph Database Model* — Angles & Gutierrez 2018. Formal definition of the property graph model. Short survey. |
| PAPER | *GraphBLAS: A Linear Algebra Approach to Graph Computation* — Mattson et al. 2013. Using sparse matrix operations for graph algorithms. The basis of high-performance graph analytics. |
| IMPL | [Oxigraph](https://github.com/oxigraph/oxigraph) — a complete Rust RDF/SPARQL library. Study the triple store and SPARQL engine. |

---

### L31 — Time-Series Databases — InfluxDB, TimescaleDB, Prometheus

**What you learn:** Time-series data characteristics (high ingest rate, time-ordered, rarely updated, frequently aggregated by time window) and the storage engine designs optimized for them.

#### Key Concepts

- Time-series data model: (metric_name, tags, timestamp, value). Tags define the time series identity. Timestamps are strictly increasing per series. Values are floats or integers.
- Columnar storage for time-series: store timestamps in one column, values in another. Delta-delta encoding for timestamps (differences of differences compress to near-zero for regular intervals). Gorilla XOR encoding for float values.
- Gorilla encoding (Facebook): store the XOR of consecutive float values. Since consecutive sensor readings are similar, the XOR has many leading zeros — use a 2-bit prefix code to compress. 1.37 bytes per value average.
- Downsampling and retention policies: automatically roll up raw data into hourly/daily aggregates after a retention period. Reduces storage by 100–1000x. Must be handled at the storage engine level.
- Time-partitioned storage: split tables by time range (hourly or daily partitions). Enables fast deletion of old data (drop a partition), fast time-range scans (skip partitions outside the range), and independent compression per partition.
- Continuous queries and materialized aggregations: pre-compute common aggregations (1-min avg, 5-min avg) in the background. Store results in separate compressed tables. Query the pre-computed tables instead of raw data.

#### Coding Exercises

1. Implement Gorilla float compression in Rust: given a `Vec<f64>` time series, compute XOR of consecutive values, count leading/trailing zeros, and pack into a bit stream. Measure the compression ratio on real sensor data.
2. Implement a time-partitioned storage engine: table is split into hourly chunks (a `HashMap`). Insert routes to the current chunk. Range queries scatter to the relevant chunks and merge results.
3. Implement delta-delta encoding for timestamps: given `Vec<i64>` timestamps (Unix ms), compute first deltas, then second deltas, then zigzag-encode and varint-pack. Measure compression ratio vs raw storage.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *Gorilla: A Fast, Scalable, In-Memory Time Series Database* — Pelkonen et al. VLDB 2015. Facebook's time-series DB paper. Defines the Gorilla encoding and the in-memory architecture. |
| PAPER | *TimescaleDB: Time-Series Database Design Challenges and Solutions* — Freedman et al. 2018. How TimescaleDB extends PostgreSQL for time-series workloads. |
| BLOG | [How Prometheus TSDB Works](https://fabxc.org/tsdb) — Fabian Reinartz. The most detailed explanation of Prometheus's time-series storage engine. Essential. |

---

### L32 — Stream Processing — Kafka Streams, Flink, Materialize

**What you learn:** Continuous query processing over unbounded streams of events: windowing, watermarks, out-of-order handling, exactly-once semantics, and incremental view maintenance.

#### Key Concepts

- Stream processing model: a potentially infinite sequence of events, each with a timestamp. The system processes events as they arrive and maintains continuously updated results.
- Windowing: tumbling windows (non-overlapping fixed-size time periods), sliding windows (overlapping, emit every T seconds with width W), session windows (group events within a session gap threshold).
- Watermarks: a watermark at time T means 'no event with timestamp < T will arrive in the future'. The system emits window results when the watermark passes the window's end time. Late events trigger retractions.
- Exactly-once semantics: each event affects the output exactly once. Requires idempotent sinks, distributed snapshots (Flink's Chandy-Lamport checkpointing), and transactional Kafka producers/consumers.
- Incremental view maintenance (IVM): maintain a materialized view of a SQL query over a changing table. When a row is inserted or deleted, compute the delta to the view and apply it. Used in Materialize and DeltaStream.
- Dataflow model: a query is a directed acyclic graph of operators. Events flow through the graph. Each operator is stateful (windowed aggregation) or stateless (filter, projection). Flink and Differential Dataflow use this model.

#### Coding Exercises

1. Implement a tumbling window aggregation in Rust: process a stream of `(timestamp, value)` events using tokio channels. Emit the sum of each 10-second window when the window closes.
2. Implement watermark-based late event handling: events may arrive up to 5 seconds late. Buffer events in a priority queue ordered by timestamp. When a watermark arrives, emit all events behind the watermark.
3. Implement incremental view maintenance for a simple `COUNT(*)` query: given inserts and deletes to a base table, maintain a running count without re-scanning the table. Generalize to SUM and AVG.

#### Papers & Resources

| Type | Resource |
|------|----------|
| PAPER | *The Dataflow Model: A Practical Approach to Balancing Correctness, Latency, and Cost* — Akidau et al. VLDB 2015. The Google Dataflow paper. Defines the windowing and watermark model used by Flink. |
| PAPER | *Naiad: A Timely Dataflow System* — Murray et al. SOSP 2013. Differential dataflow for incremental computation. The theoretical basis of Materialize. |
| VIDEO | Flink Architecture Deep Dive — Stephan Ewen. Search 'Flink architecture Stephan Ewen' on YouTube. The best 45-minute Flink internals talk. |

---

# Consolidated Resources

All books, papers, implementations, and tools referenced in this roadmap.

## Books

| Resource | Description |
|----------|-------------|
| *Database System Concepts* — Silberschatz, Korth, Sudarshan | The standard textbook for database theory. Read chapters 1–6 (relational model), 12–14 (storage), 17–19 (transactions). |
| *Transaction Processing: Concepts and Techniques* — Gray & Reuter 1992 | The definitive reference on transactions, locking, and recovery. Read chapters 1–4 and 7–10. |
| *Designing Data-Intensive Applications* — Martin Kleppmann | The clearest systems-level treatment of databases, replication, and consistency. Read all of it. |
| *Database Internals* — Alex Petrov 2019 | Implementation-focused treatment of B-trees, LSM-trees, and distributed consensus. Pairs perfectly with this roadmap. |
| *Readings in Database Systems (Red Book)* — Hellerstein & Stonebraker | A curated collection of the most important database papers with editorial commentary. Free online at [redbook.io](https://redbook.io). |

## Foundational Papers — Read in Order

| Paper | Notes |
|-------|-------|
| *A Relational Model of Data for Large Shared Data Banks* — Codd 1970 | The paper that started it all. ACM DL. |
| *ARIES: A Transaction Recovery Method* — Mohan et al. ACM TODS 1992 | The crash recovery algorithm used by every major DBMS. |
| *The Log-Structured Merge-Tree* — O'Neil et al. 1996 | LSM-tree definition. Prerequisite for understanding RocksDB, Cassandra, InfluxDB. |
| *C-Store: A Column-Oriented DBMS* — Stonebraker et al. VLDB 2005 | The paper that launched columnar databases. Prerequisite for Phase 3. |
| *Bigtable: A Distributed Storage System* — Chang et al. OSDI 2006 | Range-sharded distributed storage. Prerequisite for distributed databases. |
| *Dynamo: Amazon's Highly Available Key-Value Store* — DeCandia et al. SOSP 2007 | Consistent hashing and eventual consistency at scale. |
| *Spanner: Google's Globally Distributed Database* — Corbett et al. OSDI 2012 | TrueTime and external consistency. The most important distributed DB paper. |
| *In Search of an Understandable Consensus Algorithm (Raft)* — Ongaro 2014 | Implement Raft. This is the paper. |
| *Efficiently Compiling Efficient Query Plans* — Neumann VLDB 2011 | Query compilation. Read before Phase 2 execution work. |
| *Morsel-Driven Parallelism* — Leis et al. SIGMOD 2014 | NUMA-aware parallel query execution. Read before Phase 2 scheduling work. |

## Implementations — Study and Build Alongside

| Project | Description |
|---------|-------------|
| [BusTub](https://github.com/cmu-db/bustub) — CMU 15-445 reference DBMS | Implement all 4 projects in Rust. The foundation of Phase 1. **This is db-labs.** |
| [MiniLSM](https://skyzh.github.io/mini-lsm) — skyzh | 7-week Rust LSM-tree implementation guide. Complete before Phase 3. |
| [TinyKV](https://github.com/pingcap/tinykv) — PingCAP | Guided Raft-based KV store implementation. Use for Phase 4 consensus. |
| [DuckDB](https://github.com/duckdb/duckdb) | In-process analytical DBMS. Read the storage and execution engine source. Best columnar reference. |
| [RisingWave](https://github.com/risingwavelabs/risingwave) | Production-quality Rust stream processing system. Study for Phase 5. |
| [Oxigraph](https://github.com/oxigraph/oxigraph) | Complete RDF/SPARQL implementation in Rust. |

## CMU Lecture Videos — All Free on YouTube

| Course | Link |
|--------|------|
| CMU 15-445 — Intro to Database Systems (all semesters) | [youtube.com/@CMUDatabaseGroup](https://youtube.com/@CMUDatabaseGroup) — watch the most recent semester. Pavlo's lectures are the best freely available DBMS content. |
| CMU 15-721 — Advanced Database Systems Spring 2024 | [YouTube playlist](https://youtube.com/playlist?list=PLSE8ODhjZXjYa_zX-KeMJui7pcN1rIaIJ) — paper-per-lecture graduate course. Watch after completing Phase 1. |
| Andy Pavlo — Database Systems talks (VLDB, SIGMOD) | Search 'Andy Pavlo VLDB' on YouTube. His conference talks are denser and more current than the course lectures. |

## Tools & Benchmarks

| Tool | Description |
|------|-------------|
| [TPC-H benchmark](https://tpc.org/tpch) | The standard OLAP benchmark. Generate data with tpch-dbgen. Run on your column store. |
| [TPC-C benchmark](https://tpc.org/tpcc) | The standard OLTP benchmark. Run on your B-tree KV store. |
| [YCSB](https://github.com/brianfrankcooper/YCSB) | The standard KV store benchmark. 6 workload presets. Run on your LSM-tree. |
| [Jepsen](https://jepsen.io) | Distributed systems correctness testing. Test your distributed KV store for consistency violations. Install and run against your Phase 4 system. |
| perf + flamegraph | [brendangregg.com/flamegraphs.html](https://brendangregg.com/flamegraphs.html) — profile your query execution hot paths in Phase 2. `cargo-flamegraph` for Rust. |
| pprof + criterion.rs | [docs.rs/criterion](https://docs.rs/criterion) — use for all micro-benchmarks in Phases 1–3. Always compare before and after optimizations. |

---

# Final Note

The progression through these five phases maps to how production database engineers think:

- **Phase 1 (15-445):** you understand how any disk-oriented DBMS works internally. You can read the PostgreSQL or SQLite source and understand every subsystem.
- **Phase 2 (15-721):** you understand why OLAP systems are a different class of problem from OLTP, and what the state of the art looks like. You can contribute to DuckDB, ClickHouse, or Velox.
- **Phase 3 (Storage Engines):** you understand the tradeoffs between B-tree and LSM-tree designs. You can evaluate RocksDB vs BoltDB vs LMDB for a given workload and justify your choice quantitatively.
- **Phase 4 (Distributed):** you understand how Spanner, CockroachDB, and TiDB achieve the consistency guarantees they advertise. You can debug Jepsen test failures and understand what went wrong.
- **Phase 5 (Specialized):** you can evaluate vector, graph, and time-series databases against a relational alternative and decide when a specialized system is warranted and when it is over-engineering.

## What to Build After This Roadmap

- Contribute to [RisingWave](https://github.com/risingwavelabs/risingwave) (Rust streaming DBMS) or [Databend](https://github.com/databendlabs/databend) (Rust cloud data warehouse). Both are actively maintained production Rust database systems.
- Implement a simple distributed SQL layer over your Raft-based KV store from Phase 4 — this is exactly how TiDB is built over TiKV.
- Implement a columnar OLAP engine with LLVM query compilation (`inkwell`) — combine your Phase 2 knowledge with your compiler roadmap.
- Read *Database Internals* (Petrov) cover to cover — it is the best implementation-focused database book and will tie everything together.

---

> **Implement in Rust. Benchmark everything. The numbers don't lie.**

---

## db-labs Project Milestones

Track progress against the four BusTub projects that anchor Phase 1:

| Project | Component | Lectures | Status |
|---------|-----------|----------|--------|
| **#1** | Disk Manager + Buffer Pool Manager (LRU-K) | L03, L05 | ☐ Not started |
| **#2** | B+ Tree Index (with concurrency) | L07, L08 | ☐ Not started |
| **#3** | Query Executors (Volcano model) | L09, L10 | ☐ Not started |
| **#4** | Lock Manager + Deadlock Detection | L13 | ☐ Not started |

Additional Phase 1 components to integrate:

| Component | Lectures | Status |
|-----------|----------|--------|
| ARIES Recovery / WAL | L16 | ☐ Not started |
| SQL Parser + Optimizer | L02, L11 | ☐ Not started |
| MVCC / Timestamp Ordering | L14, L15 | ☐ Not started |
| Hash Indexes | L06 | ☐ Not started |
| LSM-Tree (MiniLSM) | L04 | ☐ Not started |
