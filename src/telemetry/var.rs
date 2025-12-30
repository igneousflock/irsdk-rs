use std::ffi::c_char;

use indexmap::IndexMap;
use num_enum::TryFromPrimitive;

use crate::raw;

#[derive(Clone, Debug)]
pub struct VarSet(IndexMap<String, VarHeader>);

impl VarSet {
    pub fn new(mut vars: Vec<VarHeader>) -> Self {
        vars.sort_by_key(|v| v.offset);
        let map = vars.into_iter().map(|v| (v.name.clone(), v)).collect();
        Self(map)
    }

    pub fn var(&self, name: &str) -> Option<&VarHeader> {
        self.0.get(name)
    }

    pub fn all_vars(&self) -> impl Iterator<Item = &VarHeader> {
        self.0.values()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive, serde::Serialize)]
#[repr(i32)]
pub enum VarType {
    Char,
    Bool,
    Int,
    Bitfield,
    Float,
    Double,
}

impl VarType {
    pub fn size(&self) -> usize {
        match self {
            VarType::Char | VarType::Bool => 1,
            VarType::Int | VarType::Bitfield | VarType::Float => 4,
            VarType::Double => 8,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct VarHeader {
    pub ty: VarType,
    pub offset: usize,
    pub count: usize,
    pub count_as_time: bool,

    pub name: String,
    pub description: String,
    pub unit: String,
}

impl VarHeader {
    pub fn from_raw(raw: &raw::VarHeader) -> Self {
        let ty = raw
            .ty
            .try_into()
            .unwrap_or_else(|_| panic!("invalid var type: `{}`", raw.ty));

        Self {
            ty,
            offset: raw.offset.try_into().expect("`offset` should be positive"),
            count: raw.count.try_into().expect("`count` should be positive"),
            count_as_time: raw.count_as_time == 0,
            name: string_from_c_chars(&raw.name),
            description: string_from_c_chars(&raw.desc),
            unit: string_from_c_chars(&raw.unit),
        }
    }
}

fn string_from_c_chars(buf: &[c_char]) -> String {
    assert!(buf.contains(&0));
    // Strings in iRacing are all ISO-8859-1, which is effectively a subset of UTF-8. Therefore, it
    // is safe to interpret a string buffer as unsiged bytes and cast them to UTF-8 codepoints.
    //
    // https://forums.iracing.com/discussion/comment/703469/#Comment_703469
    buf.iter()
        .map(|c| *c as u8 as char)
        .take_while(|c| *c != '\0')
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        raw,
        telemetry::{VarHeader, VarType},
    };

    #[test]
    fn decodes_var_header() {
        let raw = raw::VarHeader::new(
            5,
            0,
            1,
            0,
            b"SessionTime",
            b"Seconds since session start",
            b"s",
        );
        let var_header = VarHeader::from_raw(&raw);

        assert_eq!(
            var_header,
            VarHeader {
                ty: VarType::Double,
                offset: 0,
                count: 1,
                count_as_time: true,
                name: "SessionTime".to_string(),
                description: "Seconds since session start".to_string(),
                unit: "s".to_string(),
            }
        );
    }

    #[test]
    #[should_panic = "invalid var type: `99`"]
    fn panics_with_invalid_var_type() {
        let raw = raw::VarHeader::new(
            99, // invalid
            0, 1, 0, b"", b"", b"",
        );
        VarHeader::from_raw(&raw);
    }
}
