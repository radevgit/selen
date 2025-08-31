


#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn new_var() {
        // constraint: v0 * 1.5 < 5.0
        // solving for maximum v0
        let mut m = Model::default();

        let v0 = m.new_var_int(1, 3);
        println!("v0 domain: [1, 3]");
        
        
        m.less_than(v0.times_pos(float(1.5)), float(5.0));

        let solution = m.maximize(v0).unwrap();
        let x = match solution[v0] {
            Val::ValI(int_val) => int_val,
            _ => panic!("Expected integer value"),
        };
        
        assert!(x == 3);
    }
}