use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use stacks_voting::{SignatureData, StacksVotingProofVerifier};
use vdf::{VdfProofGenerator, VdfProofVerifier};  use core::fmt;
// Import serde_with for handling u128
use std::result::Result;
use crate::stacks::{proofs::generate_proof, utils::Transaction};

pub mod vdf;
pub mod stacks_voting;

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
impl From<warp::Rejection> for Error {
    fn from(_rejection: warp::Rejection) -> Self {
        Error::ProofGenerationError("Warp rejection occurred".to_string())
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
pub trait VotingProofGenerator {
    fn generate_proof(data: SignatureData, transactions: Vec<Transaction>) -> (Vec<u8>, u128);
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
#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct VerificationResponse {
    ok : bool,
    error: String
}

#[derive(Serialize, Deserialize, Debug)]

pub enum ApplicationResponseMessage {
    ProofGenerationResponse(ProofResponse),
    ProofVerificationResponse(VerificationResponse),
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "proof_type")]

enum ProofGenerationMessage {
    VdfProof {
        #[serde_as(as = "DisplayFromStr")]
        start: u128,
        n: usize,
    },
    StacksVotingProof {
        signature_data: SignatureData
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
    StacksVotingProof {
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
    StacksVotingProof {
        #[serde_as(as = "DisplayFromStr")]
        start: u128,
        #[serde_as(as = "DisplayFromStr")]
        result: u128,
        #[serde(with = "serde_bytes")]
        proof: Vec<u8>,
    },
}


pub async fn handle_message(msg: &str) -> Result<ApplicationResponseMessage, Error> {
    // Deserialize the incoming JSON into ApplicationMessage
    let app_message: ApplicationMessage = serde_json::from_str(msg)?;

    match app_message {
        ApplicationMessage::ProofGeneration(proof_gen_msg) => {
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
                ProofGenerationMessage::StacksVotingProof { signature_data } => {
                    let signature_data_clone = signature_data.clone();
                    let response = generate_proof(signature_data_clone)
                        .await
                        .map_err(|e| Error::from(e))?; // Convert warp::Rejection to proofs::Error
                    //let response_json = serde_json::to_string(&response)?;
                    //let mut sender = Box::pin(sender); 
                    //sender.send(Message::Text(response_json)).await?;
                    return Ok(response)
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
                ProofVerificationMessage::StacksVotingProof { start, result, proof } => {
                    let result = StacksVotingProofVerifier::verify_proof(start, result, proof);
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

