use cspsolver::Model;
use cspsolver::optimization::classification::ProblemClassifier;

fn main() {
    // Test the classification for the failing test case
    let mut model = Model::with_float_precision(6);
    let float_x = model.float(0.0, 10.0);
    let float_y = model.float(5.0, 15.0);
    let int_a = model.int(0, 10);
    let int_b = model.int(5, 15);
    
    // Add constraints within each type (simulating separable problem)
    model.less_than_or_equals(float_x, float_y);
    model.less_than_or_equals(int_a, int_b);
    
    let vars = model.get_vars();
    let props = model.get_props();
    let problem_type = ProblemClassifier::classify(vars, props);
    
    println!("Problem classification: {:?}", problem_type);
}
