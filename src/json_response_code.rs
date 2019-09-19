/// The code of your JSON response.
pub trait JSONResponseCode {
    /// Assume the code **0** means **OK**. You can define other codes by yourself.
    /// This method will be called for one time when the response is being triggered. You can do something (perhaps keep a log?) at the moment.
    fn get_code(&self) -> i32;
}

impl JSONResponseCode for i8 {
    #[inline]
    fn get_code(&self) -> i32 {
        i32::from(*self)
    }
}

impl JSONResponseCode for i16 {
    #[inline]
    fn get_code(&self) -> i32 {
        i32::from(*self)
    }
}

impl JSONResponseCode for i32 {
    #[inline]
    fn get_code(&self) -> i32 {
        *self
    }
}

impl JSONResponseCode for u8 {
    #[inline]
    fn get_code(&self) -> i32 {
        i32::from(*self)
    }
}

impl JSONResponseCode for u16 {
    #[inline]
    fn get_code(&self) -> i32 {
        i32::from(*self)
    }
}

impl JSONResponseCode for u32 {
    #[inline]
    fn get_code(&self) -> i32 {
        *self as i32
    }
}
