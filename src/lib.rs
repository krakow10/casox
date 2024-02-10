pub mod core;
#[cfg(test)]
mod tests{
	use crate::core::*;

	#[test]
	fn sum(){
		let sum=Add::new(Scalar::new(1.0),Scalar::new(2.0));
		assert_eq!(sum.evaluate(),3.0);
		assert_eq!(sum.derivative().evaluate(),0.0);
	}
}
