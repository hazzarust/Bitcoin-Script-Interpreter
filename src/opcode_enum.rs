use crate::tbytes_stuff::TBytes;

#[derive(Debug, Clone, PartialEq)]
pub enum Function{
    Opcode(Opcode),
    Plain,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op{
    Push(TBytes),
    Opcode(Opcode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode{
    OpCat,
    OpSha256,
    OpEqualVerify,
    OpDrop,
    OpDup,
    OpNip,
    OpOver,
    OpRot,
    OpTuck,
    OpTwoDrop,
    OpTwoDup,
    OpThreeDup,
    OpTwoRot,
    OpSwap,
    OpTwoSwap,
    OpToAltStack,
    OpFromAltStack,
    OpRipeMD160,
    OpSha1,
    OpHash160,
    OpHash256,
    OpEqual,
    OpNOT,
    OpOr,
    OpAdd,
    OpBoolAnd,
    OpBoolOr,
    OpAddOne,
    OpSubOne,
    OpNegate,
    OpAbs,
    OpDiv,
    OpDivTwo,
    OpMulTwo,
    OpNumEqualverify,
    OpNumNotEqual,
    OpWithin,
    OpLessThan,
    OpGreaterThan,
    OpLessThanOrEqual,
    OpGreaterThanOrEqual,
    OpMin,
    OpMax,
    OpInvert,
    OpAnd,
    Op1add,
    Op1sub,
    Op0NotEqual,
    OpSub,
    OpPick,
    OpRoll,

}


#[derive(Debug)]
pub struct State {
    pub main_stack: Vec<TBytes>,
    pub alt_stack: Vec<TBytes>,

}