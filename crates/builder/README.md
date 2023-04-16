# Rust Derive Builder macro from easydev-rs package

## Description

This package implements the traditional builder patterns you'll know from OOP models.

## Usage

Import builder to your crate:

```rust
use easydev-rs::builder::*;
```

Then attach derive macro (ex: `WithBuilder`) to your struct:

```rust
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
```

And you can use it like this:

```rust
let sampler = Sampler::new()
    .with_id("super-audio-sampler")
    .with_sample_rate(44100)
    .with_file_path("audio/test")
    .with_format(AudioFormat::Wav)
    .build();

assert_eq!(sampler.sample_rate, 44100);
assert_eq!(sampler.file_path, Some(String::from("audio/test")));
assert_eq!(sampler.format, Some(AudioFormat::Wav));
```

## Notes

As you can see, for fields that are of type `Option<T>` our builder methods remove some boilerplate.

Instead of writing `.with_format(Some(AudioFormat::Wav))` you can omit the `Some()` part.

Moreover, instead of writing this:

```rust
.with_file_path(Some(String::from("audio_test")))
```

You can write this:

```rust
.with_file_path("audio_test")
```

Builder will automatically infer the type and crete builder method that stores the correct format. Yay!
