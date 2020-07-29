use crate::json_gettext::serde_json::Value;
use crate::json_gettext::JSONGetTextValue;

/// A data type that can be converted to a JSON-format string.
pub trait ToJSON {
    fn to_json(&self) -> String;
}

impl<'a> ToJSON for JSONGetTextValue<'a> {
    #[inline]
    fn to_json(&self) -> String {
        self.to_json_string()
    }
}

impl<'a> ToJSON for str {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_str(self).to_json_string()
    }
}

impl<'a> ToJSON for &'a str {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_str(self).to_json_string()
    }
}

impl<'a> ToJSON for String {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_str(self).to_json_string()
    }
}

impl<'a> ToJSON for bool {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_bool(*self).to_json_string()
    }
}

impl<'a> ToJSON for i8 {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_i8(*self).to_json_string()
    }
}

impl<'a> ToJSON for i16 {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_i16(*self).to_json_string()
    }
}

impl<'a> ToJSON for i32 {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_i32(*self).to_json_string()
    }
}

impl<'a> ToJSON for i64 {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_i64(*self).to_json_string()
    }
}

impl<'a> ToJSON for u8 {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_u8(*self).to_json_string()
    }
}

impl<'a> ToJSON for u16 {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_u16(*self).to_json_string()
    }
}

impl<'a> ToJSON for u32 {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_u32(*self).to_json_string()
    }
}

impl<'a> ToJSON for u64 {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_u64(*self).to_json_string()
    }
}

impl<'a> ToJSON for f32 {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_f32(*self).to_json_string()
    }
}

impl<'a> ToJSON for f64 {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_f64(*self).to_json_string()
    }
}

impl ToJSON for Value {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_json_value_ref(self).to_json_string()
    }
}

impl ToJSON for &Value {
    #[inline]
    fn to_json(&self) -> String {
        JSONGetTextValue::from_json_value_ref(self).to_json_string()
    }
}

impl<T: ToJSON> ToJSON for Option<T> {
    #[inline]
    fn to_json(&self) -> String {
        match self {
            Some(s) => s.to_json(),
            None => JSONGetTextValue::null().to_json_string(),
        }
    }
}

#[macro_export]
macro_rules! serialize_to_json {
    ($t:ty) => {
        impl crate::rocket_json_response::ToJSON for $t {
            #[inline]
            fn to_json(&self) -> String {
                $crate::rocket_json_response::json_gettext::serde_json::to_value(self)
                    .unwrap()
                    .to_json()
            }
        }
    };
}
