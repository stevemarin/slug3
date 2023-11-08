
use crate::chunk::Chunk;

pub enum ObjectType {
    // OBJ_BOUND_METHOD,
    // OBJ_CLASS,
    // OBJ_CLOSURE,
    OBJ_FUNCTION,
    // OBJ_INSTANCE,
    OBJ_NATIVE,
    // OBJ_STRING,
    // OBJ_UPVALUE,
}
pub struct Object {
    pub objecttype: ObjectType,
    pub marked: bool
}

pub struct ObjFucntion {
    obj: Object,
    arity: u8,
    // upvalue_count: u8,
    chunk: Chunk,
    name: String
}