
pub fn calculate_single_fuel(val :i64) -> i64 {
    let mut result:i64;
    result = 0;
    if val > 0 {
        let test = (val / 3 -2 ) as i64;
        
        if test > 0 {
            result = test as i64;
        } else {
            result = 0;
        }
    }
    return result;
} 

