# Microsoft Graph Auto Mail

## Usage Setup

This section focuses on the prerequisite steps to be able to use the application.

1. [Register an application with the Microsoft identity platform](https://learn.microsoft.com/en-us/graph/auth-register-app-v2).
    - Add Web Redirect Uri under Authentication: http://localhost:8383/oauth2/redirect

## Crates

This section focuses on the third-party libraries being used.

- [Chrono](https://github.com/chronotope/chrono) provides timezone-aware date and time operations.
- [Env Logger](https://github.com/rust-cli/env_logger) is a logger.
- [Log](https://github.com/rust-lang/log) is a logging facade.
- [Rand](https://github.com/rust-random/rand) is a collection of random number generators.
- [Reqwest](https://github.com/seanmonstar/reqwest) is an ergonomic HTTP-client.
- [Serde](https://github.com/serde-rs/serde) is a framework for serializing and deserializing data structures.
- [Serde JSON](https://github.com/serde-rs/json) is a Serde extension that enables serializing and deserializing JSON format.
- [Tokio](https://github.com/tokio-rs/tokio) is an asynchronous runtime.
- [Tokio Macros](https://github.com/tokio-rs/tokio/tree/master/tokio-macros) are procedural macros for use with Tokio.
- [Url](https://github.com/servo/rust-url) is an URL parser that enables splitting URLs into their parts and modifying them.