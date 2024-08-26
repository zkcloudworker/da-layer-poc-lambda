(function() {var implementors = {
"folding":[["impl&lt;'a, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = F&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"enum\" href=\"folding/eval_leaf/enum.EvalLeaf.html\" title=\"enum folding::eval_leaf::EvalLeaf\">EvalLeaf</a>&lt;'a, F&gt;&gt; for <a class=\"enum\" href=\"folding/eval_leaf/enum.EvalLeaf.html\" title=\"enum folding::eval_leaf::EvalLeaf\">EvalLeaf</a>&lt;'a, F&gt;"],["impl&lt;C: <a class=\"trait\" href=\"folding/trait.FoldingConfig.html\" title=\"trait folding::FoldingConfig\">FoldingConfig</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"enum\" href=\"folding/expressions/enum.FoldingCompatibleExpr.html\" title=\"enum folding::expressions::FoldingCompatibleExpr\">FoldingCompatibleExpr</a>&lt;C&gt;&gt; for <a class=\"enum\" href=\"folding/expressions/enum.FoldingCompatibleExpr.html\" title=\"enum folding::expressions::FoldingCompatibleExpr\">FoldingCompatibleExpr</a>&lt;C&gt;"],["impl&lt;C: <a class=\"trait\" href=\"folding/trait.FoldingConfig.html\" title=\"trait folding::FoldingConfig\">FoldingConfig</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"enum\" href=\"folding/expressions/enum.FoldingExp.html\" title=\"enum folding::expressions::FoldingExp\">FoldingExp</a>&lt;C&gt;&gt; for <a class=\"enum\" href=\"folding/expressions/enum.FoldingExp.html\" title=\"enum folding::expressions::FoldingExp\">FoldingExp</a>&lt;C&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"enum\" href=\"folding/expressions/enum.Degree.html\" title=\"enum folding::expressions::Degree\">Degree</a>&gt; for <a class=\"enum\" href=\"folding/expressions/enum.Degree.html\" title=\"enum folding::expressions::Degree\">Degree</a>"]],
"kimchi":[["impl&lt;F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;&amp;<a class=\"enum\" href=\"kimchi/snarky/cvar/enum.FieldVar.html\" title=\"enum kimchi::snarky::cvar::FieldVar\">FieldVar</a>&lt;F&gt;&gt; for &amp;<a class=\"enum\" href=\"kimchi/snarky/cvar/enum.FieldVar.html\" title=\"enum kimchi::snarky::cvar::FieldVar\">FieldVar</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: PrimeField,</span>"],["impl&lt;'a, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;&amp;'a <a class=\"enum\" href=\"kimchi/snarky/cvar/enum.FieldVar.html\" title=\"enum kimchi::snarky::cvar::FieldVar\">FieldVar</a>&lt;F&gt;&gt; for <a class=\"enum\" href=\"kimchi/snarky/cvar/enum.FieldVar.html\" title=\"enum kimchi::snarky::cvar::FieldVar\">FieldVar</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: PrimeField,</span>"],["impl&lt;T: <a class=\"trait\" href=\"kimchi/circuits/expr/trait.Literal.html\" title=\"trait kimchi::circuits::expr::Literal\">Literal</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"enum\" href=\"kimchi/circuits/expr/enum.Operations.html\" title=\"enum kimchi::circuits::expr::Operations\">Operations</a>&lt;T&gt;&gt; for <a class=\"enum\" href=\"kimchi/circuits/expr/enum.Operations.html\" title=\"enum kimchi::circuits::expr::Operations\">Operations</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"kimchi/circuits/expr/trait.Literal.html#associatedtype.F\" title=\"type kimchi::circuits::expr::Literal::F\">F</a>: Field,</span>"],["impl&lt;F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"enum\" href=\"kimchi/snarky/cvar/enum.FieldVar.html\" title=\"enum kimchi::snarky::cvar::FieldVar\">FieldVar</a>&lt;F&gt;&gt; for &amp;<a class=\"enum\" href=\"kimchi/snarky/cvar/enum.FieldVar.html\" title=\"enum kimchi::snarky::cvar::FieldVar\">FieldVar</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: PrimeField,</span>"],["impl&lt;F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"enum\" href=\"kimchi/snarky/cvar/enum.FieldVar.html\" title=\"enum kimchi::snarky::cvar::FieldVar\">FieldVar</a>&lt;F&gt;&gt; for <a class=\"enum\" href=\"kimchi/snarky/cvar/enum.FieldVar.html\" title=\"enum kimchi::snarky::cvar::FieldVar\">FieldVar</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F: PrimeField,</span>"]],
"mvpoly":[["impl&lt;F: PrimeField, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;&amp;<a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;&gt; for &amp;<a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;"],["impl&lt;F: PrimeField, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;&gt; for <a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;"],["impl&lt;F: PrimeField, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;&gt; for &amp;<a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;"],["impl&lt;F: PrimeField, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;&amp;<a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;&gt; for <a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;"]],
"poly_commitment":[["impl&lt;'a, 'b, C: AffineCurve&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;&amp;'a <a class=\"struct\" href=\"poly_commitment/commitment/struct.PolyComm.html\" title=\"struct poly_commitment::commitment::PolyComm\">PolyComm</a>&lt;C&gt;&gt; for &amp;'b <a class=\"struct\" href=\"poly_commitment/commitment/struct.PolyComm.html\" title=\"struct poly_commitment::commitment::PolyComm\">PolyComm</a>&lt;C&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()