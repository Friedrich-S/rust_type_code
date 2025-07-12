use crate::functions::*;
use crate::list::*;

mod functions;
mod list;
mod macros;
mod string;

pub trait Program {
    type Execute: IoState;
}

struct TestProgram;
impl Program for TestProgram {
    type Execute = chain! { io:
        Print(type_str!(H e l l o , __ W o r l d 1 !));
        Print(type_str!(H e l l o , __ W o r l d 2 !));
        Print(type_str!(H e l l o , __ W o r l d 3 !));
    };
}

type TestOut = <TestProgram as Program>::Execute;

compile_string!(TEST_STR = TestOut);

fn main() {
    println!("{}", std::any::type_name::<TestOut>());
    println!("{}", TEST_STR.get());
}
