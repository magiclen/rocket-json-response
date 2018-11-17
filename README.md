JSON Response for Rocket Framework
====================

[![Build Status](https://travis-ci.org/magiclen/rocket-json-response.svg?branch=master)](https://travis-ci.org/magiclen/rocket-json-response)

This is a crate which provides `JSONResponse` and `JSONResponseWithoutData` structs to response JSON format data with an additional **code** integer value.

Typically, the code **0** means **OK**. You can define other codes by yourself by implementing `JSONResponseCode` trait for your struct.

See `examples`.

## Crates.io

https://crates.io/crates/rocket-json-response

## Documentation

https://docs.rs/rocket-json-response

## License

[MIT](LICENSE)