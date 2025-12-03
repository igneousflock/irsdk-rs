use bytemuck::pod_collect_to_vec;

use crate::{
    aligned::align_cast,
    telemetry::{VarHeader, VarType},
};

pub struct Sample<'data>(&'data [u8]);

impl<'data> Sample<'data> {
    pub fn new(data: &'data [u8]) -> Self {
        Self(data)
    }

    pub fn read_var(&self, var: &VarHeader) -> Value {
        let size = var.ty.size() * var.count;
        let slice = &self.0[var.offset..var.offset + size];

        if var.count > 1 {
            match var.ty {
                VarType::Int => Value::IntArray(pod_collect_to_vec(slice)),
                VarType::Float => Value::FloatArray(pod_collect_to_vec(slice)),
                _ => panic!("unsupported array type"),
            }
        } else {
            match var.ty {
                VarType::Char => Value::Char(slice[0] as char),
                VarType::Bool => Value::Bool(slice[0] != 0),
                VarType::Int => Value::Int(align_cast(slice)),
                VarType::Bitfield => Value::Bitfield(align_cast(slice)),
                VarType::Float => Value::Float(align_cast(slice)),
                VarType::Double => Value::Double(align_cast(slice)),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Char(char),
    Bool(bool),
    Int(i32),
    Bitfield(u32),
    Float(f32),
    Double(f64),

    IntArray(Vec<u32>),
    FloatArray(Vec<f32>),
}
