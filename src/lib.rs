use std::fs::File;

use anyhow::Result;
use num_bigint::BigUint;
use num_traits::Num;
use scalarff::Bn128FieldElement;
use scalarff::FieldElement;
use serde::{Deserialize, Serialize};

/// Representation for use with serde
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PoseidonParamsSerialized {
    pub C: Vec<String>,
    pub M: Vec<Vec<String>>,
}

/// Representation for use with the poseidon logic
pub struct PoseidonParams {
    pub c: Vec<Bn128FieldElement>,
    pub m: Vec<Vec<Bn128FieldElement>>,
    pub num_full_rounds: usize,
    pub num_partial_rounds: usize,
}

fn pow5(v: Bn128FieldElement) -> Bn128FieldElement {
    let square = v * v;
    let quad = square * square;
    quad * v
}

fn mix(state: Vec<Bn128FieldElement>, params: &PoseidonParams) -> Vec<Bn128FieldElement> {
    let mut out = vec![];
    for i in 0..state.len() {
        let mut o = Bn128FieldElement::zero();
        #[allow(clippy::needless_range_loop)]
        for j in 0..state.len() {
            o += params.m[i][j] * state[j];
        }
        out.push(o);
    }
    out
}

pub fn poseidon(input_count: u8, input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    if input.len() != usize::from(input_count) {
        anyhow::bail!("expected {} inputs, received {}", input_count, input.len());
    }
    // constants are stored by number of inputs
    let params = read_constants(input_count)?;
    let t = usize::try_from(input_count + 1)?;

    let mut state = [Bn128FieldElement::zero()]
        .iter()
        .chain(input)
        .copied()
        .collect::<Vec<Bn128FieldElement>>();

    for x in 0..(params.num_full_rounds + params.num_partial_rounds) {
        #[allow(clippy::needless_range_loop)]
        for y in 0..state.len() {
            state[y] += params.c[x * t + y];
            if y == 0
                || x < params.num_full_rounds / 2
                || x >= params.num_full_rounds / 2 + params.num_partial_rounds
            {
                state[y] = pow5(state[y]);
            }
        }
        state = mix(state, &params);
    }
    Ok(state[0])
}

/// Read the constants from file and parse into field elements
pub fn read_constants(input_count: u8) -> Result<PoseidonParams> {
    let f = File::open(format!("./src/params-json/{}.json", input_count))?;
    let params: PoseidonParamsSerialized = serde_json::from_reader(f)?;
    let partial_round_counts = [
        56, 57, 56, 60, 60, 63, 64, 63, 60, 66, 60, 65, 70, 60, 64, 68,
    ];
    // TODO: move this into scalarff?
    let hex_str_to_field_element = |x: &String| {
        Ok(Bn128FieldElement::from_biguint(&BigUint::from_str_radix(
            &x[2..],
            16,
        )?))
    };
    Ok(PoseidonParams {
        num_full_rounds: 8,
        num_partial_rounds: partial_round_counts[usize::from(input_count) - 1],
        c: params
            .C
            .iter()
            .map(hex_str_to_field_element)
            .collect::<Result<_>>()?,
        m: params
            .M
            .iter()
            .map(|internal| internal.iter().map(hex_str_to_field_element).collect())
            .collect::<Result<_>>()?,
    })
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::time::Instant;

    use anyhow::Result;
    use scalarff::Bn128FieldElement;
    use scalarff::FieldElement;

    #[test]
    fn compare_hashes() -> Result<()>{
        let f = File::open(format!("./src/test_hashes.json"))?;
        let expected: Vec<Vec<String>> = serde_json::from_reader(f)?;
        for i in 0..expected.len() {
            let input_count = u8::try_from(i + 1)?;
            let hash_count = expected[i].len();
            let start = Instant::now();
            for j in 0..hash_count {
                let hash = super::poseidon(input_count, &vec![Bn128FieldElement::from(u64::try_from(j)?); usize::from(input_count)])?;
                assert_eq!(hash.to_biguint().to_str_radix(16), expected[i][j][2..]);
            }
            let elapsed = start.elapsed();
            println!("Calculated {hash_count} poseidon{input_count} hashes in: {:.2?}", elapsed);
        }
        Ok(())
    }
}
