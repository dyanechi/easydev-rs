
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
            let sampler = Sampler::new()
                .with_id("super-audio-sampler")
                .with_sample_rate(44100)
                .with_file_path("audio/test")
                .with_format(AudioFormat::Wav)
                .build();

            assert_eq!(sampler.sample_rate, 44100);
            assert_eq!(sampler.file_path, Some(String::from("audio/test")));
            assert_eq!(sampler.format, Some(AudioFormat::Wav));
        }
    }