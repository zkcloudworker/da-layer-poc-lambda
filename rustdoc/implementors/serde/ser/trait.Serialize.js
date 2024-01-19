(function() {var implementors = {
"export_test_vectors":[["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"export_test_vectors/vectors/struct.TestVectors.html\" title=\"struct export_test_vectors::vectors::TestVectors\">TestVectors</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"export_test_vectors/vectors/struct.TestVector.html\" title=\"struct export_test_vectors::vectors::TestVector\">TestVector</a>"]],
"kimchi":[["impl&lt;G, OpeningProof&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/proof/struct.ProverProof.html\" title=\"struct kimchi::proof::ProverProof\">ProverProof</a>&lt;G, OpeningProof&gt;<span class=\"where fmt-newline\">where\n    OpeningProof: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,\n    G: CanonicalDeserialize + CanonicalSerialize + AffineCurve,</span>"],["impl&lt;F: PrimeField&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/constraints/struct.ColumnEvaluations.html\" title=\"struct kimchi::circuits::constraints::ColumnEvaluations\">ColumnEvaluations</a>&lt;F&gt;"],["impl&lt;G: CommitmentCurve&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/verifier_index/struct.LookupVerifierIndex.html\" title=\"struct kimchi::verifier_index::LookupVerifierIndex\">LookupVerifierIndex</a>&lt;G&gt;<span class=\"where fmt-newline\">where\n    PolyComm&lt;G&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,</span>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/berkeley_columns/enum.Column.html\" title=\"enum kimchi::circuits::berkeley_columns::Column\">Column</a>"],["impl&lt;F&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/runtime_tables/struct.RuntimeTableCfg.html\" title=\"struct kimchi::circuits::lookup::runtime_tables::RuntimeTableCfg\">RuntimeTableCfg</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;G: <a class=\"trait\" href=\"kimchi/curve/trait.KimchiCurve.html\" title=\"trait kimchi::curve::KimchiCurve\">KimchiCurve</a>, OpeningProof: OpenProof&lt;G&gt;&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/prover_index/struct.ProverIndex.html\" title=\"struct kimchi::prover_index::ProverIndex\">ProverIndex</a>&lt;G, OpeningProof&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"kimchi/circuits/constraints/struct.ConstraintSystem.html\" title=\"struct kimchi::circuits::constraints::ConstraintSystem\">ConstraintSystem</a>&lt;G::ScalarField&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,\n    <a class=\"struct\" href=\"kimchi/circuits/constraints/struct.ColumnEvaluations.html\" title=\"struct kimchi::circuits::constraints::ColumnEvaluations\">ColumnEvaluations</a>&lt;G::ScalarField&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,</span>"],["impl&lt;F&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/constraints/struct.LookupConfiguration.html\" title=\"struct kimchi::circuits::lookup::constraints::LookupConfiguration\">LookupConfiguration</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: CanonicalSerialize + CanonicalDeserialize,</span>"],["impl&lt;F: PrimeField&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/constraints/struct.ConstraintSystem.html\" title=\"struct kimchi::circuits::constraints::ConstraintSystem\">ConstraintSystem</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"kimchi/circuits/domains/struct.EvaluationDomains.html\" title=\"struct kimchi::circuits::domains::EvaluationDomains\">EvaluationDomains</a>&lt;F&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,\n    <a class=\"struct\" href=\"kimchi/circuits/gate/struct.CircuitGate.html\" title=\"struct kimchi::circuits::gate::CircuitGate\">CircuitGate</a>&lt;F&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,\n    <a class=\"struct\" href=\"kimchi/circuits/lookup/index/struct.LookupConstraintSystem.html\" title=\"struct kimchi::circuits::lookup::index::LookupConstraintSystem\">LookupConstraintSystem</a>&lt;F&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,</span>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/constraints/struct.FeatureFlags.html\" title=\"struct kimchi::circuits::constraints::FeatureFlags\">FeatureFlags</a>"],["impl&lt;G&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/proof/struct.LookupCommitments.html\" title=\"struct kimchi::proof::LookupCommitments\">LookupCommitments</a>&lt;G&gt;<span class=\"where fmt-newline\">where\n    G: CanonicalDeserialize + CanonicalSerialize + AffineCurve,</span>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/expr/enum.FeatureFlag.html\" title=\"enum kimchi::circuits::expr::FeatureFlag\">FeatureFlag</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.LookupFeatures.html\" title=\"struct kimchi::circuits::lookup::lookups::LookupFeatures\">LookupFeatures</a>"],["impl&lt;G: <a class=\"trait\" href=\"kimchi/curve/trait.KimchiCurve.html\" title=\"trait kimchi::curve::KimchiCurve\">KimchiCurve</a>, OpeningProof: OpenProof&lt;G&gt;&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/verifier_index/struct.VerifierIndex.html\" title=\"struct kimchi::verifier_index::VerifierIndex\">VerifierIndex</a>&lt;G, OpeningProof&gt;<span class=\"where fmt-newline\">where\n    PolyComm&lt;G&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;PolyComm&lt;G&gt;&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,</span>"],["impl&lt;F&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/alphas/struct.Alphas.html\" title=\"struct kimchi::alphas::Alphas\">Alphas</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;'a, F: PrimeField&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/gate/struct.Circuit.html\" title=\"struct kimchi::circuits::gate::Circuit\">Circuit</a>&lt;'a, F&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"kimchi/circuits/gate/struct.CircuitGate.html\" title=\"struct kimchi::circuits::gate::CircuitGate\">CircuitGate</a>&lt;F&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/index/struct.LookupSelectors.html\" title=\"struct kimchi::circuits::lookup::index::LookupSelectors\">LookupSelectors</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;F: FftField&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/domains/struct.EvaluationDomains.html\" title=\"struct kimchi::circuits::domains::EvaluationDomains\">EvaluationDomains</a>&lt;F&gt;"],["impl&lt;F&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/expr/enum.ConstantTerm.html\" title=\"enum kimchi::circuits::expr::ConstantTerm\">ConstantTerm</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/gate/enum.CurrOrNext.html\" title=\"enum kimchi::circuits::gate::CurrOrNext\">CurrOrNext</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/argument/enum.ArgumentType.html\" title=\"enum kimchi::circuits::argument::ArgumentType\">ArgumentType</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/expr/enum.ChallengeTerm.html\" title=\"enum kimchi::circuits::expr::ChallengeTerm\">ChallengeTerm</a>"],["impl&lt;F: PrimeField&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/constraints/struct.EvaluatedColumnCoefficients.html\" title=\"struct kimchi::circuits::constraints::EvaluatedColumnCoefficients\">EvaluatedColumnCoefficients</a>&lt;F&gt;"],["impl&lt;Evals&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/proof/struct.PointEvaluations.html\" title=\"struct kimchi::proof::PointEvaluations\">PointEvaluations</a>&lt;Evals&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;SerdeAs&gt;: <a class=\"trait\" href=\"https://docs.rs/serde_with/1.14.0/serde_with/ser/trait.SerializeAs.html\" title=\"trait serde_with::ser::SerializeAs\">SerializeAs</a>&lt;Evals&gt;,</span>"],["impl&lt;Column&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/expr/struct.Variable.html\" title=\"struct kimchi::circuits::expr::Variable\">Variable</a>&lt;Column&gt;<span class=\"where fmt-newline\">where\n    Column: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.LookupInfo.html\" title=\"struct kimchi::circuits::lookup::lookups::LookupInfo\">LookupInfo</a>"],["impl&lt;F: FftField&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/domain_constant_evaluation/struct.DomainConstantEvaluations.html\" title=\"struct kimchi::circuits::domain_constant_evaluation::DomainConstantEvaluations\">DomainConstantEvaluations</a>&lt;F&gt;"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/wires/struct.Wire.html\" title=\"struct kimchi::circuits::wires::Wire\">Wire</a>"],["impl&lt;F&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.SingleLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::SingleLookup\">SingleLookup</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;G&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/proof/struct.RecursionChallenge.html\" title=\"struct kimchi::proof::RecursionChallenge\">RecursionChallenge</a>&lt;G&gt;<span class=\"where fmt-newline\">where\n    G: AffineCurve + CanonicalDeserialize + CanonicalSerialize,</span>"],["impl&lt;F, Column&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/expr/enum.PolishToken.html\" title=\"enum kimchi::circuits::expr::PolishToken\">PolishToken</a>&lt;F, Column&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,\n    Column: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;SingleLookup, LookupTableID&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::JointLookup\">JointLookup</a>&lt;SingleLookup, LookupTableID&gt;<span class=\"where fmt-newline\">where\n    SingleLookup: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,\n    LookupTableID: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/lookup/lookups/enum.LookupPattern.html\" title=\"enum kimchi::circuits::lookup::lookups::LookupPattern\">LookupPattern</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/runtime_tables/struct.RuntimeTableSpec.html\" title=\"struct kimchi::circuits::lookup::runtime_tables::RuntimeTableSpec\">RuntimeTableSpec</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/lookup/tables/enum.GateLookupTable.html\" title=\"enum kimchi::circuits::lookup::tables::GateLookupTable\">GateLookupTable</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.LookupPatterns.html\" title=\"struct kimchi::circuits::lookup::lookups::LookupPatterns\">LookupPatterns</a>"],["impl&lt;F: PrimeField&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/gate/struct.CircuitGate.html\" title=\"struct kimchi::circuits::gate::CircuitGate\">CircuitGate</a>&lt;F&gt;"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/lookup/lookups/enum.LookupTableID.html\" title=\"enum kimchi::circuits::lookup::lookups::LookupTableID\">LookupTableID</a>"],["impl&lt;F: FftField&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/index/struct.LookupConstraintSystem.html\" title=\"struct kimchi::circuits::lookup::index::LookupConstraintSystem\">LookupConstraintSystem</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"kimchi/circuits/lookup/constraints/struct.LookupConfiguration.html\" title=\"struct kimchi::circuits::lookup::constraints::LookupConfiguration\">LookupConfiguration</a>&lt;F&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,</span>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.LocalPosition.html\" title=\"struct kimchi::circuits::lookup::lookups::LocalPosition\">LocalPosition</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/tables/struct.GateLookupTables.html\" title=\"struct kimchi::circuits::lookup::tables::GateLookupTables\">GateLookupTables</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/expr/struct.RowOffset.html\" title=\"struct kimchi::circuits::expr::RowOffset\">RowOffset</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"kimchi/circuits/gate/enum.GateType.html\" title=\"enum kimchi::circuits::gate::GateType\">GateType</a>"],["impl&lt;G&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/proof/struct.ProverCommitments.html\" title=\"struct kimchi::proof::ProverCommitments\">ProverCommitments</a>&lt;G&gt;<span class=\"where fmt-newline\">where\n    G: CanonicalDeserialize + CanonicalSerialize + AffineCurve,</span>"],["impl&lt;E, Column&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/expr/struct.Linearization.html\" title=\"struct kimchi::circuits::expr::Linearization\">Linearization</a>&lt;E, Column&gt;<span class=\"where fmt-newline\">where\n    E: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,\n    Column: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;Evals&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/proof/struct.ProofEvaluations.html\" title=\"struct kimchi::proof::ProofEvaluations\">ProofEvaluations</a>&lt;Evals&gt;<span class=\"where fmt-newline\">where\n    Evals: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"]],
"kimchi_optimism":[["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi_optimism/mips/interpreter/debugging/struct.InstructionParts.html\" title=\"struct kimchi_optimism::mips::interpreter::debugging::InstructionParts\">InstructionParts</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi_optimism/cannon/struct.Page.html\" title=\"struct kimchi_optimism::cannon::Page\">Page</a>"],["impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi_optimism/cannon/struct.State.html\" title=\"struct kimchi_optimism::cannon::State\">State</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi_optimism/mips/registers/struct.Registers.html\" title=\"struct kimchi_optimism::mips::registers::Registers\">Registers</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"]],
"kimchi_visu":[["impl&lt;F&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi_visu/witness/struct.Witness.html\" title=\"struct kimchi_visu::witness::Witness\">Witness</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: Field,</span>"]],
"mina_poseidon":[["impl&lt;F: Field&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mina_poseidon/poseidon/struct.ArithmeticSpongeParams.html\" title=\"struct mina_poseidon::poseidon::ArithmeticSpongeParams\">ArithmeticSpongeParams</a>&lt;F&gt;"]],
"o1_utils":[["impl&lt;F&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"o1_utils/chunked_evaluations/struct.ChunkedEvaluations.html\" title=\"struct o1_utils::chunked_evaluations::ChunkedEvaluations\">ChunkedEvaluations</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: PrimeField,</span>"]],
"poly_commitment":[["impl&lt;C&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"poly_commitment/commitment/struct.PolyComm.html\" title=\"struct poly_commitment::commitment::PolyComm\">PolyComm</a>&lt;C&gt;<span class=\"where fmt-newline\">where\n    C: CanonicalDeserialize + CanonicalSerialize,</span>"],["impl&lt;G&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"poly_commitment/commitment/struct.BlindedCommitment.html\" title=\"struct poly_commitment::commitment::BlindedCommitment\">BlindedCommitment</a>&lt;G&gt;<span class=\"where fmt-newline\">where\n    G: <a class=\"trait\" href=\"poly_commitment/commitment/trait.CommitmentCurve.html\" title=\"trait poly_commitment::commitment::CommitmentCurve\">CommitmentCurve</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;G&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"poly_commitment/evaluation_proof/struct.OpeningProof.html\" title=\"struct poly_commitment::evaluation_proof::OpeningProof\">OpeningProof</a>&lt;G&gt;<span class=\"where fmt-newline\">where\n    G: CanonicalDeserialize + CanonicalSerialize + AffineCurve,</span>"],["impl&lt;Pair: PairingEngine&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"poly_commitment/pairing_proof/struct.PairingSRS.html\" title=\"struct poly_commitment::pairing_proof::PairingSRS\">PairingSRS</a>&lt;Pair&gt;"],["impl&lt;Pair: PairingEngine&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"poly_commitment/pairing_proof/struct.PairingProof.html\" title=\"struct poly_commitment::pairing_proof::PairingProof\">PairingProof</a>&lt;Pair&gt;<span class=\"where fmt-newline\">where\n    Pair::G1Affine: CanonicalDeserialize + CanonicalSerialize,</span>"],["impl&lt;G&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"poly_commitment/srs/struct.SRS.html\" title=\"struct poly_commitment::srs::SRS\">SRS</a>&lt;G&gt;<span class=\"where fmt-newline\">where\n    G: CanonicalDeserialize + CanonicalSerialize,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()