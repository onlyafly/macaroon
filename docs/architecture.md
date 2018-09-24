# Architecture Research

## Implementation of Tail Call Optimization (TCO)

https://en.wikipedia.org/wiki/Tail_call

### TCO Option 0: Clojure-style loop/recur

    (defn fib_iterative
        [n]
        (loop [n n
                accum1 1
                accum2 1]
            (if (< n 2)
            accum1
            (recur (- n 1) (+ accum1 accum2) accum1))))

### TCO Option 1: Trampoline

https://eli.thegreenplace.net/2017/on-recursion-continuations-and-trampolines/

### TCO Option 2: Transformation to Continuation Passing Style

"CONS Should Not CONS Its Arguments, Part II: Cheney on the M.T.A." by Henry Baker

* http://home.pipeline.com/~hbaker1/CheneyMTA.html

### Other

"Three Implementation Models for Scheme" by R. Kent Dybvig

* http://www.cs.indiana.edu/~dyb/papers/3imp.pdf
* http://agl.cs.unm.edu/~williams/cs491/three-imp.pdf

## Options for creating a AST

### Option 1: Enum with Boxed Children

    pub enum Expr{
        Unary(Operator, Box<Expr>),
        Binary(Box<Expr>, Operator, Box<Expr>),
        Grouping(Box<Expr>),
        Literal(Literal)
    }

Used by:

A. https://github.com/joncatanio/cannoli/blob/master/src/parser/ast.rs
B. https://github.com/jDomantas/plank/blob/master/plank-syntax/src/ast.rs
C. https://github.com/rpjohnst/dejavu/blob/master/src/front/ast.rs
D. https://github.com/wu-lang/wu/blob/master/src/wu/parser/ast.rs
E. https://github.com/murarth/ketos/blob/master/src/ketos/value.rs

To do errors:

B. Box enum's fields in "Spanned", and then dereference it away.

https://github.com/jDomantas/plank/blob/master/plank-syntax/src/position.rs
https://github.com/antoyo/tiger-rs/blob/master/tiger/src/position.rs <-- CLEAN!
https://github.com/antoyo/tiger-rs/blob/master/tiger/src/ast.rs

D. Each Value is a struct first, containing a position and an enum, witht the remaining fields. See below and: https://github.com/wu-lang/wu/blob/master/src/wu/parser/ast.rs

    pub struct Expression {
        pub value: ExpressionValue,
        pub pos:  Pos
    }

https://stackoverflow.com/questions/45024211/menhir-associate-ast-Values-with-token-locations-in-source-file

Position can be an enum with None,Full(filename, line, char)... Use match patterns well

### Option 2: Structs Wrapped in Enums

    enum Expr {
        ...
        Binary(Box<BinaryExpr>)
    }

    struct BinaryExpr {
        op: Operator
        lhs: Expr, rhs: Expr,
    }

A. https://github.com/matklad/miniml/blob/master/ast/src/exprs.rs

### Option 3: 

Parse tree:

    struct ValueType(u32);

    struct Value {
        type: ValueType,  
        text_range: (usize, usize),
        children: Vec<Value>
    }

Type Layer on Top:

    struct BinaryExpression {
        value: Value
    }

    impl BinaryExpression {
        fn lhs(&self) -> Expression { Expression { value:  self.children[0] } }
    }