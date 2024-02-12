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
	MissingUnknown(VariableId),
}
impl std::fmt::Display for TryEvaluateError{
	fn fmt(&self,state:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		write!(state,"{self:?}")
	}
}
impl std::error::Error for TryEvaluateError{}
pub trait TryEvaluate<T>{
	fn try_evaluate(&self,values:&HashMap<VariableId,T>)->Result<T,TryEvaluateError>;
}
//TODO: implement TryEvaluate implicitly if the type implements Evaluate
// impl<T,Eval:Evaluate<T>> TryEvaluate<T> for Eval{
// 	fn try_evaluate(&self,_values:&HashMap<UnknownId,T>)->Result<T,TryEvaluateError>{
// 		Ok(self.evaluate())
// 	}
// }
pub trait Derivative{
	type Derivative;
	fn derivative(&self,unknown_id:VariableId)->Self::Derivative;
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
pub struct VariableId(u32);
impl VariableId{
	pub const fn new(value:u32)->Self{
		Self(value)
	}
}
impl<T:Copy> TryEvaluate<T> for VariableId{
	fn try_evaluate(&self,values:&HashMap<VariableId,T>)->Result<T,TryEvaluateError>{
		values.get(self).copied().ok_or(TryEvaluateError::MissingUnknown(*self))
	}
}
impl Derivative for VariableId{
	type Derivative=Morph;
	fn derivative(&self,unknown_id:VariableId)->Self::Derivative{
		if *self==unknown_id{
			Morph::Identity
		}else{
			Morph::Zero
		}
	}
}
//TODO: generalize arithmetic
impl<C> std::ops::Add<C> for VariableId{
	type Output=Plus<Self,C>;
	fn add(self,c:C)->Self::Output{
		Plus(self,c)
	}
}
impl<C> std::ops::Mul<C> for VariableId{
	type Output=Times<Self,C>;
	fn mul(self,c:C)->Self::Output{
		Times(self,c)
	}
}
impl<C> std::ops::Sub<C> for VariableId{
	type Output=Minus<Self,C>;
	fn sub(self,c:C)->Self::Output{
		Minus(self,c)
	}
}
impl<C> std::ops::Div<C> for VariableId{
	type Output=Divide<Self,C>;
	fn div(self,c:C)->Self::Output{
		Divide(self,c)
	}
}

#[derive(Clone,Copy)]
pub struct Constant<T>(T);
impl<T> Constant<T>{
	pub const fn new(value:T)->Self{
		Self(value)
	}
}
impl<T:Copy> Evaluate<T> for Constant<T>{
	fn evaluate(&self)->T{
		self.0
	}
}
//TODO: make this implicit
impl<T:Copy> TryEvaluate<T> for Constant<T>{
	fn try_evaluate(&self,_values:&HashMap<VariableId,T>)->Result<T,TryEvaluateError>{
		Ok(self.evaluate())
	}
}
impl<T:Zero> Derivative for Constant<T>{
	type Derivative=Self;
	fn derivative(&self,_unknown_id:VariableId)->Self::Derivative{
		Self(T::zero())
	}
}
//TODO: generalize arithmetic
//use macros?
impl<A,C> std::ops::Add<C> for Constant<A>{
	type Output=Plus<Self,C>;
	fn add(self,c:C)->Self::Output{
		Plus(self,c)
	}
}
impl<A,C> std::ops::Mul<C> for Constant<A>{
	type Output=Times<Self,C>;
	fn mul(self,c:C)->Self::Output{
		Times(self,c)
	}
}
impl<A,C> std::ops::Sub<C> for Constant<A>{
	type Output=Minus<Self,C>;
	fn sub(self,c:C)->Self::Output{
		Minus(self,c)
	}
}
impl<A,C> std::ops::Div<C> for Constant<A>{
	type Output=Divide<Self,C>;
	fn div(self,c:C)->Self::Output{
		Divide(self,c)
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
#[derive(Clone,Copy)]
pub struct Plus<A,B>(A,B);
impl<A,B> Plus<A,B>{
	pub const fn new(a:A,b:B)->Self{
		Self(a,b)
	}
}
impl<T:std::ops::Add<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Plus<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()+self.1.evaluate()
	}
}
impl<T:std::ops::Add<Output=T>,A:TryEvaluate<T>,B:TryEvaluate<T>> TryEvaluate<T> for Plus<A,B>{
	fn try_evaluate(&self,values:&HashMap<VariableId,T>)->Result<T,TryEvaluateError>{
		Ok(self.0.try_evaluate(values)?+self.1.try_evaluate(values)?)
	}
}
impl<A:Derivative,B:Derivative> Derivative for Plus<A,B>{
	type Derivative=Plus<A::Derivative,B::Derivative>;
	fn derivative(&self,unknown_id:VariableId)->Self::Derivative{
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
impl<A,B,C> std::ops::Sub<C> for Plus<A,B>{
	type Output=Minus<Self,C>;
	fn sub(self,c:C)->Self::Output{
		Minus(self,c)
	}
}
impl<A,B,C> std::ops::Div<C> for Plus<A,B>{
	type Output=Divide<Self,C>;
	fn div(self,c:C)->Self::Output{
		Divide(self,c)
	}
}

#[derive(Clone,Copy)]
pub struct Minus<A,B>(A,B);
impl<A,B> Minus<A,B>{
	pub const fn new(a:A,b:B)->Self{
		Self(a,b)
	}
}
impl<T:std::ops::Sub<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Minus<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()-self.1.evaluate()
	}
}
impl<T:std::ops::Sub<Output=T>,A:TryEvaluate<T>,B:TryEvaluate<T>> TryEvaluate<T> for Minus<A,B>{
	fn try_evaluate(&self,values:&HashMap<VariableId,T>)->Result<T,TryEvaluateError>{
		Ok(self.0.try_evaluate(values)?-self.1.try_evaluate(values)?)
	}
}
impl<A:Derivative,B:Derivative> Derivative for Minus<A,B>{
	type Derivative=Minus<A::Derivative,B::Derivative>;
	fn derivative(&self,unknown_id:VariableId)->Self::Derivative{
		Minus(self.0.derivative(unknown_id),self.1.derivative(unknown_id))
	}
}
//arithmetic
impl<A,B,C> std::ops::Add<C> for Minus<A,B>{
	type Output=Plus<Self,C>;
	fn add(self,c:C)->Self::Output{
		Plus(self,c)
	}
}
impl<A,B,C> std::ops::Mul<C> for Minus<A,B>{
	type Output=Times<Self,C>;
	fn mul(self,c:C)->Self::Output{
		Times(self,c)
	}
}
impl<A,B,C> std::ops::Sub<C> for Minus<A,B>{
	type Output=Minus<Self,C>;
	fn sub(self,c:C)->Self::Output{
		Minus(self,c)
	}
}
impl<A,B,C> std::ops::Div<C> for Minus<A,B>{
	type Output=Divide<Self,C>;
	fn div(self,c:C)->Self::Output{
		Divide(self,c)
	}
}

#[derive(Clone,Copy)]
pub struct Times<A,B>(A,B);
impl<A,B> Times<A,B>{
	pub const fn new(a:A,b:B)->Self{
		Self(a,b)
	}
}
impl<T:std::ops::Mul<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Times<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()*self.1.evaluate()
	}
}
impl<T:std::ops::Mul<Output=T>,A:TryEvaluate<T>,B:TryEvaluate<T>> TryEvaluate<T> for Times<A,B>{
	fn try_evaluate(&self,values:&HashMap<VariableId,T>)->Result<T,TryEvaluateError>{
		Ok(self.0.try_evaluate(values)?*self.1.try_evaluate(values)?)
	}
}
impl<A:Derivative+Copy,B:Derivative+Copy> Derivative for Times<A,B>{
	type Derivative=Plus<Times<A,B::Derivative>,Times<A::Derivative,B>>;
	fn derivative(&self,unknown_id:VariableId)->Self::Derivative{
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
impl<A,B,C> std::ops::Sub<C> for Times<A,B>{
	type Output=Minus<Self,C>;
	fn sub(self,c:C)->Self::Output{
		Minus(self,c)
	}
}
impl<A,B,C> std::ops::Div<C> for Times<A,B>{
	type Output=Divide<Self,C>;
	fn div(self,c:C)->Self::Output{
		Divide(self,c)
	}
}

#[derive(Clone,Copy)]
pub struct Divide<A,B>(A,B);
impl<A,B> Divide<A,B>{
	pub const fn new(a:A,b:B)->Self{
		Self(a,b)
	}
}
impl<T:std::ops::Div<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Divide<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()/self.1.evaluate()
	}
}
impl<T:std::ops::Div<Output=T>,A:TryEvaluate<T>,B:TryEvaluate<T>> TryEvaluate<T> for Divide<A,B>{
	fn try_evaluate(&self,values:&HashMap<VariableId,T>)->Result<T,TryEvaluateError>{
		Ok(self.0.try_evaluate(values)?/self.1.try_evaluate(values)?)
	}
}
impl<A:Derivative+Copy,B:Derivative+Copy> Derivative for Divide<A,B>{
	type Derivative=Minus<Divide<A::Derivative,B>,Divide<Times<A,B::Derivative>,Times<B,B>>>;
	fn derivative(&self,unknown_id:VariableId)->Self::Derivative{
		Minus(Divide(self.0.derivative(unknown_id),self.1),Divide(Times(self.0,self.1.derivative(unknown_id)),Times(self.1,self.1)))
	}
}
//arithmetic
impl<A,B,C> std::ops::Add<C> for Divide<A,B>{
	type Output=Plus<Self,C>;
	fn add(self,c:C)->Self::Output{
		Plus(self,c)
	}
}
impl<A,B,C> std::ops::Mul<C> for Divide<A,B>{
	type Output=Times<Self,C>;
	fn mul(self,c:C)->Self::Output{
		Times(self,c)
	}
}
impl<A,B,C> std::ops::Sub<C> for Divide<A,B>{
	type Output=Minus<Self,C>;
	fn sub(self,c:C)->Self::Output{
		Minus(self,c)
	}
}
impl<A,B,C> std::ops::Div<C> for Divide<A,B>{
	type Output=Divide<A,Times<B,C>>;
	fn div(self,c:C)->Self::Output{
		Divide(self.0,Times(self.1,c))
	}
}

pub trait Pow<Rhs=Self>{
	type Output;
	fn pow(self,rhs:Rhs)->Self::Output;
}
impl Pow for f32{
	type Output=Self;
	fn pow(self,rhs:Self)->Self::Output{
		self.powf(rhs)
	}
}
impl Pow for i32{
	type Output=Self;
	fn pow(self,rhs:Self)->Self::Output{
		self.pow(rhs as u32)
	}
}

#[derive(Clone,Copy)]
pub struct Power<A,B>(A,B);
impl<A,B> Power<A,B>{
	pub const fn new(a:A,b:B)->Self{
		Self(a,b)
	}
}
impl<T:Pow<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Power<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate().pow(self.1.evaluate())
	}
}
impl<T:Pow<Output=T>,A:TryEvaluate<T>,B:TryEvaluate<T>> TryEvaluate<T> for Power<A,B>{
	fn try_evaluate(&self,values:&HashMap<VariableId,T>)->Result<T,TryEvaluateError>{
		Ok(self.0.try_evaluate(values)?.pow(self.1.try_evaluate(values)?))
	}
}
impl<A:Derivative+Copy,B:Derivative+Copy> Derivative for Power<A,B>{
	type Derivative=Times<Self,Plus<Divide<Times<B,A::Derivative>,A>,Times<Log<A>,B::Derivative>>>;
	fn derivative(&self,unknown_id:VariableId)->Self::Derivative{
		Times(*self,Plus(Divide(Times(self.1,self.0.derivative(unknown_id)),self.0),Times(Log(self.0),self.1.derivative(unknown_id))))
	}
}
//arithmetic
impl<A,B,C> std::ops::Add<C> for Power<A,B>{
	type Output=Plus<Self,C>;
	fn add(self,c:C)->Self::Output{
		Plus(self,c)
	}
}
impl<A,B,C> std::ops::Mul<C> for Power<A,B>{
	type Output=Times<Self,C>;
	fn mul(self,c:C)->Self::Output{
		Times(self,c)
	}
}
impl<A,B,C> std::ops::Sub<C> for Power<A,B>{
	type Output=Minus<Self,C>;
	fn sub(self,c:C)->Self::Output{
		Minus(self,c)
	}
}
impl<A,B,C> std::ops::Div<C> for Power<A,B>{
	type Output=Divide<Self,C>;
	fn div(self,c:C)->Self::Output{
		Divide(self,c)
	}
}

pub trait Logarithm{
	type Output;
	fn log(self)->Self::Output;
}
impl Logarithm for f32{
	type Output=Self;
	fn log(self)->Self::Output{
		self.ln()
	}
}

#[derive(Clone,Copy)]
pub struct Log<A>(A);
impl<A> Log<A>{
	pub const fn new(a:A)->Self{
		Self(a)
	}
}
impl<T:Logarithm<Output=T>,A:Evaluate<T>> Evaluate<T> for Log<A>{
	fn evaluate(&self)->T{
		self.0.evaluate().log()
	}
}
impl<T:Logarithm<Output=T>,A:TryEvaluate<T>> TryEvaluate<T> for Log<A>{
	fn try_evaluate(&self,values:&HashMap<VariableId,T>)->Result<T,TryEvaluateError>{
		Ok(self.0.try_evaluate(values)?.log())
	}
}
impl<A:Derivative+Copy> Derivative for Log<A>{
	type Derivative=Divide<A,A::Derivative>;
	fn derivative(&self,unknown_id:VariableId)->Self::Derivative{
		Divide(self.0,self.0.derivative(unknown_id))
	}
}
//arithmetic
impl<A,C> std::ops::Add<C> for Log<A>{
	type Output=Plus<Self,C>;
	fn add(self,c:C)->Self::Output{
		Plus(self,c)
	}
}
impl<A,C> std::ops::Mul<C> for Log<A>{
	type Output=Times<Self,C>;
	fn mul(self,c:C)->Self::Output{
		Times(self,c)
	}
}
impl<A,C> std::ops::Sub<C> for Log<A>{
	type Output=Minus<Self,C>;
	fn sub(self,c:C)->Self::Output{
		Minus(self,c)
	}
}
impl<A,C> std::ops::Div<C> for Log<A>{
	type Output=Divide<Self,C>;
	fn div(self,c:C)->Self::Output{
		Divide(self,c)
	}
}

pub trait Expable{
	type Output;
	fn exp(self)->Self::Output;
}
impl Expable for f32{
	type Output=Self;
	fn exp(self)->Self::Output{
		self.exp()
	}
}

#[derive(Clone,Copy)]
pub struct Exp<A>(A);
impl<A> Exp<A>{
	pub const fn new(a:A)->Self{
		Self(a)
	}
}
impl<T:Expable<Output=T>,A:Evaluate<T>> Evaluate<T> for Exp<A>{
	fn evaluate(&self)->T{
		self.0.evaluate().exp()
	}
}
impl<T:Expable<Output=T>,A:TryEvaluate<T>> TryEvaluate<T> for Exp<A>{
	fn try_evaluate(&self,values:&HashMap<VariableId,T>)->Result<T,TryEvaluateError>{
		Ok(self.0.try_evaluate(values)?.exp())
	}
}
impl<A:Derivative+Copy> Derivative for Exp<A>{
	type Derivative=Times<Self,A::Derivative>;
	fn derivative(&self,unknown_id:VariableId)->Self::Derivative{
		Times(Self(self.0),self.0.derivative(unknown_id))
	}
}
//arithmetic
impl<A,C> std::ops::Add<C> for Exp<A>{
	type Output=Plus<Self,C>;
	fn add(self,c:C)->Self::Output{
		Plus(self,c)
	}
}
impl<A,C> std::ops::Mul<C> for Exp<A>{
	type Output=Times<Self,C>;
	fn mul(self,c:C)->Self::Output{
		Times(self,c)
	}
}
impl<A,C> std::ops::Sub<C> for Exp<A>{
	type Output=Minus<Self,C>;
	fn sub(self,c:C)->Self::Output{
		Minus(self,c)
	}
}
impl<A,C> std::ops::Div<C> for Exp<A>{
	type Output=Divide<Self,C>;
	fn div(self,c:C)->Self::Output{
		Divide(self,c)
	}
}
