pub mod core;
#[cfg(test)]
mod tests{
	use std::collections::HashMap;
	use crate::core::*;

	#[test]
	fn evaluate(){
		let x=UnknownId::new(0);
		let sum=Plus::new(Scalar::new(1.0),Scalar::new(2.0));
		assert_eq!(sum.evaluate(),3.0);
		assert_eq!(sum.derivative(x).evaluate(),0.0);
	}
	#[test]
	fn try_evaluate(){
		let x=UnknownId::new(0);
		let sum=Plus::new(Scalar::new(2),x);
		let mut environment=HashMap::new();
		environment.insert(x,1);
		assert_eq!(sum.try_evaluate(&environment).unwrap(),3);
		assert_eq!(sum.derivative(x).evaluate(),1);
	}
}
