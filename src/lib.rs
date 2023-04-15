

// #[cfg(feature = "builder")]
pub mod builder {
    // extern crate builder;
    pub use ::builder::*;
    pub use paste::paste;

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_should_work() {
            
            #[derive(WithBuilder)]
            struct Test {
                name: String,
            }
        }
    }
}
