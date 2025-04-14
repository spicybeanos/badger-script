use crate::expression::Value;

pub fn get_line_from_index(lines: &Vec<usize>, index: &usize) -> usize {
    // Find the first newline index greater than the given index
    match lines.binary_search(index) {
        // If the index matches exactly a newline index, return the corresponding line number
        Ok(pos) => pos + 1,
        // If the index is within a range, return the line number of the closest previous newline
        Err(pos) => pos + 1,
    }
}
pub fn get_col(index: &usize, lines: &Vec<usize>) -> usize {
    let l = get_line_from_index(lines, index);
    if lines.len() <= 0 {
        return *index;
    }

    if lines[l - 1] > *index {
        return *index ;
    } else {
        return index - lines[l - 1];
    }
}

pub fn error(msg: &str, index: &usize, lines: &Vec<usize>) -> Result<Value, String> {
    let l = get_line_from_index(lines, index);
    let c = get_col(index, lines);
    return Result::Err(format!("{} at line {}, {}", msg, l,c));
}
