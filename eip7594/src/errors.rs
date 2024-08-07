use erasure_codes::errors::RSError;

use crate::CellIndex;

/// Errors that can occur while calling a method in the Prover API
#[derive(Debug)]
pub enum ProverError {
    // TODO: This will be getting removed, waiting for consensus-specs PR
    NumProofsDoesNotEqualNumCells,
    Serialization(SerializationError),
    RecoveryFailure(VerifierError),
}

impl From<SerializationError> for ProverError {
    fn from(value: SerializationError) -> Self {
        ProverError::Serialization(value)
    }
}

impl From<VerifierError> for ProverError {
    fn from(value: VerifierError) -> Self {
        ProverError::RecoveryFailure(value)
    }
}

/// Errors that can occur while calling a method in the Verifier API
#[derive(Debug)]
pub enum VerifierError {
    Serialization(SerializationError),
    NumCellIndicesNotEqualToNumCells {
        num_cell_indices: usize,
        num_cells: usize,
    },
    CellIndicesNotUnique,
    NotEnoughCellsToReconstruct {
        num_cells_received: usize,
        min_cells_needed: usize,
    },
    TooManyCellsReceived {
        num_cells_received: usize,
        max_cells_needed: usize,
    },
    CellDoesNotContainEnoughBytes {
        cell_index: CellIndex,
        num_bytes: usize,
        expected_num_bytes: usize,
    },
    CellIndexOutOfRange {
        cell_index: CellIndex,
        max_number_of_cells: u64,
    },
    InvalidRowIndex {
        row_index: u64,
        max_number_of_rows: u64,
    },
    InvalidProof,
    BatchVerificationInputsMustHaveSameLength {
        row_indices_len: usize,
        cell_indices_len: usize,
        cells_len: usize,
        proofs_len: usize,
    },
    ReedSolomon(RSError),
    PolynomialHasInvalidLength {
        num_coefficients: usize,
        expected_num_coefficients: usize,
    },
}

impl From<RSError> for VerifierError {
    fn from(value: RSError) -> Self {
        VerifierError::ReedSolomon(value)
    }
}

impl From<SerializationError> for VerifierError {
    fn from(value: SerializationError) -> Self {
        VerifierError::Serialization(value)
    }
}

/// Errors that can occur during deserialization of untrusted input from the public API
/// or the trusted setup.
#[derive(Debug)]
pub enum SerializationError {
    CouldNotDeserializeScalar { bytes: Vec<u8> },
    CouldNotDeserializeG1Point { bytes: Vec<u8> },
    ScalarHasInvalidLength { bytes: Vec<u8>, length: usize },
    BlobHasInvalidLength { bytes: Vec<u8>, length: usize },
    G1PointHasInvalidLength { bytes: Vec<u8>, length: usize },
}
