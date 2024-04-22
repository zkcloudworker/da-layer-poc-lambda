(function() {var type_impls = {
"kimchi_msm":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Logup%3CF,+ID%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_msm/logup.rs.html#167-180\">source</a><a href=\"#impl-Logup%3CF,+ID%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F, ID&gt; <a class=\"struct\" href=\"kimchi_msm/logup/struct.Logup.html\" title=\"struct kimchi_msm::logup::Logup\">Logup</a>&lt;F, ID&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    ID: <a class=\"trait\" href=\"kimchi_msm/logup/trait.LookupTableID.html\" title=\"trait kimchi_msm::logup::LookupTableID\">LookupTableID</a>,</div></h3></section></summary><div class=\"docblock\"><p>Basic trait for logarithmic lookups.</p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/kimchi_msm/logup.rs.html#173-179\">source</a><h4 class=\"code-header\">pub fn <a href=\"kimchi_msm/logup/struct.Logup.html#tymethod.new\" class=\"fn\">new</a>(table_id: ID, numerator: F, value: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[F]</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new Logup</p>\n</div></details></div></details>",0,"kimchi_msm::lookups::Lookup","kimchi_msm::serialization::Lookup"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Logup%3CF,+ID%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_msm/logup.rs.html#159\">source</a><a href=\"#impl-Clone-for-Logup%3CF,+ID%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, ID: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"kimchi_msm/logup/trait.LookupTableID.html\" title=\"trait kimchi_msm::logup::LookupTableID\">LookupTableID</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"kimchi_msm/logup/struct.Logup.html\" title=\"struct kimchi_msm::logup::Logup\">Logup</a>&lt;F, ID&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_msm/logup.rs.html#159\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"kimchi_msm/logup/struct.Logup.html\" title=\"struct kimchi_msm::logup::Logup\">Logup</a>&lt;F, ID&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","kimchi_msm::lookups::Lookup","kimchi_msm::serialization::Lookup"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Logup%3CF,+ID%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi_msm/logup.rs.html#159\">source</a><a href=\"#impl-Debug-for-Logup%3CF,+ID%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, ID: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"kimchi_msm/logup/trait.LookupTableID.html\" title=\"trait kimchi_msm::logup::LookupTableID\">LookupTableID</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"kimchi_msm/logup/struct.Logup.html\" title=\"struct kimchi_msm::logup::Logup\">Logup</a>&lt;F, ID&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi_msm/logup.rs.html#159\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","kimchi_msm::lookups::Lookup","kimchi_msm::serialization::Lookup"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()