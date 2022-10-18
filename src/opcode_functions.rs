
use crate::opcode_enum::*;




pub fn fn_pop(stack: &mut State) -> Result<TBytes, StackError> {
    if stack.main_stack.len() <= 0{
        return Err(StackError::PopError);
    }
    let x = match stack.main_stack.pop(){
        Some(value) => value,
        None => return Err(StackError::PopError),

    };
    Ok(x)
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackError{
    PartsError,
    FunctionError,
    NameError,
    ConcatError,
    OpdupError,
    OpnipError,
    PopError,
    PushError,
    InvalidStackOperation,
    Verify,
    OpWithinError,
    StackTooLong,
}

use crate::tbytes_stuff::TBytes;

pub fn byte_array_to_number(byte_array: &TBytes) -> i64{
    let mut result = 0;
    for (x, &byte) in byte_array.data.iter().enumerate(){
        result |= (*&byte as i64) << (8 * x);
    };

    if byte_array.data[byte_array.data.len() -1] & 0x80 == 128{
        result = result & !((0x80 as i64) << (8 * (byte_array.data.len() -1)));
        result = -result;
        return result;
    }
    else{
        return result;
    }

}

pub fn serialize(value: i64) -> Result<Vec<u8>, StackError>{
    if value == 0{
        return Err(StackError::InvalidStackOperation);
    }

    let mut result: Vec<u8> = vec![];
    let neg: bool = value < 0;
    let mut absvalue:i64 = if neg == true{
        ! value + 1
    }
        else{
            value
        };
    
    while absvalue != 0{
    result.push((absvalue & 0xff) as u8);
        
        absvalue >>= 8;
    }
    
    let x = &result.len();
    
    if result[x -1] & 0x80 == 128{
        if neg == true{
            println!("value is negative");
            result.push(0x80);
        }
        else if neg == false{
            println!("value is positive");
            result.push(0);
        }
    else if neg{
        result[x -1] |= 0x80;
        println!("cvcvcv");
        }
    }
    Ok(result)
}

//iterate from top, down the stack (.rev())
//when we reach the nValue index we will pop then stop popping
//retrieve that value (at nValue index) and then push everything else back on in order
//once everything is pushed back on, we will push the retrieved value
//drain the stack into a vec, then push that vec as a loop onto the stack
pub fn op_roll(stack: &mut State) -> Result<(), StackError>{
    let popped = byte_array_to_number(&fn_pop(stack)?);
    let n_value_index = stack.main_stack.len() -(popped as usize);
    let vec: Vec<TBytes> = stack.main_stack.drain(n_value_index + 1..).rev().collect();
    let n_value = stack.main_stack.pop().unwrap();
    for byte in vec.into_iter().rev(){
        op_push(stack, byte).unwrap();
    }
    op_push(stack, n_value).unwrap();


    Ok(())
}

pub fn op_pick(stack: &mut State) -> Result<(), StackError>{
    let popped = byte_array_to_number(&fn_pop(stack)?);
    let n_value_index = stack.main_stack.len() -(popped as usize);
    let vec: Vec<TBytes> = stack.main_stack.drain(n_value_index..).collect();
    let n_value = stack.main_stack.pop().unwrap();
    op_push(stack, n_value.clone()).unwrap();
    for byte in vec.into_iter(){
        op_push(stack, byte).unwrap();
    }
    op_push(stack, n_value).unwrap();
    
    Ok(())
}
pub fn op_0_not_equal(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    if cast_to_bool(&x) == false{
        op_push(stack, x.op_0_not_equal_false(None)).unwrap();
    }else{
        op_push(stack, x.op_0_not_equal_true(None)).unwrap();
    }
    Ok(())
}

pub fn op_sub(stack: &mut State) -> Result<(), StackError> {
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let a = byte_array_to_number(&x);
    let b = byte_array_to_number(&y);
    let result = serialize(a - b).unwrap();
    op_push(stack, x.op_sub_impl(result, y, None)).unwrap();
    Ok(())
}

pub fn op_less_than(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    if byte_array_to_number(&x) < byte_array_to_number(&y){
        op_push(stack, x.op_less_than_true(y, None)).unwrap();
    }else{
        op_push(stack, x.op_less_than_false(y, None)).unwrap();
    }
    Ok(())
}

pub fn op_greater_than(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    if byte_array_to_number(&x) > byte_array_to_number(&y){
        op_push(stack, x.op_greater_than_true(y, None)).unwrap();
    }else{
        op_push(stack, x.op_greater_than_false(y, None)).unwrap();
    }
    Ok(())
}

pub fn op_less_than_or_equal(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    if byte_array_to_number(&x) <= byte_array_to_number(&y){
        op_push(stack, x.op_less_than_or_equal_true(y, None)).unwrap();
    }else{
        op_push(stack, x.op_less_than_or_equal_false(y, None)).unwrap();
    }
    Ok(())
}
pub fn op_greater_than_or_equal(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    if byte_array_to_number(&x) >= byte_array_to_number(&y){
        op_push(stack, x.op_greater_than_or_equal_true(y, None)).unwrap();
    }else{
        op_push(stack, x.op_greater_than_or_equal_false(y, None)).unwrap();
    }
    Ok(())
}

pub fn op_min(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let b = byte_array_to_number(&x);
    let c = byte_array_to_number(&y);
    if b < c{
        op_push(stack, x.op_min_true(y, serialize(b).unwrap(), None)).unwrap();
    }else{
        op_push(stack, x.op_min_false(y, serialize(c).unwrap(), None)).unwrap();
    }
    Ok(())

}

pub fn op_max(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let b = byte_array_to_number(&x);
    let c = byte_array_to_number(&y);
    if b > c{
        op_push(stack, x.op_min_true(y, serialize(b).unwrap(), None)).unwrap();
    }else{
        op_push(stack, x.op_min_false(y, serialize(c).unwrap(), None)).unwrap();
    }
    Ok(())

}





pub fn op_within(stack: &mut State) -> Result<(), StackError>{
    if stack.main_stack.len() < 3{
        return Err(StackError::OpWithinError)
    }

    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let z = fn_pop(stack)?;
    if byte_array_to_number(&x) >= byte_array_to_number(&y) 
    && byte_array_to_number(&x) < byte_array_to_number(&z){
        op_push(stack, x.op_within_impl(y, z, None)).unwrap();
    }
    else{
        op_push(stack, x.op_within_impl_false(y, z, None)).unwrap();
    }

    Ok(()) 
}

pub fn op_num_not_equal(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    if byte_array_to_number(&x) !=  byte_array_to_number(&y){
        x.op_num_not_equal_true(y, None);
    }else{
        x.op_num_not_equal_false(y, None);
    }
    Ok(())
}
pub fn op_div_two(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let i64num = byte_array_to_number(&x);
    op_push(stack, x.op_div_two_impl(serialize(i64num / 2).unwrap(), None)).unwrap();
    Ok(())
}
pub fn op_mul_two(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let i64num = byte_array_to_number(&x);
    op_push(stack, x.op_mul_two_impl(serialize(i64num * 2).unwrap(), None)).unwrap();
    Ok(())
}

pub fn op_equal_verify(stack: &mut State) -> Result<(), StackError>{
    op_equal(stack).unwrap();
    op_verify(stack).unwrap();
    Ok(())
}

pub fn op_negate(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let i64num = -byte_array_to_number(&x); 
    op_push(stack, x.op_negate(serialize(i64num).unwrap(), None)).unwrap();
    Ok(())
}

pub fn op_abs(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let i64num = byte_array_to_number(&x);
    op_push(stack, x.op_abs_impl(serialize(i64num.abs()).unwrap(), None)).unwrap();
    Ok(())
}

pub fn op_div(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let i64num = byte_array_to_number(&x) / byte_array_to_number(&y);
    op_push(stack, x.op_div_impl(serialize(i64num).unwrap(), y, None)).unwrap();
    Ok(())

}


pub fn op_add_1(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let a = byte_array_to_number(&x);
    let result = serialize(a +1).unwrap();
    op_push(stack, x.op_add_one(result, None)).unwrap();
    Ok(())
}

pub fn op_sub_1(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let a = byte_array_to_number(&x);
    let result = serialize(a - 1).unwrap();
    op_push(stack, x.op_add_one(result, None)).unwrap();
    Ok(())
}

pub fn op_bool_or(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    if cast_to_bool(&x) || cast_to_bool(&y) == true{
        op_push(stack, x.op_bool_or_true(y, None)).unwrap();
    }
    else{
        op_push(stack, x.op_bool_or_false(y, None)).unwrap();
    }
    Ok(())
}
pub fn op_bool_and(stack: &mut State) -> Result<(), StackError> {
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    if cast_to_bool(&x) && cast_to_bool(&y) == true{
        op_push(stack, x.op_bool_and_true(y, None)).unwrap();
    }
    else{
        op_push(stack, x.op_bool_and_false(y, None)).unwrap();
    }
    Ok(())
}

pub fn op_add(stack: &mut State) -> Result<(), StackError> {
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let a = byte_array_to_number(&x);
    let b = byte_array_to_number(&y);
    let result = serialize(a + b).unwrap();
    op_push(stack, x.op_add(result, y, None)).unwrap();
    Ok(())
}



pub fn op_invert(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y: Vec<u8> = x.data.iter().map(|x| !x).collect();
    op_push(stack, x.op_invert_impl(y, None)).unwrap();
    Ok(())
}

pub fn op_and(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let z: Vec<u8> = x.data.iter()
                    .zip(y.data
                    .iter())
                    .map(|(a,b)| a & b)
                    .collect();
    op_push(stack, x.op_and_impl(y, z, None)).unwrap();
    Ok(())
}

pub fn op_or(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let z: Vec<u8> = x.data.iter()
                    .zip(y.data
                    .iter())
                    .map(|(a,b)| a | b)
                    .collect();
    op_push(stack, x.op_or(z, y, None)).unwrap();
    Ok(())
}

pub fn op_not(stack: &mut State) -> Result<(), StackError>{
    let x = stack.main_stack.len() -1;
    if cast_to_bool(&stack.main_stack[x]) == false{
        let y = fn_pop(stack)?;
        op_push(stack, y.op_not_false(None)).unwrap();
        }
    else{ 
        let y = fn_pop(stack)?;
        op_push(stack, y.op_not_true(None)).unwrap();
    }

    Ok(())
}

pub fn cast_to_bool(referenced_top: &TBytes) -> bool{
    for (x, &byte) in referenced_top.data.iter().enumerate(){
        if byte != 0{
            if x == referenced_top.data.len() -1 && byte == 0x80{
                return false;
            }
            return true
        }
    }
    return false;
}

pub fn op_verify(stack: &mut State) -> Result<bool, StackError>{
    if stack.main_stack.len() < 1 {
        return Err(StackError::InvalidStackOperation);
    }
    let boolean_returned = cast_to_bool(&stack.main_stack.as_slice()[stack.main_stack.len() -1]);
    if boolean_returned{
        fn_pop(stack)?;
        Ok(true)
    }else{
        return Err(StackError::Verify);
    }
}



pub fn op_equal(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    if byte_array_to_number(&x) == byte_array_to_number(&y){
        op_push(stack, x.op_equal_true(y, None)).unwrap();
    }else{
        op_push(stack, x.op_equal_false( y, None)).unwrap();
    }
    Ok(())
}

pub fn op_hash256(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    op_push(stack, x.op_hash256(None)).unwrap();
    Ok(())
}


pub fn op_hash160(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;

    op_push(stack, x.op_hash160(None)).unwrap();
    
    Ok(())
}

pub fn op_sha256(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;

    op_push(stack, x.sha256(None)).unwrap();

    Ok(())
    
}

pub fn op_ripemd160(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;

    op_push(stack, x.ripemd160(None)).unwrap();

    Ok(())
}

pub fn op_sha1(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;

    op_push(stack, x.sha1(None)).unwrap();

    Ok(())
}


pub fn op_push(stack: &mut State, other: TBytes) -> Result<(), StackError>{
    if stack.main_stack.len() > 5{
        return Err(StackError::StackTooLong);
    }
    stack.main_stack.push(other);

    Ok(())
   
}

pub fn op_alt_pop(stack: &mut State) -> Result<TBytes, StackError>{
    let x = match stack.alt_stack.pop(){
        Some(value) => value,
        None => return Err(StackError::PopError),
    };

    Ok(x)


}

pub fn op_alt_push(stack: &mut State, other: TBytes) -> Result<(), StackError>{
    stack.alt_stack.push(other);
    
    Ok(())
}


pub fn op_concat(stack: &mut State) -> Result<(), StackError> {
    let z = String::from("Concat");
    let x = fn_pop(stack)?;

    let y = fn_pop(stack)?;
    
    stack.main_stack.push(x.concat(y, Some(z)));
    Ok(())
}

pub fn op_drop(stack: &mut State) -> Result<(), StackError>{
    fn_pop(stack)?;

     
    Ok(())  
}

pub fn op_dup(stack: &mut State) -> Result<(), StackError> {
    let x = fn_pop(stack)?;
    

    op_push(stack, x.clone()).unwrap();
    op_push(stack,x).unwrap();
    
    Ok(())
}

pub fn op_to_alt_stack(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;

    op_alt_push(stack, x).unwrap();

    Ok(())

}

pub fn op_from_alt_stack(stack: &mut State) -> Result<(), StackError>{
    let x = op_alt_pop(stack)?;
    
    op_push(stack, x).unwrap();
    
    Ok(())
}

pub fn op_nip(stack: &mut State) -> Result<(), StackError> {
    let x = fn_pop(stack)?;
    fn_pop(stack)?;
    
    op_push(stack, x).unwrap();

    Ok(())
}

pub fn op_over(stack: &mut State) -> Result<(), StackError> {
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;

    op_push(stack, y.clone()).unwrap();
    op_push(stack, x).unwrap();
    op_push(stack, y).unwrap();

    Ok(())
}

pub fn op_rot(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let z = fn_pop(stack)?;

    op_push(stack, y).unwrap();
    op_push(stack, x).unwrap();
    op_push(stack, z).unwrap();

    Ok(())
}


pub fn op_tuck(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let z = fn_pop(stack)?;

    op_push(stack, x.clone()).unwrap();
    op_push(stack, z).unwrap();
    op_push(stack, y).unwrap();
    op_push(stack, x).unwrap();
    Ok(())

}

pub fn op_swap(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;

    op_push(stack, x).unwrap();
    op_push(stack, y).unwrap();
    
    Ok(())
}

pub fn op_two_swap(stack: &mut State) -> Result<(), StackError>{
    let a = fn_pop(stack)?;
    let b = fn_pop(stack)?;
    let c = fn_pop(stack)?;
    let d = fn_pop(stack)?;

    op_push(stack,b).unwrap();
    op_push(stack,a).unwrap();
    op_push(stack,d).unwrap();
    op_push(stack,c).unwrap();


    Ok(())
}

pub fn op_two_drop(stack: &mut State) -> Result<(), StackError>{
    fn_pop(stack)?;
    fn_pop(stack)?;
    
    Ok(())
}

pub fn op_two_dup(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;

    op_push(stack, y.clone()).unwrap();
    op_push(stack, x.clone()).unwrap();
    op_push(stack, y).unwrap();
    op_push(stack, x).unwrap();

    Ok(())
}

pub fn op_three_dup(stack: &mut State) -> Result<(), StackError>{
    let x = fn_pop(stack)?;
    let y = fn_pop(stack)?;
    let z = fn_pop(stack)?;

    op_push(stack, z.clone()).unwrap();
    op_push(stack, y.clone()).unwrap();
    op_push(stack, x.clone()).unwrap();
    op_push(stack, z).unwrap();
    op_push(stack, y).unwrap();
    op_push(stack, x).unwrap();
    
    Ok(())
}

pub fn op_two_over(stack: &mut State) -> Result<(), StackError>{
    todo!();
    
    Ok(())
}

pub fn op_two_rot(stack: &mut State) -> Result<(), StackError>{
    let a = fn_pop(stack)?;
    let b = fn_pop(stack)?;
    let c = fn_pop(stack)?;
    let d = fn_pop(stack)?;
    let e = fn_pop(stack)?;
    let f = fn_pop(stack)?;

    op_push(stack, d).unwrap();
    op_push(stack, c).unwrap();
    op_push(stack, b).unwrap();
    op_push(stack, a).unwrap();
    op_push(stack, f).unwrap();
    op_push(stack, e).unwrap();

     
    Ok(()) 
} 