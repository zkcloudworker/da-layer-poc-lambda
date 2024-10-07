(function() {var implementors = {
"folding":[["impl&lt;'a, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;Output = F&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;F&gt; for <a class=\"enum\" href=\"folding/eval_leaf/enum.EvalLeaf.html\" title=\"enum folding::eval_leaf::EvalLeaf\">EvalLeaf</a>&lt;'a, F&gt;"],["impl&lt;'a, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;Output = F&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"enum\" href=\"folding/eval_leaf/enum.EvalLeaf.html\" title=\"enum folding::eval_leaf::EvalLeaf\">EvalLeaf</a>&lt;'a, F&gt;&gt; for <a class=\"enum\" href=\"folding/eval_leaf/enum.EvalLeaf.html\" title=\"enum folding::eval_leaf::EvalLeaf\">EvalLeaf</a>&lt;'a, F&gt;"],["impl&lt;C: <a class=\"trait\" href=\"folding/trait.FoldingConfig.html\" title=\"trait folding::FoldingConfig\">FoldingConfig</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;<a class=\"struct\" href=\"folding/expressions/struct.Term.html\" title=\"struct folding::expressions::Term\">Term</a>&lt;C&gt;&gt; for &amp;<a class=\"struct\" href=\"folding/expressions/struct.Term.html\" title=\"struct folding::expressions::Term\">Term</a>&lt;C&gt;"],["impl&lt;C: <a class=\"trait\" href=\"folding/trait.FoldingConfig.html\" title=\"trait folding::FoldingConfig\">FoldingConfig</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"enum\" href=\"folding/expressions/enum.FoldingCompatibleExpr.html\" title=\"enum folding::expressions::FoldingCompatibleExpr\">FoldingCompatibleExpr</a>&lt;C&gt;&gt; for <a class=\"enum\" href=\"folding/expressions/enum.FoldingCompatibleExpr.html\" title=\"enum folding::expressions::FoldingCompatibleExpr\">FoldingCompatibleExpr</a>&lt;C&gt;"],["impl&lt;C: <a class=\"trait\" href=\"folding/trait.FoldingConfig.html\" title=\"trait folding::FoldingConfig\">FoldingConfig</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"enum\" href=\"folding/expressions/enum.FoldingExp.html\" title=\"enum folding::expressions::FoldingExp\">FoldingExp</a>&lt;C&gt;&gt; for <a class=\"enum\" href=\"folding/expressions/enum.FoldingExp.html\" title=\"enum folding::expressions::FoldingExp\">FoldingExp</a>&lt;C&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;<a class=\"enum\" href=\"folding/expressions/enum.Degree.html\" title=\"enum folding::expressions::Degree\">Degree</a>&gt; for &amp;<a class=\"enum\" href=\"folding/expressions/enum.Degree.html\" title=\"enum folding::expressions::Degree\">Degree</a>"]],
"kimchi":[["impl&lt;'a, F: Field, Column: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, ChallengeTerm: <a class=\"trait\" href=\"kimchi/circuits/expr/trait.AlphaChallengeTerm.html\" title=\"trait kimchi::circuits::expr::AlphaChallengeTerm\">AlphaChallengeTerm</a>&lt;'a&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;F&gt; for <a class=\"type\" href=\"kimchi/circuits/expr/type.Expr.html\" title=\"type kimchi::circuits::expr::Expr\">Expr</a>&lt;<a class=\"type\" href=\"kimchi/circuits/expr/type.ConstantExpr.html\" title=\"type kimchi::circuits::expr::ConstantExpr\">ConstantExpr</a>&lt;F, ChallengeTerm&gt;, Column&gt;"],["impl&lt;T: <a class=\"trait\" href=\"kimchi/circuits/expr/trait.Literal.html\" title=\"trait kimchi::circuits::expr::Literal\">Literal</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"enum\" href=\"kimchi/circuits/expr/enum.Operations.html\" title=\"enum kimchi::circuits::expr::Operations\">Operations</a>&lt;T&gt;&gt; for <a class=\"enum\" href=\"kimchi/circuits/expr/enum.Operations.html\" title=\"enum kimchi::circuits::expr::Operations\">Operations</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"kimchi/circuits/expr/trait.Literal.html#associatedtype.F\" title=\"type kimchi::circuits::expr::Literal::F\">F</a>: Field,</span>"]],
"mvpoly":[["impl&lt;F: PrimeField, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;&gt; for <a class=\"struct\" href=\"mvpoly/prime/struct.Dense.html\" title=\"struct mvpoly::prime::Dense\">Dense</a>&lt;F, N, D&gt;"],["impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.usize.html\">usize</a>, F: PrimeField&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"struct\" href=\"mvpoly/monomials/struct.Sparse.html\" title=\"struct mvpoly::monomials::Sparse\">Sparse</a>&lt;F, N, D&gt;&gt; for <a class=\"struct\" href=\"mvpoly/monomials/struct.Sparse.html\" title=\"struct mvpoly::monomials::Sparse\">Sparse</a>&lt;F, N, D&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()