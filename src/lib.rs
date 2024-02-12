pub mod core;
#[cfg(test)]
mod tests{
	use std::collections::HashMap;
	use crate::core::*;

	#[test]
	fn evaluate(){
		let x=VariableId::new(0);
		let one=Constant::new(1.0);
		let two=Constant::new(2.0);
		let sum=one+two;
		assert_eq!(sum.evaluate(),3.0);
		assert_eq!(sum.derivative(x).evaluate(),0.0);
	}
	#[test]
	fn try_evaluate(){
		let x=VariableId::new(0);
		let sum=Constant::new(2)+x;
		let mut environment=HashMap::new();
		environment.insert(x,1);
		assert_eq!(sum.try_evaluate(&environment).unwrap(),3);
		assert_eq!(sum.derivative(x).evaluate(),1);
	}
	#[test]
	fn derivative(){
		let mut gen=VariableGenerator::new();
		let x=gen.var();//VariableId::new(0);
		let y=gen.var();//VariableId::new(1);
		let expr=x+y*x+pow(x,y);
		let mut environment=HashMap::new();
		environment.insert(x,3.0);//x=3.0
		environment.insert(y,5.0);//y=5.0
		assert_eq!(expr.try_evaluate(&environment).unwrap(),261.0);
		assert_eq!(expr.derivative(x).try_evaluate(&environment).unwrap(),411.0);
		assert_eq!(expr.derivative(y).derivative(x).try_evaluate(&environment).unwrap(),1174.938);
		assert_eq!(expr.derivative(x).derivative(x).derivative(x).try_evaluate(&environment).unwrap(),540.0);
	}
}
