#![allow(dead_code)]

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{
    con_not_eq::ConNotEq,
    constraint::{ConId, Constraint},
    scheduler::Scheduler,
    solver_stats::SolverStats,
    state::State,
    var::{Var, VarId, VarRef},
    vari::VarI, solver_options::SolverOptions,
};

pub type Variables = Vec<VarRef>;
pub type Constrains = Vec<Box<dyn Constraint>>;


#[derive(Debug)]
pub struct Inconsistency {}

pub enum FixPoint {
    Failure,
    Success,
}

pub struct Solver {
    op: SolverOptions,
    pub(crate) vars: Variables,
    pub(crate) cons: Constrains,
    sched: Rc<RefCell<Scheduler>>,
    state: Rc<RefCell<State>>,
    pub(crate) stat: SolverStats,
}

impl Solver {
    pub fn new(opt: SolverOptions) -> Self {
        let sched_observer = Rc::new(RefCell::new(Scheduler::new()));
        let state_observer = Rc::new(RefCell::new(State::new()));
        Solver {
            op: opt,
            vars: Vec::new(),
            cons: Vec::new(),
            sched: sched_observer,
            state: state_observer,
            stat: SolverStats::new(),
        }
    }

    pub fn events(&mut self) -> Ref<'_, Scheduler> {
        (&self.sched).borrow()
    }

    pub fn states(&mut self) -> Ref<'_, State> {
        (&self.state).borrow()
    }

    pub fn fix_point(&mut self) -> FixPoint {
        // notify
        loop {
            let mut sched = (self.sched).borrow_mut();
            let id = sched.pop();
            drop(sched);
            match id {
                Some(id) => {
                    if !self.propagate(id) {
                        break;
                    }
                }
                None => {
                    return FixPoint::Success;
                }
            }
        }
        let mut sched = (self.sched).borrow_mut();
        // Clear left constraints after failure to propagate
        sched.clear();
        return FixPoint::Failure;
    }

    // On successfull propagation return true
    pub fn propagate(&mut self, id: ConId) -> bool {
        return self.cons[id].propagate();
    }

    // Creates new variable, returns var id.
    pub fn var_int(&mut self, min: i32, max: i32) -> VarId {
        let id = self.vars.len();
        // Subscribe event stack to this variable
        let sched: Rc<RefCell<Scheduler>> = Rc::clone(&self.sched);
        // Subscribe state manager to this variable
        let state: Rc<RefCell<State>> = Rc::clone(&self.state);
        let v = VarI::new(id, min, max, sched, state);
        self.vars.push(Rc::new(RefCell::new(Var::VarI(v))));
        id
    }

    // Creates new state int
    // pub fn state_int(&mut self) -> StateInt {
    //     // Subscribe state manager to this state int
    //     let state: Rc<RefCell<State>> = Rc::clone(&self.state);
    //     StateInt::new(state)
    // }

    // creates new constraint: not eq, returns constraint id
    pub fn not_eq(&mut self, x: VarId, y: VarId, offset: i32) -> ConId {
        let conid = self.cons.len();
        // references to variables
        let xr = Rc::clone(&self.vars[x]);
        let yr = Rc::clone(&self.vars[y]);
        let c = ConNotEq::new_boxed(conid, xr, yr, offset);
        self.cons.push(c);
        // Subsctibe schedule for variable->constraint
        //let sched = Rc::clone(&self.sched);
        //sched.borrow_mut().subs_add_con(x, conid);
        //sched.borrow_mut().subs_add_con(y, conid);
        conid
    }
}

#[cfg(test)]
mod test_solver {

    use std::ops::Deref;

    use crate::scheduler::PropagateEvent;

    use super::*;

    #[test]
    fn test_solver_new() {
        let opt = SolverOptions {};
        let mut sol = Solver::new(opt);
        // create variables
        let v0 = sol.var_int(1, 2);
        let v1 = sol.var_int(3, 4);
        // add constraint
        sol.not_eq(v0, v1, 5);
        assert_eq!(sol.cons.len(), 1);
    }

    #[test]
    fn test_solver_notify() {
        let opt = SolverOptions {};
        let mut sol = Solver::new(opt);
        // create variables
        let v0 = sol.var_int(1, 2);
        let v1 = sol.var_int(3, 4);
        // add constraint
        let c0 = sol.not_eq(v0, v1, 5);

        let vvv = sol.vars[v0].clone();
        let txxx = (*vvv).borrow_mut();
        if let Var::VarI(xxx) = txxx.deref() {
            xxx.notify(&PropagateEvent::SubsOnBoundChange(v0, c0));
        }
    }

    #[test]
    fn test_solver_events_subs_size() {
        let opt = SolverOptions {};
        let mut sol = Solver::new(opt);
        // create variables
        sol.var_int(1, 2);
        sol.var_int(3, 4);
        let e0 = sol.events().subs_on_fix_size();
        let e1 = sol.events().subs_on_domain_size();
        let e2 = sol.events().subs_on_bound_size();
        assert_eq!(e0, 2);
        assert_eq!(e1, 2);
        assert_eq!(e2, 2);
    }

    #[test]
    fn test_solver_var_size() {
        let opt = SolverOptions {};
        let mut sol = Solver::new(opt);
        // create variables
        let v0 = sol.var_int(1, 2);
        let v1 = sol.var_int(3, 4);
        let v2 = sol.var_int(3, 4);
        let c0 = sol.not_eq(v0, v1, 5);
        let c1 = sol.not_eq(v1, v2, 6);
        sol.cons[c0].post(); // post constraint
        sol.cons[c1].post(); // post constraint
        
        let zzz = sol.events().subs_var_con_size(v0);
        assert_eq!(zzz, (1, 0, 0)); // subs on fix only
        
        let zzz = sol.events().subs_var_con_size(v1);
        assert_eq!(zzz, (2, 0, 0)); // subs on fix only
    }

    #[test]
    fn test_solver_not_eq() {
        let opt = SolverOptions {};
        let mut sol = Solver::new(opt);
        let v0 = sol.var_int(1, 5);
        let v1 = sol.var_int(3, 7);
        sol.not_eq(v0, v1, 0);
    }

    #[test]
    fn test_state_notify() {
        let opt = SolverOptions {};
        let mut sol = Solver::new(opt);
        // create variables
        let v0 = sol.var_int(1, 2);
        let v1 = sol.var_int(3, 4);
        // add constraint
        let c0 = sol.not_eq(v0, v1, 5);

        let vvv = sol.vars[v0].clone();
        let txxx = (*vvv).borrow_mut();
        if let Var::VarI(xxx) = txxx.deref() {
            xxx.notify(&PropagateEvent::SubsOnBoundChange(v0, c0));
        }
    }
}
