(function() {var type_impls = {
"kimchi_optimism":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Trace%3CCOLUMNS,+SELECTOR,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_optimism/trace.rs.html#23-35\">source</a><a href=\"#impl-Trace%3CCOLUMNS,+SELECTOR,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const COLUMNS: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, SELECTOR: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>, F: <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a>&gt; <a class=\"struct\" href=\"kimchi_optimism/trace/struct.Trace.html\" title=\"struct kimchi_optimism::trace::Trace\">Trace</a>&lt;COLUMNS, SELECTOR, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.in_circuit\" class=\"method\"><a class=\"src rightside\" href=\"src/kimchi_optimism/trace.rs.html#25-27\">source</a><h4 class=\"code-header\">pub fn <a href=\"kimchi_optimism/trace/struct.Trace.html#tymethod.in_circuit\" class=\"fn\">in_circuit</a>(&amp;self, step: SELECTOR) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Returns a boolean indicating whether the witness for the given selector was ever found in the cirucit or not.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.reset\" class=\"method\"><a class=\"src rightside\" href=\"src/kimchi_optimism/trace.rs.html#30-34\">source</a><h4 class=\"code-header\">pub fn <a href=\"kimchi_optimism/trace/struct.Trace.html#tymethod.reset\" class=\"fn\">reset</a>(&amp;mut self, step: SELECTOR)</h4></section></summary><div class=\"docblock\"><p>Resets the witness after folding</p>\n</div></details></div></details>",0,"kimchi_optimism::keccak::trace::KeccakTrace","kimchi_optimism::mips::trace::MIPSTrace"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()