
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
            name: Option<String>,
            opt_name: Option<Option<String>>,
            tuple: (String, usize),
            age: usize,
            ancestor: Option<Box<Tester>>,
            some: Option<TestOpt>
        }

        #[derive(PartialEq, Debug)]
        enum AudioFormat {
            Wav,
            Mp3
        }

        #[derive(WithBuilder)]
        struct Sampler {
            id: String,
            sample_rate: u32,
            file_path: Option<String>,
            format: Option<AudioFormat>,
        }

        #[test]
        fn test_should_work() {
            let name = "tester";
            let age = 72;
            let t = Tester::new()
                .with_name(name.to_string())
                .with_opt_name(None)
                .with_tuple((String::new(), 3))
                .with_age(age)
                .with_ancestor(Box::new(Tester::default()))
                .with_some(TestOpt::new().with_id(721).build())
                .build();

            let sampler = Sampler::new()
                .with_id("super-audio-sampler")
                .with_sample_rate(44100)
                .with_file_path("audio/test")
                .with_format(AudioFormat::Wav)
                .build();

            assert_eq!(sampler.sample_rate, 44100);
            assert_eq!(sampler.file_path, Some(String::from("audio/test")));
            assert_eq!(sampler.format, Some(AudioFormat::Wav));

            assert_eq!(t.name.unwrap(), name, "should work properly");
            assert_eq!(t.age, age, "should work properly");
            
            assert!(t.ancestor.is_some(), "should work properly");

            assert!(t.some.is_some(), "should work properly");
            assert_eq!(t.some.unwrap().id, 721);
        }
    }