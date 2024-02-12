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
		let mut env=HashMap::new();
		env.insert(x,1);
		assert_eq!(sum.try_replace(&env).unwrap().evaluate(),3);
		assert_eq!(sum.derivative(x).evaluate(),1);
	}
	#[test]
	fn derivative(){
		let mut gen=VariableGenerator::new();
		let x=gen.var();//VariableId::new(0);
		let y=gen.var();//VariableId::new(1);
		let expr=x+y*x+pow(x,y);
		let mut env=HashMap::new();
		env.insert(x,3.0);//x=3.0
		env.insert(y,5.0);//y=5.0
		assert_eq!(expr.try_replace(&env).unwrap().evaluate(),261.0);
		assert_eq!(expr.derivative(x).try_replace(&env).unwrap().evaluate(),411.0);
		assert_eq!(expr.derivative(y).derivative(x).try_replace(&env).unwrap().evaluate(),1174.938);
		assert_eq!(expr.derivative(x).derivative(x).derivative(x).try_replace(&env).unwrap().evaluate(),540.0);
	}
	#[test]
	fn display(){
		let mut gen=VariableGenerator::new();
		let x=gen.var();
		let y=gen.var();
		let _1=Constant::new("1");
		let _2=Constant::new("2");
		let expr=x+y*(x+_1)+pow(pow(x,_2),y)+log(x);
		let mut env=HashMap::new();
		env.insert(x,"x");
		env.insert(y,"y");
		assert_eq!(format!("{}",expr.try_replace(&env).unwrap()),"x+y*(x+1)+(x^2)^y+log(x)");
	}
}
