use std::collections::HashMap;

pub trait Zero{
	fn zero()->Self;
}
pub trait Identity{
	fn identity()->Self;
}

pub trait Evaluate<T>{
	fn evaluate(&self)->T;
}

#[derive(Debug)]
pub enum TryReplaceError{
	MissingUnknown(VariableId),
}
impl std::fmt::Display for TryReplaceError{
	fn fmt(&self,state:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		write!(state,"{self:?}")
	}
}
impl std::error::Error for TryReplaceError{}
/// Replaces VariableId with Constant<T>.  If any variable is missing a replacement, it fails.
pub trait TryReplace<T>{
	type Output;
	fn try_replace(&self,values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>;
}

pub trait Derivative{
	type Derivative;
	fn derivative(&self,unknown_id:VariableId)->Self::Derivative;
}

#[derive(Eq,PartialEq,Ord,PartialOrd)]
enum OperationOrder{
	Add,
	Sub,
	Mul,
	Div,
	Pow,
}
trait Operation{
	//if this is None it means it's a function or something with its own parentheses / parentheses not applicable
	fn operation(&self)->Option<OperationOrder>{
		None
	}
}
trait DisplayExpr:std::fmt::Display+Operation{
	fn display_expr(&self,f:&mut std::fmt::Formatter<'_>,parent_operation:&Option<OperationOrder>)->std::fmt::Result{
		let add_parentheses=match (self.operation(),parent_operation){
			//always use parentheses for powers within powers
			(Some(OperationOrder::Pow),Some(OperationOrder::Pow))=>true,
			(Some(op),Some(parent_op))=>match op.cmp(parent_op){
				std::cmp::Ordering::Less=>true,
				_=>false,
			},
			_=>false,
		};
		match add_parentheses{
			true=>write!(f,"({})",self),
			false=>write!(f,"{}",self),
		}
	}
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
#[derive(Clone,Copy)]
pub enum Morph{
	Zero,
	Identity,
}
impl std::fmt::Display for Morph{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		match self{
			Morph::Zero=>write!(f,"0"),
			Morph::Identity=>write!(f,"1"),
		}
	}
}
impl Operation for Morph{}
impl DisplayExpr for Morph{}
impl<T:Zero+Identity> Evaluate<T> for Morph{
	fn evaluate(&self)->T{
		match self{
			Morph::Zero=>T::zero(),
			Morph::Identity=>T::identity(),
		}
	}
}
impl<T> TryReplace<T> for Morph{
	type Output=Self;
	fn try_replace(&self,_values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>{
		Ok(*self)
	}
}
impl Derivative for Morph{
	type Derivative=Morph;
	fn derivative(&self,_unknown_id:VariableId)->Self::Derivative{
		Morph::Zero
	}
}

pub struct VariableGenerator(u32);
impl VariableGenerator{
	pub const fn new()->Self{
		Self(0)
	}
	pub fn var(&mut self)->VariableId{
		let variable=VariableId::new(self.0);
		self.0+=1;
		variable
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
impl std::fmt::Display for VariableId{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		write!(f,"variable(#{})",self.0)
	}
}
impl Operation for VariableId{}
impl DisplayExpr for VariableId{}
impl<T:Copy> TryReplace<T> for VariableId{
	type Output=Constant<T>;
	fn try_replace(&self,values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>{
		values.get(self).copied().map(Constant::new).ok_or(TryReplaceError::MissingUnknown(*self))
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
pub const fn constant<A>(a:A)->Constant<A>{
	Constant::new(a)
}
impl<T:std::fmt::Display> std::fmt::Display for Constant<T>{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		write!(f,"{}",self.0)
	}
}
impl<T> Operation for Constant<T>{}
impl<T:std::fmt::Display> DisplayExpr for Constant<T>{}
impl<T:Copy> Evaluate<T> for Constant<T>{
	fn evaluate(&self)->T{
		self.0
	}
}
impl<T:Copy> TryReplace<T> for Constant<T>{
	type Output=Self;
	fn try_replace(&self,_values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>{
		Ok(*self)
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
impl<A:DisplayExpr,B:DisplayExpr> std::fmt::Display for Plus<A,B>{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		let op=self.operation();
		self.0.display_expr(f,&op)?;
		write!(f,"+")?;
		self.1.display_expr(f,&op)
	}
}
impl<A,B> Operation for Plus<A,B>{
	fn operation(&self)->Option<OperationOrder>{
		Some(OperationOrder::Add)
	}
}
impl<A:DisplayExpr,B:DisplayExpr> DisplayExpr for Plus<A,B>{}
impl<T:std::ops::Add<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Plus<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()+self.1.evaluate()
	}
}
impl<T,A:TryReplace<T>,B:TryReplace<T>> TryReplace<T> for Plus<A,B>{
	//wtf did I just write
	type Output=Plus<<A as TryReplace<T>>::Output,<B as TryReplace<T>>::Output>;
	fn try_replace(&self,values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>{
		Ok(Plus(self.0.try_replace(values)?,self.1.try_replace(values)?))
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
impl<A:DisplayExpr,B:DisplayExpr> std::fmt::Display for Minus<A,B>{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		let op=self.operation();
		self.0.display_expr(f,&op)?;
		write!(f,"-")?;
		self.1.display_expr(f,&op)
	}
}
impl<A,B> Operation for Minus<A,B>{
	fn operation(&self)->Option<OperationOrder>{
		Some(OperationOrder::Sub)
	}
}
impl<A:DisplayExpr,B:DisplayExpr> DisplayExpr for Minus<A,B>{}
impl<T:std::ops::Sub<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Minus<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()-self.1.evaluate()
	}
}
impl<T,A:TryReplace<T>,B:TryReplace<T>> TryReplace<T> for Minus<A,B>{
	type Output=Minus<<A as TryReplace<T>>::Output,<B as TryReplace<T>>::Output>;
	fn try_replace(&self,values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>{
		Ok(Minus(self.0.try_replace(values)?,self.1.try_replace(values)?))
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
impl<A:DisplayExpr,B:DisplayExpr> std::fmt::Display for Times<A,B>{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		let op=self.operation();
		self.0.display_expr(f,&op)?;
		write!(f,"*")?;
		self.1.display_expr(f,&op)
	}
}
impl<A,B> Operation for Times<A,B>{
	fn operation(&self)->Option<OperationOrder>{
		Some(OperationOrder::Mul)
	}
}
impl<A:DisplayExpr,B:DisplayExpr> DisplayExpr for Times<A,B>{}
impl<T:std::ops::Mul<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Times<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()*self.1.evaluate()
	}
}
impl<T,A:TryReplace<T>,B:TryReplace<T>> TryReplace<T> for Times<A,B>{
	type Output=Times<<A as TryReplace<T>>::Output,<B as TryReplace<T>>::Output>;
	fn try_replace(&self,values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>{
		Ok(Times(self.0.try_replace(values)?,self.1.try_replace(values)?))
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
impl<A:DisplayExpr,B:DisplayExpr> std::fmt::Display for Divide<A,B>{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		let op=self.operation();
		self.0.display_expr(f,&op)?;
		write!(f,"/")?;
		self.1.display_expr(f,&op)
	}
}
impl<A,B> Operation for Divide<A,B>{
	fn operation(&self)->Option<OperationOrder>{
		Some(OperationOrder::Div)
	}
}
impl<A:DisplayExpr,B:DisplayExpr> DisplayExpr for Divide<A,B>{}
impl<T:std::ops::Div<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Divide<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate()/self.1.evaluate()
	}
}
impl<T,A:TryReplace<T>,B:TryReplace<T>> TryReplace<T> for Divide<A,B>{
	type Output=Divide<<A as TryReplace<T>>::Output,<B as TryReplace<T>>::Output>;
	fn try_replace(&self,values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>{
		Ok(Divide(self.0.try_replace(values)?,self.1.try_replace(values)?))
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
pub const fn pow<A,B>(a:A,b:B)->Power<A,B>{
	Power::new(a,b)
}
impl<A:DisplayExpr,B:DisplayExpr> std::fmt::Display for Power<A,B>{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		let op=self.operation();
		self.0.display_expr(f,&op)?;
		write!(f,"^")?;
		self.1.display_expr(f,&op)
	}
}
impl<A,B> Operation for Power<A,B>{
	fn operation(&self)->Option<OperationOrder>{
		Some(OperationOrder::Pow)
	}
}
impl<A:DisplayExpr,B:DisplayExpr> DisplayExpr for Power<A,B>{}
impl<T:Pow<Output=T>,A:Evaluate<T>,B:Evaluate<T>> Evaluate<T> for Power<A,B>{
	fn evaluate(&self)->T{
		self.0.evaluate().pow(self.1.evaluate())
	}
}
impl<T,A:TryReplace<T>,B:TryReplace<T>> TryReplace<T> for Power<A,B>{
	type Output=Power<<A as TryReplace<T>>::Output,<B as TryReplace<T>>::Output>;
	fn try_replace(&self,values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>{
		Ok(Power(self.0.try_replace(values)?,self.1.try_replace(values)?))
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
pub const fn log<A>(a:A)->Log<A>{
	Log::new(a)
}
impl<A:DisplayExpr> std::fmt::Display for Log<A>{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		write!(f,"log(")?;
		self.0.display_expr(f,&self.operation())?;
		write!(f,")")
	}
}
impl<A> Operation for Log<A>{}
impl<A:DisplayExpr> DisplayExpr for Log<A>{}
impl<T:Logarithm<Output=T>,A:Evaluate<T>> Evaluate<T> for Log<A>{
	fn evaluate(&self)->T{
		self.0.evaluate().log()
	}
}
impl<T,A:TryReplace<T>> TryReplace<T> for Log<A>{
	type Output=Log<<A as TryReplace<T>>::Output>;
	fn try_replace(&self,values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>{
		Ok(Log(self.0.try_replace(values)?))
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
pub const fn exp<A>(a:A)->Exp<A>{
	Exp::new(a)
}
impl<A:DisplayExpr> std::fmt::Display for Exp<A>{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		write!(f,"exp(")?;
		self.0.display_expr(f,&self.operation())?;
		write!(f,")")
	}
}
impl<A> Operation for Exp<A>{}
impl<A:DisplayExpr> DisplayExpr for Exp<A>{}
impl<T:Expable<Output=T>,A:Evaluate<T>> Evaluate<T> for Exp<A>{
	fn evaluate(&self)->T{
		self.0.evaluate().exp()
	}
}
impl<T,A:TryReplace<T>> TryReplace<T> for Exp<A>{
	type Output=Exp<<A as TryReplace<T>>::Output>;
	fn try_replace(&self,values:&HashMap<VariableId,T>)->Result<Self::Output,TryReplaceError>{
		Ok(Exp(self.0.try_replace(values)?))
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
