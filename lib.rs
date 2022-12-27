#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    use ink_prelude::vec::Vec;
    use ink_prelude::vec;
    use winterfell::{
      math::{fields::f128::BaseElement, FieldElement},
      Air, AirContext, Assertion, ByteWriter, EvaluationFrame, ProofOptions, Serializable,
      TraceInfo, TransitionConstraintDegree, StarkProof
    };
    // use winterfell::{
    //   math::{fields::f128::BaseElement, FieldElement},
    //   Air, AirContext, Assertion, ByteWriter, EvaluationFrame, ProofOptions, Serializable,
    //   Prover, Trace, TraceTable, FieldExtension, StarkProof, 
    //   TraceInfo, TransitionConstraintDegree, HashFunction
    // };
    
    pub struct PublicInputs {
      start: BaseElement,
      result: BaseElement,
    }
    
    impl Serializable for PublicInputs {
      fn write_into<W: ByteWriter>(&self, target: &mut W) {
          target.write(self.start);
          target.write(self.result);
      }
    }
    
    pub struct WorkAir {
      context: AirContext<BaseElement>,
      start: BaseElement,
      result: BaseElement,
    }
    
    impl Air for WorkAir {
      type BaseField = BaseElement;
      type PublicInputs = PublicInputs;
    
      fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: ProofOptions) -> Self {
          assert_eq!(1, trace_info.width());
          let degrees = vec![TransitionConstraintDegree::new(3)];
          let num_assertions = 2;
    
          WorkAir {
              context: AirContext::new(trace_info, degrees, num_assertions, options),
              start: pub_inputs.start,
              result: pub_inputs.result,
          }
      }
    
      fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
          &self,
          frame: &EvaluationFrame<E>,
          _periodic_values: &[E],
          result: &mut [E],
      ) {
          let current_state = &frame.current()[0];
          let next_state = current_state.exp(3u32.into()) + E::from(42u32);
          result[0] = frame.next()[0] - next_state;
      }
    
      fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
          let last_step = self.trace_length() - 1;
          vec![
              Assertion::single(0, 0, self.start),
              Assertion::single(0, last_step, self.result),
          ]
      }
    
      fn context(&self) -> &AirContext<Self::BaseField> {
          &self.context
      }
    }
    
    // struct WorkProver {
    //   options: ProofOptions
    // }
    
    // impl WorkProver {
    //   pub fn new(options: ProofOptions) -> Self {
    //       Self { options }
    //   }
    // }
    
    // impl Prover for WorkProver {
    //   type BaseField = BaseElement;
    //   type Air = WorkAir;
    //   type Trace = TraceTable<Self::BaseField>;
    
    //   // Our public inputs consist of the first and last value in the execution trace.
    //   fn get_pub_inputs(&self, trace: &Self::Trace) -> PublicInputs {
    //       let last_step = trace.length() - 1;
    //       PublicInputs {
    //           start: trace.get(0, 0),
    //           result: trace.get(0, last_step),
    //       }
    //   }
    
    //   fn options(&self) -> &ProofOptions {
    //       &self.options
    //   }
    // }
    
    // pub fn build_do_work_trace(start: BaseElement, n: usize) -> TraceTable<BaseElement> {
    //   let trace_width = 1;
    //   let mut trace = TraceTable::new(trace_width, n);
    
    //   trace.fill(
    //       |state| {
    //           state[0] = start;
    //       },
    //       |_, state| {
    //           state[0] = state[0].exp(3u32.into()) + BaseElement::new(42);
    //       },
    //   );
    //   trace
    // }
    
    // pub fn prove_work() -> (BaseElement, StarkProof) {
    //   let start = BaseElement::new(3);
    //   let n = 1024;
    //   let trace = build_do_work_trace(start, n);
    //   let result = trace.get(0, n - 1);
    
    //   let options = ProofOptions::new(
    //       32, // number of queries
    //       8,  // blowup factor
    //       0,  // grinding factor
    //       HashFunction::Blake3_256,
    //       FieldExtension::None,
    //       8,   // FRI folding factor
    //       128, // FRI max remainder length
    //   );
    
    //   let prover = WorkProver::new(options);
    //   let proof = prover.prove(trace).unwrap();
    
    //   (result, proof)
    // }
    
    // pub fn verify_work(start: BaseElement, result: BaseElement, proof: StarkProof) {
    //   let pub_inputs = PublicInputs { start, result };
    //   // match winterfell::verify::<WorkAir>(proof, pub_inputs) {
    //   //     Ok(_) => ink_env::debug_println!("Everything seems fine and dandy!"),
    //   //     Err(_) => ink_env::debug_println!("Invalid! This will not end well for you, heathen!"),
    //   // }
    //   winterfell::verify::<WorkAir>(proof, pub_inputs);
    // }

    #[ink(storage)]
    pub struct Incrementer {}

    impl Incrementer {
        #[ink(constructor)]
        pub fn default() -> Self {
          Self {}
        }

        #[ink(message)]
        pub fn verify(&self, start: BaseElement, result: BaseElement, proof: StarkProof) {
          // let result_and_proof = prove_work();
          let pub_inputs = PublicInputs { start, result };
          winterfell::verify::<WorkAir>(proof, pub_inputs);
          // verify_work(BaseElement::new(3), result_and_proof.0, result_and_proof.1);
        }

    }
}
