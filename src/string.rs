use crate::list::BuildStringChars;
use typenum::Integer;

pub mod chars {
    use super::Character;
    use crate::list::TypeListElem;
    use typenum::*;

    macro_rules! def_char {
        ($name:ident, $num:ty) => {
            #[allow(non_camel_case_types, unused)]
            pub struct $name;

            impl Character for $name {
                type Number = $num;
            }

            impl Character for TypeListElem<$name> {
                type Number = $num;
            }
        };
    }

    def_char!(Space, P32);
    def_char!(ExclamationMark, P33);
    def_char!(DoubleQuote, P34);
    def_char!(Hashtag, P35);
    def_char!(Dollar, P36);
    def_char!(Percent, P37);
    def_char!(Ampersand, P38);
    def_char!(SingleQuote, P39);
    def_char!(OpenParentheses, P40);
    def_char!(CloseParentheses, P41);
    def_char!(Star, P42);
    def_char!(Plus, P43);
    def_char!(Comma, P44);
    def_char!(Minus, P45);
    def_char!(Dot, P46);
    def_char!(Slash, P47);
    def_char!(Zero, P48);
    def_char!(One, P49);
    def_char!(Two, P50);
    def_char!(Three, P51);
    def_char!(Four, P52);
    def_char!(Five, P53);
    def_char!(Six, P54);
    def_char!(Seven, P55);
    def_char!(Eight, P56);
    def_char!(Nine, P57);
    def_char!(Colon, P58);
    def_char!(SemiColon, P59);
    def_char!(LessThan, P60);
    def_char!(Equals, P61);
    def_char!(GreaterThan, P62);
    def_char!(Questionmark, P63);
    def_char!(At, P64);
    def_char!(A, P65);
    def_char!(B, P66);
    def_char!(C, P67);
    def_char!(D, P68);
    def_char!(E, P69);
    def_char!(F, P70);
    def_char!(G, P71);
    def_char!(H, P72);
    def_char!(I, P73);
    def_char!(J, P74);
    def_char!(K, P75);
    def_char!(L, P76);
    def_char!(M, P77);
    def_char!(N, P78);
    def_char!(O, P79);
    def_char!(P, P80);
    def_char!(Q, P81);
    def_char!(R, P82);
    def_char!(S, P83);
    def_char!(T, P84);
    def_char!(U, P85);
    def_char!(V, P86);
    def_char!(W, P87);
    def_char!(X, P88);
    def_char!(Y, P89);
    def_char!(Z, P90);
    def_char!(OpenSquareBracket, P91);
    def_char!(Backslash, P92);
    def_char!(CloseSquareBracket, P93);
    def_char!(Caret, P94);
    def_char!(Underscore, P95);
    def_char!(Backtick, P96);
    def_char!(a, P97);
    def_char!(b, P98);
    def_char!(c, P99);
    def_char!(d, P100);
    def_char!(e, P101);
    def_char!(f, P102);
    def_char!(g, P103);
    def_char!(h, P104);
    def_char!(i, P105);
    def_char!(j, P106);
    def_char!(k, P107);
    def_char!(l, P108);
    def_char!(m, P109);
    def_char!(n, P110);
    def_char!(o, P111);
    def_char!(p, P112);
    def_char!(q, P113);
    def_char!(r, P114);
    def_char!(s, P115);
    def_char!(t, P116);
    def_char!(u, P117);
    def_char!(v, P118);
    def_char!(w, P119);
    def_char!(x, P120);
    def_char!(y, P121);
    def_char!(z, P122);
    def_char!(OpenCurlyBracket, P123);
    def_char!(VericalLine, P124);
    def_char!(CloseCurlyBracket, P125);
    def_char!(Tilde, P126);
}

pub trait Character {
    type Number: Integer;
}

#[repr(C)]
pub struct TString<T> {
    val: T,
    len: usize,
}

impl<T: 'static> TString<T> {
    pub const fn new<B: BuildStringChars<T>>() -> Self {
        Self {
            val: B::RESULT,
            len: B::LEN,
        }
    }

    pub const fn get(&self) -> &str {
        let ptr = (&raw const self.val).cast::<u8>();
        let data = unsafe { std::slice::from_raw_parts(ptr, self.len) };
        match std::str::from_utf8(data) {
            Ok(x) => x,
            Err(_) => "",
        }
    }
}
