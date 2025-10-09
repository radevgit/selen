// Auto-generated Selen test program from FlatZinc
// This program can be compiled and run independently to debug Selen behavior

use selen::prelude::*;
use selen::variables::Val;

fn main() {
    let mut model = Model::default();

    // ===== VARIABLES =====
    // Array parameter: X_INTRODUCED_212_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_214_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_216_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_218_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_220_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_222_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_224_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_226_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_301_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_491_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_563_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_753_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_755_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_798_ (initialization skipped in export)
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
    // Array parameter: lmilk (initialization skipped in export)
    // Array parameter: lbutt (initialization skipped in export)
    // Array parameter: lcha (initialization skipped in export)
    // Array parameter: lchb (initialization skipped in export)
    // Array parameter: X_INTRODUCED_300_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_339_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_377_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_415_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_490_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_562_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_601_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_639_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_677_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_752_ (initialization skipped in export)
    // Array parameter: X_INTRODUCED_797_ (initialization skipped in export)

    // ===== CONSTRAINTS =====
    model.le(-0, milk);
    model.le(-0, milksq);
    model.le(-0, butt);
    model.le(-0, buttsq);
    model.le(-0, cha);
    model.le(-0, chasq);
    model.le(-0, chb);
    model.le(-0, chbsq);
    model.le(-0, xm);
    model.le(-0, xb);
    model.le(-0, xca);
    model.le(-0, xcb);
    model.le(-0, qsq);
    model.le(-0, x_introduced_0_);
    model.le(-0, x_introduced_1_);
    model.le(-0, x_introduced_2_);
    model.le(-0, x_introduced_3_);
    model.le(-0, x_introduced_4_);
    model.le(-0, x_introduced_5_);
    model.le(-0, x_introduced_6_);
    model.le(-0, x_introduced_7_);
    model.le(-0, x_introduced_8_);
    model.le(-0, x_introduced_9_);
    model.le(-0, x_introduced_10_);
    model.le(-0, x_introduced_11_);
    model.le(-0, x_introduced_12_);
    model.le(-0, x_introduced_13_);
    model.le(-0, x_introduced_14_);
    model.le(-0, x_introduced_15_);
    model.le(-0, x_introduced_16_);
    model.le(-0, x_introduced_17_);
    model.le(-0, x_introduced_18_);
    model.le(-0, x_introduced_19_);
    model.le(-0, x_introduced_20_);
    model.le(-0, x_introduced_21_);
    model.le(-0, x_introduced_22_);
    model.le(-0, x_introduced_23_);
    model.le(-0, x_introduced_24_);
    model.le(-0, x_introduced_25_);
    model.le(-0, x_introduced_26_);
    model.le(-0, x_introduced_27_);
    model.le(-0, x_introduced_28_);
    model.le(-0, x_introduced_29_);
    model.le(-0, x_introduced_30_);
    model.le(-0, x_introduced_31_);
    model.le(-0, x_introduced_32_);
    model.le(-0, x_introduced_33_);
    model.le(-0, x_introduced_34_);
    model.le(-0, x_introduced_35_);
    model.le(-0, x_introduced_36_);
    model.le(-0, x_introduced_37_);
    model.le(-0, x_introduced_38_);
    model.le(-0, x_introduced_39_);
    model.le(-0, x_introduced_40_);
    model.le(-0, x_introduced_41_);
    model.le(-0, x_introduced_42_);
    model.le(-0, x_introduced_43_);
    model.le(-0, x_introduced_44_);
    model.le(-0, x_introduced_45_);
    model.le(-0, x_introduced_46_);
    model.le(-0, x_introduced_47_);
    model.le(-0, x_introduced_48_);
    model.le(-0, x_introduced_49_);
    model.le(-0, x_introduced_50_);
    model.le(-0, x_introduced_51_);
    model.le(-0, x_introduced_52_);
    model.le(-0, x_introduced_53_);
    model.le(-0, x_introduced_54_);
    model.le(-0, x_introduced_55_);
    model.le(-0, x_introduced_56_);
    model.le(-0, x_introduced_57_);
    model.le(-0, x_introduced_58_);
    model.le(-0, x_introduced_59_);
    model.le(-0, x_introduced_60_);
    model.le(-0, x_introduced_61_);
    model.le(-0, x_introduced_62_);
    model.le(-0, x_introduced_63_);
    model.le(-0, x_introduced_64_);
    model.le(-0, x_introduced_65_);
    model.le(-0, x_introduced_66_);
    model.le(-0, x_introduced_67_);
    model.le(-0, x_introduced_68_);
    model.le(-0, x_introduced_69_);
    model.le(-0, x_introduced_70_);
    model.le(-0, x_introduced_71_);
    model.le(-0, x_introduced_72_);
    model.le(-0, x_introduced_73_);
    model.le(-0, x_introduced_74_);
    model.le(-0, x_introduced_75_);
    model.le(-0, x_introduced_76_);
    model.le(-0, x_introduced_77_);
    model.le(-0, x_introduced_78_);
    model.le(-0, x_introduced_79_);
    model.le(-0, x_introduced_80_);
    model.le(-0, x_introduced_81_);
    model.le(-0, x_introduced_82_);
    model.le(-0, x_introduced_83_);
    model.le(-0, x_introduced_84_);
    model.le(-0, x_introduced_85_);
    model.le(-0, x_introduced_86_);
    model.le(-0, x_introduced_87_);
    model.le(-0, x_introduced_88_);
    model.le(-0, x_introduced_89_);
    model.le(-0, x_introduced_90_);
    model.le(-0, x_introduced_91_);
    model.le(-0, x_introduced_92_);
    model.le(-0, x_introduced_93_);
    model.le(-0, x_introduced_94_);
    model.le(-0, x_introduced_95_);
    model.le(-0, x_introduced_96_);
    model.le(-0, x_introduced_97_);
    model.le(-0, x_introduced_98_);
    model.le(-0, x_introduced_99_);
    model.le(-0, x_introduced_100_);
    model.le(-0, x_introduced_101_);
    model.le(-0, x_introduced_102_);
    model.le(-0, x_introduced_103_);
    model.le(-0, x_introduced_104_);
    model.le(-0, x_introduced_105_);
    model.le(-0, x_introduced_106_);
    model.le(-0, x_introduced_107_);
    model.le(-0, x_introduced_108_);
    model.le(-0, x_introduced_109_);
    model.le(-0, x_introduced_110_);
    model.le(-0, x_introduced_111_);
    model.le(-0, x_introduced_112_);
    model.le(-0, x_introduced_113_);
    model.le(-0, x_introduced_114_);
    model.le(-0, x_introduced_115_);
    model.le(-0, x_introduced_116_);
    model.le(-0, x_introduced_117_);
    model.le(-0, x_introduced_118_);
    model.le(-0, x_introduced_119_);
    model.le(-0, x_introduced_120_);
    model.le(-0, x_introduced_121_);
    model.le(-0, x_introduced_122_);
    model.le(-0, x_introduced_123_);
    model.le(-0, x_introduced_124_);
    model.le(-0, x_introduced_125_);
    model.le(-0, x_introduced_126_);
    model.le(-0, x_introduced_127_);
    model.le(-0, x_introduced_128_);
    model.le(-0, x_introduced_129_);
    model.le(-0, x_introduced_130_);
    model.le(-0, x_introduced_131_);
    model.le(-0, x_introduced_132_);
    model.le(-0, x_introduced_133_);
    model.le(-0, x_introduced_134_);
    model.le(-0, x_introduced_135_);
    model.le(-0, x_introduced_136_);
    model.le(-0, x_introduced_137_);
    model.le(-0, x_introduced_138_);
    model.le(-0, x_introduced_139_);
    model.le(-0, x_introduced_140_);
    model.le(-0, x_introduced_141_);
    model.le(-0, x_introduced_142_);
    model.le(-0, x_introduced_143_);
    model.le(-0, x_introduced_144_);
    model.le(-0, x_introduced_145_);
    model.le(-0, x_introduced_146_);
    model.le(-0, x_introduced_147_);
    model.le(-0, x_introduced_148_);
    model.le(-0, x_introduced_149_);
    model.le(-0, x_introduced_150_);
    model.le(-0, x_introduced_151_);
    model.le(-0, x_introduced_152_);
    model.le(-0, x_introduced_153_);
    model.le(-0, x_introduced_154_);
    model.le(-0, x_introduced_155_);
    model.le(-0, x_introduced_156_);
    model.le(-0, x_introduced_157_);
    model.le(-0, x_introduced_158_);
    model.le(-0, x_introduced_159_);
    model.le(-0, x_introduced_160_);
    model.le(-0, x_introduced_161_);
    model.le(-0, x_introduced_162_);
    model.le(-0, x_introduced_163_);
    model.le(-0, x_introduced_164_);
    model.le(-0, x_introduced_165_);
    model.le(-0, x_introduced_166_);
    model.le(-0, x_introduced_167_);
    model.le(-0, x_introduced_168_);
    model.le(-0, x_introduced_169_);
    model.le(-0, x_introduced_170_);
    model.le(-0, x_introduced_171_);
    model.le(-0, x_introduced_172_);
    model.le(-0, x_introduced_173_);
    model.le(-0, x_introduced_174_);
    model.le(-0, x_introduced_175_);
    model.le(-0, x_introduced_176_);
    model.le(-0, x_introduced_177_);
    model.le(-0, x_introduced_178_);
    model.le(-0, x_introduced_179_);
    model.le(-0, x_introduced_180_);
    model.le(-0, x_introduced_181_);
    model.le(-0, x_introduced_182_);
    model.le(-0, x_introduced_183_);
    model.le(-0, x_introduced_184_);
    model.le(-0, x_introduced_185_);
    model.le(-0, x_introduced_186_);
    model.le(-0, x_introduced_187_);
    model.le(-0, x_introduced_188_);
    model.le(-0, x_introduced_189_);
    model.le(-0, x_introduced_190_);
    model.le(-0, x_introduced_191_);
    model.le(-0, x_introduced_192_);
    model.le(-0, x_introduced_193_);
    model.le(-0, x_introduced_194_);
    model.le(-0, x_introduced_195_);
    model.le(-0, x_introduced_196_);
    model.le(-0, x_introduced_197_);
    model.le(-0, x_introduced_198_);
    model.le(-0, x_introduced_199_);
    model.le(-0, x_introduced_200_);
    model.le(-0, x_introduced_201_);
    model.le(-0, x_introduced_202_);
    model.le(-0, x_introduced_203_);
    model.le(-0, x_introduced_204_);
    model.le(-0, x_introduced_205_);
    model.le(-0, x_introduced_206_);
    model.le(-0, x_introduced_207_);
    model.le(-0, x_introduced_208_);
    model.le(-0, x_introduced_209_);
    model.float_lin_eq(&x_introduced_212_, &vec![xm, milk], 1.4);
    model.float_lin_eq(&x_introduced_214_, &vec![xb, butt], 3.7);
    model.float_lin_eq(&x_introduced_216_, &vec![cha, xca, chb], 2);
    model.float_lin_eq(&x_introduced_218_, &vec![chb, xcb, cha], 1);
    model.float_lin_le(&x_introduced_220_, &vec![xca, xb, xm, xcb], 0.6);
    model.float_lin_le(&x_introduced_222_, &vec![xca, xb, xm, xcb], 0.75);
    model.float_lin_le(&x_introduced_224_, &vec![cha, butt, milk, chb], 1.939);
    model.float_lin_eq(&x_introduced_226_, &vec![chb, cha, q], -0);
    model.float_lin_eq(&x_introduced_301_, &x_introduced_300_, -0);
    model.float_lin_eq(&x_introduced_301_, &x_introduced_339_, -0);
    model.float_lin_eq(&x_introduced_301_, &x_introduced_377_, -0);
    model.float_lin_eq(&x_introduced_301_, &x_introduced_415_, -0);
    model.float_lin_eq(&x_introduced_491_, &x_introduced_490_, -0);
    model.float_lin_eq(&x_introduced_563_, &x_introduced_562_, -0);
    model.float_lin_eq(&x_introduced_563_, &x_introduced_601_, -0);
    model.float_lin_eq(&x_introduced_563_, &x_introduced_639_, -0);
    model.float_lin_eq(&x_introduced_563_, &x_introduced_677_, -0);
    model.float_lin_eq(&x_introduced_753_, &x_introduced_752_, -0);
    model.float_lin_le(&x_introduced_755_, &lmilk, 1);
    model.float_lin_le(&x_introduced_755_, &lbutt, 1);
    model.float_lin_le(&x_introduced_755_, &lcha, 1);
    model.float_lin_le(&x_introduced_755_, &lchb, 1);
    model.float_lin_le(&x_introduced_798_, &x_introduced_797_, 1);
    model.float_lin_eq(&vec![420, 1185, 6748, -1, -8, -194, -1200, -6492, 70, -1], &vec![cha, butt, milk, qsq, chbsq, chasq, buttsq, milksq, chb, revenue], -0);

    // ===== SOLVE GOAL =====
    // solve maximize Ident("revenue");
    // TODO: Implement maximization

    // ===== SOLVE =====
    match model.solve() {
        Ok(solution) => {
            println!("Solution found:");
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
            match solution[q] {
                Val::ValI(i) => println!("  q = {}", i),
                Val::ValF(f) => println!("  q = {}", f),
            }
            match solution[qsq] {
                Val::ValI(i) => println!("  qsq = {}", i),
                Val::ValF(f) => println!("  qsq = {}", f),
            }
            match solution[revenue] {
                Val::ValI(i) => println!("  revenue = {}", i),
                Val::ValF(f) => println!("  revenue = {}", f),
            }
            match solution[lmilk] {
                Val::ValI(i) => println!("  lmilk = {}", i),
                Val::ValF(f) => println!("  lmilk = {}", f),
            }
            match solution[lbutt] {
                Val::ValI(i) => println!("  lbutt = {}", i),
                Val::ValF(f) => println!("  lbutt = {}", f),
            }
            match solution[lcha] {
                Val::ValI(i) => println!("  lcha = {}", i),
                Val::ValF(f) => println!("  lcha = {}", f),
            }
            match solution[lchb] {
                Val::ValI(i) => println!("  lchb = {}", i),
                Val::ValF(f) => println!("  lchb = {}", f),
            }
        }
        Err(e) => {
            println!("No solution: {:?}", e);
        }
    }
}
