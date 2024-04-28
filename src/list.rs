use crate::string::Character;
use std::marker::PhantomData;
use typenum::assert_type_eq;
use typenum::Integer;

pub trait TypeList {
    type Head;
    type Prev: TypeList;
    type Tail;
    type PushFront<T>: TypeList;
    type PushBack<T>: TypeList;
    type RemoveFront;
    type PopBack;

    /// Appends `L` onto this list in forward order by first reversing `L` and
    /// then performing a reverse append.
    type Append<L>: TypeList
    where
        L: TypeList;
    /// Appends this list onto `L` in the forward order by performing the same
    /// steps as [`TypeList::Append`].
    type AppendTo<L>: TypeList
    where
        L: TypeList;
    /// Appends this list onto `L` in the default reverse order.
    type RevAppendTo<L>: TypeList
    where
        L: TypeList;
    /// Reverses the list.
    type Reverse: TypeList;

    type RemoveFrontInner<Next>;
}

pub trait TypePair<A, B> {
    type ValA;
    type ValB;
}

impl<A, B> TypePair<A, B> for (A, B) {
    type ValA = A;
    type ValB = B;
}

pub struct TypeListElem<T>(PhantomData<T>);

impl<Prev: TypeList, T> TypeList for (Prev, TypeListElem<T>) {
    type Head = Prev::Head;
    type Prev = Prev;
    type Tail = TypeListElem<T>;
    type PushFront<T1> = (Prev::PushFront<T1>, TypeListElem<T>);
    type PushBack<T1> = ((Prev, TypeListElem<T>), TypeListElem<T1>);
    type RemoveFront = Prev::RemoveFrontInner<T>;
    type PopBack = (T, Prev);

    type Append<L> = <L::Reverse as TypeList>::RevAppendTo<Self>
    where
        L: TypeList;
    type AppendTo<L> = L::Append<Self>
    where
        L: TypeList;
    type RevAppendTo<L> = Prev::RevAppendTo<L::PushBack<T>>
    where
        L: TypeList;
    type Reverse = Self::RevAppendTo<()>;

    type RemoveFrontInner<Next> = (Prev::RemoveFrontInner<T>, Next);
}

impl<T> TypeList for (TypeListElem<T>,) {
    type Head = TypeListElem<T>;
    type Prev = ();
    type Tail = TypeListElem<T>;
    type PushFront<T1> = ((TypeListElem<T1>,), TypeListElem<T>);
    type PushBack<T1> = ((TypeListElem<T>,), TypeListElem<T1>);
    type RemoveFront = ();
    type PopBack = (T, ());

    type Append<L> = <L::Reverse as TypeList>::RevAppendTo<Self>
    where
        L: TypeList;
    type AppendTo<L> = L::PushBack<T>
    where
        L: TypeList;
    type RevAppendTo<L> = L::PushBack<T>
    where
        L: TypeList;
    type Reverse = (TypeListElem<T>,);

    type RemoveFrontInner<Next> = (Next,);
}

impl TypeList for () {
    type Head = ();
    type Prev = ();
    type Tail = ();
    type PushFront<T> = Self::PushBack<T>;
    type PushBack<T> = (TypeListElem<T>,);
    type RemoveFront = ();
    type PopBack = ((), ());

    type Append<L> = L
    where
        L: TypeList;
    type AppendTo<L> = L
    where
        L: TypeList;
    type RevAppendTo<L> = L
    where
        L: TypeList;
    type Reverse = Self;

    type RemoveFrontInner<Next> = ();
}

pub trait BuildStringChars<T: 'static> {
    const RESULT: T;
    const LEN: usize;
}

impl<Prev, C, T> BuildStringChars<(T, u8)> for (Prev, C)
where
    Prev: BuildStringChars<T>,
    C: Character,
    T: 'static,
{
    const RESULT: (T, u8) = (Prev::RESULT, <C::Number as Integer>::I8 as u8);
    const LEN: usize = Prev::LEN + 1;
}

impl<C: Character> BuildStringChars<u8> for (C,) {
    const RESULT: u8 = <C::Number as Integer>::I8 as u8;
    const LEN: usize = 1;
}

pub trait StringCharsTy {
    type Result: 'static;
}

impl<Prev, C> StringCharsTy for (Prev, C)
where
    Prev: StringCharsTy,
    C: Character,
{
    type Result = (Prev::Result, u8);
}

impl<C: Character> StringCharsTy for (C,) {
    type Result = u8;
}

assert_type_eq!(
    <((TypeListElem<u32>,), TypeListElem<f32>) as TypeList>::Append<(
        (TypeListElem<i32>,),
        TypeListElem<i8>
    )>,
    (
        (((TypeListElem<u32>,), TypeListElem<f32>), TypeListElem<i32>),
        TypeListElem<i8>
    )
);
assert_type_eq!(
    <((TypeListElem<u32>,), TypeListElem<f32>) as TypeList>::Reverse,
    ((TypeListElem<f32>,), TypeListElem<u32>)
);
