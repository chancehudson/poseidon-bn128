use std::fs::File;

use anyhow::Result;
use num_bigint::BigUint;
use num_traits::Num;
use scalarff::Bn128FieldElement;
use scalarff::FieldElement;
use serde::{Deserialize, Serialize};

/// Representation for use with serde
#[derive(Debug, Serialize, Deserialize)]
pub struct PoseidonParamsSerialized {
    pub C: Vec<String>,
    pub M: Vec<Vec<String>>,
}

/// Representation for use with the poseidon logic
pub struct PoseidonParams {
    pub C: Vec<Bn128FieldElement>,
    pub M: Vec<Vec<Bn128FieldElement>>,
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
        for j in 0..state.len() {
            o = o + params.M[i][j] * state[j];
        }
        out.push(o);
    }
    out
}

/// Internal implementation of poseidon
/// Invoke this function using the public functions below
/// e.g. poseidon2(&[Bn128FieldElement::zero(), Bn128FieldElement::one()])
fn poseidon(input: &[Bn128FieldElement], t: u8) -> Result<Bn128FieldElement> {
    if input.len() != usize::try_from(t - 1)? {
        anyhow::bail!("expected {} inputs, received {}", t - 1, input.len());
    }
    // constants are stored by number of inputs
    let params = read_constants(t - 1)?;
    let t = input.len() + 1;

    let mut state = [Bn128FieldElement::zero()]
        .iter()
        .chain(input)
        .map(|v| v.clone())
        .collect::<Vec<Bn128FieldElement>>();

    for x in 0..(params.num_full_rounds + params.num_partial_rounds) {
        for y in 0..state.len() {
            state[y] = state[y] + params.C[x * t + y];
            if x < params.num_full_rounds / 2
                || x >= params.num_full_rounds / 2 + params.num_partial_rounds
            {
                state[y] = pow5(state[y]);
            } else if y == 0 {
                state[y] = pow5(state[y]);
            }
        }
        state = mix(state, &params);
    }
    Ok(state[0].clone())
}

/// Read the constants from file and parse into field elements
pub fn read_constants(input_count: u8) -> Result<PoseidonParams> {
    let f = File::open(format!("./src/params-json/{}.json", input_count))?;
    let params: PoseidonParamsSerialized = serde_json::from_reader(f)?;
    let partial_round_counts = vec![
        56, 57, 56, 60, 60, 63, 64, 63, 60, 66, 60, 65, 70, 60, 64, 68,
    ];
    let out_params = PoseidonParams {
        num_full_rounds: 8,
        num_partial_rounds: partial_round_counts[usize::try_from(input_count)? - 1],
        C: params
            .C
            .iter()
            .map(|x| {
                // TODO: clean this up :(
                Ok(Bn128FieldElement::from_biguint(&BigUint::from_str_radix(
                    &x[2..],
                    16,
                )?))
            })
            .collect::<Result<_>>()?,
        M: params
            .M
            .iter()
            .map(|internal| {
                internal
                    .iter()
                    .map(|x| {
                        Ok(Bn128FieldElement::from_biguint(&BigUint::from_str_radix(
                            &x[2..],
                            16,
                        )?))
                    })
                    .collect::<Result<_>>()
            })
            .collect::<Result<_>>()?,
    };
    Ok(out_params)
}

// ***************************************
// public interface
//
// generated with
/*
for (let x = 1; x <= 16; x++) {
  console.log(`pub fn poseidon${x}(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, ${x+1})
  }`)
}
*/
// ***************************************

pub fn poseidon1(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 2)
}

pub fn poseidon2(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 3)
}

pub fn poseidon3(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 4)
}

pub fn poseidon4(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 5)
}

pub fn poseidon5(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 6)
}

pub fn poseidon6(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 7)
}

pub fn poseidon7(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 8)
}

pub fn poseidon8(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 9)
}

pub fn poseidon9(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 10)
}

pub fn poseidon10(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 11)
}

pub fn poseidon11(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 12)
}

pub fn poseidon12(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 13)
}

pub fn poseidon13(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 14)
}

pub fn poseidon14(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 15)
}

pub fn poseidon15(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 16)
}

pub fn poseidon16(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    poseidon(input, 17)
}
