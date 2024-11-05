use base64::{alphabet, engine::{self, general_purpose}, Engine};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};  use core::fmt;
// Import serde_with for handling u128
use std::result::Result;
use crate::{shoe_size::{ShoeSizeProofGenerator, ShoeSizeProofVerifier}, vdf::{VdfProofGenerator, VdfProofVerifier}};

// Assuming you are using some kind of error type, define it or use a generic one
#[derive(Debug)]
pub enum Error {
    SerializationError(serde_json::Error),  // Example error type
    ProofGenerationError(String),           // Another example error
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerializationError(err)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ProofGenerationError(msg) => write!(f, "Proof generation error: {}", msg),
            _ => write!(f, "Proof generation error"),
        }
    }
}



pub trait ProofGenerator {
    fn generate_proof(start: u128, n: usize) -> (Vec<u8>, u128);
}

pub trait ProofVerifier {
    fn verify_proof(start: u128, result_in: u128, proof: Vec<u8>) -> bool;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "message_type")]  // Top-level message type
enum ApplicationMessage {
    ProofGeneration(ProofGenerationMessage),
    ProofVerification(ProofVerificationMessage),
    Other(String),  // Placeholder for future message types
}

#[derive(serde::Serialize)]
pub struct VerificationResponse {
    ok : bool,
    error: String
}

#[derive(serde::Serialize)]
pub enum ApplicationResponseMessage {
    ProofGenerationResponse(ProofResponse),
    ProofVerificationResponse(VerificationResponse),
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "proof_type")]  // Nested message type for proof generation
enum ProofGenerationMessage {
    VdfProof {
        #[serde_as(as = "DisplayFromStr")]
        start: u128,
        n: usize,
    },
    ShoeSizeProof {
        #[serde_as(as = "DisplayFromStr")]
        start: u128,
        n: usize,
    },
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "proof_type")]  // Nested message type for proof generation
pub enum ProofResponse {
    ProofError {
        message: String,
    },
    VdfProof {
        result: String,
        proof: String
    },
    ShoeSizeProof {
        result: String,
        proof: String
    },
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "proof_type")]  // Nested message type for proof verification
enum ProofVerificationMessage {
    VdfProof {
        #[serde_as(as = "DisplayFromStr")]
        start: u128,
        #[serde_as(as = "DisplayFromStr")]
        result: u128,
        #[serde(with = "serde_bytes")]  // Handle Vec<u8> as a byte array
        proof: Vec<u8>,
    },
    ShoeSizeProof {
        #[serde_as(as = "DisplayFromStr")]
        start: u128,
        #[serde_as(as = "DisplayFromStr")]
        result: u128,
        #[serde(with = "serde_bytes")]
        proof: Vec<u8>,
    },
}


pub fn handle_message(msg: &str) -> Result<ApplicationResponseMessage, Error> {
    // Deserialize the incoming JSON into ApplicationMessage
    let app_message: ApplicationMessage = serde_json::from_str(msg)?;

    match app_message {
        ApplicationMessage::ProofGeneration(proof_gen_msg) => {
            // Match on specific proof type for generation
            match proof_gen_msg {
                ProofGenerationMessage::VdfProof { start, n } => {
                    let (proof, result) = VdfProofGenerator::generate_proof(start, n);
                    if proof.is_empty() {
                        return Err(Error::ProofGenerationError("Proof generation failed".to_string()));
                    }
                                    let b64_proof = general_purpose::STANDARD.encode(proof);
                    let response: ProofResponse = ProofResponse::VdfProof {
                        result: result.to_string(),            // Result from the proof generation
                        proof: b64_proof,
                    };
                    let application_response: ApplicationResponseMessage = ApplicationResponseMessage::ProofGenerationResponse(response);
                    return Ok(application_response);
                }
                ProofGenerationMessage::ShoeSizeProof { start, n } => {
                    let (proof, result) = ShoeSizeProofGenerator::generate_proof(start, n);
                    let b64_proof = general_purpose::STANDARD.encode(proof);
                    let response: ProofResponse = ProofResponse::VdfProof {
                        result: result.to_string(),            // Result from the proof generation
                        proof: b64_proof,
                    };
                    let application_response: ApplicationResponseMessage = ApplicationResponseMessage::ProofGenerationResponse(response);
                    return Ok(application_response);
                }
            }
        }
        ApplicationMessage::ProofVerification(proof_ver_msg) => {
            // Match on specific proof type for verification
            match proof_ver_msg {
                ProofVerificationMessage::VdfProof { start, result, proof } => {
                    let result = VdfProofVerifier::verify_proof(start, result, proof);
                    let response: VerificationResponse = VerificationResponse {
                        ok:result,            // Result from the proof generation
                        error: "None".to_string(),
                    };
                    let application_response: ApplicationResponseMessage = ApplicationResponseMessage::ProofVerificationResponse(response);
                    return Ok(application_response);
                }
                ProofVerificationMessage::ShoeSizeProof { start, result, proof } => {
                    let result = ShoeSizeProofVerifier::verify_proof(start, result, proof);
                    let response: VerificationResponse = VerificationResponse {
                        ok:result,            // Result from the proof generation
                        error: "None".to_string(),
                    };
                    let application_response: ApplicationResponseMessage = ApplicationResponseMessage::ProofVerificationResponse(response);
                    return Ok(application_response);
                }
            }
        }
        ApplicationMessage::Other(description) => {
            println!("Received other message type: {}", description);
            ()
        }
    }

    let message:String = "oops".to_string();
    let response: ProofResponse = ProofResponse::ProofError {
        message,
    };
    let application_response: ApplicationResponseMessage = ApplicationResponseMessage::ProofGenerationResponse(response);
    return Ok(application_response);
}
