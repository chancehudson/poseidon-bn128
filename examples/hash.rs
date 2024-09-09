use poseidon_bn128::poseidon;
use scalarff::Bn128FieldElement;
use scalarff::FieldElement;
use anyhow::Result;

fn main() -> Result<()>{
    println!("{}", poseidon(2, &[Bn128FieldElement::zero(), Bn128FieldElement::one()])?);
    Ok(())
}
