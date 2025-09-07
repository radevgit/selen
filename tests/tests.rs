use cspsolver::{model::Model, prelude::{int, Solution}, vars::Val, views::ViewExt};

#[test]
fn debug_minimal_hang() {
    println!("=== Testing simple positive domain [0, 2] ===");
    let mut model = Model::default();
    let x = model.new_var(Val::ValI(0), Val::ValI(2));
    
    let mut count = 0;
    let start = std::time::Instant::now();
    
    for solution in model.enumerate() {
        count += 1;
        println!("Solution {}: x = {:?}", count, solution[x]);
        
        if count > 10 || start.elapsed().as_secs() > 1 {
            println!("Breaking early - too many solutions or taking too long");
            break;
        }
    }
    
    println!("Found {} solutions in {:?}", count, start.elapsed());
    assert_eq!(count, 3, "Should find exactly 3 solutions for domain [0, 2]");
}

#[test]
fn debug_negative_domain() {
    println!("=== Testing negative domain [-1, 1] ===");
    let mut model = Model::default();
    let x = model.new_var(Val::ValI(-1), Val::ValI(1));
    
    let mut count = 0;
    let start = std::time::Instant::now();
    
    for solution in model.enumerate() {
        count += 1;
        println!("Solution {}: x = {:?}", count, solution[x]);
        
        if count > 10 || start.elapsed().as_secs() > 1 {
            println!("Breaking early - too many solutions or taking too long");
            break;
        }
    }
    
    println!("Found {} solutions in {:?}", count, start.elapsed());
    assert_eq!(count, 3, "Should find exactly 3 solutions for domain [-1, 1]");
}

    #[test]
    fn new_var() {
        let mut m = Model::default();

        // Using the old verbose syntax
        let _v1 = m.new_var(Val::ValI(1), Val::ValI(1));
        let _v2 = m.new_var(Val::ValI(1), Val::ValI(0));
        let _v3 = m.new_var(Val::ValI(0), Val::ValI(1));

        // Using the new convenient methods
        let _v4 = m.new_var(Val::int(1), Val::int(1));
        let _v5 = m.new_var(Val::int(1), Val::int(0));
        let _v6 = m.new_var(Val::int(0), Val::int(1));

        // Using the prelude functions (shortest)
        let _v7 = m.new_var(int(1), int(1));
        let _v8 = m.new_var(int(1), int(0));
        let _v9 = m.new_var(int(0), int(1));
    }

    #[test]
    fn new_vars() {
        let mut m = Model::default();

        // new_vars now always succeeds and handles bound inversion automatically
        let _vars1: Vec<_> = m.new_vars(5, int(1), int(1)).collect();
        let _vars2: Vec<_> = m.new_vars(5, int(1), int(0)).collect();
        let _vars3: Vec<_> = m.new_vars(5, int(0), int(1)).collect();
    }

    #[test]
    fn enumerate() {
        let mut m = Model::default();

        let (min, max) = (-7, 9);

        let _x = m.new_var(int(min), int(max));

        let mut solutions: Vec<_> = m.enumerate().collect();
        solutions.sort();

        let expected: Vec<_> = (min..=max).map(|v| Solution::from(vec![int(v)])).collect();

        assert_eq!(solutions, expected);
    }

    #[test]
    fn minimize() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        assert_eq!(m.minimize(x).unwrap()[x], int(-7));
    }

    #[test]
    fn maximize() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        assert_eq!(m.maximize(x).unwrap()[x], int(9));
    }

    #[test]
    fn opposite() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.opposite(), int(5));

        assert_eq!(m.solve().unwrap()[x], int(-5));
    }

    #[test]
    fn opposite_of_opposite() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.opposite().opposite(), int(6));

        assert_eq!(m.solve().unwrap()[x], int(6));
    }

    #[test]
    fn plus() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.plus(int(5)), int(7));

        assert_eq!(m.solve().unwrap()[x], int(2));
    }

    #[test]
    fn plus_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.plus(int(10)), int(1));

        assert!(m.solve().is_none());
    }

    #[test]
    fn times_with_neg_scale() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.times(int(-2)), int(4));

        assert_eq!(m.solve().unwrap()[x], int(-2));
    }

    #[test]
    fn times_with_neg_scale_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.times(int(-2)), int(3));

        assert!(m.solve().is_none());
    }

    #[test]
    fn times_with_zero_scale() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.times(int(0)), int(0));

        assert_eq!(m.maximize(x).unwrap()[x], int(9));
    }

    #[test]
    fn times_with_zero_scale_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.times(int(0)), int(4));

        assert!(m.solve().is_none());
    }

    #[test]
    fn times_with_pos_scale() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.times(int(2)), int(4));

        assert_eq!(m.solve().unwrap()[x], int(2));
    }

    #[test]
    fn times_with_pos_scale_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.times(int(2)), int(3));

        assert!(m.solve().is_none());
    }

    #[test]
    fn times_pos() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.times_pos(int(2)), int(4));

        assert_eq!(m.solve().unwrap()[x], int(2));
    }

    #[test]
    fn times_pos_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.times_pos(int(2)), int(3));

        assert!(m.solve().is_none());
    }

    #[test]
    fn times_neg() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.times_neg(int(-2)), int(4));

        assert_eq!(m.solve().unwrap()[x], int(-2));
    }

    #[test]
    fn times_neg_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x.times_neg(int(-2)), int(3));

        assert!(m.solve().is_none());
    }

    #[test]
    fn add() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));
        let y = m.new_var(int(-7), int(9));
        let p = m.add(x, y);

        m.equals(p, int(18));

        let solution = m.solve().unwrap();

        assert_eq!(solution[x], int(9));
        assert_eq!(solution[y], int(9));
        assert_eq!(solution[p], int(18));
    }

    #[test]
    fn sum() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));
        let y = m.new_var(int(-7), int(9));
        let s = m.sum(&[x, y]);

        let solution = m.maximize(s).unwrap();

        assert_eq!(solution[x], int(9));
        assert_eq!(solution[y], int(9));
        assert_eq!(solution[s], int(18));
    }

    #[test]
    fn equals() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));
        let y = m.new_var(int(4), int(8));

        m.equals(x, y);

        let solution = m.minimize(x).unwrap();

        assert_eq!(solution[x], int(4));
        assert_eq!(solution[y], int(4));
    }

    #[test]
    fn equals_with_constant() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.equals(x, int(4));

        assert_eq!(m.solve().unwrap()[x], int(4));
    }

    #[test]
    fn less_than_or_equals() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));
        let y = m.new_var(int(1), int(3));

        m.less_than_or_equals(x, y);

        let solution = m.maximize(x).unwrap();

        assert_eq!(solution[x], int(3));
        assert_eq!(solution[y], int(3));
    }

    #[test]
    fn less_than_or_equals_with_constant() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));

        m.less_than_or_equals(x, int(1));

        assert_eq!(m.maximize(x).unwrap()[x], int(1));
    }

    #[test]
    fn less_than() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));
        let y = m.new_var(int(1), int(3));

        m.less_than(x, y);

        let solution = m.maximize(x).unwrap();

        assert_eq!(solution[x], int(2));
        assert_eq!(solution[y], int(3));
    }

    #[test]
    fn greater_than_or_equals() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));
        let y = m.new_var(int(1), int(3));

        m.greater_than_or_equals(x, y);

        let solution = m.minimize(x).unwrap();

        assert_eq!(solution[x], int(1));
        assert_eq!(solution[y], int(1));
    }

    #[test]
    fn greater_than() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9));
        let y = m.new_var(int(1), int(3));

        m.greater_than(x, y);

        let solution = m.minimize(x).unwrap();

        assert_eq!(solution[x], int(2));
        assert_eq!(solution[y], int(1));
    }