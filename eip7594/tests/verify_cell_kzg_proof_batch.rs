use common::collect_test_files;
use serde_::TestVector;
use std::fs;

mod common;

mod serde_ {
    use crate::common::{bytes_from_hex, UnsafeBytes};

    use serde::Deserialize;

    #[derive(Deserialize)]
    struct YamlInput {
        commitments: Vec<String>,
        cell_indices: Vec<u64>,
        cells: Vec<String>,
        proofs: Vec<String>,
    }

    type YamlOutput = bool;

    #[derive(Deserialize)]
    struct YamlTestVector {
        input: YamlInput,
        output: Option<YamlOutput>,
    }

    pub struct TestVector {
        pub commitments: Vec<UnsafeBytes>,
        pub cell_indices: Vec<u64>,
        pub cells: Vec<UnsafeBytes>,
        pub proofs: Vec<UnsafeBytes>,
        pub output: Option<bool>,
    }

    impl TestVector {
        pub fn from_str(yaml_data: &str) -> Self {
            let yaml_test_vector: YamlTestVector = serde_yaml::from_str(yaml_data).unwrap();
            Self::from(yaml_test_vector)
        }
    }

    impl From<YamlTestVector> for TestVector {
        fn from(yaml_test_vector: YamlTestVector) -> Self {
            let commitments = yaml_test_vector
                .input
                .commitments
                .into_iter()
                .map(|commitment| bytes_from_hex(&commitment))
                .collect();
            let cells = yaml_test_vector
                .input
                .cells
                .into_iter()
                .map(|cell| bytes_from_hex(&cell))
                .collect();
            let proofs: Vec<_> = yaml_test_vector
                .input
                .proofs
                .into_iter()
                .map(|proof| bytes_from_hex(&proof))
                .collect();
            let cell_indices = yaml_test_vector.input.cell_indices;

            let output = yaml_test_vector.output;

            Self {
                commitments,
                cell_indices,
                cells,
                proofs,
                output,
            }
        }
    }
}

const TEST_DIR: &str = "../test_vectors/verify_cell_kzg_proof_batch";
#[test]
fn test_verify_cell_kzg_proof_batch() {
    let test_files = collect_test_files(TEST_DIR).unwrap();

    let ctx = rust_eth_kzg::DASContext::default();

    for test_file in test_files {
        let yaml_data = fs::read_to_string(&test_file).unwrap();
        let test = TestVector::from_str(&yaml_data);

        let cells: Result<_, _> = test
            .cells
            .iter()
            .map(Vec::as_slice)
            .map(|v| v.try_into())
            .collect();

        let cells = match cells {
            Ok(cells) => cells,
            Err(_) => {
                assert!(test.output.is_none());
                continue;
            }
        };

        let commitments: Result<_, _> = test
            .commitments
            .iter()
            .map(Vec::as_slice)
            .map(|v| v.try_into())
            .collect();

        let commitments = match commitments {
            Ok(commitments) => commitments,
            Err(_) => {
                assert!(test.output.is_none());
                continue;
            }
        };

        let proofs: Result<_, _> = test
            .proofs
            .iter()
            .map(Vec::as_slice)
            .map(|v| v.try_into())
            .collect();

        let proofs = match proofs {
            Ok(proofs) => proofs,
            Err(_) => {
                assert!(test.output.is_none());
                continue;
            }
        };

        match ctx.verify_cell_kzg_proof_batch(commitments, test.cell_indices, cells, proofs) {
            Ok(_) => {
                // We arrive at this point if the proof verified as true
                assert!(test.output.unwrap())
            }
            Err(x) if x.invalid_proof() => {
                assert!(!test.output.unwrap());
            }
            Err(_) => {
                assert!(test.output.is_none());
            }
        };
    }
}
