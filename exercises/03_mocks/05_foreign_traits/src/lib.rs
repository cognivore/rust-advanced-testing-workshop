use std::str::FromStr;

/*
*
* Mocking methodology:
*
* Mock a trait. This is the simplest use case.

       trait Foo {
           fn foo(&self, x: u32);
       }
       mock!{
           pub MyStruct<T: Clone + 'static> {
               fn bar(&self) -> u8;
           }
           impl<T: Clone + 'static> Foo for MyStruct<T> {
               fn foo(&self, x: u32);
           }
       }

*
* ===================================================
*
* FromStr only required method:
*
* type Err;
* fn from_str(s: &str) -> Result<Self, Self::Err>;
*
* ===================================================
*
*/
mockall::mock! {

    pub Parsed {}
    impl FromStr for Parsed {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, ()>;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implements() {
        static_assertions::assert_impl_one!(MockParsed: FromStr);
    }
}
