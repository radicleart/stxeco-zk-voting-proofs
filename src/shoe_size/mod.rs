use winterfell::math::StarkField;
use winterfell::{
    Air, AirContext, Assertion, AuxRandElements, DefaultConstraintEvaluator, EvaluationFrame, FieldExtension, Proof, TraceInfo, TransitionConstraintDegree, VerifierError
};
use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin},
    math::{fields::f128::BaseElement, FieldElement, ToElements},
    matrix::ColMatrix,
    DefaultTraceLde, ProofOptions, Prover, StarkDomain, Trace, TracePolyTable, TraceTable,
};

use crate::core::{ProofGenerator, ProofVerifier};

// Generation
// ===========================================================================================

// Example struct for `proof1` proof generation.
pub struct ShoeSizeProofGenerator;

impl ProofGenerator for ShoeSizeProofGenerator {
    fn generate_proof(start: u128, n: usize) -> (Vec<u8>, u128) {
        generate_shoe_size_proof(start, n)
    }
}

// Define the proof1-specific proof generation function.
fn generate_shoe_size_proof(start_in: u128, n_in: usize) -> (Vec<u8>, u128) {
    // We'll just hard-code the parameters here for this example. We'll also just run the
    // computation just for 1024 steps to save time during testing.
    let start: BaseElement = BaseElement::new(start_in);
    let n: usize = n_in;

    // Build the execution trace and get the result from the last step.
    let trace = build_do_work_trace(start, n);
    let result: BaseElement = trace.get(0, n - 1);

    // Define proof options; these will be enough for ~96-bit security level.
    let options = ProofOptions::new(
        32, // number of queries
        8,  // blowup factor
        0,  // grinding factor
        FieldExtension::None,
        8,  // FRI folding factor
        31, // FRI max remainder polynomial degree
    );

    // Instantiate the prover and generate the proof.
    let prover = WorkProver::new(options);
    let proof = prover.prove(trace).unwrap();
    let proof_bytes: Vec<u8> = proof.to_bytes();

    (proof_bytes, result.as_int())

    // The verifier will accept proofs with parameters which guarantee 95 bits or more of
    // conjectured security
    // let min_opts: winterfell::AcceptableOptions = winterfell::AcceptableOptions::MinConjecturedSecurity(95);

    // Verify the proof. The number of steps and options are encoded in the proof itself,
    // so we don't need to pass them explicitly to the verifier.
    // let pub_inputs = PublicInputs { start, result };
    // assert!(winterfell::verify::<WorkAir,
    //     Blake3_256<BaseElement>,
    //     DefaultRandomCoin<Blake3_256<BaseElement>>
    //    >(proof, pub_inputs, &min_opts).is_ok());
    // proof
}

pub struct ShoeSizeProofVerifier;

impl ProofVerifier for ShoeSizeProofVerifier {
    fn verify_proof(start_in: u128, result_in: u128, proof_in: Vec<u8>) -> bool {
        // Your proof generation logic for `proof1`
        // Call the `proof1` specific function (this is just an example)
        verify_shoe_size_proof(start_in, result_in, &proof_in)
    }
}

fn verify_shoe_size_proof(start_in: u128, result_in: u128, proof_in: &[u8]) -> bool {
    // The verifier will accept proofs with parameters which guarantee 95 bits or more of
    // conjectured security
    let start: BaseElement = BaseElement::new(start_in);
    let result: BaseElement = BaseElement::new(result_in);
    let proof: Proof = Proof::from_bytes(proof_in).unwrap();
    let min_opts = winterfell::AcceptableOptions::MinConjecturedSecurity(95);

    // Verify the proof. The number of steps and options are encoded in the proof itself,
    // so we don't need to pass them explicitly to the verifier.
    let pub_inputs = PublicInputs { start, result };
    winterfell::verify::<WorkAir,
                                Blake3_256<BaseElement>,
                                DefaultRandomCoin<Blake3_256<BaseElement>>
                                >(proof, pub_inputs, &min_opts).is_ok()
}


// Prover Implementation
// ===========================================================================================

// Our prover needs to hold STARK protocol parameters which are specified via ProofOptions
// struct.
struct WorkProver {
    options: ProofOptions
}

impl WorkProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }
}

// When implementing the Prover trait we set the `Air` associated type to the AIR of the
// computation we defined previously, and set the `Trace` associated type to `TraceTable`
// struct as we don't need to define a custom trace for our computation. For other
// associated types, we'll use default implementation provided by Winterfell.
impl Prover for WorkProver {
    type BaseField = BaseElement;
    type Air = WorkAir;
    type Trace = TraceTable<Self::BaseField>;
    type HashFn = Blake3_256<Self::BaseField>;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;
    type TraceLde<E: FieldElement<BaseField = Self::BaseField>> = DefaultTraceLde<E, Self::HashFn>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = Self::BaseField>> =
        DefaultConstraintEvaluator<'a, Self::Air, E>;

    // Our public inputs consist of the first and last value in the execution trace.
    fn get_pub_inputs(&self, trace: &Self::Trace) -> PublicInputs {
        let last_step = trace.length() - 1;
        PublicInputs {
            start: trace.get(0, 0),
            result: trace.get(0, last_step),
        }
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }

    fn new_trace_lde<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Self::BaseField>,
        domain: &StarkDomain<Self::BaseField>,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        DefaultTraceLde::new(trace_info, main_trace, domain)
    }

    fn new_evaluator<'a, E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        air: &'a Self::Air,
        aux_rand_elements: Option<AuxRandElements<E>>,
        composition_coefficients: winterfell::ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
    }
}

// Air Implementation
// ===========================================================================================

// Public inputs for our computation will consist of the starting value and the end result.
pub struct PublicInputs {
    start: BaseElement,
    result: BaseElement,
}

// We need to describe how public inputs can be converted to field elements.
impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        vec![self.start, self.result]
    }
}

// For a specific instance of our computation, we'll keep track of the public inputs and
// the computation's context which we'll build in the constructor. The context is used
// internally by the Winterfell prover/verifier when interpreting this AIR.
pub struct WorkAir {
    context: AirContext<BaseElement>,
    start: BaseElement,
    result: BaseElement,
}

impl Air for WorkAir {
    // We'll specify which finite field to use for our computation, and also how
    // the public inputs must look like.
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;
    type GkrProof = ();
    type GkrVerifier = ();

    // Here, we'll construct a new instance of our computation which is defined by 3
    // parameters: starting value, number of steps, and the end result. Another way to
    // think about it is that an instance of our computation is a specific invocation of
    // the do_work() function.
    fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: ProofOptions) -> Self {
        // our execution trace should have only one column.
        assert_eq!(1, trace_info.width());

        // Our computation requires a single transition constraint. The constraint itself
        // is defined in the evaluate_transition() method below, but here we need to specify
        // the expected degree of the constraint. If the expected and actual degrees of the
        // constraints don't match, an error will be thrown in the debug mode, but in release
        // mode, an invalid proof will be generated which will not be accepted by any verifier.
        let degrees = vec![TransitionConstraintDegree::new(3)];

        // We also need to specify the exact number of assertions we will place against the
        // execution trace. This number must be the same as the number of items in a vector
        // returned from the get_assertions() method below.
        let num_assertions = 2;

        WorkAir {
            context: AirContext::new(trace_info, degrees, num_assertions, options),
            start: pub_inputs.start,
            result: pub_inputs.result,
        }
    }

    // In this method we'll define our transition constraints; a computation is considered to
    // be valid, if for all valid state transitions, transition constraints evaluate to all
    // zeros, and for any invalid transition, at least one constraint evaluates to a non-zero
    // value. The `frame` parameter will contain current and next states of the computation.
    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        // First, we'll read the current state, and use it to compute the expected next state
        let current_state = frame.current()[0];
        let next_state = current_state.exp(3u32.into()) + E::from(42u32);

        // Then, we'll subtract the expected next state from the actual next state; this will
        // evaluate to zero if and only if the expected and actual states are the same.
        result[0] = frame.next()[0] - next_state;
    }

    // Here, we'll define a set of assertions about the execution trace which must be
    // satisfied for the computation to be valid. Essentially, this ties computation's
    // execution trace to the public inputs.
    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        // for our computation to be valid, value in column 0 at step 0 must be equal to the
        // starting value, and at the last step it must be equal to the result.
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(0, 0, self.start),
            Assertion::single(0, last_step, self.result),
        ]
    }

    // This is just boilerplate which is used by the Winterfell prover/verifier to retrieve
    // the context of the computation.
    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }
}


// Work function with trace recording
// ===========================================================================================
pub fn build_do_work_trace(start: BaseElement, n: usize) -> TraceTable<BaseElement> {
    // Instantiate the trace with a given width and length; this will allocate all
    // required memory for the trace
    let trace_width = 1;
    let mut trace = TraceTable::new(trace_width, n);

    // Fill the trace with data; the first closure initializes the first state of the
    // computation; the second closure computes the next state of the computation based
    // on its current state.
    trace.fill(
        |state| {
            state[0] = start;
        },
        |_, state| {
            state[0] = state[0].exp(3u32.into()) + BaseElement::new(42);
        },
    );

    trace
}

fn do_work(start: BaseElement, n: usize) -> BaseElement {
   let mut result = start;
   for _ in 1..n {
       result = result.exp(3) + BaseElement::new(42);
   }
   result
}

