use crate::functions::*;
use crate::list::*;
use crate::string::CharString;
use std::marker::PhantomData;

mod functions;
mod list;
mod macros;
mod string;

pub trait Program {
    type Execute: IoState;
}

struct TestProgram<S1: CharString, S2: CharString, S3: CharString>(PhantomData<(S1, S2, S3)>);
impl<S1: CharString, S2: CharString, S3: CharString> Program for TestProgram<S1, S2, S3> {
    type Execute = chain! { io: Call<Print<S1>, ()>, Call<Print<S2>, ()>, Call<Print<S3>, ()>, };
}

type HelloWorldStr1 = type_str!(H e l l o , __ W o r l d 1 !);
type HelloWorldStr2 = type_str!(H e l l o , __ W o r l d 2 !);
type HelloWorldStr3 = type_str!(H e l l o , __ W o r l d 3 !);
type TestOut = <TestProgram<HelloWorldStr1, HelloWorldStr2, HelloWorldStr3> as Program>::Execute;

compile_string!(TEST_STR = TestOut);

fn main() {
    println!("{}", std::any::type_name::<TestOut>());
    println!("{}", TEST_STR.get());
}
