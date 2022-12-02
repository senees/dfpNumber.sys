/*
 * MIT License
 *
 * Copyright (c) 2022 senees
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::path::PathBuf;

#[cfg(not(feature = "call-by-reference"))]
const DECIMAL_CALL_BY_REFERENCE: &str = "0";
#[cfg(feature = "call-by-reference")]
const DECIMAL_CALL_BY_REFERENCE: &str = "1";

#[cfg(not(feature = "global-rounding"))]
const DECIMAL_GLOBAL_ROUNDING: &str = "0";
#[cfg(feature = "global-rounding")]
const DECIMAL_GLOBAL_ROUNDING: &str = "1";

#[cfg(not(feature = "global-exception-flags"))]
const DECIMAL_GLOBAL_EXCEPTION_FLAGS: &str = "0";
#[cfg(feature = "global-exception-flags")]
const DECIMAL_GLOBAL_EXCEPTION_FLAGS: &str = "1";

#[cfg(target_os = "linux")]
const OPERATING_SYSTEM: &str = "linux";
#[cfg(target_os = "windows")]
const OPERATING_SYSTEM: &str = "win64";
#[cfg(target_os = "macos")]
const OPERATING_SYSTEM: &str = "darwin";

fn main() {
  let output_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
  cc::Build::new()
    .define("DECIMAL_CALL_BY_REFERENCE", DECIMAL_CALL_BY_REFERENCE)
    .define("DECIMAL_GLOBAL_ROUNDING", DECIMAL_GLOBAL_ROUNDING)
    .define("DECIMAL_GLOBAL_EXCEPTION_FLAGS", DECIMAL_GLOBAL_EXCEPTION_FLAGS)
    .define("UNCHANGED_BINARY_STATUS_FLAGS", "0")
    .define("USE_COMPILER_F128_TYPE", "0")
    .define("USE_COMPILER_F80_TYPE", "0")
    .define("USE_NATIVE_QUAD_TYPE", "0")
    .define("ia64", None)
    .define(OPERATING_SYSTEM, None)
    .flag_if_supported("-Wno-attributes")
    .flag_if_supported("-Wno-unused-value")
    .flag_if_supported("-Wno-unused-variable")
    .flag_if_supported("-Wno-unused-but-set-variable")
    .flag_if_supported("-Wno-missing-braces")
    .flag_if_supported("-Wno-unused-const-variable")
    .flag_if_supported("-Wno-unused-parameter")
    .flag_if_supported("-Wno-dangling-else")
    .flag_if_supported("-Wno-sign-compare")
    .flag_if_supported("-Wno-implicit-function-declaration")
    .flag_if_supported("-Wno-unknown-pragmas")
    .flag_if_supported("-Wno-shift-negative-value")
    .flag_if_supported("-Wno-comment")
    .flag_if_supported("-Wno-parentheses")
    .flag_if_supported("-Wno-maybe-uninitialized")
    .flag_if_supported("-Wno-array-bounds")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_exception.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_four_over_pi.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_bessel.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_bid.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_cbrt.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_erf.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_exp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_int.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_inv_hyper.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_inv_trig.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_lgamma.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_log.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_mod.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_ops.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_ops_64.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_pow.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_powi.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_sqrt.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/dpml_ux_trig.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/float128/sqrt_tab_t.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_2_str_tables.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_acos.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_acosh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_add.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_asin.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_asinh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_atan2.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_atan.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_atanh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_cbrt.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_compare.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_cos.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_cosh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_div.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_erf.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_erfc.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_exp10.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_exp2.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_exp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_expm1.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_fdimd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_fma.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_fmod.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_frexp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_hypot.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_ldexp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_lgamma.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_llquantexpd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_llrintd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_llround.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_log10.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_log1p.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_log2.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_logb.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_logbd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_log.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_lrintd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_lround.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_minmax.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_modf.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_mul.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_nearbyintd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_next.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_nexttowardd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_noncomp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_pow.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_quantexpd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_quantize.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_quantumd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_rem.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_round_integral.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_scalb.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_scalbl.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_sin.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_sinh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_sqrt.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_string.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_tan.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_tanh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_tgamma.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_to_int16.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_to_int32.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_to_int64.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_to_int8.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_to_uint16.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_to_uint32.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_to_uint64.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid128_to_uint8.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_acos.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_acosh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_add.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_asin.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_asinh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_atan2.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_atan.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_atanh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_cbrt.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_compare.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_cos.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_cosh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_div.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_erf.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_erfc.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_exp10.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_exp2.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_exp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_expm1.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_fdimd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_fma.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_fmod.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_frexp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_hypot.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_ldexp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_lgamma.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_llquantexpd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_llrintd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_llround.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_log10.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_log1p.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_log2.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_logb.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_logbd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_log.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_lrintd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_lround.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_minmax.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_modf.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_mul.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_nearbyintd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_next.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_nexttowardd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_noncomp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_pow.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_quantexpd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_quantize.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_quantumd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_rem.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_round_integral.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_scalb.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_scalbl.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_sin.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_sinh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_sqrt.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_string.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_tan.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_tanh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_tgamma.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_to_bid128.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_to_int16.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_to_int32.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_to_int64.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_to_int8.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_to_uint16.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_to_uint32.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_to_uint64.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid64_to_uint8.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_acos.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_acosh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_add.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_asin.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_asinh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_atan2.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_atan.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_atanh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_cbrt.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_compare.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_cos.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_cosh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_div.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_erf.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_erfc.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_exp10.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_exp2.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_exp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_expm1.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_fdimd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_fma.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_fmod.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_frexp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_hypot.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_ldexp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_lgamma.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_llquantexpd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_llrintd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_llround.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_log10.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_log1p.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_log2.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_logb.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_logbd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_log.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_lrintd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_lround.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_minmax.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_modf.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_mul.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_nearbyintd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_next.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_nexttowardd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_noncomp.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_pow.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_quantexpd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_quantize.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_quantumd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_rem.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_round_integral.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_scalb.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_scalbl.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_sin.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_sinh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_sqrt.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_string.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_sub.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_tan.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_tanh.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_tgamma.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_to_bid128.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_to_bid64.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_to_int16.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_to_int32.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_to_int64.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_to_int8.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_to_uint16.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_to_uint32.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_to_uint64.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid32_to_uint8.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_binarydecimal.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_convert_data.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_decimal_data.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_decimal_globals.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_dpd.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_feclearexcept.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_fegetexceptflag.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_feraiseexcept.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_fesetexceptflag.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_fetestexcept.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_flag_operations.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_from_int.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/bid_round.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/strtod128.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/strtod32.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/strtod64.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/wcstod128.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/wcstod32.c")
    .file("IntelRDFPMathLib20U2/LIBRARY/src/wcstod64.c")
    .out_dir(output_dir.join("lib"))
    .compile("dfp-22");
}
