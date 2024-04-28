use crate::list::*;
use std::marker::PhantomData;

/// A simple function that can accept some arguments, return a result and emit
/// an IO state.
pub trait Function<Args> {
    type Result;
    type Io: IoState;
}

/// A type to bind specific arguments to a [`Function`].
pub struct Call<F, Args>(PhantomData<(F, Args)>);

/// The result of the evaluation of a [`Call`].
pub trait FunctionCall {
    type Result;
    type Io: IoState;
}

impl<F: Function<Args>, Args> FunctionCall for Call<F, Args> {
    type Result = F::Result;
    type Io = F::Io;
}

/// The wrapper containing the raw IO state. Here represented as a [`TypeList`]
/// of characters.
pub trait IoState: TypeList {
    type Print<S>: IoState
    where
        S: IoState;
}

impl<L: TypeList> IoState for L {
    type Print<S> = Self::AppendTo<S> where S: IoState;
}

/// Print a string to the IO state output.
pub struct Print<S: TypeList>(PhantomData<S>);

impl<Str: TypeList> Function<()> for Print<Str> {
    type Result = ();
    type Io = Str;
}

/// Chain two function invocations.
pub struct Then<A, B>(PhantomData<(A, B)>);

impl<A, B> FunctionCall for Then<A, B>
where
    A: FunctionCall,
    B: FunctionCall,
{
    type Result = B::Result;
    type Io = <A::Io as IoState>::Print<B::Io>;
}

impl<A, B> Function<()> for Then<A, B>
where
    A: FunctionCall,
    B: FunctionCall,
{
    type Result = B::Result;
    type Io = <A::Io as IoState>::Print<B::Io>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_str;
    use static_assertions::assert_impl_all;

    type Str = type_str!(H I !);
    type Call = super::Call<Print<Str>, ()>;
    assert_impl_all!(Print<Str>: Function<()>);
    assert_impl_all!(Call: FunctionCall);
    assert_impl_all!(Then<Call, Call>: Function<()>);
    assert_impl_all!(Then<Then<Call, Call>, Call>: Function<()>);
    assert_impl_all!(Then<Call, Then<Call, Call>>: Function<()>);
}
