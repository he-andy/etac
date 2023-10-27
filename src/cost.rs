use crate::translate_types::ins;

//compute costs for translations
pub fn ins_cost(instruction: &ins) -> usize {
    instruction.0.cost()
}

/// computes total cost of a vector of instructions
pub fn total_cost(seq: &Vec<ins>) -> usize {
    seq.iter().map(|x| ins_cost(x)).sum()
}
