use bytemuck::pod_collect_to_vec;
use std::borrow::Cow;

use crate::{
    aligned::align_cast,
    telemetry::{VarHeader, VarType, bitfields::Bitfield, enums::Enum},
};

/// A set of telemetry values at a specific point in time
///
/// Obtained from an [`IbtFile`][crate::IbtFile] or live telemetry.
#[derive(Clone, Debug)]
pub struct Sample<'data>(Cow<'data, [u8]>);

impl<'data> Sample<'data> {
    pub fn new(data: &'data [u8]) -> Self {
        Self(Cow::Borrowed(data))
    }

    pub fn new_as_owned(data: &'_ [u8]) -> Self {
        Self(Cow::Owned(data.to_vec()))
    }

    /// Extract a value from the sample
    pub fn read_var(&self, var: &VarHeader) -> Value {
        let size = var.ty.size() * var.count;
        let slice = &self.0[var.offset..var.offset + size];

        if let Some(e) = Enum::parse(slice, var.unit.as_str()) {
            return Value::Enum(e);
        }

        if var.count > 1 {
            match var.ty {
                VarType::Bool => Value::BoolArray(slice.iter().map(|b| *b != 0).collect()),
                VarType::Int => Value::IntArray(pod_collect_to_vec(slice)),
                VarType::Float => Value::FloatArray(pod_collect_to_vec(slice)),
                _ => panic!("unsupported array type"),
            }
        } else {
            match var.ty {
                VarType::Char => Value::Char(slice[0] as char),
                VarType::Bool => Value::Bool(slice[0] != 0),
                VarType::Int => Value::Int(align_cast(slice)),
                VarType::Bitfield => {
                    let b = Bitfield::parse_unit(align_cast(slice), var.unit.as_str());
                    Value::Bitfield(b)
                }
                VarType::Float => Value::Float(align_cast(slice)),
                VarType::Double => Value::Double(align_cast(slice)),
            }
        }
    }
}

/// The value of a variable in a [`Sample`]
#[derive(Clone, Debug)]
pub enum Value {
    Char(char),
    Bool(bool),
    Int(i32),
    Bitfield(Bitfield),
    Float(f32),
    Double(f64),

    Enum(Enum),

    BoolArray(Vec<bool>),
    IntArray(Vec<i32>),
    FloatArray(Vec<f32>),
}
