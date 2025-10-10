// Auto-generated Selen test program from FlatZinc
// This program can be compiled and run independently to debug Selen behavior
//
// PROBLEM DESCRIPTION:
//   Type: Maximization problem
//   Objective: maximize Ident("revenue")
//   Expected: Find solution with largest objective value
//   Variables: 254
//   Constraints: 247
//
// NOTE: If all output variables are zero in a maximization problem,
//       this suggests the solver is not optimizing correctly.

use selen::prelude::*;
use selen::variables::Val;

fn main() {
    use selen::utils::config::SolverConfig;
    let config = SolverConfig {
        timeout_ms: Some(600_000), // 5 minute timeout (in milliseconds)
        max_memory_mb: Some(4096), // 4GB memory limit
        ..Default::default()
    };
    let mut model = Model::with_config(config);

    // ===== PARAMETER ARRAYS =====
    let x_introduced_212_ = vec![0.2074688796680498, 1.346801346801347]; // 2 elements
    let x_introduced_214_ = vec![3.125, 3.75]; // 2 elements
    let x_introduced_216_ = vec![1.047619047619048, 4.761904761904762, -0.1226993865030675]; // 3 elements
    let x_introduced_218_ = vec![0.49079754601227, 14.28571428571428, -0.3809523809523809]; // 3 elements
    let x_introduced_220_ = vec![0.35, 0.8, 0.04, 0.25]; // 4 elements
    let x_introduced_222_ = vec![0.3, 0.02, 0.09, 0.4]; // 4 elements
    let x_introduced_224_ = vec![0.21, 0.32, 4.82, 0.07]; // 4 elements
    let x_introduced_226_ = vec![-1.0, 1.0, -0.195]; // 3 elements
    let x_introduced_301_ = vec![1.0, -0.06, -0.1, -0.15, -0.2, -0.25, -0.3, -0.3125, -0.325, -0.35, -0.4, -0.45, -0.5, -0.55, -0.6, -0.65, -0.6625, -0.66875, -0.7, -0.75, -0.8, -0.85, -0.9, -0.95, -1.0, -1.025, -1.05, -1.1, -1.15, -1.2, -1.25, -1.3, -1.35, -1.4, -1.45, -1.5]; // 36 elements
    let x_introduced_491_ = vec![0.06, 0.1, 0.15, 0.2, 0.25, 0.3, 0.3125, 0.325, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.6625, 0.66875, 0.7, 0.75, 0.8, 0.85, 0.9, 0.95, 1.0, 1.025, 1.05, 1.1, 1.15, 1.2, 1.25, 1.3, 1.35, 1.4, 1.45, 1.5, 1.0, -0.06, -0.1, -0.15, -0.2, -0.25, -0.3, -0.3125, -0.325, -0.35, -0.4, -0.45, -0.5, -0.55, -0.6, -0.65, -0.6625, -0.66875, -0.7, -0.75, -0.8, -0.85, -0.9, -0.95, -1.0, -1.025, -1.05, -1.1, -1.15, -1.2, -1.25, -1.3, -1.35, -1.4, -1.45, -1.5]; // 71 elements
    let x_introduced_563_ = vec![1.0, -0.0036, -0.01, -0.0225, -0.04000000000000001, -0.0625, -0.09, -0.09765625, -0.105625, -0.1225, -0.16, -0.2025, -0.25, -0.3025, -0.36, -0.4225, -0.43890625, -0.4472265624999999, -0.4899999999999999, -0.5625, -0.6400000000000001, -0.7224999999999999, -0.81, -0.9025, -1.0, -1.050625, -1.1025, -1.21, -1.3225, -1.44, -1.5625, -1.69, -1.8225, -1.96, -2.1025, -2.25]; // 36 elements
    let x_introduced_753_ = vec![-0.0036, -0.01, -0.0225, -0.04000000000000001, -0.0625, -0.09, -0.09765625, -0.105625, -0.1225, -0.16, -0.2025, -0.25, -0.3025, -0.36, -0.4225, -0.43890625, -0.4472265624999999, -0.4899999999999999, -0.5625, -0.6400000000000001, -0.7224999999999999, -0.81, -0.9025, -1.0, -1.050625, -1.1025, -1.21, -1.3225, -1.44, -1.5625, -1.69, -1.8225, -1.96, -2.1025, -2.25, 1.0, -0.0036, -0.01, -0.0225, -0.04000000000000001, -0.0625, -0.09, -0.09765625, -0.105625, -0.1225, -0.16, -0.2025, -0.25, -0.3025, -0.36, -0.4225, -0.43890625, -0.4472265624999999, -0.4899999999999999, -0.5625, -0.6400000000000001, -0.7224999999999999, -0.81, -0.9025, -1.0, -1.050625, -1.1025, -1.21, -1.3225, -1.44, -1.5625, -1.69, -1.8225, -1.96, -2.1025, -2.25]; // 71 elements
    let x_introduced_755_ = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]; // 35 elements
    let x_introduced_798_ = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]; // 70 elements

    // ===== VARIABLES =====
    let milk = model.float(f64::NEG_INFINITY, f64::INFINITY); // milk (unbounded)
    let milksq = model.float(f64::NEG_INFINITY, f64::INFINITY); // milksq (unbounded)
    let butt = model.float(f64::NEG_INFINITY, f64::INFINITY); // butt (unbounded)
    let buttsq = model.float(f64::NEG_INFINITY, f64::INFINITY); // buttsq (unbounded)
    let cha = model.float(f64::NEG_INFINITY, f64::INFINITY); // cha (unbounded)
    let chasq = model.float(f64::NEG_INFINITY, f64::INFINITY); // chasq (unbounded)
    let chb = model.float(f64::NEG_INFINITY, f64::INFINITY); // chb (unbounded)
    let chbsq = model.float(f64::NEG_INFINITY, f64::INFINITY); // chbsq (unbounded)
    let xm = model.float(f64::NEG_INFINITY, f64::INFINITY); // xm (unbounded)
    let xb = model.float(f64::NEG_INFINITY, f64::INFINITY); // xb (unbounded)
    let xca = model.float(f64::NEG_INFINITY, f64::INFINITY); // xca (unbounded)
    let xcb = model.float(f64::NEG_INFINITY, f64::INFINITY); // xcb (unbounded)
    let q = model.float(f64::NEG_INFINITY, f64::INFINITY); // q (unbounded)
    let qsq = model.float(f64::NEG_INFINITY, f64::INFINITY); // qsq (unbounded)
    let x_introduced_0_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_0_ (unbounded)
    let x_introduced_1_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_1_ (unbounded)
    let x_introduced_2_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_2_ (unbounded)
    let x_introduced_3_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_3_ (unbounded)
    let x_introduced_4_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_4_ (unbounded)
    let x_introduced_5_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_5_ (unbounded)
    let x_introduced_6_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_6_ (unbounded)
    let x_introduced_7_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_7_ (unbounded)
    let x_introduced_8_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_8_ (unbounded)
    let x_introduced_9_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_9_ (unbounded)
    let x_introduced_10_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_10_ (unbounded)
    let x_introduced_11_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_11_ (unbounded)
    let x_introduced_12_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_12_ (unbounded)
    let x_introduced_13_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_13_ (unbounded)
    let x_introduced_14_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_14_ (unbounded)
    let x_introduced_15_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_15_ (unbounded)
    let x_introduced_16_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_16_ (unbounded)
    let x_introduced_17_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_17_ (unbounded)
    let x_introduced_18_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_18_ (unbounded)
    let x_introduced_19_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_19_ (unbounded)
    let x_introduced_20_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_20_ (unbounded)
    let x_introduced_21_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_21_ (unbounded)
    let x_introduced_22_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_22_ (unbounded)
    let x_introduced_23_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_23_ (unbounded)
    let x_introduced_24_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_24_ (unbounded)
    let x_introduced_25_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_25_ (unbounded)
    let x_introduced_26_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_26_ (unbounded)
    let x_introduced_27_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_27_ (unbounded)
    let x_introduced_28_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_28_ (unbounded)
    let x_introduced_29_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_29_ (unbounded)
    let x_introduced_30_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_30_ (unbounded)
    let x_introduced_31_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_31_ (unbounded)
    let x_introduced_32_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_32_ (unbounded)
    let x_introduced_33_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_33_ (unbounded)
    let x_introduced_34_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_34_ (unbounded)
    let x_introduced_35_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_35_ (unbounded)
    let x_introduced_36_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_36_ (unbounded)
    let x_introduced_37_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_37_ (unbounded)
    let x_introduced_38_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_38_ (unbounded)
    let x_introduced_39_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_39_ (unbounded)
    let x_introduced_40_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_40_ (unbounded)
    let x_introduced_41_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_41_ (unbounded)
    let x_introduced_42_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_42_ (unbounded)
    let x_introduced_43_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_43_ (unbounded)
    let x_introduced_44_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_44_ (unbounded)
    let x_introduced_45_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_45_ (unbounded)
    let x_introduced_46_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_46_ (unbounded)
    let x_introduced_47_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_47_ (unbounded)
    let x_introduced_48_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_48_ (unbounded)
    let x_introduced_49_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_49_ (unbounded)
    let x_introduced_50_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_50_ (unbounded)
    let x_introduced_51_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_51_ (unbounded)
    let x_introduced_52_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_52_ (unbounded)
    let x_introduced_53_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_53_ (unbounded)
    let x_introduced_54_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_54_ (unbounded)
    let x_introduced_55_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_55_ (unbounded)
    let x_introduced_56_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_56_ (unbounded)
    let x_introduced_57_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_57_ (unbounded)
    let x_introduced_58_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_58_ (unbounded)
    let x_introduced_59_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_59_ (unbounded)
    let x_introduced_60_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_60_ (unbounded)
    let x_introduced_61_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_61_ (unbounded)
    let x_introduced_62_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_62_ (unbounded)
    let x_introduced_63_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_63_ (unbounded)
    let x_introduced_64_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_64_ (unbounded)
    let x_introduced_65_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_65_ (unbounded)
    let x_introduced_66_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_66_ (unbounded)
    let x_introduced_67_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_67_ (unbounded)
    let x_introduced_68_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_68_ (unbounded)
    let x_introduced_69_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_69_ (unbounded)
    let x_introduced_70_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_70_ (unbounded)
    let x_introduced_71_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_71_ (unbounded)
    let x_introduced_72_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_72_ (unbounded)
    let x_introduced_73_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_73_ (unbounded)
    let x_introduced_74_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_74_ (unbounded)
    let x_introduced_75_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_75_ (unbounded)
    let x_introduced_76_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_76_ (unbounded)
    let x_introduced_77_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_77_ (unbounded)
    let x_introduced_78_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_78_ (unbounded)
    let x_introduced_79_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_79_ (unbounded)
    let x_introduced_80_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_80_ (unbounded)
    let x_introduced_81_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_81_ (unbounded)
    let x_introduced_82_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_82_ (unbounded)
    let x_introduced_83_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_83_ (unbounded)
    let x_introduced_84_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_84_ (unbounded)
    let x_introduced_85_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_85_ (unbounded)
    let x_introduced_86_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_86_ (unbounded)
    let x_introduced_87_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_87_ (unbounded)
    let x_introduced_88_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_88_ (unbounded)
    let x_introduced_89_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_89_ (unbounded)
    let x_introduced_90_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_90_ (unbounded)
    let x_introduced_91_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_91_ (unbounded)
    let x_introduced_92_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_92_ (unbounded)
    let x_introduced_93_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_93_ (unbounded)
    let x_introduced_94_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_94_ (unbounded)
    let x_introduced_95_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_95_ (unbounded)
    let x_introduced_96_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_96_ (unbounded)
    let x_introduced_97_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_97_ (unbounded)
    let x_introduced_98_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_98_ (unbounded)
    let x_introduced_99_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_99_ (unbounded)
    let x_introduced_100_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_100_ (unbounded)
    let x_introduced_101_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_101_ (unbounded)
    let x_introduced_102_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_102_ (unbounded)
    let x_introduced_103_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_103_ (unbounded)
    let x_introduced_104_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_104_ (unbounded)
    let x_introduced_105_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_105_ (unbounded)
    let x_introduced_106_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_106_ (unbounded)
    let x_introduced_107_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_107_ (unbounded)
    let x_introduced_108_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_108_ (unbounded)
    let x_introduced_109_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_109_ (unbounded)
    let x_introduced_110_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_110_ (unbounded)
    let x_introduced_111_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_111_ (unbounded)
    let x_introduced_112_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_112_ (unbounded)
    let x_introduced_113_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_113_ (unbounded)
    let x_introduced_114_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_114_ (unbounded)
    let x_introduced_115_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_115_ (unbounded)
    let x_introduced_116_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_116_ (unbounded)
    let x_introduced_117_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_117_ (unbounded)
    let x_introduced_118_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_118_ (unbounded)
    let x_introduced_119_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_119_ (unbounded)
    let x_introduced_120_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_120_ (unbounded)
    let x_introduced_121_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_121_ (unbounded)
    let x_introduced_122_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_122_ (unbounded)
    let x_introduced_123_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_123_ (unbounded)
    let x_introduced_124_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_124_ (unbounded)
    let x_introduced_125_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_125_ (unbounded)
    let x_introduced_126_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_126_ (unbounded)
    let x_introduced_127_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_127_ (unbounded)
    let x_introduced_128_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_128_ (unbounded)
    let x_introduced_129_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_129_ (unbounded)
    let x_introduced_130_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_130_ (unbounded)
    let x_introduced_131_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_131_ (unbounded)
    let x_introduced_132_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_132_ (unbounded)
    let x_introduced_133_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_133_ (unbounded)
    let x_introduced_134_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_134_ (unbounded)
    let x_introduced_135_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_135_ (unbounded)
    let x_introduced_136_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_136_ (unbounded)
    let x_introduced_137_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_137_ (unbounded)
    let x_introduced_138_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_138_ (unbounded)
    let x_introduced_139_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_139_ (unbounded)
    let x_introduced_140_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_140_ (unbounded)
    let x_introduced_141_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_141_ (unbounded)
    let x_introduced_142_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_142_ (unbounded)
    let x_introduced_143_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_143_ (unbounded)
    let x_introduced_144_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_144_ (unbounded)
    let x_introduced_145_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_145_ (unbounded)
    let x_introduced_146_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_146_ (unbounded)
    let x_introduced_147_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_147_ (unbounded)
    let x_introduced_148_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_148_ (unbounded)
    let x_introduced_149_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_149_ (unbounded)
    let x_introduced_150_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_150_ (unbounded)
    let x_introduced_151_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_151_ (unbounded)
    let x_introduced_152_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_152_ (unbounded)
    let x_introduced_153_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_153_ (unbounded)
    let x_introduced_154_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_154_ (unbounded)
    let x_introduced_155_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_155_ (unbounded)
    let x_introduced_156_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_156_ (unbounded)
    let x_introduced_157_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_157_ (unbounded)
    let x_introduced_158_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_158_ (unbounded)
    let x_introduced_159_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_159_ (unbounded)
    let x_introduced_160_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_160_ (unbounded)
    let x_introduced_161_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_161_ (unbounded)
    let x_introduced_162_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_162_ (unbounded)
    let x_introduced_163_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_163_ (unbounded)
    let x_introduced_164_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_164_ (unbounded)
    let x_introduced_165_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_165_ (unbounded)
    let x_introduced_166_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_166_ (unbounded)
    let x_introduced_167_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_167_ (unbounded)
    let x_introduced_168_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_168_ (unbounded)
    let x_introduced_169_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_169_ (unbounded)
    let x_introduced_170_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_170_ (unbounded)
    let x_introduced_171_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_171_ (unbounded)
    let x_introduced_172_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_172_ (unbounded)
    let x_introduced_173_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_173_ (unbounded)
    let x_introduced_174_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_174_ (unbounded)
    let x_introduced_175_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_175_ (unbounded)
    let x_introduced_176_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_176_ (unbounded)
    let x_introduced_177_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_177_ (unbounded)
    let x_introduced_178_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_178_ (unbounded)
    let x_introduced_179_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_179_ (unbounded)
    let x_introduced_180_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_180_ (unbounded)
    let x_introduced_181_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_181_ (unbounded)
    let x_introduced_182_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_182_ (unbounded)
    let x_introduced_183_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_183_ (unbounded)
    let x_introduced_184_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_184_ (unbounded)
    let x_introduced_185_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_185_ (unbounded)
    let x_introduced_186_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_186_ (unbounded)
    let x_introduced_187_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_187_ (unbounded)
    let x_introduced_188_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_188_ (unbounded)
    let x_introduced_189_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_189_ (unbounded)
    let x_introduced_190_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_190_ (unbounded)
    let x_introduced_191_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_191_ (unbounded)
    let x_introduced_192_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_192_ (unbounded)
    let x_introduced_193_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_193_ (unbounded)
    let x_introduced_194_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_194_ (unbounded)
    let x_introduced_195_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_195_ (unbounded)
    let x_introduced_196_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_196_ (unbounded)
    let x_introduced_197_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_197_ (unbounded)
    let x_introduced_198_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_198_ (unbounded)
    let x_introduced_199_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_199_ (unbounded)
    let x_introduced_200_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_200_ (unbounded)
    let x_introduced_201_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_201_ (unbounded)
    let x_introduced_202_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_202_ (unbounded)
    let x_introduced_203_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_203_ (unbounded)
    let x_introduced_204_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_204_ (unbounded)
    let x_introduced_205_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_205_ (unbounded)
    let x_introduced_206_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_206_ (unbounded)
    let x_introduced_207_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_207_ (unbounded)
    let x_introduced_208_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_208_ (unbounded)
    let x_introduced_209_ = model.float(f64::NEG_INFINITY, f64::INFINITY); // X_INTRODUCED_209_ (unbounded)
    let revenue = model.float(f64::NEG_INFINITY, f64::INFINITY); // revenue (unbounded)

    // ===== VARIABLE ARRAYS =====
    // Array of variables: lmilk (35 elements)
    let lmilk = vec![x_introduced_0_, x_introduced_1_, x_introduced_2_, x_introduced_3_, x_introduced_4_, x_introduced_5_, x_introduced_6_, x_introduced_7_, x_introduced_8_, x_introduced_9_, x_introduced_10_, x_introduced_11_, x_introduced_12_, x_introduced_13_, x_introduced_14_, x_introduced_15_, x_introduced_16_, x_introduced_17_, x_introduced_18_, x_introduced_19_, x_introduced_20_, x_introduced_21_, x_introduced_22_, x_introduced_23_, x_introduced_24_, x_introduced_25_, x_introduced_26_, x_introduced_27_, x_introduced_28_, x_introduced_29_, x_introduced_30_, x_introduced_31_, x_introduced_32_, x_introduced_33_, x_introduced_34_];
    // Array of variables: lbutt (35 elements)
    let lbutt = vec![x_introduced_35_, x_introduced_36_, x_introduced_37_, x_introduced_38_, x_introduced_39_, x_introduced_40_, x_introduced_41_, x_introduced_42_, x_introduced_43_, x_introduced_44_, x_introduced_45_, x_introduced_46_, x_introduced_47_, x_introduced_48_, x_introduced_49_, x_introduced_50_, x_introduced_51_, x_introduced_52_, x_introduced_53_, x_introduced_54_, x_introduced_55_, x_introduced_56_, x_introduced_57_, x_introduced_58_, x_introduced_59_, x_introduced_60_, x_introduced_61_, x_introduced_62_, x_introduced_63_, x_introduced_64_, x_introduced_65_, x_introduced_66_, x_introduced_67_, x_introduced_68_, x_introduced_69_];
    // Array of variables: lcha (35 elements)
    let lcha = vec![x_introduced_70_, x_introduced_71_, x_introduced_72_, x_introduced_73_, x_introduced_74_, x_introduced_75_, x_introduced_76_, x_introduced_77_, x_introduced_78_, x_introduced_79_, x_introduced_80_, x_introduced_81_, x_introduced_82_, x_introduced_83_, x_introduced_84_, x_introduced_85_, x_introduced_86_, x_introduced_87_, x_introduced_88_, x_introduced_89_, x_introduced_90_, x_introduced_91_, x_introduced_92_, x_introduced_93_, x_introduced_94_, x_introduced_95_, x_introduced_96_, x_introduced_97_, x_introduced_98_, x_introduced_99_, x_introduced_100_, x_introduced_101_, x_introduced_102_, x_introduced_103_, x_introduced_104_];
    // Array of variables: lchb (35 elements)
    let lchb = vec![x_introduced_105_, x_introduced_106_, x_introduced_107_, x_introduced_108_, x_introduced_109_, x_introduced_110_, x_introduced_111_, x_introduced_112_, x_introduced_113_, x_introduced_114_, x_introduced_115_, x_introduced_116_, x_introduced_117_, x_introduced_118_, x_introduced_119_, x_introduced_120_, x_introduced_121_, x_introduced_122_, x_introduced_123_, x_introduced_124_, x_introduced_125_, x_introduced_126_, x_introduced_127_, x_introduced_128_, x_introduced_129_, x_introduced_130_, x_introduced_131_, x_introduced_132_, x_introduced_133_, x_introduced_134_, x_introduced_135_, x_introduced_136_, x_introduced_137_, x_introduced_138_, x_introduced_139_];
    // Array of variables: X_INTRODUCED_300_ (36 elements)
    let x_introduced_300_ = vec![milk, x_introduced_0_, x_introduced_1_, x_introduced_2_, x_introduced_3_, x_introduced_4_, x_introduced_5_, x_introduced_6_, x_introduced_7_, x_introduced_8_, x_introduced_9_, x_introduced_10_, x_introduced_11_, x_introduced_12_, x_introduced_13_, x_introduced_14_, x_introduced_15_, x_introduced_16_, x_introduced_17_, x_introduced_18_, x_introduced_19_, x_introduced_20_, x_introduced_21_, x_introduced_22_, x_introduced_23_, x_introduced_24_, x_introduced_25_, x_introduced_26_, x_introduced_27_, x_introduced_28_, x_introduced_29_, x_introduced_30_, x_introduced_31_, x_introduced_32_, x_introduced_33_, x_introduced_34_];
    // Array of variables: X_INTRODUCED_339_ (36 elements)
    let x_introduced_339_ = vec![butt, x_introduced_35_, x_introduced_36_, x_introduced_37_, x_introduced_38_, x_introduced_39_, x_introduced_40_, x_introduced_41_, x_introduced_42_, x_introduced_43_, x_introduced_44_, x_introduced_45_, x_introduced_46_, x_introduced_47_, x_introduced_48_, x_introduced_49_, x_introduced_50_, x_introduced_51_, x_introduced_52_, x_introduced_53_, x_introduced_54_, x_introduced_55_, x_introduced_56_, x_introduced_57_, x_introduced_58_, x_introduced_59_, x_introduced_60_, x_introduced_61_, x_introduced_62_, x_introduced_63_, x_introduced_64_, x_introduced_65_, x_introduced_66_, x_introduced_67_, x_introduced_68_, x_introduced_69_];
    // Array of variables: X_INTRODUCED_377_ (36 elements)
    let x_introduced_377_ = vec![cha, x_introduced_70_, x_introduced_71_, x_introduced_72_, x_introduced_73_, x_introduced_74_, x_introduced_75_, x_introduced_76_, x_introduced_77_, x_introduced_78_, x_introduced_79_, x_introduced_80_, x_introduced_81_, x_introduced_82_, x_introduced_83_, x_introduced_84_, x_introduced_85_, x_introduced_86_, x_introduced_87_, x_introduced_88_, x_introduced_89_, x_introduced_90_, x_introduced_91_, x_introduced_92_, x_introduced_93_, x_introduced_94_, x_introduced_95_, x_introduced_96_, x_introduced_97_, x_introduced_98_, x_introduced_99_, x_introduced_100_, x_introduced_101_, x_introduced_102_, x_introduced_103_, x_introduced_104_];
    // Array of variables: X_INTRODUCED_415_ (36 elements)
    let x_introduced_415_ = vec![chb, x_introduced_105_, x_introduced_106_, x_introduced_107_, x_introduced_108_, x_introduced_109_, x_introduced_110_, x_introduced_111_, x_introduced_112_, x_introduced_113_, x_introduced_114_, x_introduced_115_, x_introduced_116_, x_introduced_117_, x_introduced_118_, x_introduced_119_, x_introduced_120_, x_introduced_121_, x_introduced_122_, x_introduced_123_, x_introduced_124_, x_introduced_125_, x_introduced_126_, x_introduced_127_, x_introduced_128_, x_introduced_129_, x_introduced_130_, x_introduced_131_, x_introduced_132_, x_introduced_133_, x_introduced_134_, x_introduced_135_, x_introduced_136_, x_introduced_137_, x_introduced_138_, x_introduced_139_];
    // Array of variables: X_INTRODUCED_490_ (71 elements)
    let x_introduced_490_ = vec![x_introduced_175_, x_introduced_176_, x_introduced_177_, x_introduced_178_, x_introduced_179_, x_introduced_180_, x_introduced_181_, x_introduced_182_, x_introduced_183_, x_introduced_184_, x_introduced_185_, x_introduced_186_, x_introduced_187_, x_introduced_188_, x_introduced_189_, x_introduced_190_, x_introduced_191_, x_introduced_192_, x_introduced_193_, x_introduced_194_, x_introduced_195_, x_introduced_196_, x_introduced_197_, x_introduced_198_, x_introduced_199_, x_introduced_200_, x_introduced_201_, x_introduced_202_, x_introduced_203_, x_introduced_204_, x_introduced_205_, x_introduced_206_, x_introduced_207_, x_introduced_208_, x_introduced_209_, q, x_introduced_140_, x_introduced_141_, x_introduced_142_, x_introduced_143_, x_introduced_144_, x_introduced_145_, x_introduced_146_, x_introduced_147_, x_introduced_148_, x_introduced_149_, x_introduced_150_, x_introduced_151_, x_introduced_152_, x_introduced_153_, x_introduced_154_, x_introduced_155_, x_introduced_156_, x_introduced_157_, x_introduced_158_, x_introduced_159_, x_introduced_160_, x_introduced_161_, x_introduced_162_, x_introduced_163_, x_introduced_164_, x_introduced_165_, x_introduced_166_, x_introduced_167_, x_introduced_168_, x_introduced_169_, x_introduced_170_, x_introduced_171_, x_introduced_172_, x_introduced_173_, x_introduced_174_];
    // Array of variables: X_INTRODUCED_562_ (36 elements)
    let x_introduced_562_ = vec![milksq, x_introduced_0_, x_introduced_1_, x_introduced_2_, x_introduced_3_, x_introduced_4_, x_introduced_5_, x_introduced_6_, x_introduced_7_, x_introduced_8_, x_introduced_9_, x_introduced_10_, x_introduced_11_, x_introduced_12_, x_introduced_13_, x_introduced_14_, x_introduced_15_, x_introduced_16_, x_introduced_17_, x_introduced_18_, x_introduced_19_, x_introduced_20_, x_introduced_21_, x_introduced_22_, x_introduced_23_, x_introduced_24_, x_introduced_25_, x_introduced_26_, x_introduced_27_, x_introduced_28_, x_introduced_29_, x_introduced_30_, x_introduced_31_, x_introduced_32_, x_introduced_33_, x_introduced_34_];
    // Array of variables: X_INTRODUCED_601_ (36 elements)
    let x_introduced_601_ = vec![buttsq, x_introduced_35_, x_introduced_36_, x_introduced_37_, x_introduced_38_, x_introduced_39_, x_introduced_40_, x_introduced_41_, x_introduced_42_, x_introduced_43_, x_introduced_44_, x_introduced_45_, x_introduced_46_, x_introduced_47_, x_introduced_48_, x_introduced_49_, x_introduced_50_, x_introduced_51_, x_introduced_52_, x_introduced_53_, x_introduced_54_, x_introduced_55_, x_introduced_56_, x_introduced_57_, x_introduced_58_, x_introduced_59_, x_introduced_60_, x_introduced_61_, x_introduced_62_, x_introduced_63_, x_introduced_64_, x_introduced_65_, x_introduced_66_, x_introduced_67_, x_introduced_68_, x_introduced_69_];
    // Array of variables: X_INTRODUCED_639_ (36 elements)
    let x_introduced_639_ = vec![chasq, x_introduced_70_, x_introduced_71_, x_introduced_72_, x_introduced_73_, x_introduced_74_, x_introduced_75_, x_introduced_76_, x_introduced_77_, x_introduced_78_, x_introduced_79_, x_introduced_80_, x_introduced_81_, x_introduced_82_, x_introduced_83_, x_introduced_84_, x_introduced_85_, x_introduced_86_, x_introduced_87_, x_introduced_88_, x_introduced_89_, x_introduced_90_, x_introduced_91_, x_introduced_92_, x_introduced_93_, x_introduced_94_, x_introduced_95_, x_introduced_96_, x_introduced_97_, x_introduced_98_, x_introduced_99_, x_introduced_100_, x_introduced_101_, x_introduced_102_, x_introduced_103_, x_introduced_104_];
    // Array of variables: X_INTRODUCED_677_ (36 elements)
    let x_introduced_677_ = vec![chbsq, x_introduced_105_, x_introduced_106_, x_introduced_107_, x_introduced_108_, x_introduced_109_, x_introduced_110_, x_introduced_111_, x_introduced_112_, x_introduced_113_, x_introduced_114_, x_introduced_115_, x_introduced_116_, x_introduced_117_, x_introduced_118_, x_introduced_119_, x_introduced_120_, x_introduced_121_, x_introduced_122_, x_introduced_123_, x_introduced_124_, x_introduced_125_, x_introduced_126_, x_introduced_127_, x_introduced_128_, x_introduced_129_, x_introduced_130_, x_introduced_131_, x_introduced_132_, x_introduced_133_, x_introduced_134_, x_introduced_135_, x_introduced_136_, x_introduced_137_, x_introduced_138_, x_introduced_139_];
    // Array of variables: X_INTRODUCED_752_ (71 elements)
    let x_introduced_752_ = vec![x_introduced_175_, x_introduced_176_, x_introduced_177_, x_introduced_178_, x_introduced_179_, x_introduced_180_, x_introduced_181_, x_introduced_182_, x_introduced_183_, x_introduced_184_, x_introduced_185_, x_introduced_186_, x_introduced_187_, x_introduced_188_, x_introduced_189_, x_introduced_190_, x_introduced_191_, x_introduced_192_, x_introduced_193_, x_introduced_194_, x_introduced_195_, x_introduced_196_, x_introduced_197_, x_introduced_198_, x_introduced_199_, x_introduced_200_, x_introduced_201_, x_introduced_202_, x_introduced_203_, x_introduced_204_, x_introduced_205_, x_introduced_206_, x_introduced_207_, x_introduced_208_, x_introduced_209_, qsq, x_introduced_140_, x_introduced_141_, x_introduced_142_, x_introduced_143_, x_introduced_144_, x_introduced_145_, x_introduced_146_, x_introduced_147_, x_introduced_148_, x_introduced_149_, x_introduced_150_, x_introduced_151_, x_introduced_152_, x_introduced_153_, x_introduced_154_, x_introduced_155_, x_introduced_156_, x_introduced_157_, x_introduced_158_, x_introduced_159_, x_introduced_160_, x_introduced_161_, x_introduced_162_, x_introduced_163_, x_introduced_164_, x_introduced_165_, x_introduced_166_, x_introduced_167_, x_introduced_168_, x_introduced_169_, x_introduced_170_, x_introduced_171_, x_introduced_172_, x_introduced_173_, x_introduced_174_];
    // Array of variables: X_INTRODUCED_797_ (70 elements)
    let x_introduced_797_ = vec![x_introduced_175_, x_introduced_140_, x_introduced_176_, x_introduced_141_, x_introduced_177_, x_introduced_142_, x_introduced_178_, x_introduced_143_, x_introduced_179_, x_introduced_144_, x_introduced_180_, x_introduced_145_, x_introduced_181_, x_introduced_146_, x_introduced_182_, x_introduced_147_, x_introduced_183_, x_introduced_148_, x_introduced_184_, x_introduced_149_, x_introduced_185_, x_introduced_150_, x_introduced_186_, x_introduced_151_, x_introduced_187_, x_introduced_152_, x_introduced_188_, x_introduced_153_, x_introduced_189_, x_introduced_154_, x_introduced_190_, x_introduced_155_, x_introduced_191_, x_introduced_156_, x_introduced_192_, x_introduced_157_, x_introduced_193_, x_introduced_158_, x_introduced_194_, x_introduced_159_, x_introduced_195_, x_introduced_160_, x_introduced_196_, x_introduced_161_, x_introduced_197_, x_introduced_162_, x_introduced_198_, x_introduced_163_, x_introduced_199_, x_introduced_164_, x_introduced_200_, x_introduced_165_, x_introduced_201_, x_introduced_166_, x_introduced_202_, x_introduced_167_, x_introduced_203_, x_introduced_168_, x_introduced_204_, x_introduced_169_, x_introduced_205_, x_introduced_170_, x_introduced_206_, x_introduced_171_, x_introduced_207_, x_introduced_172_, x_introduced_208_, x_introduced_173_, x_introduced_209_, x_introduced_174_];

    // ===== CONSTRAINTS ===== (247 total)
    model.new(milk.ge(0.0));
    model.new(milksq.ge(0.0));
    model.new(butt.ge(0.0));
    model.new(buttsq.ge(0.0));
    model.new(cha.ge(0.0));
    model.new(chasq.ge(0.0));
    model.new(chb.ge(0.0));
    model.new(chbsq.ge(0.0));
    model.new(xm.ge(0.0));
    model.new(xb.ge(0.0));
    model.new(xca.ge(0.0));
    model.new(xcb.ge(0.0));
    model.new(qsq.ge(0.0));
    model.new(x_introduced_0_.ge(0.0));
    model.new(x_introduced_1_.ge(0.0));
    model.new(x_introduced_2_.ge(0.0));
    model.new(x_introduced_3_.ge(0.0));
    model.new(x_introduced_4_.ge(0.0));
    model.new(x_introduced_5_.ge(0.0));
    model.new(x_introduced_6_.ge(0.0));
    model.new(x_introduced_7_.ge(0.0));
    model.new(x_introduced_8_.ge(0.0));
    model.new(x_introduced_9_.ge(0.0));
    model.new(x_introduced_10_.ge(0.0));
    model.new(x_introduced_11_.ge(0.0));
    model.new(x_introduced_12_.ge(0.0));
    model.new(x_introduced_13_.ge(0.0));
    model.new(x_introduced_14_.ge(0.0));
    model.new(x_introduced_15_.ge(0.0));
    model.new(x_introduced_16_.ge(0.0));
    model.new(x_introduced_17_.ge(0.0));
    model.new(x_introduced_18_.ge(0.0));
    model.new(x_introduced_19_.ge(0.0));
    model.new(x_introduced_20_.ge(0.0));
    model.new(x_introduced_21_.ge(0.0));
    model.new(x_introduced_22_.ge(0.0));
    model.new(x_introduced_23_.ge(0.0));
    model.new(x_introduced_24_.ge(0.0));
    model.new(x_introduced_25_.ge(0.0));
    model.new(x_introduced_26_.ge(0.0));
    model.new(x_introduced_27_.ge(0.0));
    model.new(x_introduced_28_.ge(0.0));
    model.new(x_introduced_29_.ge(0.0));
    model.new(x_introduced_30_.ge(0.0));
    model.new(x_introduced_31_.ge(0.0));
    model.new(x_introduced_32_.ge(0.0));
    model.new(x_introduced_33_.ge(0.0));
    model.new(x_introduced_34_.ge(0.0));
    model.new(x_introduced_35_.ge(0.0));
    model.new(x_introduced_36_.ge(0.0));
    model.new(x_introduced_37_.ge(0.0));
    model.new(x_introduced_38_.ge(0.0));
    model.new(x_introduced_39_.ge(0.0));
    model.new(x_introduced_40_.ge(0.0));
    model.new(x_introduced_41_.ge(0.0));
    model.new(x_introduced_42_.ge(0.0));
    model.new(x_introduced_43_.ge(0.0));
    model.new(x_introduced_44_.ge(0.0));
    model.new(x_introduced_45_.ge(0.0));
    model.new(x_introduced_46_.ge(0.0));
    model.new(x_introduced_47_.ge(0.0));
    model.new(x_introduced_48_.ge(0.0));
    model.new(x_introduced_49_.ge(0.0));
    model.new(x_introduced_50_.ge(0.0));
    model.new(x_introduced_51_.ge(0.0));
    model.new(x_introduced_52_.ge(0.0));
    model.new(x_introduced_53_.ge(0.0));
    model.new(x_introduced_54_.ge(0.0));
    model.new(x_introduced_55_.ge(0.0));
    model.new(x_introduced_56_.ge(0.0));
    model.new(x_introduced_57_.ge(0.0));
    model.new(x_introduced_58_.ge(0.0));
    model.new(x_introduced_59_.ge(0.0));
    model.new(x_introduced_60_.ge(0.0));
    model.new(x_introduced_61_.ge(0.0));
    model.new(x_introduced_62_.ge(0.0));
    model.new(x_introduced_63_.ge(0.0));
    model.new(x_introduced_64_.ge(0.0));
    model.new(x_introduced_65_.ge(0.0));
    model.new(x_introduced_66_.ge(0.0));
    model.new(x_introduced_67_.ge(0.0));
    model.new(x_introduced_68_.ge(0.0));
    model.new(x_introduced_69_.ge(0.0));
    model.new(x_introduced_70_.ge(0.0));
    model.new(x_introduced_71_.ge(0.0));
    model.new(x_introduced_72_.ge(0.0));
    model.new(x_introduced_73_.ge(0.0));
    model.new(x_introduced_74_.ge(0.0));
    model.new(x_introduced_75_.ge(0.0));
    model.new(x_introduced_76_.ge(0.0));
    model.new(x_introduced_77_.ge(0.0));
    model.new(x_introduced_78_.ge(0.0));
    model.new(x_introduced_79_.ge(0.0));
    model.new(x_introduced_80_.ge(0.0));
    model.new(x_introduced_81_.ge(0.0));
    model.new(x_introduced_82_.ge(0.0));
    model.new(x_introduced_83_.ge(0.0));
    model.new(x_introduced_84_.ge(0.0));
    model.new(x_introduced_85_.ge(0.0));
    model.new(x_introduced_86_.ge(0.0));
    model.new(x_introduced_87_.ge(0.0));
    model.new(x_introduced_88_.ge(0.0));
    model.new(x_introduced_89_.ge(0.0));
    model.new(x_introduced_90_.ge(0.0));
    model.new(x_introduced_91_.ge(0.0));
    model.new(x_introduced_92_.ge(0.0));
    model.new(x_introduced_93_.ge(0.0));
    model.new(x_introduced_94_.ge(0.0));
    model.new(x_introduced_95_.ge(0.0));
    model.new(x_introduced_96_.ge(0.0));
    model.new(x_introduced_97_.ge(0.0));
    model.new(x_introduced_98_.ge(0.0));
    model.new(x_introduced_99_.ge(0.0));
    model.new(x_introduced_100_.ge(0.0));
    model.new(x_introduced_101_.ge(0.0));
    model.new(x_introduced_102_.ge(0.0));
    model.new(x_introduced_103_.ge(0.0));
    model.new(x_introduced_104_.ge(0.0));
    model.new(x_introduced_105_.ge(0.0));
    model.new(x_introduced_106_.ge(0.0));
    model.new(x_introduced_107_.ge(0.0));
    model.new(x_introduced_108_.ge(0.0));
    model.new(x_introduced_109_.ge(0.0));
    model.new(x_introduced_110_.ge(0.0));
    model.new(x_introduced_111_.ge(0.0));
    model.new(x_introduced_112_.ge(0.0));
    model.new(x_introduced_113_.ge(0.0));
    model.new(x_introduced_114_.ge(0.0));
    model.new(x_introduced_115_.ge(0.0));
    model.new(x_introduced_116_.ge(0.0));
    model.new(x_introduced_117_.ge(0.0));
    model.new(x_introduced_118_.ge(0.0));
    model.new(x_introduced_119_.ge(0.0));
    model.new(x_introduced_120_.ge(0.0));
    model.new(x_introduced_121_.ge(0.0));
    model.new(x_introduced_122_.ge(0.0));
    model.new(x_introduced_123_.ge(0.0));
    model.new(x_introduced_124_.ge(0.0));
    model.new(x_introduced_125_.ge(0.0));
    model.new(x_introduced_126_.ge(0.0));
    model.new(x_introduced_127_.ge(0.0));
    model.new(x_introduced_128_.ge(0.0));
    model.new(x_introduced_129_.ge(0.0));
    model.new(x_introduced_130_.ge(0.0));
    model.new(x_introduced_131_.ge(0.0));
    model.new(x_introduced_132_.ge(0.0));
    model.new(x_introduced_133_.ge(0.0));
    model.new(x_introduced_134_.ge(0.0));
    model.new(x_introduced_135_.ge(0.0));
    model.new(x_introduced_136_.ge(0.0));
    model.new(x_introduced_137_.ge(0.0));
    model.new(x_introduced_138_.ge(0.0));
    model.new(x_introduced_139_.ge(0.0));
    model.new(x_introduced_140_.ge(0.0));
    model.new(x_introduced_141_.ge(0.0));
    model.new(x_introduced_142_.ge(0.0));
    model.new(x_introduced_143_.ge(0.0));
    model.new(x_introduced_144_.ge(0.0));
    model.new(x_introduced_145_.ge(0.0));
    model.new(x_introduced_146_.ge(0.0));
    model.new(x_introduced_147_.ge(0.0));
    model.new(x_introduced_148_.ge(0.0));
    model.new(x_introduced_149_.ge(0.0));
    model.new(x_introduced_150_.ge(0.0));
    model.new(x_introduced_151_.ge(0.0));
    model.new(x_introduced_152_.ge(0.0));
    model.new(x_introduced_153_.ge(0.0));
    model.new(x_introduced_154_.ge(0.0));
    model.new(x_introduced_155_.ge(0.0));
    model.new(x_introduced_156_.ge(0.0));
    model.new(x_introduced_157_.ge(0.0));
    model.new(x_introduced_158_.ge(0.0));
    model.new(x_introduced_159_.ge(0.0));
    model.new(x_introduced_160_.ge(0.0));
    model.new(x_introduced_161_.ge(0.0));
    model.new(x_introduced_162_.ge(0.0));
    model.new(x_introduced_163_.ge(0.0));
    model.new(x_introduced_164_.ge(0.0));
    model.new(x_introduced_165_.ge(0.0));
    model.new(x_introduced_166_.ge(0.0));
    model.new(x_introduced_167_.ge(0.0));
    model.new(x_introduced_168_.ge(0.0));
    model.new(x_introduced_169_.ge(0.0));
    model.new(x_introduced_170_.ge(0.0));
    model.new(x_introduced_171_.ge(0.0));
    model.new(x_introduced_172_.ge(0.0));
    model.new(x_introduced_173_.ge(0.0));
    model.new(x_introduced_174_.ge(0.0));
    model.new(x_introduced_175_.ge(0.0));
    model.new(x_introduced_176_.ge(0.0));
    model.new(x_introduced_177_.ge(0.0));
    model.new(x_introduced_178_.ge(0.0));
    model.new(x_introduced_179_.ge(0.0));
    model.new(x_introduced_180_.ge(0.0));
    model.new(x_introduced_181_.ge(0.0));
    model.new(x_introduced_182_.ge(0.0));
    model.new(x_introduced_183_.ge(0.0));
    model.new(x_introduced_184_.ge(0.0));
    model.new(x_introduced_185_.ge(0.0));
    model.new(x_introduced_186_.ge(0.0));
    model.new(x_introduced_187_.ge(0.0));
    model.new(x_introduced_188_.ge(0.0));
    model.new(x_introduced_189_.ge(0.0));
    model.new(x_introduced_190_.ge(0.0));
    model.new(x_introduced_191_.ge(0.0));
    model.new(x_introduced_192_.ge(0.0));
    model.new(x_introduced_193_.ge(0.0));
    model.new(x_introduced_194_.ge(0.0));
    model.new(x_introduced_195_.ge(0.0));
    model.new(x_introduced_196_.ge(0.0));
    model.new(x_introduced_197_.ge(0.0));
    model.new(x_introduced_198_.ge(0.0));
    model.new(x_introduced_199_.ge(0.0));
    model.new(x_introduced_200_.ge(0.0));
    model.new(x_introduced_201_.ge(0.0));
    model.new(x_introduced_202_.ge(0.0));
    model.new(x_introduced_203_.ge(0.0));
    model.new(x_introduced_204_.ge(0.0));
    model.new(x_introduced_205_.ge(0.0));
    model.new(x_introduced_206_.ge(0.0));
    model.new(x_introduced_207_.ge(0.0));
    model.new(x_introduced_208_.ge(0.0));
    model.new(x_introduced_209_.ge(0.0));
    model.lin_eq(&x_introduced_212_, &vec![xm, milk], 1.4);
    model.lin_eq(&x_introduced_214_, &vec![xb, butt], 3.7);
    model.lin_eq(&x_introduced_216_, &vec![cha, xca, chb], 2.0);
    model.lin_eq(&x_introduced_218_, &vec![chb, xcb, cha], 1.0);
    model.lin_le(&x_introduced_220_, &vec![xca, xb, xm, xcb], 0.6);
    model.lin_le(&x_introduced_222_, &vec![xca, xb, xm, xcb], 0.75);
    model.lin_le(&x_introduced_224_, &vec![cha, butt, milk, chb], 1.939);
    model.lin_eq(&x_introduced_226_, &vec![chb, cha, q], 0.0);
    model.lin_eq(&x_introduced_301_, &x_introduced_300_, 0.0);
    model.lin_eq(&x_introduced_301_, &x_introduced_339_, 0.0);
    model.lin_eq(&x_introduced_301_, &x_introduced_377_, 0.0);
    model.lin_eq(&x_introduced_301_, &x_introduced_415_, 0.0);
    model.lin_eq(&x_introduced_491_, &x_introduced_490_, 0.0);
    model.lin_eq(&x_introduced_563_, &x_introduced_562_, 0.0);
    model.lin_eq(&x_introduced_563_, &x_introduced_601_, 0.0);
    model.lin_eq(&x_introduced_563_, &x_introduced_639_, 0.0);
    model.lin_eq(&x_introduced_563_, &x_introduced_677_, 0.0);
    model.lin_eq(&x_introduced_753_, &x_introduced_752_, 0.0);
    model.lin_le(&x_introduced_755_, &lmilk, 1.0);
    model.lin_le(&x_introduced_755_, &lbutt, 1.0);
    model.lin_le(&x_introduced_755_, &lcha, 1.0);
    model.lin_le(&x_introduced_755_, &lchb, 1.0);
    model.lin_le(&x_introduced_798_, &x_introduced_797_, 1.0);
    model.lin_eq(&vec![420.0, 1185.0, 6748.0, -1.0, -8.0, -194.0, -1200.0, -6492.0, 70.0, -1.0], &vec![cha, butt, milk, qsq, chbsq, chasq, buttsq, milksq, chb, revenue], 0.0);

    // solve maximize revenue; - Using Selen's maximize() method
    // ===== SOLVE =====
    println!("Solving...");
    match model.maximize(revenue) {
        Ok(solution) => {
            println!("\nSolution found!");
            println!("===================\n");
            // OBJECTIVE VALUE
            match solution[revenue] {
                Val::ValI(i) => println!("  OBJECTIVE = {}", i),
                Val::ValF(f) => println!("  OBJECTIVE = {}", f),
            }
            println!();
            // OUTPUT VARIABLES (marked with ::output_var annotation)
            match solution[milk] {
                Val::ValI(i) => println!("  milk = {}", i),
                Val::ValF(f) => println!("  milk = {}", f),
            }
            match solution[milksq] {
                Val::ValI(i) => println!("  milksq = {}", i),
                Val::ValF(f) => println!("  milksq = {}", f),
            }
            match solution[butt] {
                Val::ValI(i) => println!("  butt = {}", i),
                Val::ValF(f) => println!("  butt = {}", f),
            }
            match solution[buttsq] {
                Val::ValI(i) => println!("  buttsq = {}", i),
                Val::ValF(f) => println!("  buttsq = {}", f),
            }
            match solution[cha] {
                Val::ValI(i) => println!("  cha = {}", i),
                Val::ValF(f) => println!("  cha = {}", f),
            }
            match solution[chasq] {
                Val::ValI(i) => println!("  chasq = {}", i),
                Val::ValF(f) => println!("  chasq = {}", f),
            }
            match solution[chb] {
                Val::ValI(i) => println!("  chb = {}", i),
                Val::ValF(f) => println!("  chb = {}", f),
            }
            match solution[chbsq] {
                Val::ValI(i) => println!("  chbsq = {}", i),
                Val::ValF(f) => println!("  chbsq = {}", f),
            }
            match solution[xm] {
                Val::ValI(i) => println!("  xm = {}", i),
                Val::ValF(f) => println!("  xm = {}", f),
            }
            match solution[xb] {
                Val::ValI(i) => println!("  xb = {}", i),
                Val::ValF(f) => println!("  xb = {}", f),
            }
            match solution[xca] {
                Val::ValI(i) => println!("  xca = {}", i),
                Val::ValF(f) => println!("  xca = {}", f),
            }
            match solution[xcb] {
                Val::ValI(i) => println!("  xcb = {}", i),
                Val::ValF(f) => println!("  xcb = {}", f),
            }
            match solution[qsq] {
                Val::ValI(i) => println!("  qsq = {}", i),
                Val::ValF(f) => println!("  qsq = {}", f),
            }
        }
        Err(e) => {
            println!("No solution found: {:?}", e);
        }
    }
}
