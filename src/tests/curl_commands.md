curl http://127.0.0.1:3030/stacks/transactions/SP167Z6WFHMV0FZKFCRNWZ33WTB0DFBCW9QRVJ627

curl -X POST -H "Content-Type: application/json" -d '{}' http://127.0.0.1:3030/stacks/proof/generate

curl -X POST -H "Content-Type: application/json" -d '{ "message_inputs": { "message": "I vote in favour of this proposal", "vote": "for", "proposal": "SIP-028: sBTC Signer Criteria", "balance_at_height": 100706427, "block_proof_height": 868000, "voting_end_height": 869749 }, "public_key": "03440e42dc5a2bbaa710de86588f83fef5f29b01a753561f83f52b522c8c4994d7", "hash": "6673f09022db887e2781eefdd9126ad479fa00300325ad7a6f904a0d52fb3047", "signature": "c3b7e16a4c57b58af7139f08298abc2f07661ddbc4ce5e0aa6daeda59a8e53d72dc5b3ac307cdd9ee931e7eb7de39313b6f0abd4c4a0d0e98e50d3094afa844f00", "message": "Some signed message"}' http://127.0.0.1:3030/stacks/proof/generate
