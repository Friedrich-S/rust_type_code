// ToDo: refactor to use virtual tt construction to define using left-recursive list syntax
#[macro_export]
macro_rules! parse_chars {
    ($char:tt $($rest:tt)*) => { crate::parse_chars_inner!([$($rest)*] [ (TypeListElem<crate::parse_char!($char)>,) ]) };
}

#[macro_export]
macro_rules! parse_chars_inner {
    ([$char:tt $($rest:tt)*] [$($res:tt)*]) => { crate::parse_chars_inner!([$($rest)*] [($($res)*, TypeListElem<crate::parse_char!($char)>)]) };
    ([] [$($res:tt)*]) => { $($res)* };
}

#[macro_export]
macro_rules! parse_char {
    // Special cases to implement characters that cannot be parsed by macros
    (__) => {
        crate::string::chars::Space
    };

    (!) => {
        crate::string::chars::ExclamationMark
    };
    (#) => {
        crate::string::chars::Hashtag
    };
    (%) => {
        crate::string::chars::Percent
    };
    (&) => {
        crate::string::chars::Ampersand
    };
    (*) => {
        crate::string::chars::Star
    };
    (+) => {
        crate::string::chars::Plus
    };
    (,) => {
        crate::string::chars::Comma
    };
    (-) => {
        crate::string::chars::Minus
    };
    (.) => {
        crate::string::chars::Dot
    };
    (/) => {
        crate::string::chars::Slash
    };
    (0) => {
        crate::string::chars::Zero
    };
    (1) => {
        crate::string::chars::One
    };
    (2) => {
        crate::string::chars::Two
    };
    (3) => {
        crate::string::chars::Three
    };
    (4) => {
        crate::string::chars::Four
    };
    (5) => {
        crate::string::chars::Five
    };
    (6) => {
        crate::string::chars::Six
    };
    (7) => {
        crate::string::chars::Seven
    };
    (8) => {
        crate::string::chars::Eight
    };
    (9) => {
        crate::string::chars::Nine
    };
    (:) => {
        crate::string::chars::Colon
    };
    (;) => {
        crate::string::chars::SemiColon
    };
    (<) => {
        crate::string::chars::LessThan
    };
    (=) => {
        crate::string::chars::Equals
    };
    (>) => {
        crate::string::chars::GreaterThan
    };
    (?) => {
        crate::string::chars::Questionmark
    };
    (A) => {
        crate::string::chars::A
    };
    (B) => {
        crate::string::chars::B
    };
    (C) => {
        crate::string::chars::C
    };
    (D) => {
        crate::string::chars::D
    };
    (E) => {
        crate::string::chars::E
    };
    (F) => {
        crate::string::chars::F
    };
    (G) => {
        crate::string::chars::G
    };
    (H) => {
        crate::string::chars::H
    };
    (I) => {
        crate::string::chars::I
    };
    (J) => {
        crate::string::chars::J
    };
    (K) => {
        crate::string::chars::K
    };
    (L) => {
        crate::string::chars::L
    };
    (M) => {
        crate::string::chars::M
    };
    (N) => {
        crate::string::chars::N
    };
    (O) => {
        crate::string::chars::O
    };
    (P) => {
        crate::string::chars::P
    };
    (Q) => {
        crate::string::chars::Q
    };
    (R) => {
        crate::string::chars::R
    };
    (S) => {
        crate::string::chars::S
    };
    (T) => {
        crate::string::chars::T
    };
    (U) => {
        crate::string::chars::U
    };
    (V) => {
        crate::string::chars::V
    };
    (W) => {
        crate::string::chars::W
    };
    (X) => {
        crate::string::chars::X
    };
    (Y) => {
        crate::string::chars::Y
    };
    (Z) => {
        crate::string::chars::Z
    };
    (^) => {
        crate::string::chars::Caret
    };
    (a) => {
        crate::string::chars::a
    };
    (b) => {
        crate::string::chars::b
    };
    (c) => {
        crate::string::chars::c
    };
    (d) => {
        crate::string::chars::d
    };
    (e) => {
        crate::string::chars::e
    };
    (f) => {
        crate::string::chars::f
    };
    (g) => {
        crate::string::chars::g
    };
    (h) => {
        crate::string::chars::h
    };
    (i) => {
        crate::string::chars::i
    };
    (j) => {
        crate::string::chars::j
    };
    (k) => {
        crate::string::chars::k
    };
    (l) => {
        crate::string::chars::l
    };
    (m) => {
        crate::string::chars::m
    };
    (n) => {
        crate::string::chars::n
    };
    (o) => {
        crate::string::chars::o
    };
    (p) => {
        crate::string::chars::p
    };
    (q) => {
        crate::string::chars::q
    };
    (r) => {
        crate::string::chars::r
    };
    (s) => {
        crate::string::chars::s
    };
    (t) => {
        crate::string::chars::t
    };
    (u) => {
        crate::string::chars::u
    };
    (v) => {
        crate::string::chars::v
    };
    (w) => {
        crate::string::chars::w
    };
    (x) => {
        crate::string::chars::x
    };
    (y) => {
        crate::string::chars::y
    };
    (z) => {
        crate::string::chars::z
    };
    (|) => {
        crate::string::chars::VericalLine
    };
    (~) => {
        crate::string::chars::Tilde
    };
}

#[macro_export]
macro_rules! type_str {
    ($($t:tt)*) => {
        <$crate::parse_chars!($($t)*) as TypeList>::Reverse
    };
}

#[macro_export]
macro_rules! compile_string {
    ($name:ident = $tgt:ty) => {
        const $name: $crate::string::TString<
            <<$tgt as TypeList>::Reverse as StringCharsTy>::Result,
        > = $crate::string::TString::new::<<$tgt as TypeList>::Reverse>();
    };
}

#[macro_export]
macro_rules! chain {
    (io: $($rest:ty),* $(,)?) => {
        <chain! { $($rest),* } as Function<()>>::Io
    };
    ($head:ty, $($rest:ty),*) => {
        Then<$head, chain! { $($rest),* }>
    };
    ($head:ty) => {
        $head
    };
}
