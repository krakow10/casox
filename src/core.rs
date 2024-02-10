pub trait Zero{
	fn zero()->Self;
}

pub trait Evaluate<T>{
	fn evaluate(&self)->T;
}
pub trait Derivative{
	type Derivative;
	fn derivative(&self)->Self::Derivative;
}

//f32
impl Zero for f32{
	fn zero()->f32{
		0.0
	}
}

pub struct Scalar<T>(T);
impl<T> Scalar<T>{
	pub fn new(value:T)->Self{
		Self(value)
	}
}
impl<T:Copy> Evaluate<T> for Scalar<T>{
	fn evaluate(&self)->T{
		self.0
	}
}
impl<A:Zero> Derivative for Scalar<A>{
	type Derivative=Self;
	fn derivative(&self)->Self{
		Self(A::zero())
	}
}

pub struct Add<A,B>(A,B);
impl<A,B> Add<A,B>{
	pub fn new(a:A,b:B)->Self{
		Self(a,b)
	}
}
impl<T:std::ops::Add<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Add<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()+self.1.evaluate()
	}
}
impl<A:Derivative,B:Derivative> Derivative for Add<A,B>{
	type Derivative=Add<A::Derivative,B::Derivative>;
	fn derivative(&self)->Add<A::Derivative,B::Derivative>{
		Add(self.0.derivative(),self.1.derivative())
	}
}