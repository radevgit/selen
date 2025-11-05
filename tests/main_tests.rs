
//! Main integration test file that includes all other tests as modules
//! This reduces compilation time by having a single test executable instead of many.

#[path = "../tests_all/test_propagator_framework.rs"]
mod test_propagator_framework;

#[path = "../tests_all/test_between_coverage.rs"]
mod test_between_coverage;

#[path = "../tests_all/test_array_api_coverage.rs"]
mod test_array_api_coverage;

#[path = "../tests_all/test_global_constraints_coverage.rs"]
mod test_global_constraints_coverage;

#[path = "../tests_all/test_neq_coverage.rs"]
mod test_neq_coverage;

#[path = "../tests_all/test_neq_propagation.rs"]
mod test_neq_propagation;

#[path = "../tests_all/test_alldiff_coverage.rs"]
mod test_alldiff_coverage;

#[path = "../tests_all/test_error_coverage.rs"]
mod test_error_coverage;

#[path = "../tests_all/test_solution_coverage.rs"]
mod test_solution_coverage;

#[path = "../tests_all/test_validation_coverage.rs"]
mod test_validation_coverage;

#[path = "../tests_all/test_bool_constant.rs"]
mod test_bool_constant;

#[path = "../tests_all/test_array_float_constraints.rs"]
mod test_array_float_constraints;

#[path = "../tests_all/test_ast_extraction.rs"]
mod test_ast_extraction;

#[path = "../tests_all/test_ast_minimal.rs"]
mod test_ast_minimal;

#[path = "../tests_all/test_basic_gac.rs"]
mod test_basic_gac;

#[path = "../tests_all/test_bool_clause.rs"]
mod test_bool_clause;

#[path = "../tests_all/test_bool_lin_constraints.rs"]
mod test_bool_lin_constraints;

#[path = "../tests_all/test_callback_api.rs"]
mod test_callback_api;

#[path = "../tests_all/test_constraints_coverage_2.rs"]
mod test_constraints_coverage_2;

#[path = "../tests_all/test_constraints_coverage.rs"]
mod test_constraints_coverage;

#[path = "../tests_all/test_core_coverage.rs"]
mod test_core_coverage;

#[path = "../tests_all/test_div.rs"]
mod test_div;

#[path = "../tests_all/test_expression_to_linear.rs"]
mod test_expression_to_linear;

#[path = "../tests_all/test_float_comparison_reif.rs"]
mod test_float_comparison_reif;

#[path = "../tests_all/test_float_constraints.rs"]
mod test_float_constraints;

#[path = "../tests_all/test_float_lin_reif.rs"]
mod test_float_lin_reif;

#[path = "../tests_all/test_float_precision_tolerance.rs"]
mod test_float_precision_tolerance;

#[path = "../tests_all/test_int_lin_ne.rs"]
mod test_int_lin_ne;

#[path = "../tests_all/test_int_lin_non_reified.rs"]
mod test_int_lin_non_reified;

#[path = "../tests_all/test_int_lin_reif.rs"]
mod test_int_lin_reif;

#[path = "../tests_all/test_linear_constraints.rs"]
mod test_linear_constraints;

#[path = "../tests_all/test_linear_conversion_debug.rs"]
mod test_linear_conversion_debug;

#[path = "../tests_all/test_lp_csp_integration.rs"]
mod test_lp_csp_integration;

#[path = "../tests_all/test_lp_extraction.rs"]
mod test_lp_extraction;

#[path = "../tests_all/test_lp_integration.rs"]
mod test_lp_integration;

#[path = "../tests_all/test_lp_large_domains.rs"]
mod test_lp_large_domains;

#[path = "../tests_all/test_lp_performance.rs"]
mod test_lp_performance;

#[path = "../tests_all/test_lpsolver_integration.rs"]
mod test_lpsolver_integration;

#[path = "../tests_all/test_metadata_collection.rs"]
mod test_metadata_collection;

#[path = "../tests_all/test_min_max_error_handling.rs"]
mod test_min_max_error_handling;

#[path = "../tests_all/test_new_api_constants.rs"]
mod test_new_api_constants;

#[path = "../tests_all/test_new_api_linear.rs"]
mod test_new_api_linear;

#[path = "../tests_all/test_panic_fix_complete.rs"]
mod test_panic_fix_complete;

#[path = "../tests_all/test_phase2_integration.rs"]
mod test_phase2_integration;

#[path = "../tests_all/test_platinum_sudoku.rs"]
mod test_platinum_sudoku;

#[path = "../tests_all/test_precision_config.rs"]
mod test_precision_config;

#[path = "../tests_all/test_reification.rs"]
mod test_reification;

#[path = "../tests_all/test_reified_methods.rs"]
mod test_reified_methods;

#[path = "../tests_all/test_reif_minimal.rs"]
mod test_reif_minimal;

#[path = "../tests_all/test_reif_trace.rs"]
mod test_reif_trace;

#[path = "../tests_all/test_runtime_api.rs"]
mod test_runtime_api;

#[path = "../tests_all/test_safe_solution_access.rs"]
mod test_safe_solution_access;

#[path = "../tests_all/test_simple_alldiff.rs"]
mod test_simple_alldiff;

#[path = "../tests_all/test_type_conversions.rs"]
mod test_type_conversions;

#[path = "../tests_all/test_unbounded_inference.rs"]
mod test_unbounded_inference;

#[path = "../tests_all/test_unbounded_variables.rs"]
mod test_unbounded_variables;

#[path = "../tests_all/test_implies.rs"]
mod test_implies;

#[path = "../tests_all/test_count_var.rs"]
mod test_count_var;

#[path = "../tests_all/test_count_view_flexibility.rs"]
mod test_count_view_flexibility;

#[path = "../tests_all/test_bool_xor.rs"]
mod test_bool_xor;

#[path = "../tests_all/test_element_computed_index.rs"]
mod test_element_computed_index;

#[path = "../tests_all/test_modulo_comprehensive.rs"]
mod test_modulo_comprehensive;

#[path = "../tests_all/test_modulo_alleq_bug.rs"]
mod test_modulo_alleq_bug;

#[path = "../tests_all/test_newly_implemented_functions.rs"]
mod test_newly_implemented_functions;

#[path = "../tests_all/test_incremental_sum_integration.rs"]
mod test_incremental_sum_integration;

