# OS Concepts Assignments in Rust

My attempts to rewrite my Operating Systems course assignments (written in C) in Rust

## Why?

I've tried countless of times to get into Rust because of all the hype surrounding it, but always found it too hard :P  
I also had no projects that called for its use and always opted for JavaScript/TypeScript because of its familiarity and it was always good enough. This made me less motivated to learn Rust and never touched it again for a long time.
Then comes my Operating Systems course, which required C/C++ for assignments. It was almost impossible at first with a lot of C and systems programming pitfalls but I eventually got the hang of it by the third assignment. These are the kinds of things that Rust is supposed to fix, so it was the perfect applications for learning Rust.

## Shell - Thoughts

Rust is hard :'(

### Cons

- I have to handle every single error that can happen, which makes error handling messy. I will probably have to try creating my own custom `Result` types and stop spamming `Result<(), Box<dyn Error>>` everywhere.

- Dealing with string types was also difficult. While C only has `char *` (sometimes with `const`), I have to use `str`, `String`, `Vec<u8>`, and call whatever conversions between those types to get it to compile.

### Pros

- Everything (almost) went smoothly when it did compile. Aside from logical errors that came up because of differences in string methods and the deadlock happening with the signal handler, there were no other runtime errors. This might be because I already wrote this program, though, and I might already have smoothed out any problems I would have encountered before.

- Still better error handling than C. The `?` operator kind of reminds me of the `await` construct in Javascript, passing the error handling to the caller. I've leveraged that a lot in this application.
