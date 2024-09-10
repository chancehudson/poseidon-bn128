# poseidon-bn128 [![Build](https://img.shields.io/circleci/build/github/chancehudson/poseidon-bn128/main)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/poseidon-bn128/tree/main) [![Docs](https://img.shields.io/docsrs/poseidon-bn128)](https://docs.rs/poseidon-bn128) [![Version](https://img.shields.io/crates/v/poseidon-bn128)](https://crates.io/crates/poseidon-bn128)

Poseidon over alt_bn128 compatible with circomlib. Uses [scalarff](https://crates.io/crates/scalarff) for representing field elements.

## Example

`cargo add poseidon-bn128`

```rust
use poseidon_bn128::poseidon;
use scalarff::Bn128FieldElement;
use anyhow::Result;

fn main() -> Result<()> {
    let inputs = vec![
        Bn128FieldElement::from(99),
        Bn128FieldElement::from(100)
    ];
    let result = poseidon(2, &inputs)?;
    println!("{:?}", result);
    Ok(())
}
```
