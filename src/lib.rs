
// #[cfg(feature = "builder")]
pub mod builder {
    // extern crate builder;
    pub use ::builder::*;
    pub use traits::*;
    pub mod traits {
        pub trait BuildAble {}
        pub trait WithBuilder<T: BuildAble> {
            fn build(self) -> T;
        }
    }



    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_should_work() {
            
            #[derive(WithBuilder)]
            struct Tester {
                name: String,
            }

            let t = Tester::new()
                .with_name("tester")
                .build()
            ;
        }
    }
}
// use builder::WithBuilder;

#[cfg(test)]
    mod tests {
        use super::*;
        use super::builder::*;

        #[derive(WithBuilder, Debug, Default)]
        struct TestOpt {
            id: usize,
        }
    
        #[derive(WithBuilder, Debug, Default)]
        struct Tester {
            name: String,
            age: usize,
            ancestor: Option<Box<Tester>>,
            some: Option<TestOpt>
        }

        #[test]
        fn test_should_work() {
            let name = "tester";
            let age = 72;
            let t = Tester::new()
                .with_name(name)
                .with_age(age)
                .with_ancestor(Some(Box::new(Tester::default())))
                .with_some(Some(TestOpt::new().with_id(721).build()))
                .build();

            assert_eq!(format!("{}{}", t.name, t.age ), format!("{name}{age}"), "should work properly");
        }
    }