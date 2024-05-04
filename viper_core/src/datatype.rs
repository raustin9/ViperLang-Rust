use std::sync::Arc;

pub enum DataType {
    Byte,
    Integer(Sign, Size),
    Float(Sign, Size),
    Pointer(Arc<DataType>),
    Array(Arc<DataType>, usize),
    Function(Arc<DataType>, Vec<DataType>),
    Tuple(Vec<Arc<DataType>>),
}

pub type TypeName = String;

pub enum Sign {
    Signed,
    Unsigned
}

/// Represents the size of a data type (in bits) to 
pub enum Size {
    S8,
    S16,
    S32,
    S64,
}
