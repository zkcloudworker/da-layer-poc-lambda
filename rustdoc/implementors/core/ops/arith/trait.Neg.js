(function() {var implementors = {
"folding":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"enum\" href=\"folding/expressions/enum.Sign.html\" title=\"enum folding::expressions::Sign\">Sign</a>"],["impl&lt;C: <a class=\"trait\" href=\"folding/trait.FoldingConfig.html\" title=\"trait folding::FoldingConfig\">FoldingConfig</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"folding/expressions/struct.Term.html\" title=\"struct folding::expressions::Term\">Term</a>&lt;C&gt;"]],
"kimchi":[["impl&lt;F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for &amp;<a class=\"enum\" href=\"kimchi/snarky/cvar/enum.FieldVar.html\" title=\"enum kimchi::snarky::cvar::FieldVar\">FieldVar</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: PrimeField,</span>"],["impl&lt;T: <a class=\"trait\" href=\"kimchi/circuits/expr/trait.Literal.html\" title=\"trait kimchi::circuits::expr::Literal\">Literal</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"enum\" href=\"kimchi/circuits/expr/enum.Operations.html\" title=\"enum kimchi::circuits::expr::Operations\">Operations</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"kimchi/circuits/expr/trait.Literal.html#associatedtype.F\" title=\"type kimchi::circuits::expr::Literal::F\">F</a>: <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = T::<a class=\"associatedtype\" href=\"kimchi/circuits/expr/trait.Literal.html#associatedtype.F\" title=\"type kimchi::circuits::expr::Literal::F\">F</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,</span>"],["impl&lt;F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"enum\" href=\"kimchi/snarky/cvar/enum.FieldVar.html\" title=\"enum kimchi::snarky::cvar::FieldVar\">FieldVar</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: PrimeField,</span>"]],
"mvpoly":[["impl&lt;F: PrimeField, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for &amp;<a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;"],["impl&lt;F: PrimeField, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()