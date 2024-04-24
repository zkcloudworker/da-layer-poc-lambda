var srcIndex = new Map(JSON.parse('[\
["export_test_vectors",["",[],["main.rs","vectors.rs"]]],\
["ffa",["",[],["main.rs"]]],\
["flamegraph",["",[],["flamegraph.rs"]]],\
["groupmap",["",[],["lib.rs"]]],\
["internal_tracing",["",[],["lib.rs"]]],\
["ivc",["",[["poseidon",[],["constraint.rs","interpreter.rs","mod.rs","witness.rs"]]],["lib.rs"]]],\
["kimchi",["",[["circuits",[["lookup",[["tables",[],["mod.rs","range_check.rs","xor.rs"]]],["constraints.rs","index.rs","lookups.rs","mod.rs","runtime_tables.rs"]],["polynomials",[["foreign_field_add",[],["circuitgates.rs","gadget.rs","mod.rs","witness.rs"]],["foreign_field_mul",[],["circuitgates.rs","gadget.rs","mod.rs","witness.rs"]],["keccak",[],["circuitgates.rs","constants.rs","gadget.rs","mod.rs","witness.rs"]],["range_check",[],["circuitgates.rs","gadget.rs","mod.rs","witness.rs"]]],["and.rs","complete_add.rs","endomul_scalar.rs","endosclmul.rs","foreign_field_common.rs","generic.rs","mod.rs","not.rs","permutation.rs","poseidon.rs","rot.rs","turshi.rs","varbasemul.rs","xor.rs"]],["witness",[],["constant_cell.rs","copy_bits_cell.rs","copy_cell.rs","copy_shift_cell.rs","index_cell.rs","mod.rs","variable_bits_cell.rs","variable_cell.rs","variables.rs"]]],["argument.rs","berkeley_columns.rs","constraints.rs","domain_constant_evaluation.rs","domains.rs","expr.rs","gate.rs","macros.rs","mod.rs","polynomial.rs","scalars.rs","serialization_helper.rs","wires.rs"]],["folding",[],["decomposable_folding.rs","error_term.rs","example.rs","expressions.rs","instance_witness.rs","mod.rs","quadraticization.rs"]],["snarky",[["folding",[],["instance.rs"]]],["api.rs","asm.rs","boolean.rs","constants.rs","constraint_system.rs","cvar.rs","errors.rs","folding.rs","mod.rs","poseidon.rs","range_checks.rs","runner.rs","snarky_type.rs","union_find.rs"]]],["alphas.rs","bench.rs","curve.rs","error.rs","lagrange_basis_evaluations.rs","lib.rs","linearization.rs","oracles.rs","plonk_sponge.rs","precomputed_srs.rs","proof.rs","prover.rs","prover_index.rs","verifier.rs","verifier_index.rs"]]],\
["kimchi_msm",["",[["circuit_design",[],["capabilities.rs","composition.rs","constraints.rs","mod.rs","witness.rs"]],["fec",[],["columns.rs","interpreter.rs","lookups.rs","mod.rs"]],["ffa",[],["columns.rs","constraint.rs","interpreter.rs","mod.rs","witness.rs"]],["serialization",[],["column.rs","interpreter.rs","lookups.rs","mod.rs"]],["test",[],["columns.rs","constraint.rs","interpreter.rs","mod.rs","witness.rs"]]],["column_env.rs","columns.rs","expr.rs","lib.rs","logup.rs","lookups.rs","precomputed_srs.rs","proof.rs","prover.rs","verifier.rs","witness.rs"]]],\
["kimchi_optimism",["",[["keccak",[],["column.rs","constraints.rs","environment.rs","helpers.rs","interpreter.rs","mod.rs","trace.rs","witness.rs"]],["mips",[],["column.rs","constraints.rs","interpreter.rs","mod.rs","registers.rs","trace.rs","witness.rs"]]],["cannon.rs","cannon_cli.rs","lib.rs","lookups.rs","preimage_oracle.rs","proof.rs","ramlookup.rs","trace.rs"]]],\
["kimchi_visu",["",[],["lib.rs","witness.rs"]]],\
["mina_book",["",[],["lib.rs"]]],\
["mina_curves",["",[["pasta",[["curves",[],["mod.rs","pallas.rs","vesta.rs"]],["fields",[],["fp.rs","fq.rs","mod.rs"]]],["mod.rs"]]],["lib.rs"]]],\
["mina_hasher",["",[],["lib.rs","poseidon.rs","roinput.rs"]]],\
["mina_poseidon",["",[["pasta",[],["fp_kimchi.rs","fp_legacy.rs","fq_kimchi.rs","fq_legacy.rs","mod.rs"]]],["constants.rs","dummy_values.rs","lib.rs","permutation.rs","poseidon.rs","sponge.rs"]]],\
["mina_signer",["",[],["keypair.rs","lib.rs","pubkey.rs","schnorr.rs","seckey.rs","signature.rs"]]],\
["o1_utils",["",[],["adjacent_pairs.rs","biguint_helpers.rs","bitwise_operations.rs","chunked_evaluations.rs","chunked_polynomial.rs","dense_polynomial.rs","evaluations.rs","field_helpers.rs","foreign_field.rs","hasher.rs","lib.rs","math.rs","serialization.rs"]]],\
["poly_commitment",["",[],["chunked.rs","combine.rs","commitment.rs","error.rs","evaluation_proof.rs","lib.rs","pairing_proof.rs","srs.rs"]]],\
["snarky_deriver",["",[],["lib.rs"]]],\
["test_optimism_preimage_read",["",[],["test_preimage_read.rs"]]],\
["turshi",["",[],["flags.rs","helper.rs","lib.rs","memory.rs","runner.rs","word.rs"]]]\
]'));
createSrcSidebar();
