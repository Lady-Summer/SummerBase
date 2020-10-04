use std::any::{Any, TypeId};

pub trait Type {}


pub struct NumberType<S> {
    value: S
}

impl <S> From<S> for NumberType<S> {

    fn from(v: S) -> Self {
        match v.type_id() {
            TypeId::of::<i64>() |
            TypeId::of::<u64>() |
            TypeId::of::<i32>() |
            TypeId::of::<u32>() |
            TypeId::of::<f32>() |
            TypeId::of::<f64>() => NumberType {value: v},
            _ => {
                panic!("NumberType only support i32, u32, i64, u64, f32, f64. Please check variable's type! ")
            }
        }
    }
}

impl <S> Type for NumberType<S> {}