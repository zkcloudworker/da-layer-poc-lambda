(function() {var type_impls = {
"kimchi_optimism":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Witness%3CG%3E-for-FoldingWitness%3CN,+%3CG+as+AffineCurve%3E::ScalarField%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#90-103\">source</a><a href=\"#impl-Witness%3CG%3E-for-FoldingWitness%3CN,+%3CG+as+AffineCurve%3E::ScalarField%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, G: CommitmentCurve&gt; Witness&lt;G&gt; for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingWitness.html\" title=\"struct kimchi_optimism::folding::FoldingWitness\">FoldingWitness</a>&lt;N, G::ScalarField&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.combine\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#91-98\">source</a><a href=\"#method.combine\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">combine</a>(a: Self, b: Self, challenge: G::ScalarField) -&gt; Self</h4></section></summary><div class='docblock'>Should return a linear combination</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.rows\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#100-102\">source</a><a href=\"#method.rows\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">rows</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Returns the number of rows in the witness</div></details><section id=\"method.relax\" class=\"method trait-impl\"><a href=\"#method.relax\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">relax</a>(\n    self,\n    zero_poly: &amp;Evaluations&lt;&lt;G as AffineCurve&gt;::ScalarField, Radix2EvaluationDomain&lt;&lt;G as AffineCurve&gt;::ScalarField&gt;&gt;\n) -&gt; RelaxedWitness&lt;G, Self&gt;</h4></section></div></details>","Witness<G>","kimchi_optimism::keccak::folding::KeccakFoldingWitness","kimchi_optimism::mips::folding::MIPSFoldingWitness"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-FoldingWitness%3CN,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#impl-PartialEq-for-FoldingWitness%3CN,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + FftField&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingWitness.html\" title=\"struct kimchi_optimism::folding::FoldingWitness\">FoldingWitness</a>&lt;N, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingWitness.html\" title=\"struct kimchi_optimism::folding::FoldingWitness\">FoldingWitness</a>&lt;N, F&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","kimchi_optimism::keccak::folding::KeccakFoldingWitness","kimchi_optimism::mips::folding::MIPSFoldingWitness"],["<section id=\"impl-StructuralPartialEq-for-FoldingWitness%3CN,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#impl-StructuralPartialEq-for-FoldingWitness%3CN,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, F: FftField&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingWitness.html\" title=\"struct kimchi_optimism::folding::FoldingWitness\">FoldingWitness</a>&lt;N, F&gt;</h3></section>","StructuralPartialEq","kimchi_optimism::keccak::folding::KeccakFoldingWitness","kimchi_optimism::mips::folding::MIPSFoldingWitness"],["<section id=\"impl-StructuralEq-for-FoldingWitness%3CN,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#impl-StructuralEq-for-FoldingWitness%3CN,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, F: FftField&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralEq.html\" title=\"trait core::marker::StructuralEq\">StructuralEq</a> for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingWitness.html\" title=\"struct kimchi_optimism::folding::FoldingWitness\">FoldingWitness</a>&lt;N, F&gt;</h3></section>","StructuralEq","kimchi_optimism::keccak::folding::KeccakFoldingWitness","kimchi_optimism::mips::folding::MIPSFoldingWitness"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-FoldingWitness%3CN,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#impl-Clone-for-FoldingWitness%3CN,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + FftField&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingWitness.html\" title=\"struct kimchi_optimism::folding::FoldingWitness\">FoldingWitness</a>&lt;N, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingWitness.html\" title=\"struct kimchi_optimism::folding::FoldingWitness\">FoldingWitness</a>&lt;N, F&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","kimchi_optimism::keccak::folding::KeccakFoldingWitness","kimchi_optimism::mips::folding::MIPSFoldingWitness"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-FoldingWitness%3CN,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#impl-Debug-for-FoldingWitness%3CN,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + FftField&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingWitness.html\" title=\"struct kimchi_optimism::folding::FoldingWitness\">FoldingWitness</a>&lt;N, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","kimchi_optimism::keccak::folding::KeccakFoldingWitness","kimchi_optimism::mips::folding::MIPSFoldingWitness"],["<section id=\"impl-Eq-for-FoldingWitness%3CN,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#impl-Eq-for-FoldingWitness%3CN,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + FftField&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingWitness.html\" title=\"struct kimchi_optimism::folding::FoldingWitness\">FoldingWitness</a>&lt;N, F&gt;</h3></section>","Eq","kimchi_optimism::keccak::folding::KeccakFoldingWitness","kimchi_optimism::mips::folding::MIPSFoldingWitness"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hash-for-FoldingWitness%3CN,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#impl-Hash-for-FoldingWitness%3CN,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + FftField&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingWitness.html\" title=\"struct kimchi_optimism::folding::FoldingWitness\">FoldingWitness</a>&lt;N, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#85\">source</a><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash\" class=\"fn\">hash</a>&lt;__H: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>&gt;(&amp;self, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __H</a>)</h4></section></summary><div class='docblock'>Feeds this value into the given <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_slice\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.3.0\">1.3.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#238-240\">source</a></span><a href=\"#method.hash_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice\" class=\"fn\">hash_slice</a>&lt;H&gt;(data: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[Self]</a>, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Feeds a slice of this type into the given <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice\">Read more</a></div></details></div></details>","Hash","kimchi_optimism::keccak::folding::KeccakFoldingWitness","kimchi_optimism::mips::folding::MIPSFoldingWitness"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()