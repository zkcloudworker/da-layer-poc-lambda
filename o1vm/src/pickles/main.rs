use ark_ff::UniformRand;
use kimchi::circuits::domains::EvaluationDomains;
use kimchi_msm::expr::E;
use log::debug;
use mina_curves::pasta::VestaParameters;
use mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    sponge::{DefaultFqSponge, DefaultFrSponge},
};
use o1vm::{
    cannon::{self, Meta, Start, State},
    cannon_cli,
    interpreters::mips::{
        column::N_MIPS_REL_COLS,
        constraints as mips_constraints,
        interpreter::{self, InterpreterEnv},
        witness::{self as mips_witness},
        Instruction,
    },
    pickles::{proof::ProofInputs, prover, verifier},
    preimage_oracle::PreImageOracle,
};
use poly_commitment::{ipa::SRS, SRS as _};
use std::{fs::File, io::BufReader, process::ExitCode, time::Instant};
use strum::IntoEnumIterator;

use mina_curves::pasta::{Fp, Vesta};

pub const DOMAIN_SIZE: usize = 1 << 15;

pub fn main() -> ExitCode {
    let cli = cannon_cli::main_cli();

    let mut rng = rand::thread_rng();

    let configuration = cannon_cli::read_configuration(&cli.get_matches());

    let file =
        File::open(&configuration.input_state_file).expect("Error opening input state file ");

    let reader = BufReader::new(file);
    // Read the JSON contents of the file as an instance of `State`.
    let state: State = serde_json::from_reader(reader).expect("Error reading input state file");

    let meta_file = File::open(&configuration.metadata_file).unwrap_or_else(|_| {
        panic!(
            "Could not open metadata file {}",
            &configuration.metadata_file
        )
    });

    let meta: Meta = serde_json::from_reader(BufReader::new(meta_file)).unwrap_or_else(|_| {
        panic!(
            "Error deserializing metadata file {}",
            &configuration.metadata_file
        )
    });

    let mut po = PreImageOracle::create(&configuration.host);
    let _child = po.start();

    // Initialize some data used for statistical computations
    let start = Start::create(state.step as usize);

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let domain_fp = EvaluationDomains::<Fp>::create(DOMAIN_SIZE).unwrap();
    let srs: SRS<Vesta> = {
        let srs = SRS::create(DOMAIN_SIZE);
        srs.get_lagrange_basis(domain_fp.d1);
        srs
    };

    // Initialize the environments
    let mut mips_wit_env =
        mips_witness::Env::<Fp, PreImageOracle>::create(cannon::PAGE_SIZE as usize, state, po);

    let constraints = {
        let mut mips_con_env = mips_constraints::Env::<Fp>::default();
        let mut constraints = Instruction::iter()
            .flat_map(|instr_typ| instr_typ.into_iter())
            .fold(vec![], |mut acc, instr| {
                interpreter::interpret_instruction(&mut mips_con_env, instr);
                let selector = mips_con_env.get_selector();
                let constraints_with_selector: Vec<E<Fp>> = mips_con_env
                    .get_constraints()
                    .into_iter()
                    .map(|c| selector.clone() * c)
                    .collect();
                acc.extend(constraints_with_selector);
                mips_con_env.reset();
                acc
            });
        constraints.extend(mips_con_env.get_selector_constraints());
        constraints
    };

    while !mips_wit_env.halt {
        // Create a proof input for DOMAIN_SIZE steps
        let mut curr_proof_inputs = ProofInputs::new(DOMAIN_SIZE);
        for _ in 0..DOMAIN_SIZE {
            if !mips_wit_env.halt {
                let _instr = mips_wit_env.step(&configuration, &meta, &start);
                for (scratch, scratch_chunk) in mips_wit_env
                    .scratch_state
                    .iter()
                    .zip(curr_proof_inputs.evaluations.scratch.iter_mut())
                {
                    scratch_chunk.push(*scratch);
                }
                curr_proof_inputs
                    .evaluations
                    .instruction_counter
                    .push(Fp::from(mips_wit_env.instruction_counter));
                // FIXME: Might be another value
                curr_proof_inputs.evaluations.error.push(Fp::rand(&mut rng));

                curr_proof_inputs
                    .evaluations
                    .selector
                    .push(Fp::from((mips_wit_env.selector - N_MIPS_REL_COLS) as u64));
            }
        }
        // Start a proof
        let start_iteration = Instant::now();
        debug!("We make a proof, verify it (for testing) and start with a new chunk");
        let proof = prover::prove::<
            Vesta,
            DefaultFqSponge<VestaParameters, PlonkSpongeConstantsKimchi>,
            DefaultFrSponge<Fp, PlonkSpongeConstantsKimchi>,
            _,
        >(domain_fp, &srs, curr_proof_inputs, &constraints, &mut rng)
        .unwrap();
        debug!(
            "Proof generated in {elapsed} μs",
            elapsed = start_iteration.elapsed().as_micros()
        );
        {
            let start_iteration = Instant::now();
            let verif = verifier::verify::<
                Vesta,
                DefaultFqSponge<VestaParameters, PlonkSpongeConstantsKimchi>,
                DefaultFrSponge<Fp, PlonkSpongeConstantsKimchi>,
            >(domain_fp, &srs, &constraints, &proof);
            debug!(
                "Verification done in {elapsed} μs",
                elapsed = start_iteration.elapsed().as_micros()
            );
            assert!(verif);
        }
    }
    ExitCode::SUCCESS
}
