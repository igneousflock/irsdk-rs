use crate::{
    aligned::align_cast,
    telemetry::{VarHeader, VarType},
};

pub struct Sample<'data>(&'data [u8]);

impl<'data> Sample<'data> {
    pub fn new(data: &'data [u8]) -> Self {
        Self(data)
    }

    pub fn read_var(&self, var: &VarHeader) -> Record {
        let size = var.ty.size() * var.count;
        let slice = &self.0[var.offset..var.offset + size];

        if var.count > 1 {
            todo!("array values");
        } else {
            let value = match var.ty {
                VarType::Char => Value::Char(slice[0] as char),
                VarType::Bool => Value::Bool(slice[0] != 0),
                VarType::Int => Value::Int(align_cast(slice)),
                VarType::Bitfield => Value::Bitfield(align_cast(slice)),
                VarType::Float => Value::Float(align_cast(slice)),
                VarType::Double => Value::Double(align_cast(slice)),
            };
            Record::SingleValue(value)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Record {
    SingleValue(Value),
    Array(Vec<Value>),
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Char(char),
    Bool(bool),
    Int(u32),
    Bitfield(u32),
    Float(f32),
    Double(f64),
}
