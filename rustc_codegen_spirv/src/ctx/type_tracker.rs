use rspirv::spirv::Word;

pub struct TypeTracker {}

impl TypeTracker {
    pub fn new() -> Self {
        Self {}
    }
}

impl SpirvType {
    pub fn def(&self) -> Word {
        match *self {
            SpirvType::Bool(def) => def,
            SpirvType::Integer(def, _, _) => def,
            SpirvType::Float(def, _) => def,
            SpirvType::ZST(def) => def,
            SpirvType::Adt { def, .. } => def,
            SpirvType::Pointer { def, .. } => def,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpirvType {
    Bool(Word),
    Integer(Word, u32, bool),
    Float(Word, u32),
    // TODO: Do we fold this into Adt?
    /// Zero Sized Type
    ZST(Word),
    /// This uses the rustc definition of "adt", i.e. a struct, enum, or union
    Adt {
        def: Word,
        // TODO: enums/unions
        field_types: Vec<SpirvType>,
    },
    Pointer {
        def: Word,
        pointee: Box<SpirvType>,
    },
}
