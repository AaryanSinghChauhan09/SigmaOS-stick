# SigmaOS Stick

This crate exports the `StickManager` API from its Mintstick module. The
current manager returns hard-coded USB device examples, records write/format
requests in memory, and returns a placeholder checksum. It does not enumerate
devices, write an image, format a disk, or calculate a real checksum.

Do not treat a successful request result as evidence that a block device was
changed. Hardware discovery, privileged write safeguards, cancellation of an
in-flight operation, and checksum verification need implementation and direct
device or isolated-image evidence.

The crate depends on the sibling `SigmaOS-kernel` checkout through a local path.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).
