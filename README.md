# car-nostd

[![crates.io](https://img.shields.io/crates/v/car-nostd.svg?style=flat-square)](https://crates.io/crates/car-nostd)
[![Released API docs](https://img.shields.io/docsrs/car-nostd?style=flat-square)](https://docs.rs/car-nostd)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/car-nostd?style=flat-square)](../LICENSE-MIT)

General [CAR file](https://ipld.io/specs/transport/car/) support. "CAR" stands
for Content Addressable aRchives. A CAR file typically contains a serialized
representation of an [IPLD DAG](https://docs.ipfs.tech/concepts/merkle-dag/#merkle-directed-acyclic-graphs-dags),
though is general enough to contain arbitrary IPLD blocks.

Currently supports only [v1](https://ipld.io/specs/transport/car/carv1/).

This library was forked from [**iroh-car**](https://github.com/n0-computer/iroh-car) and adapted to work in `[no_std]` builds.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

