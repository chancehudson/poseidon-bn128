use anyhow::Result;
use poseidon_bn128::poseidon;
use scalarff::Bn128FieldElement;

fn main() -> Result<()> {
    let inputs = vec![Bn128FieldElement::from(99), Bn128FieldElement::from(100)];
    let result = poseidon(2, &inputs)?;
    println!("{:?}", result);
    Ok(())
}
