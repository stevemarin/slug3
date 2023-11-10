
use crate::chunk::Chunk;

#[derive(Debug)]
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
#[derive(Debug)]
pub struct Object {
    pub objecttype: ObjectType,
    pub marked: bool
}

#[derive(Debug)]
pub struct ObjFucntion<'vm> {
    pub obj: Object,
    pub arity: u8,
    // upvalue_count: u8,
    pub chunk: &'vm Chunk<'vm>,
    pub name: String
}