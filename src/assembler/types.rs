#[derive(Debug, Clone)]
pub enum Directive {
    Instruction(Vec<String>),
    DataByte(u8),
}
