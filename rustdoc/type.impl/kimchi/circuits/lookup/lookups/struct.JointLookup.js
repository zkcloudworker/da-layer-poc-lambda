(function() {var type_impls = {
"kimchi":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-JointLookup%3CF,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#325-336\">source</a><a href=\"#impl-JointLookup%3CF,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F: <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = F&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;&gt; <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::JointLookup\">JointLookup</a>&lt;F, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.evaluate\" class=\"method\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#328-335\">source</a><h4 class=\"code-header\">pub fn <a href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html#tymethod.evaluate\" class=\"fn\">evaluate</a>(&amp;self, joint_combiner: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;F</a>, table_id_combiner: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;F</a>) -&gt; F</h4></section></summary><div class=\"docblock\"><p>Evaluate the combined value of a joint-lookup.</p>\n</div></details></div></details>",0,"kimchi::circuits::lookup::lookups::JointLookupSpec"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-JointLookup%3CSingleLookup%3CF%3E,+LookupTableID%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#338-378\">source</a><a href=\"#impl-JointLookup%3CSingleLookup%3CF%3E,+LookupTableID%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>&gt; <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::JointLookup\">JointLookup</a>&lt;<a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.SingleLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::SingleLookup\">SingleLookup</a>&lt;F&gt;, <a class=\"enum\" href=\"kimchi/circuits/lookup/lookups/enum.LookupTableID.html\" title=\"enum kimchi::circuits::lookup::lookups::LookupTableID\">LookupTableID</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.reduce\" class=\"method\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#341-359\">source</a><h4 class=\"code-header\">pub fn <a href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html#tymethod.reduce\" class=\"fn\">reduce</a>&lt;K, G: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(<a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.LocalPosition.html\" title=\"struct kimchi::circuits::lookup::lookups::LocalPosition\">LocalPosition</a>) -&gt; K&gt;(\n    &amp;self,\n    eval: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;G</a>\n) -&gt; <a class=\"type\" href=\"kimchi/circuits/lookup/lookups/type.JointLookupValue.html\" title=\"type kimchi::circuits::lookup::lookups::JointLookupValue\">JointLookupValue</a>&lt;K&gt;<span class=\"where fmt-newline\">where\n    K: <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;F, Output = K&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = K&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;,</span></h4></section></summary><div class=\"docblock\"><p>Reduce linear combinations in the lookup entries to a single value, resolving local\npositions using the given function.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.evaluate\" class=\"method\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#363-377\">source</a><h4 class=\"code-header\">pub fn <a href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html#tymethod.evaluate\" class=\"fn\">evaluate</a>&lt;K, G: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(<a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.LocalPosition.html\" title=\"struct kimchi::circuits::lookup::lookups::LocalPosition\">LocalPosition</a>) -&gt; K&gt;(\n    &amp;self,\n    joint_combiner: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;K</a>,\n    table_id_combiner: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;K</a>,\n    eval: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;G</a>\n) -&gt; K<span class=\"where fmt-newline\">where\n    K: <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;F, Output = K&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = K&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;,</span></h4></section></summary><div class=\"docblock\"><p>Evaluate the combined value of a joint-lookup, resolving local positions using the given\nfunction.</p>\n</div></details></div></details>",0,"kimchi::circuits::lookup::lookups::JointLookupSpec","kimchi::circuits::lookup::lookups::JointLookupValue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-JointLookup%3CSingleLookup,+LookupTableID%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#308\">source</a><a href=\"#impl-Debug-for-JointLookup%3CSingleLookup,+LookupTableID%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;SingleLookup: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, LookupTableID: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::JointLookup\">JointLookup</a>&lt;SingleLookup, LookupTableID&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#308\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","kimchi::circuits::lookup::lookups::JointLookupSpec","kimchi::circuits::lookup::lookups::JointLookupValue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SerializeAs%3CJointLookup%3CF,+F%3E%3E-for-JointLookup%3CG,+G%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/serialization_helper.rs.html#11-27\">source</a><a href=\"#impl-SerializeAs%3CJointLookup%3CF,+F%3E%3E-for-JointLookup%3CG,+G%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F, G&gt; <a class=\"trait\" href=\"https://docs.rs/serde_with/1.14.0/serde_with/ser/trait.SerializeAs.html\" title=\"trait serde_with::ser::SerializeAs\">SerializeAs</a>&lt;<a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::JointLookup\">JointLookup</a>&lt;F, F&gt;&gt; for <a class=\"type\" href=\"kimchi/circuits/lookup/lookups/type.JointLookupValue.html\" title=\"type kimchi::circuits::lookup::lookups::JointLookupValue\">JointLookupValue</a>&lt;G&gt;<span class=\"where fmt-newline\">where\n    G: <a class=\"trait\" href=\"https://docs.rs/serde_with/1.14.0/serde_with/ser/trait.SerializeAs.html\" title=\"trait serde_with::ser::SerializeAs\">SerializeAs</a>&lt;F&gt;,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.serialize_as\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/serialization_helper.rs.html#15-26\">source</a><a href=\"#method.serialize_as\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde_with/1.14.0/serde_with/ser/trait.SerializeAs.html#tymethod.serialize_as\" class=\"fn\">serialize_as</a>&lt;S&gt;(\n    source: &amp;<a class=\"type\" href=\"kimchi/circuits/lookup/lookups/type.JointLookupValue.html\" title=\"type kimchi::circuits::lookup::lookups::JointLookupValue\">JointLookupValue</a>&lt;F&gt;,\n    serializer: S\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;S::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serializer.html#associatedtype.Ok\" title=\"type serde::ser::Serializer::Ok\">Ok</a>, S::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serializer.html#associatedtype.Error\" title=\"type serde::ser::Serializer::Error\">Error</a>&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,</span></h4></section></summary><div class='docblock'>Serialize this value into the given Serde serializer.</div></details></div></details>","SerializeAs<JointLookup<F, F>>","kimchi::circuits::lookup::lookups::JointLookupSpec"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DeserializeAs%3C'de,+JointLookup%3CF,+F%3E%3E-for-JointLookup%3CG,+G%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/serialization_helper.rs.html#29-193\">source</a><a href=\"#impl-DeserializeAs%3C'de,+JointLookup%3CF,+F%3E%3E-for-JointLookup%3CG,+G%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de, F, G: <a class=\"trait\" href=\"https://docs.rs/serde_with/1.14.0/serde_with/de/trait.DeserializeAs.html\" title=\"trait serde_with::de::DeserializeAs\">DeserializeAs</a>&lt;'de, F&gt;&gt; <a class=\"trait\" href=\"https://docs.rs/serde_with/1.14.0/serde_with/de/trait.DeserializeAs.html\" title=\"trait serde_with::de::DeserializeAs\">DeserializeAs</a>&lt;'de, <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::JointLookup\">JointLookup</a>&lt;F, F&gt;&gt; for <a class=\"type\" href=\"kimchi/circuits/lookup/lookups/type.JointLookupValue.html\" title=\"type kimchi::circuits::lookup::lookups::JointLookupValue\">JointLookupValue</a>&lt;G&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deserialize_as\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/serialization_helper.rs.html#32-192\">source</a><a href=\"#method.deserialize_as\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde_with/1.14.0/serde_with/de/trait.DeserializeAs.html#tymethod.deserialize_as\" class=\"fn\">deserialize_as</a>&lt;D&gt;(deserializer: D) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"type\" href=\"kimchi/circuits/lookup/lookups/type.JointLookupValue.html\" title=\"type kimchi::circuits::lookup::lookups::JointLookupValue\">JointLookupValue</a>&lt;F&gt;, D::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.171/serde/de/trait.Deserializer.html#associatedtype.Error\" title=\"type serde::de::Deserializer::Error\">Error</a>&gt;<span class=\"where fmt-newline\">where\n    D: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.171/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,</span></h4></section></summary><div class='docblock'>Deserialize this value from the given Serde deserializer.</div></details></div></details>","DeserializeAs<'de, JointLookup<F, F>>","kimchi::circuits::lookup::lookups::JointLookupSpec"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-JointLookup%3CSingleLookup,+LookupTableID%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#308\">source</a><a href=\"#impl-Clone-for-JointLookup%3CSingleLookup,+LookupTableID%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;SingleLookup: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, LookupTableID: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::JointLookup\">JointLookup</a>&lt;SingleLookup, LookupTableID&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#308\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::JointLookup\">JointLookup</a>&lt;SingleLookup, LookupTableID&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","kimchi::circuits::lookup::lookups::JointLookupSpec","kimchi::circuits::lookup::lookups::JointLookupValue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deserialize%3C'de%3E-for-JointLookup%3CSingleLookup,+LookupTableID%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#308\">source</a><a href=\"#impl-Deserialize%3C'de%3E-for-JointLookup%3CSingleLookup,+LookupTableID%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de, SingleLookup, LookupTableID&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.171/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::JointLookup\">JointLookup</a>&lt;SingleLookup, LookupTableID&gt;<span class=\"where fmt-newline\">where\n    SingleLookup: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.171/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,\n    LookupTableID: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.171/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deserialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#308\">source</a><a href=\"#method.deserialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.171/serde/de/trait.Deserialize.html#tymethod.deserialize\" class=\"fn\">deserialize</a>&lt;__D&gt;(__deserializer: __D) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, __D::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.171/serde/de/trait.Deserializer.html#associatedtype.Error\" title=\"type serde::de::Deserializer::Error\">Error</a>&gt;<span class=\"where fmt-newline\">where\n    __D: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.171/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,</span></h4></section></summary><div class='docblock'>Deserialize this value from the given Serde deserializer. <a href=\"https://docs.rs/serde/1.0.171/serde/de/trait.Deserialize.html#tymethod.deserialize\">Read more</a></div></details></div></details>","Deserialize<'de>","kimchi::circuits::lookup::lookups::JointLookupSpec","kimchi::circuits::lookup::lookups::JointLookupValue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Serialize-for-JointLookup%3CSingleLookup,+LookupTableID%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#308\">source</a><a href=\"#impl-Serialize-for-JointLookup%3CSingleLookup,+LookupTableID%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;SingleLookup, LookupTableID&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"kimchi/circuits/lookup/lookups/struct.JointLookup.html\" title=\"struct kimchi::circuits::lookup::lookups::JointLookup\">JointLookup</a>&lt;SingleLookup, LookupTableID&gt;<span class=\"where fmt-newline\">where\n    SingleLookup: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,\n    LookupTableID: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.serialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/kimchi/circuits/lookup/lookups.rs.html#308\">source</a><a href=\"#method.serialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serialize.html#tymethod.serialize\" class=\"fn\">serialize</a>&lt;__S&gt;(&amp;self, __serializer: __S) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;__S::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serializer.html#associatedtype.Ok\" title=\"type serde::ser::Serializer::Ok\">Ok</a>, __S::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serializer.html#associatedtype.Error\" title=\"type serde::ser::Serializer::Error\">Error</a>&gt;<span class=\"where fmt-newline\">where\n    __S: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,</span></h4></section></summary><div class='docblock'>Serialize this value into the given Serde serializer. <a href=\"https://docs.rs/serde/1.0.171/serde/ser/trait.Serialize.html#tymethod.serialize\">Read more</a></div></details></div></details>","Serialize","kimchi::circuits::lookup::lookups::JointLookupSpec","kimchi::circuits::lookup::lookups::JointLookupValue"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()