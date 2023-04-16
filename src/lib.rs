

// #[cfg(feature = "builder")]
pub mod builder {
    // extern crate builder;
    pub use ::builder::*;

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
    
        #[derive(WithBuilder, Debug)]
        struct Tester {
            name: String,
            age: usize,
        }

        #[test]
        fn test_should_work() {
            let name = "tester";
            let age = 72;
            let t = Tester::new()
                .with_name(name)
                .with_age(age)
                .build();

            assert_eq!(format!("{}{}", t.name, t.age ), format!("{name}{age}"), "should work properly");
        }
    }