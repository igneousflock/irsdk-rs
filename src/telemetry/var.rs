use std::ffi::{CStr, c_char};

use indexmap::IndexMap;

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

#[derive(Clone, Copy, Debug, serde::Serialize)]
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

#[derive(Clone, Debug, serde::Serialize)]
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
        let ty = match raw.ty {
            0 => VarType::Char,
            1 => VarType::Bool,
            2 => VarType::Int,
            3 => VarType::Bitfield,
            4 => VarType::Float,
            5 => VarType::Double,
            _ => panic!("invalid var type: `{}`", raw.ty),
        };

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

// TODO: There may be some weird encoding on these strings
fn string_from_c_chars(buf: &[c_char]) -> String {
    assert!(buf.contains(&0));
    let cstr = unsafe { CStr::from_ptr(buf.as_ptr()) };
    cstr.to_string_lossy().into_owned()
}
