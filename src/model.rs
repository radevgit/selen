use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Model {
    vars: Vars,
    props: Propagators,
}

impl Model {
    /// Create a new integer decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// This function will only create a decision variable if `min < max`.
    pub fn new_var(&mut self, min: Val, max: Val) -> Option<VarId> {
        if min < max {
            Some(self.new_var_unchecked(min, max))
        } else {
            None
        }
    }

    /// Create new decision variables, with the provided domain bounds.
    ///
    /// All created variables will have the same starting domain bounds.
    /// Both lower and upper bounds are included in the domain.
    /// This function will only create decision variables if `min < max`.
    pub fn new_vars(
        &mut self,
        n: usize,
        min: Val,
        max: Val,
    ) -> Option<impl Iterator<Item = VarId> + '_> {
        if min < max {
            Some(core::iter::repeat_with(move || self.new_var_unchecked(min, max)).take(n))
        } else {
            None
        }
    }

    /// Create a new binary decision variable.
    pub fn new_var_binary(&mut self) -> VarIdBin {
        VarIdBin(self.new_var_unchecked(Val::ValI(0), Val::ValI(1)))
    }

    /// Create new binary decision variables.
    pub fn new_vars_binary(&mut self, n: usize) -> impl Iterator<Item = VarIdBin> + '_ {
        core::iter::repeat_with(|| self.new_var_binary()).take(n)
    }

    /// Create a new integer decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    ///
    /// This function assumes that `min < max`.
    fn new_var_unchecked(&mut self, min: Val, max: Val) -> VarId {
        self.props.on_new_var();
        self.vars.new_var_with_bounds(min, max)
    }

    /// Create an expression of two views added together.
    pub fn add(&mut self, x: impl View, y: impl View) -> VarId {
        let min = x.min_raw(&self.vars) + y.min_raw(&self.vars);
        let max = x.max_raw(&self.vars) + y.max_raw(&self.vars);
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.add(x, y, s);

        s
    }

    /// Create an expression of the sum of a slice of views.
    pub fn sum(&mut self, xs: &[impl View]) -> VarId {
        self.sum_iter(xs.iter().copied())
    }

    /// Create an expression of the sum of an iterator of views.
    pub fn sum_iter(&mut self, xs: impl IntoIterator<Item = impl View>) -> VarId {
        let xs: Vec<_> = xs.into_iter().collect();

        let min: Val = xs.iter().map(|x| x.min_raw(&self.vars)).sum();
        let max: Val = xs.iter().map(|x| x.max_raw(&self.vars)).sum();
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.sum(xs, s);

        s
    }

    /// Declare two expressions to be equal.
    pub fn equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.equals(x, y);
    }

    /// Declare constraint `x <= y`.
    pub fn less_than_or_equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.less_than_or_equals(x, y);
    }

    /// Declare constraint `x < y`.
    pub fn less_than(&mut self, x: impl View, y: impl View) {
        let _p = self.props.less_than(x, y);
    }

    /// Declare constraint `x >= y`.
    pub fn greater_than_or_equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.greater_than_or_equals(x, y);
    }

    /// Declare constraint `x > y`.
    pub fn greater_than(&mut self, x: impl View, y: impl View) {
        let _p = self.props.greater_than(x, y);
    }

    /// Find assignment that minimizes objective expression while satisfying all constraints.
    #[must_use]
    pub fn minimize(self, objective: impl View) -> Option<Solution> {
        self.minimize_and_iterate(objective).last()
    }

    /// Enumerate assignments that satisfy all constraints, while minimizing objective expression.
    ///
    /// The order in which assignments are yielded is not stable.
    pub fn minimize_and_iterate(self, objective: impl View) -> impl Iterator<Item = Solution> {
        search(self.vars, self.props, mode::Minimize::new(objective))
    }

    /// Find assignment that maximizes objective expression while satisfying all constraints.
    #[must_use]
    pub fn maximize(self, objective: impl View) -> Option<Solution> {
        self.minimize(objective.opposite())
    }

    /// Enumerate assignments that satisfy all constraints, while maximizing objective expression.
    ///
    /// The order in which assignments are yielded is not stable.
    pub fn maximize_and_iterate(self, objective: impl View) -> impl Iterator<Item = Solution> {
        self.minimize_and_iterate(objective.opposite())
    }

    /// Search for assignment that satisfies all constraints within bounds of decision variables.
    #[must_use]
    pub fn solve(self) -> Option<Solution> {
        self.enumerate().next()
    }

    /// Enumerate all assignments that satisfy all constraints.
    ///
    /// The order in which assignments are yielded is not stable.
    pub fn enumerate(self) -> impl Iterator<Item = Solution> {
        search(self.vars, self.props, mode::Enumerate)
    }
}

#[cfg(test)]
mod test_model {
    use crate::{model::Model, prelude::int, solution::Solution, vars::Val, views::ViewExt};

    #[test]
    fn new_var() {
        let mut m = Model::default();

        // Using the old verbose syntax
        assert!(m.new_var(Val::ValI(1), Val::ValI(1)).is_none());
        assert!(m.new_var(Val::ValI(1), Val::ValI(0)).is_none());
        assert!(m.new_var(Val::ValI(0), Val::ValI(1)).is_some());

        // Using the new convenient methods
        assert!(m.new_var(Val::int(1), Val::int(1)).is_none());
        assert!(m.new_var(Val::int(1), Val::int(0)).is_none());
        assert!(m.new_var(Val::int(0), Val::int(1)).is_some());

        // Using the prelude functions (shortest)
        assert!(m.new_var(int(1), int(1)).is_none());
        assert!(m.new_var(int(1), int(0)).is_none());
        assert!(m.new_var(int(0), int(1)).is_some());
    }

    #[test]
    fn new_vars() {
        let mut m = Model::default();

        assert!(m.new_vars(5, int(1), int(1)).is_none());
        assert!(m.new_vars(5, int(1), int(0)).is_none());
        assert!(m.new_vars(5, int(0), int(1)).is_some());
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

        let x = m.new_var(int(-7), int(9)).unwrap();

        assert_eq!(m.minimize(x).unwrap()[x], int(-7));
    }

    #[test]
    fn maximize() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        assert_eq!(m.maximize(x).unwrap()[x], int(9));
    }

    #[test]
    fn opposite() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.opposite(), int(5));

        assert_eq!(m.solve().unwrap()[x], int(-5));
    }

    #[test]
    fn opposite_of_opposite() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.opposite().opposite(), int(6));

        assert_eq!(m.solve().unwrap()[x], int(6));
    }

    #[test]
    fn plus() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.plus(int(5)), int(7));

        assert_eq!(m.solve().unwrap()[x], int(2));
    }

    #[test]
    fn plus_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.plus(int(10)), int(1));

        assert!(m.solve().is_none());
    }

    #[test]
    fn times_with_neg_scale() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.times(int(-2)), int(4));

        assert_eq!(m.solve().unwrap()[x], int(-2));
    }

    #[test]
    fn times_with_neg_scale_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.times(int(-2)), int(3));

        assert!(m.solve().is_none());
    }

    #[test]
    fn times_with_zero_scale() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.times(int(0)), int(0));

        assert_eq!(m.maximize(x).unwrap()[x], int(9));
    }

    #[test]
    fn times_with_zero_scale_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.times(int(0)), int(4));

        assert!(m.solve().is_none());
    }

    #[test]
    fn times_with_pos_scale() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.times(int(2)), int(4));

        assert_eq!(m.solve().unwrap()[x], int(2));
    }

    #[test]
    fn times_with_pos_scale_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.times(int(2)), int(3));

        assert!(m.solve().is_none());
    }

    #[test]
    fn times_pos() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.times_pos(int(2)), int(4));

        assert_eq!(m.solve().unwrap()[x], int(2));
    }

    #[test]
    fn times_pos_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.times_pos(int(2)), int(3));

        assert!(m.solve().is_none());
    }

    #[test]
    fn times_neg() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.times_neg(int(-2)), int(4));

        assert_eq!(m.solve().unwrap()[x], int(-2));
    }

    #[test]
    fn times_neg_unfeasible() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x.times_neg(int(-2)), int(3));

        assert!(m.solve().is_none());
    }

    #[test]
    fn add() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();
        let y = m.new_var(int(-7), int(9)).unwrap();
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

        let x = m.new_var(int(-7), int(9)).unwrap();
        let y = m.new_var(int(-7), int(9)).unwrap();
        let s = m.sum(&[x, y]);

        let solution = m.maximize(s).unwrap();

        assert_eq!(solution[x], int(9));
        assert_eq!(solution[y], int(9));
        assert_eq!(solution[s], int(18));
    }

    #[test]
    fn equals() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();
        let y = m.new_var(int(4), int(8)).unwrap();

        m.equals(x, y);

        let solution = m.minimize(x).unwrap();

        assert_eq!(solution[x], int(4));
        assert_eq!(solution[y], int(4));
    }

    #[test]
    fn equals_with_constant() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.equals(x, int(4));

        assert_eq!(m.solve().unwrap()[x], int(4));
    }

    #[test]
    fn less_than_or_equals() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();
        let y = m.new_var(int(1), int(3)).unwrap();

        m.less_than_or_equals(x, y);

        let solution = m.maximize(x).unwrap();

        assert_eq!(solution[x], int(3));
        assert_eq!(solution[y], int(3));
    }

    #[test]
    fn less_than_or_equals_with_constant() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();

        m.less_than_or_equals(x, int(1));

        assert_eq!(m.maximize(x).unwrap()[x], int(1));
    }

    #[test]
    fn less_than() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();
        let y = m.new_var(int(1), int(3)).unwrap();

        m.less_than(x, y);

        let solution = m.maximize(x).unwrap();

        assert_eq!(solution[x], int(2));
        assert_eq!(solution[y], int(3));
    }

    #[test]
    fn greater_than_or_equals() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();
        let y = m.new_var(int(1), int(3)).unwrap();

        m.greater_than_or_equals(x, y);

        let solution = m.minimize(x).unwrap();

        assert_eq!(solution[x], int(1));
        assert_eq!(solution[y], int(1));
    }

    #[test]
    fn greater_than() {
        let mut m = Model::default();

        let x = m.new_var(int(-7), int(9)).unwrap();
        let y = m.new_var(int(1), int(3)).unwrap();

        m.greater_than(x, y);

        let solution = m.minimize(x).unwrap();

        assert_eq!(solution[x], int(2));
        assert_eq!(solution[y], int(1));
    }
}
