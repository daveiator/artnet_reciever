# artnet_reciever &emsp; [![Latest Release][crates-io-badge]][crates-io-url] [![Documentation][docs-rs-img]][docs-rs-url] [![License][license-badge]]()

[crates-io-badge]: https://img.shields.io/crates/v/artnet_reciever.svg?style=for-the-badge
[crates-io-url]: https://crates.io/crates/artnet_reciever
[docs-rs-img]: https://img.shields.io/docsrs/artnet_reciever?style=for-the-badge
[docs-rs-url]: https://docs.rs/artnet_reciever
[license-badge]: https://img.shields.io/crates/l/artnet_reciever.svg?style=for-the-badge

 ### A simple artnet reciever based on the [artnet_protocol](https://crates.io/crates/artnet_protocol) crate


## Usage

```rust
use artnet_reciever::ArtnetRecieverBuilder;

fn main() {
    let reciever = ArtnetRecieverBuilder::default().build().unwrap();
    for packet in reciever {
        println!("{:?}", packet);
    }
}
```