use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin},
    math::{fields::f128::BaseElement, FieldElement},
    matrix::ColMatrix,
    DefaultConstraintEvaluator, DefaultTraceLde, ProofOptions, Prover, StarkDomain, Trace,
    TraceInfo, TracePolyTable, TraceTable,
};
use winterfell::AuxRandElements;

use super::{PublicInputs, WorkAir};

// We'll use BLAKE3 as the hash function during proof generation.
type Blake3 = Blake3_256<BaseElement>;

// Our prover needs to hold STARK protocol parameters which are specified via ProofOptions
// struct.
pub struct WorkProver {
    options: ProofOptions,
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
    type Trace = TraceTable<BaseElement>;
    type HashFn = Blake3;
    type RandomCoin = DefaultRandomCoin<Blake3>;
    type TraceLde<E: FieldElement<BaseField = BaseElement>> = DefaultTraceLde<E, Blake3>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = BaseElement>> =
        DefaultConstraintEvaluator<'a, WorkAir, E>;

    // Our public inputs consist of the first and last value in the execution trace.
    fn get_pub_inputs(&self, trace: &Self::Trace) -> PublicInputs {
        let last_step = trace.length() - 1;
        PublicInputs {
            balance: trace.get(0, 0),
            height: trace.get(0, last_step),
        }
    }

    // We'll use the default trace low-degree extension.
    fn new_trace_lde<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Self::BaseField>,
        domain: &StarkDomain<Self::BaseField>,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        DefaultTraceLde::new(trace_info, main_trace, domain)
    }

    // We'll use the default constraint evaluator to evaluate AIR constraints.
    fn new_evaluator<'a, E: FieldElement<BaseField = BaseElement>>(
        &self,
        air: &'a WorkAir,
        aux_rand_elements: Option<AuxRandElements<E>>,
        composition_coefficients: winterfell::ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}








// Prover Implementation
// ===========================================================================================

// Our prover needs to hold STARK protocol parameters which are specified via ProofOptions
// struct.
// struct WorkProver {
//     options: ProofOptions
// }

// impl WorkProver {
//     pub fn new(options: ProofOptions) -> Self {
//         Self { options }
//     }
// }

// // When implementing the Prover trait we set the `Air` associated type to the AIR of the
// // computation we defined previously, and set the `Trace` associated type to `TraceTable`
// // struct as we don't need to define a custom trace for our computation. For other
// // associated types, we'll use default implementation provided by Winterfell.
// impl Prover for WorkProver {
//     type BaseField = BaseElement;
//     type Air = WorkAir;
//     type Trace = TraceTable<Self::BaseField>;
//     type HashFn = Blake3_256<Self::BaseField>;
//     type RandomCoin = DefaultRandomCoin<Self::HashFn>;
//     type TraceLde<E: FieldElement<BaseField = Self::BaseField>> = DefaultTraceLde<E, Self::HashFn>;
//     type ConstraintEvaluator<'a, E: FieldElement<BaseField = Self::BaseField>> =
//         DefaultConstraintEvaluator<'a, Self::Air, E>;

//     // Our public inputs consist of the first and last value in the execution trace.
//     fn get_pub_inputs(&self, trace: &Self::Trace) -> PublicInputs {
//         let last_step = trace.length() - 1;
//         PublicInputs {
//             balance_at_height: trace.get(0, 0),
//             block_proof_height: trace.get(0, last_step),
//         }
//     }

//     fn options(&self) -> &ProofOptions {
//         &self.options
//     }

//     fn new_trace_lde<E: FieldElement<BaseField = Self::BaseField>>(
//         &self,
//         trace_info: &TraceInfo,
//         main_trace: &ColMatrix<Self::BaseField>,
//         domain: &StarkDomain<Self::BaseField>,
//     ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
//         DefaultTraceLde::new(trace_info, main_trace, domain)
//     }

//     fn new_evaluator<'a, E: FieldElement<BaseField = Self::BaseField>>(
//         &self,
//         air: &'a Self::Air,
//         aux_rand_elements: Option<AuxRandElements<E>>,
//         composition_coefficients: winterfell::ConstraintCompositionCoefficients<E>,
//     ) -> Self::ConstraintEvaluator<'a, E> {
//         DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
//     }
// }
