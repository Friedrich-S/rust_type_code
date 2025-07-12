use crate::string::Character;
use std::marker::PhantomData;
use typenum::Integer;
use typenum::assert_type_eq;

pub trait TypeList {
    type Head;
    type Tail: TypeList;
    type Last;
    type PushFront<T>: TypeList;
    type PushBack<T>: TypeList;
    type PopFront;
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

    type PopFrontInner<Next>;
}

pub struct TypeListElem<T>(PhantomData<T>);

impl<Tail: TypeList, T> TypeList for (Tail, TypeListElem<T>) {
    type Head = Tail::Head;
    type Tail = Tail;
    type Last = TypeListElem<T>;
    type PushFront<T1> = (Tail::PushFront<T1>, TypeListElem<T>);
    type PushBack<T1> = ((Tail, TypeListElem<T>), TypeListElem<T1>);
    type PopFront = Tail::PopFrontInner<T>;
    type PopBack = Tail;

    type Append<L>
        = <L::Reverse as TypeList>::RevAppendTo<Self>
    where
        L: TypeList;
    type AppendTo<L>
        = L::Append<Self>
    where
        L: TypeList;
    type RevAppendTo<L>
        = Tail::RevAppendTo<L::PushBack<T>>
    where
        L: TypeList;
    type Reverse = Self::RevAppendTo<()>;

    type PopFrontInner<Next> = (Tail::PopFrontInner<T>, Next);
}

impl<T> TypeList for (TypeListElem<T>,) {
    type Head = TypeListElem<T>;
    type Tail = ();
    type Last = TypeListElem<T>;
    type PushFront<T1> = ((TypeListElem<T1>,), TypeListElem<T>);
    type PushBack<T1> = ((TypeListElem<T>,), TypeListElem<T1>);
    type PopFront = ();
    type PopBack = ();

    type Append<L>
        = <L::Reverse as TypeList>::RevAppendTo<Self>
    where
        L: TypeList;
    type AppendTo<L>
        = L::PushBack<T>
    where
        L: TypeList;
    type RevAppendTo<L>
        = L::PushBack<T>
    where
        L: TypeList;
    type Reverse = (TypeListElem<T>,);

    type PopFrontInner<Next> = (Next,);
}

impl TypeList for () {
    type Head = ();
    type Tail = ();
    type Last = ();
    type PushFront<T> = Self::PushBack<T>;
    type PushBack<T> = (TypeListElem<T>,);
    type PopFront = ();
    type PopBack = ();

    type Append<L>
        = L
    where
        L: TypeList;
    type AppendTo<L>
        = L
    where
        L: TypeList;
    type RevAppendTo<L>
        = L
    where
        L: TypeList;
    type Reverse = Self;

    type PopFrontInner<Next> = ();
}

pub trait BuildStringChars<T: 'static> {
    const RESULT: T;
    const LEN: usize;
}

impl<Tail, C, T> BuildStringChars<(T, u8)> for (Tail, C)
where
    Tail: BuildStringChars<T>,
    C: Character,
    T: 'static,
{
    const RESULT: (T, u8) = (Tail::RESULT, <C::Number as Integer>::I8 as u8);
    const LEN: usize = Tail::LEN + 1;
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
