/*
 * MIT License
 *
 * Copyright (c) 2022 Dariusz Depta
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

//! # Sanity tests
//!
//! ```text
//! 0 0 0
//! │ │ │
//! │ │ └─ status flags passed as an argument
//! │ └─── rounding mode passed as an argument
//! └───── result returned by value
/// ```

#[cfg(all(not(feature = "call-by-reference"), not(feature = "global-rounding"), not(feature = "global-exception-flags")))]
mod tests_000 {
  use dfp_number_sys::*;

  macro_rules! f {
    () => {
      &mut FB_CLEAR.clone()
    };
  }

  fn eq(expected: &str, actual: BID128) {
    let mut flags: u32 = 0;
    assert_eq!(expected, bid128_to_string(actual, &mut flags));
    assert_eq!(0, flags);
  }

  fn d128(s: &str) -> BID128 {
    let mut flags = FB_CLEAR;
    let x = bid128_from_string(s, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    x
  }

  #[test]
  fn test_bid128_add_0001() {
    let x = bid128_from_int32(2);
    let y = bid128_from_int32(5);
    let mut flags = FB_CLEAR;
    let z = bid128_add(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+7E+0", z);
  }

  #[test]
  fn test_bid128_copy() {
    eq("+12345E-4", bid128_copy(d128("1.2345")));
    eq("+1234500E-6", bid128_copy(d128("1.234500")));
  }

  #[test]
  fn test_bid128_div_0001() {
    let x = bid128_from_int32(2);
    let y = bid128_from_int32(5);
    let mut flags = FB_CLEAR;
    let z = bid128_div(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+4E-1", z);
  }

  #[test]
  fn test_bid128_exp_0001() {
    let x = bid128_from_int32(0);
    let mut flags = FB_CLEAR;
    let z = bid128_exp(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+1E+0", z);
  }

  #[test]
  fn test_bid128_exp_0002() {
    let x = bid128_from_int32(1);
    let mut flags = FB_CLEAR;
    let z = bid128_exp(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+2718281828459045235360287471352662E-33", z);
  }

  #[test]
  fn test_bid128_exp_0003() {
    let x = d128("2.5");
    let mut flags = FB_CLEAR;
    let z = bid128_exp(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+1218249396070347343807017595116797E-32", z);
  }

  #[test]
  fn test_bid128_frexp() {
    let x = d128("25.4300");
    let mut exp = 0_i32;
    let z = bid128_frexp(x, &mut exp);
    eq("+254300E-6", z);
    assert_eq!(2, exp);
  }

  #[test]
  fn test_bid128_from_int32() {
    eq("-2147483648E+0", bid128_from_int32(i32::MIN));
    eq("-10E+0", bid128_from_int32(-10));
    eq("-1E+0", bid128_from_int32(-1));
    eq("+0E+0", bid128_from_int32(0));
    eq("+1E+0", bid128_from_int32(1));
    eq("+10E+0", bid128_from_int32(10));
    eq("+2147483647E+0", bid128_from_int32(i32::MAX));
  }

  #[test]
  fn test_bid128_from_int64() {
    eq("-9223372036854775808E+0", bid128_from_int64(i64::MIN));
    eq("-10E+0", bid128_from_int64(-10));
    eq("-1E+0", bid128_from_int64(-1));
    eq("+0E+0", bid128_from_int64(0));
    eq("+1E+0", bid128_from_int64(1));
    eq("+10E+0", bid128_from_int64(10));
    eq("+9223372036854775807E+0", bid128_from_int64(i64::MAX));
  }

  #[test]
  fn test_bid128_from_string_0001() {
    let mut flags = FB_CLEAR;
    let x = bid128_from_string("-123.45", RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("-12345E-2", x);
  }

  #[test]
  fn test_bid128_from_string_0002() {
    let mut flags = FB_CLEAR;
    let x = bid128_from_string("-12345e-2", RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("-12345E-2", x);
  }

  #[test]
  fn test_bid128_from_uint32() {
    eq("+0E+0", bid128_from_uint32(0));
    eq("+1E+0", bid128_from_uint32(1));
    eq("+10E+0", bid128_from_uint32(10));
    eq("+4294967295E+0", bid128_from_uint32(u32::MAX));
  }

  #[test]
  fn test_bid128_from_uint64() {
    eq("+0E+0", bid128_from_uint64(0));
    eq("+1E+0", bid128_from_uint64(1));
    eq("+10E+0", bid128_from_uint64(10));
    eq("+18446744073709551615E+0", bid128_from_uint64(u64::MAX));
  }

  #[test]
  fn test_bid128_ilogb() {
    assert_eq!(-308, bid128_ilogb(d128("2.22507E-308"), f!()));
    assert_eq!(1, bid128_ilogb(d128("22.200"), f!()));
  }

  #[test]
  fn test_bid128_is_finite() {
    assert!(bid128_is_finite(bid128_from_int32(-1)));
    assert!(!bid128_is_finite(d128("NaN")));
  }

  #[test]
  fn test_bid128_is_zero() {
    assert!(!bid128_is_zero(bid128_from_int32(-1)));
    assert!(bid128_is_zero(bid128_from_int32(0)));
    assert!(!bid128_is_zero(bid128_from_int32(1)));
  }

  #[test]
  fn test_bid128_log_0001() {
    let x = bid128_from_int32(0);
    let mut flags = FB_CLEAR;
    let z = bid128_log(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_ZERO_DIVIDE, flags);
    eq("-Inf", z);
  }

  #[test]
  fn test_bid128_log_0002() {
    let x = bid128_from_int32(1);
    let mut flags = FB_CLEAR;
    let z = bid128_log(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+0E+0", z);
  }

  #[test]
  fn test_bid128_log_0003() {
    let x = d128("2.7182818284590452353602874713527");
    let mut flags = FB_CLEAR;
    let z = bid128_log(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+1000000000000000000000000000000014E-33", z);
  }

  #[test]
  fn test_bid128_log_0004() {
    let x = d128("10.0");
    let mut flags = FB_CLEAR;
    let z = bid128_log(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+2302585092994045684017991454684364E-33", z);
  }

  #[test]
  fn test_bid128_log_0005() {
    let x = d128("+Inf");
    let mut flags = FB_CLEAR;
    let z = bid128_log(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+Inf", z);
  }

  #[test]
  fn test_bid128_maxnum_0001() {
    let x = d128("1.234");
    let y = d128("2.256");
    let mut flags = FB_CLEAR;
    let z = bid128_maxnum(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+2256E-3", z);
  }

  #[test]
  fn test_bid128_minnum_0001() {
    let x = d128("1.2340000000");
    let y = d128("2.256000");
    let mut flags = FB_CLEAR;
    let z = bid128_minnum(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+12340000000E-10", z);
  }

  #[test]
  fn test_bid128_mul_0001() {
    let x = bid128_from_int32(2);
    let y = bid128_from_int32(5);
    let mut flags = FB_CLEAR;
    let z = bid128_mul(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+10E+0", z);
  }

  #[test]
  fn test_bid128_mul_0002() {
    let x = bid128_from_int32(i32::MAX);
    let y = bid128_from_int32(i32::MAX);
    let mut flags = FB_CLEAR;
    let z = bid128_mul(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+4611686014132420609E+0", z);
  }

  #[test]
  fn test_bid128_mul_0003() {
    let x = bid128_from_int64(i64::MAX);
    let y = bid128_from_int64(i64::MAX);
    let mut flags = FB_CLEAR;
    let z = bid128_mul(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+8507059173023461584739690778423250E+4", z);
  }

  #[test]
  fn test_bid128_negate_0001() {
    eq("-12345E-4", bid128_negate(d128("+1.2345")));
  }

  #[test]
  fn test_bid128_negate_0002() {
    eq("+12345E-4", bid128_negate(d128("-1.2345")));
  }

  #[test]
  fn test_bid128_negate_0003() {
    eq("-0E+0", bid128_negate(d128("+0")));
  }

  #[test]
  fn test_bid128_negate_0004() {
    eq("+0E+0", bid128_negate(d128("-0")));
  }

  #[test]
  fn test_bid128_pow() {
    eq("+8E+0", bid128_pow(d128("2"), d128("3"), RM_NEAREST_EVEN, f!()));
  }

  #[test]
  fn test_bid128_quantexp() {
    assert_eq!(-4, bid128_quantexp(d128("2.3456")));
    assert_eq!(-7, bid128_quantexp(d128("122.4567000")));
  }

  #[test]
  fn test_bid128_quantum() {
    eq("+1E-4", bid128_quantum(d128("2.3456")));
    eq("+1E-7", bid128_quantum(d128("122.4567000")));
  }

  #[test]
  fn test_bid128_quantize_0001() {
    let x = d128("2.3456");
    let y = d128("0.001");
    let mut flags = FB_CLEAR;
    let z = bid128_quantize(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+2346E-3", z);
  }

  #[test]
  fn test_bid128_quiet_equal_0001() {
    let x = d128("2.3456");
    let y = d128("2.3456");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_equal(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(z);
  }

  #[test]
  fn test_bid128_quiet_equal_0002() {
    let x = d128("2.3456");
    let y = d128("2.34561");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_equal(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(!z);
  }

  #[test]
  fn test_bid128_quiet_greater_0001() {
    let x = d128("2.3456");
    let y = d128("2.3456");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_greater(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(!z);
  }

  #[test]
  fn test_bid128_quiet_greater_0002() {
    let x = d128("2.34561");
    let y = d128("2.3456");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_greater(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(z);
  }

  #[test]
  fn test_bid128_quiet_greater_equal_0001() {
    let x = d128("2.3456");
    let y = d128("2.3456");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_greater_equal(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(z);
  }

  #[test]
  fn test_bid128_quiet_greater_equal_0002() {
    let x = d128("2.34561");
    let y = d128("2.3456");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_greater_equal(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(z);
  }

  #[test]
  fn test_bid128_quiet_greater_equal_0003() {
    let x = d128("2.3456");
    let y = d128("2.34561");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_greater_equal(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(!z);
  }

  #[test]
  fn test_bid128_quiet_less_0001() {
    let x = d128("2.3456");
    let y = d128("2.3456");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_less(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(!z);
  }

  #[test]
  fn test_bid128_quiet_less_0002() {
    let x = d128("2.3456");
    let y = d128("2.34561");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_less(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(z);
  }

  #[test]
  fn test_bid128_quiet_less_equal_0001() {
    let x = d128("2.3456");
    let y = d128("2.3456");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_less_equal(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(z);
  }

  #[test]
  fn test_bid128_quiet_less_equal_0002() {
    let x = d128("2.3456");
    let y = d128("2.34561");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_less_equal(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(z);
  }

  #[test]
  fn test_bid128_quiet_less_equal_0003() {
    let x = d128("2.34561");
    let y = d128("2.34560");
    let mut flags = FB_CLEAR;
    let z = bid128_quiet_less_equal(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    assert!(!z);
  }

  #[test]
  fn test_bid128_round_integral_nearest_away() {
    eq("-2E+0", bid128_round_integral_nearest_away(d128("-1.75"), f!()));
    eq("-2E+0", bid128_round_integral_nearest_away(d128("-1.5"), f!()));
    eq("-1E+0", bid128_round_integral_nearest_away(d128("-1.25"), f!()));
    eq("+1E+0", bid128_round_integral_nearest_away(d128("1.25"), f!()));
    eq("+2E+0", bid128_round_integral_nearest_away(d128("1.5"), f!()));
    eq("+2E+0", bid128_round_integral_nearest_away(d128("1.75"), f!()));
  }

  #[test]
  fn test_bid128_round_integral_exact_nearest_away() {
    eq("-2E+0", bid128_round_integral_exact(d128("-1.75"), RM_NEAREST_AWAY, f!()));
    eq("-2E+0", bid128_round_integral_exact(d128("-1.5"), RM_NEAREST_AWAY, f!()));
    eq("-1E+0", bid128_round_integral_exact(d128("-1.25"), RM_NEAREST_AWAY, f!()));
    eq("+1E+0", bid128_round_integral_exact(d128("1.25"), RM_NEAREST_AWAY, f!()));
    eq("+2E+0", bid128_round_integral_exact(d128("1.5"), RM_NEAREST_AWAY, f!()));
    eq("+2E+0", bid128_round_integral_exact(d128("1.75"), RM_NEAREST_AWAY, f!()));
  }

  #[test]
  fn test_bid128_round_integral_nearest_even() {
    eq("-3E+0", bid128_round_integral_nearest_even(d128("-2.75"), f!()));
    eq("-2E+0", bid128_round_integral_nearest_even(d128("-2.5"), f!()));
    eq("-2E+0", bid128_round_integral_nearest_even(d128("-2.25"), f!()));
    eq("+2E+0", bid128_round_integral_nearest_even(d128("2.25"), f!()));
    eq("+2E+0", bid128_round_integral_nearest_even(d128("2.5"), f!()));
    eq("+3E+0", bid128_round_integral_nearest_even(d128("2.75"), f!()));
  }

  #[test]
  fn test_bid128_round_integral_exact_nearest_even() {
    eq("-3E+0", bid128_round_integral_exact(d128("-2.75"), RM_NEAREST_EVEN, f!()));
    eq("-2E+0", bid128_round_integral_exact(d128("-2.5"), RM_NEAREST_EVEN, f!()));
    eq("-2E+0", bid128_round_integral_exact(d128("-2.25"), RM_NEAREST_EVEN, f!()));
    eq("+2E+0", bid128_round_integral_exact(d128("2.25"), RM_NEAREST_EVEN, f!()));
    eq("+2E+0", bid128_round_integral_exact(d128("2.5"), RM_NEAREST_EVEN, f!()));
    eq("+3E+0", bid128_round_integral_exact(d128("2.75"), RM_NEAREST_EVEN, f!()));
  }

  #[test]
  fn test_bid128_round_integral_positive() {
    eq("-1E+0", bid128_round_integral_positive(d128("-1.75"), f!()));
    eq("-1E+0", bid128_round_integral_positive(d128("-1.5"), f!()));
    eq("-1E+0", bid128_round_integral_positive(d128("-1.25"), f!()));
    eq("+2E+0", bid128_round_integral_positive(d128("1.25"), f!()));
    eq("+2E+0", bid128_round_integral_positive(d128("1.5"), f!()));
    eq("+2E+0", bid128_round_integral_positive(d128("1.75"), f!()));
  }

  #[test]
  fn test_bid128_round_integral_exact_upward() {
    eq("-1E+0", bid128_round_integral_exact(d128("-1.75"), RM_UPWARD, f!()));
    eq("-1E+0", bid128_round_integral_exact(d128("-1.5"), RM_UPWARD, f!()));
    eq("-1E+0", bid128_round_integral_exact(d128("-1.25"), RM_UPWARD, f!()));
    eq("+2E+0", bid128_round_integral_exact(d128("1.25"), RM_UPWARD, f!()));
    eq("+2E+0", bid128_round_integral_exact(d128("1.5"), RM_UPWARD, f!()));
    eq("+2E+0", bid128_round_integral_exact(d128("1.75"), RM_UPWARD, f!()));
  }

  #[test]
  fn test_bid128_round_integral_negative() {
    eq("-2E+0", bid128_round_integral_negative(d128("-1.75"), f!()));
    eq("-2E+0", bid128_round_integral_negative(d128("-1.5"), f!()));
    eq("-2E+0", bid128_round_integral_negative(d128("-1.25"), f!()));
    eq("+1E+0", bid128_round_integral_negative(d128("1.25"), f!()));
    eq("+1E+0", bid128_round_integral_negative(d128("1.5"), f!()));
    eq("+1E+0", bid128_round_integral_negative(d128("1.75"), f!()));
  }

  #[test]
  fn test_bid128_round_integral_downward() {
    eq("-2E+0", bid128_round_integral_exact(d128("-1.75"), RM_DOWNWARD, f!()));
    eq("-2E+0", bid128_round_integral_exact(d128("-1.5"), RM_DOWNWARD, f!()));
    eq("-2E+0", bid128_round_integral_exact(d128("-1.25"), RM_DOWNWARD, f!()));
    eq("+1E+0", bid128_round_integral_exact(d128("1.25"), RM_DOWNWARD, f!()));
    eq("+1E+0", bid128_round_integral_exact(d128("1.5"), RM_DOWNWARD, f!()));
    eq("+1E+0", bid128_round_integral_exact(d128("1.75"), RM_DOWNWARD, f!()));
  }

  #[test]
  fn test_bid128_round_integral_zero() {
    eq("-1E+0", bid128_round_integral_zero(d128("-1.75"), f!()));
    eq("-1E+0", bid128_round_integral_zero(d128("-1.5"), f!()));
    eq("-1E+0", bid128_round_integral_zero(d128("-1.25"), f!()));
    eq("+1E+0", bid128_round_integral_zero(d128("1.25"), f!()));
    eq("+1E+0", bid128_round_integral_zero(d128("1.5"), f!()));
    eq("+1E+0", bid128_round_integral_zero(d128("1.75"), f!()));
  }

  #[test]
  fn test_bid128_round_integral_toward_zero() {
    eq("-1E+0", bid128_round_integral_exact(d128("-1.75"), RM_TOWARD_ZERO, f!()));
    eq("-1E+0", bid128_round_integral_exact(d128("-1.5"), RM_TOWARD_ZERO, f!()));
    eq("-1E+0", bid128_round_integral_exact(d128("-1.25"), RM_TOWARD_ZERO, f!()));
    eq("+1E+0", bid128_round_integral_exact(d128("1.25"), RM_TOWARD_ZERO, f!()));
    eq("+1E+0", bid128_round_integral_exact(d128("1.5"), RM_TOWARD_ZERO, f!()));
    eq("+1E+0", bid128_round_integral_exact(d128("1.75"), RM_TOWARD_ZERO, f!()));
  }

  #[test]
  fn test_bid128_scalbn_0001() {
    let x = bid128_scalbn(bid128_from_int64(2356789100), -9);
    eq("+2356789100E-9", x);
  }

  #[test]
  fn test_bid128_sqrt() {
    eq("+1414213562373095048801688724209698E-33", bid128_sqrt(d128("2"), RM_NEAREST_EVEN, f!()));
  }

  #[test]
  fn test_bid128_sub_0001() {
    let x = bid128_from_int32(2);
    let y = bid128_from_int32(5);
    let mut flags = FB_CLEAR;
    let z = bid128_sub(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("-3E+0", z);
  }

  #[test]
  fn test_bid128_scalbn_0002() {
    let x = bid128_scalbn(bid128_from_int64(2356789100), -9);
    let y = bid128_scalbn(x, 2);
    eq("+2356789100E-7", y);
  }

  #[test]
  fn test_bid128_to_int32_int() {
    assert_eq!(0, bid128_to_int32_int(d128("0"), f!()));
    assert_eq!(0, bid128_to_int32_int(d128("0.12"), f!()));
    assert_eq!(0, bid128_to_int32_int(d128("0.99"), f!()));
    assert_eq!(0, bid128_to_int32_int(d128("-0.12"), f!()));
    assert_eq!(0, bid128_to_int32_int(d128("-0.99"), f!()));
    assert_eq!(2147483647, bid128_to_int32_int(d128("2147483647.999"), f!()));
    assert_eq!(-2147483648, bid128_to_int32_int(d128("-2147483648.999"), f!()));
    let mut flags = FB_CLEAR;
    assert_eq!(-2147483648, bid128_to_int32_int(d128("21474836483453459382.7423947"), &mut flags));
    assert_eq!(FB_INVALID, flags);
    let mut flags = FB_CLEAR;
    assert_eq!(-2147483648, bid128_to_int32_int(d128("-21474836483453459.3827423947"), &mut flags));
    assert_eq!(FB_INVALID, flags);
  }

  #[test]
  fn test_bid128_to_uint32_int() {
    assert_eq!(0, bid128_to_uint32_int(d128("0"), f!()));
    assert_eq!(0, bid128_to_uint32_int(d128("0.12"), f!()));
    assert_eq!(0, bid128_to_uint32_int(d128("0.99"), f!()));
    assert_eq!(4294967295, bid128_to_uint32_int(d128("4294967295.999"), f!()));
    let mut flags = FB_CLEAR;
    assert_eq!(2147483648, bid128_to_uint32_int(d128("214748364834534593829384"), &mut flags));
    assert_eq!(FB_INVALID, flags);
    let mut flags = FB_CLEAR;
    assert_eq!(2147483648, bid128_to_uint32_int(d128("-21474836483453459.3827423947"), &mut flags));
    assert_eq!(FB_INVALID, flags);
  }

  #[test]
  fn test_bid128_to_int64_int() {
    assert_eq!(0, bid128_to_int64_int(d128("0"), f!()));
    assert_eq!(0, bid128_to_int64_int(d128("0.12"), f!()));
    assert_eq!(0, bid128_to_int64_int(d128("0.99"), f!()));
    assert_eq!(0, bid128_to_int64_int(d128("-0.12"), f!()));
    assert_eq!(0, bid128_to_int64_int(d128("-0.99"), f!()));
    assert_eq!(9223372036854775807, bid128_to_int64_int(d128("9223372036854775807.999"), f!()));
    assert_eq!(-9223372036854775808, bid128_to_int64_int(d128("-9223372036854775808.999"), f!()));
    let mut flags = FB_CLEAR;
    assert_eq!(-9223372036854775808, bid128_to_int64_int(d128("921474836483453459382349857.74239"), &mut flags));
    assert_eq!(FB_INVALID, flags);
    let mut flags = FB_CLEAR;
    assert_eq!(-9223372036854775808, bid128_to_int64_int(d128("-9214748364834534599487453534.3827"), &mut flags));
    assert_eq!(FB_INVALID, flags);
  }

  #[test]
  fn test_bid128_to_uint64_int() {
    assert_eq!(0, bid128_to_uint64_int(d128("0"), f!()));
    assert_eq!(0, bid128_to_uint64_int(d128("0.12"), f!()));
    assert_eq!(0, bid128_to_uint64_int(d128("0.99"), f!()));
    assert_eq!(18446744073709551615, bid128_to_uint64_int(d128("18446744073709551615.999"), f!()));
    let mut flags = FB_CLEAR;
    assert_eq!(9223372036854775808, bid128_to_uint64_int(d128("3498375214748364834534593829384"), &mut flags));
    assert_eq!(FB_INVALID, flags);
    let mut flags = FB_CLEAR;
    assert_eq!(9223372036854775808, bid128_to_uint64_int(d128("-21474836483453459.3827423947"), &mut flags));
    assert_eq!(FB_INVALID, flags);
  }
}
