(function() {var type_impls = {
"kimchi_optimism":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-FoldingInstance%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#117\">source</a><a href=\"#impl-Debug-for-FoldingInstance%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingInstance.html\" title=\"struct kimchi_optimism::folding::FoldingInstance\">FoldingInstance</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#117\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","kimchi_optimism::keccak::folding::KeccakFoldingInstance","kimchi_optimism::mips::folding::MIPSFoldingInstance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-FoldingInstance%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#117\">source</a><a href=\"#impl-Clone-for-FoldingInstance%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingInstance.html\" title=\"struct kimchi_optimism::folding::FoldingInstance\">FoldingInstance</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#117\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingInstance.html\" title=\"struct kimchi_optimism::folding::FoldingInstance\">FoldingInstance</a>&lt;N&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","kimchi_optimism::keccak::folding::KeccakFoldingInstance","kimchi_optimism::mips::folding::MIPSFoldingInstance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Instance%3CGroupAffine%3C%3CParameters+as+BnParameters%3E::G1Parameters%3E%3E-for-FoldingInstance%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#131-141\">source</a><a href=\"#impl-Instance%3CGroupAffine%3C%3CParameters+as+BnParameters%3E::G1Parameters%3E%3E-for-FoldingInstance%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; Instance&lt;GroupAffine&lt;&lt;Parameters as BnParameters&gt;::G1Parameters&gt;&gt; for <a class=\"struct\" href=\"kimchi_optimism/folding/struct.FoldingInstance.html\" title=\"struct kimchi_optimism::folding::FoldingInstance\">FoldingInstance</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.combine\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/folding.rs.html#132-140\">source</a><a href=\"#method.combine\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">combine</a>(a: Self, b: Self, challenge: <a class=\"type\" href=\"kimchi_optimism/folding/type.Fp.html\" title=\"type kimchi_optimism::folding::Fp\">Fp</a>) -&gt; Self</h4></section></summary><div class='docblock'>Combine two instances ‘a’ and ‘b’ into a new instance.\nSee page 15.</div></details><section id=\"method.relax\" class=\"method trait-impl\"><a href=\"#method.relax\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">relax</a>(self, zero_commit: PolyComm&lt;G&gt;) -&gt; RelaxedInstance&lt;G, Self&gt;</h4></section></div></details>","Instance<GroupAffine<<Parameters as BnParameters>::G1Parameters>>","kimchi_optimism::keccak::folding::KeccakFoldingInstance","kimchi_optimism::mips::folding::MIPSFoldingInstance"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()