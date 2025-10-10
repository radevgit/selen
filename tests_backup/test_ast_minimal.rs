use selen::prelude::*;

#[test]
fn test_minimal_ast() {
    eprintln!("Creating model...");
    let mut model = Model::default();
    
    eprintln!("Creating variables...");
    let x = model.float(0.0, 10.0);
    let y = model.float(0.0, 10.0);
    let z = model.float(0.0, 20.0);
    
    eprintln!("Posting x.add(y).eq(z)...");
    model.new(x.add(y).eq(z));
    
    eprintln!("Posting z.le(5.0)...");
    model.new(z.le(5.0));
    
    eprintln!("Calling minimize...");
    let result = model.minimize(x);
    
    eprintln!("Result: {:?}", result.is_ok());
    assert!(result.is_ok());
}
