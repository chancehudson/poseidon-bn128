use std::fs::File;

use scalarff::Bn128FieldElement;
use scalarff::FieldElement;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use num_bigint::BigUint;
use num_traits::Num;

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

pub fn poseidon2(input: &[Bn128FieldElement]) -> Result<Bn128FieldElement> {
    if input.len() != 2 {
        panic!("expected 2 inputs");
    }
    let params = read_constants(2)?;
    let num_full_rounds = 8;
    let num_partial_rounds = 57;
    let t = input.len() + 1;

    let mut state = [Bn128FieldElement::zero()].iter().chain(input).map(|v| v.clone()).collect::<Vec<Bn128FieldElement>>();

    for x in 0..(num_full_rounds + num_partial_rounds) {
        for y in 0..state.len() {
            state[y] = state[y] + params.C[x * t + y];
            if x < num_full_rounds / 2 || x >= num_full_rounds/2 + num_partial_rounds {
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
pub fn read_constants(t: u32) -> Result<PoseidonParams> {
    let f = File::open(format!("./src/params-json/{}.json", t))?;
    let params: PoseidonParamsSerialized = serde_json::from_reader(f)?;
    let out_params = PoseidonParams {
        C: params.C.iter().map(|x| {
            // TODO: clean this up :(
            Ok(Bn128FieldElement::from_biguint(&BigUint::from_str_radix(&x[2..], 16)?))
        }).collect::<Result<_>>()?,
        M: params.M.iter().map(|internal| {
            internal.iter().map(|x| {
                Ok(Bn128FieldElement::from_biguint(&BigUint::from_str_radix(&x[2..], 16)?))
            }).collect::<Result<_>>()
        }).collect::<Result<_>>()?,
    };
    Ok(out_params)
}
