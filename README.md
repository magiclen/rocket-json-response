JSON Response for Rocket Framework
====================

This is a crate which provides `JSONResponse` and `JSONResponseWithoutData` structs to response JSON format data with an additional **code** which is an integer value.

Typically, the code **0** means **OK**. You can define other codes by yourself by implementing `JSONResponseCode` trait for your struct.

Refer to `tests/index.rs` to see a complete example.

## License

[MIT](LICENSE)