(function() {var implementors = {
"folding":[],
"ivc":[["impl&lt;G: CommitmentCurve, const N_COL: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const N_CHALS: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const N_ALPHAS: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>&gt; Foldable&lt;&lt;G as AffineRepr&gt;::ScalarField&gt; for <a class=\"struct\" href=\"ivc/plonkish_lang/struct.PlonkishInstance.html\" title=\"struct ivc::plonkish_lang::PlonkishInstance\">PlonkishInstance</a>&lt;G, N_COL, N_CHALS, N_ALPHAS&gt;"],["impl&lt;const N_COL: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const N_FSEL: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, F: Field, Evals: <a class=\"trait\" href=\"ivc/plonkish_lang/trait.CombinableEvals.html\" title=\"trait ivc::plonkish_lang::CombinableEvals\">CombinableEvals</a>&lt;F&gt;&gt; Foldable&lt;F&gt; for <a class=\"struct\" href=\"ivc/plonkish_lang/struct.PlonkishWitnessGeneric.html\" title=\"struct ivc::plonkish_lang::PlonkishWitnessGeneric\">PlonkishWitnessGeneric</a>&lt;N_COL, N_FSEL, F, Evals&gt;"]],
"kimchi_msm":[["impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, F: FftField&gt; <a class=\"trait\" href=\"folding/instance_witness/trait.Foldable.html\" title=\"trait folding::instance_witness::Foldable\">Foldable</a>&lt;F&gt; for <a class=\"struct\" href=\"kimchi_msm/witness/struct.Witness.html\" title=\"struct kimchi_msm::witness::Witness\">Witness</a>&lt;N, Evaluations&lt;F, Radix2EvaluationDomain&lt;F&gt;&gt;&gt;"]],
"o1vm":[["impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, F: FftField&gt; Foldable&lt;F&gt; for <a class=\"struct\" href=\"o1vm/legacy/folding/struct.FoldingWitness.html\" title=\"struct o1vm::legacy::folding::FoldingWitness\">FoldingWitness</a>&lt;N, F&gt;"],["impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, G: CommitmentCurve&gt; Foldable&lt;&lt;G as AffineRepr&gt;::ScalarField&gt; for <a class=\"struct\" href=\"o1vm/legacy/folding/struct.FoldingInstance.html\" title=\"struct o1vm::legacy::folding::FoldingInstance\">FoldingInstance</a>&lt;N, G&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()