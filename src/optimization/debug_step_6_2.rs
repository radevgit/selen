//! Debug test to understand Step 6.2 partitioning behavior

#[cfg(test)]
mod debug_tests {
    use crate::model::Model;
    use crate::optimization::variable_partitioning::VariablePartitioner;
    use crate::optimization::classification::ProblemClassifier;

    #[test]
    fn debug_partitioning_behavior() {
        println!("\n=== Testing Empty Model ===");
        let empty_model = Model::with_float_precision(6);
        let empty_result = VariablePartitioner::partition_model(&empty_model);
        let empty_classification = ProblemClassifier::classify(empty_model.get_vars(), empty_model.get_props());
        println!("Empty model - Classification: {:?}", empty_classification);
        println!("Empty model - Partition result: float={:?}, integer={:?}, separable={}, vars={}, constraints={}", 
                 empty_result.float_partition.is_some(),
                 empty_result.integer_partition.is_some(),
                 empty_result.is_separable,
                 empty_result.total_variables,
                 empty_result.total_constraints);

        println!("\n=== Testing Mixed Model ===");
        let mut mixed_model = Model::with_float_precision(6);
        let float_x = mixed_model.float(0.0, 10.0);
        let float_y = mixed_model.float(5.0, 15.0);
        let int_a = mixed_model.int(0, 10);
        let int_b = mixed_model.int(5, 15);

        mixed_model.le(float_x, float_y);
        mixed_model.le(int_a, int_b);

        let mixed_result = VariablePartitioner::partition_model(&mixed_model);
        let mixed_classification = ProblemClassifier::classify(mixed_model.get_vars(), mixed_model.get_props());
        println!("Mixed model - Classification: {:?}", mixed_classification);
        println!("Mixed model - Partition result: float={:?}, integer={:?}, separable={}, vars={}, constraints={}", 
                 mixed_result.float_partition.is_some(),
                 mixed_result.integer_partition.is_some(),
                 mixed_result.is_separable,
                 mixed_result.total_variables,
                 mixed_result.total_constraints);

        println!("\n=== Testing Float-Only Model ===");
        let mut float_model = Model::with_float_precision(6);
        let x = float_model.float(0.0, 10.0);
        let y = float_model.float(5.0, 15.0);
        let _int_a = float_model.int(0, 10);

        float_model.le(x, y);

        let float_result = VariablePartitioner::partition_model(&float_model);
        let float_classification = ProblemClassifier::classify(float_model.get_vars(), float_model.get_props());
        println!("Float model - Classification: {:?}", float_classification);
        println!("Float model - Partition result: float={:?}, integer={:?}, separable={}, vars={}, constraints={}", 
                 float_result.float_partition.is_some(),
                 float_result.integer_partition.is_some(),
                 float_result.is_separable,
                 float_result.total_variables,
                 float_result.total_constraints);
    }
}
