use std::collections::HashMap;

pub trait Zero{
	fn zero()->Self;
}
pub trait Identity{
	fn identity()->Self;
}
pub trait Unit{
	fn unit(&self)->Self;
}

pub trait Evaluate<T>{
	fn evaluate(&self)->T;
}
#[derive(Debug)]
pub enum TryEvaluateError{
	MissingUnknown(UnknownId),
}
impl std::fmt::Display for TryEvaluateError{
	fn fmt(&self,state:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		write!(state,"{self:?}")
	}
}
impl std::error::Error for TryEvaluateError{}
pub trait TryEvaluate<T>{
	fn try_evaluate(&self,values:&HashMap<UnknownId,T>)->Result<T,TryEvaluateError>;
}
//TODO: implement TryEvaluate implicitly if the type implements Evaluate
// impl<T,Eval:Evaluate<T>> TryEvaluate<T> for Eval{
// 	fn try_evaluate(&self,_values:&HashMap<UnknownId,T>)->Result<T,TryEvaluateError>{
// 		Ok(self.evaluate())
// 	}
// }
pub trait Derivative{
	type Derivative;
	fn derivative(&self,unknown_id:UnknownId)->Self::Derivative;
}

//f32
impl Zero for f32{
	fn zero()->f32{
		0.0
	}
}
impl Identity for f32{
	fn identity()->f32{
		1.0
	}
}
//i32
impl Zero for i32{
	fn zero()->i32{
		0
	}
}
impl Identity for i32{
	fn identity()->i32{
		1
	}
}

//turns into the respective typed value during evaluation
pub enum Morph{
	Zero,
	Identity,
}
impl<T:Zero+Identity> Evaluate<T> for Morph{
	fn evaluate(&self)->T{
		match self{
			Morph::Zero=>T::zero(),
			Morph::Identity=>T::identity(),
		}
	}
}

//represents an unknown value.  gains the type of the current evaluation.
#[derive(Clone,Copy,Debug,Hash,Eq,PartialEq)]
pub struct UnknownId(u32);
impl UnknownId{
	pub fn new(value:u32)->Self{
		Self(value)
	}
}
impl<T:Copy> TryEvaluate<T> for UnknownId{
	fn try_evaluate(&self,values:&HashMap<UnknownId,T>)->Result<T,TryEvaluateError>{
		values.get(self).copied().ok_or(TryEvaluateError::MissingUnknown(*self))
	}
}
impl Derivative for UnknownId{
	type Derivative=Morph;
	fn derivative(&self,unknown_id:UnknownId)->Self::Derivative{
		if *self==unknown_id{
			Morph::Identity
		}else{
			Morph::Zero
		}
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
//TODO: make this implicit
impl<T:Copy> TryEvaluate<T> for Scalar<T>{
	fn try_evaluate(&self,_values:&HashMap<UnknownId,T>)->Result<T,TryEvaluateError>{
		Ok(self.evaluate())
	}
}
impl<T:Zero> Derivative for Scalar<T>{
	type Derivative=Self;
	fn derivative(&self,_unknown_id:UnknownId)->Self::Derivative{
		Self(T::zero())
	}
}
//TODO: generalize arithmetic
//use macros?
impl<T,B> std::ops::Add<B> for Scalar<T>{
	type Output=Plus<Self,B>;
	fn add(self,b:B)->Self::Output{
		Plus(self,b)
	}
}
impl<T,B> std::ops::Mul<B> for Scalar<T>{
	type Output=Times<Self,B>;
	fn mul(self,b:B)->Self::Output{
		Times(self,b)
	}
}

//TODO: something nice like this
// pub trait Arithmetic:Add+Sub+Mul+Div+Pow+Mod{}
// impl<A:Add,B:Add> std::ops::Add<B> for A{
// 	type Output=Plus<A,B>;
// 	fn add(a:A,b:B)->Self::Output{
// 		Plus(a,b)
// 	}
// }
pub struct Plus<A,B>(A,B);
impl<A,B> Plus<A,B>{
	pub fn new(a:A,b:B)->Self{
		Self(a,b)
	}
}
impl<T:std::ops::Add<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Plus<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()+self.1.evaluate()
	}
}
impl<T:std::ops::Add<Output=T>,A:TryEvaluate<T>,B:TryEvaluate<T>> TryEvaluate<T> for Plus<A,B>{
	fn try_evaluate(&self,values:&HashMap<UnknownId,T>)->Result<T,TryEvaluateError>{
		Ok(self.0.try_evaluate(values)?+self.1.try_evaluate(values)?)
	}
}
impl<A:Derivative,B:Derivative> Derivative for Plus<A,B>{
	type Derivative=Plus<A::Derivative,B::Derivative>;
	fn derivative(&self,unknown_id:UnknownId)->Self::Derivative{
		Plus(self.0.derivative(unknown_id),self.1.derivative(unknown_id))
	}
}
//arithmetic
impl<A,B,C> std::ops::Add<C> for Plus<A,B>{
	type Output=Plus<Self,C>;
	fn add(self,c:C)->Self::Output{
		Plus(self,c)
	}
}
impl<A,B,C> std::ops::Mul<C> for Plus<A,B>{
	type Output=Times<Self,C>;
	fn mul(self,c:C)->Self::Output{
		Times(self,c)
	}
}

pub struct Times<A,B>(A,B);
impl<A,B> Times<A,B>{
	pub fn new(a:A,b:B)->Self{
		Self(a,b)
	}
}
impl<T:std::ops::Mul<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Times<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()*self.1.evaluate()
	}
}
impl<T:std::ops::Mul<Output=T>,A:TryEvaluate<T>,B:TryEvaluate<T>> TryEvaluate<T> for Times<A,B>{
	fn try_evaluate(&self,values:&HashMap<UnknownId,T>)->Result<T,TryEvaluateError>{
		Ok(self.0.try_evaluate(values)?*self.1.try_evaluate(values)?)
	}
}
impl<A:Derivative+Copy,B:Derivative+Copy> Derivative for Times<A,B>{
	type Derivative=Plus<Times<A,B::Derivative>,Times<A::Derivative,B>>;
	fn derivative(&self,unknown_id:UnknownId)->Self::Derivative{
		Plus(
			Times(self.0,self.1.derivative(unknown_id)),
			Times(self.0.derivative(unknown_id),self.1),
		)
	}
}
//arithmetic
impl<A,B,C> std::ops::Add<C> for Times<A,B>{
	type Output=Plus<Self,C>;
	fn add(self,c:C)->Self::Output{
		Plus(self,c)
	}
}
impl<A,B,C> std::ops::Mul<C> for Times<A,B>{
	type Output=Times<Self,C>;
	fn mul(self,c:C)->Self::Output{
		Times(self,c)
	}
}