use std::fmt;

pub struct Foo {
    inner: u8,
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Foo{:03}", self.inner)
    }
}

fn main(){
   println!("{}", Foo{inner: b'1'});     // print 
}

