Casox Computer Algebra System
=============================

### Computer Algebra System Using Rust Types

I wrote this as a learning exercise to familliarize myself with Rust.  I was curious what would happen when you try to do derivatives at the type system level since I had already made a CAS in Lua (at least three actually).  It turns out it's very inflexible, and taking additional derivatives grows the expression and thus the type exponentially. There's no hope of dynamic simplification since the entire point is that the types are determined at compile time.  I still think it's cool and it might compile into decent assembly with a stricter evaluation method.

## Example
```rust
use std::collections::HashMap;
use casox::core::{pow,VariableGenerator,TryReplace,Evaluate,Derivative};

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

//try_replace will fail if any variable in the expression has no definition
assert_eq!(expr.try_replace(&env).unwrap().evaluate(),261.0);

//derivative with respect to x, then evaluate with the same environment
assert_eq!(expr.derivative(x).try_replace(&env).unwrap().evaluate(),411.0);

//create a new environment for display
let mut env2=HashMap::new();
env2.insert(x,"x");
env2.insert(y,"y");

//display the expression using the inner type's (str) Display trait implementation
//this will compile even if the Constant types do not match, where as
//.evaluate() will not compile unless mismatched types are wrapped in an
//expression that hides the inner type such as vector dot product
//which could hide vector types inside it by returning a fixed scalar type
assert_eq!(
	format!("{}",expr.derivative(x).try_replace(&env2).unwrap()),
	"1+y*1+0*x+x^y*((y*1)/x+log(x)*0)"
);
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
