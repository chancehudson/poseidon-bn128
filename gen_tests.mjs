// run `npm i poseidon-lite`
// then run `node gen_tests.mjs > test_hashes.json`
//
import poseidons from "poseidon-lite";

const hashes = [];
// generate 100 hashes for each T value
// to compare with the rust implementation
for (let x = 1; x <= 16; x++) {
  const h = [];
  for (let y = 0; y < 100; y++) {
    h.push(
      "0x" + poseidons[`poseidon${x}`](Array(x).fill(BigInt(y))).toString(16),
    );
  }
  hashes.push(h);
}
console.log(JSON.stringify(hashes));
