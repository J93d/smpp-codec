# Performance Guide

The `smpp-codec` library is designed to be flexible and efficient, decoupling **what** is encoded from **how** it is written. This gives you strict control over allocation and system calls.

## Encoding & Buffering

All PDU structures implement an `encode` method that accepts any type implementing `std::io::Write`.

```rust
pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError>
```

Your choice of `writer` dramatically impacts performance.

### 1. In-Memory Buffer (Recommended for most cases)
Writing to a `Vec<u8>` is extremely fast because it involves only memory operations (no system calls).

```rust
let mut buffer = Vec::with_capacity(512); // Pre-allocate to avoid re-sizing
pdu.encode(&mut buffer)?; // Fast memory copy
stream.write_all(&buffer)?; // ONE system call to send entire PDU
```

### 2. Buffered Writer (`BufWriter`)
If you prefer not to manage a `Vec` manually, wrap your stream in a `BufWriter`. This handles the buffering for you.

```rust
use std::io::BufWriter;

let mut writer = BufWriter::new(tcp_stream);
pdu.encode(&mut writer)?; // Writes to internal buffer (fast)
writer.flush()?;          // Flushes buffer to network (ONE system call)
```

### 3. Raw Stream (⚠️ Performance Warning)
**Avoiding this pattern is critical for high-throughput applications.**

If you pass a raw `TcpStream` (or File) directly to `encode`, the library will make a separate system call for every field it writes.

```rust
// ❌ INEFFICIENT
pdu.encode(&mut tcp_stream)?; 
```

**Why is this slow?**
`SubmitSmRequest` has ~20 fields. A raw encode results in ~20 tiny system calls. Context switching for each call kills throughput.

## Memory Reusage

For maximum performance in a loop, reuse your `Vec<u8>` to avoid repeated heap allocations.

```rust
let mut buffer = Vec::with_capacity(2048);

for pdu in pdus {
    buffer.clear(); // Resets length but keeps capacity
    pdu.encode(&mut buffer)?;
    stream.write_all(&buffer)?;
}
```
