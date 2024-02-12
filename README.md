Casox Computer Algebra System
=============================

### Computer Algebra System Using Rust Types

## Example
```rust
use std::collections::HashMap;
use casox::core::{pow,VariableGenerator,TryEvaluate,Derivative};

//Generate some varables using VariableGenerator utility
let mut gen=VariableGenerator::new();
//Variables are just wrapped u32 ids
//let x=VariableId::new(0);
let x=gen.var();
let y=gen.var();//VariableId::new(1)

//create an expression
let expr=x+y*x+pow(x,y);

//create an evaluation environment
let mut env=HashMap::new();
//set the values for the variables
env.insert(x,3.0);//x=3.0
env.insert(y,5.0);//y=5.0

//try evaluate will fail if all variables in the expression are not filled
assert_eq!(expr.try_evaluate(&env).unwrap(),261.0);

//derivative with respect to x, then evaluate with the same environment
assert_eq!(expr.derivative(x).try_evaluate(&env).unwrap(),411.0);
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>