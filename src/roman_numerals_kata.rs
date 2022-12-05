const ROMAN_SYMBOLS: [(i32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub fn convert(num: i32) -> String {
    let mut rest = num;
    let mut result = String::new();

    for (base, symbol) in ROMAN_SYMBOLS {
        while rest >= base {
            result.push_str(symbol);
            rest -= base;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_i_when_convert_1() {
        assert_eq!("I", convert(1));
    }

    #[test]
    fn should_be_ii_when_convert_2() {
        assert_eq!("II", convert(2));
    }

    #[test]
    fn should_be_iii_when_convert_3() {
        assert_eq!("III", convert(3));
    }

    #[test]
    fn should_be_iv_when_convert_4() {
        assert_eq!("IV", convert(4));
    }

    #[test]
    fn should_be_v_when_convert_5() {
        assert_eq!("V", convert(5));
    }

    #[test]
    fn should_be_vi_when_convert_6() {
        assert_eq!("VI", convert(6));
    }

    #[test]
    fn should_be_viii_when_convert_8() {
        assert_eq!("VIII", convert(8));
    }

    #[test]
    fn should_be_ix_when_convert_9() {
        assert_eq!("IX", convert(9));
    }

    #[test]
    fn should_be_x_when_convert_10() {
        assert_eq!("X", convert(10));
    }

    #[test]
    fn should_be_xii_when_convert_12() {
        assert_eq!("XII", convert(12));
    }

    #[test]
    fn should_be_xx_when_convert_20() {
        assert_eq!("XX", convert(20));
    }

    #[test]
    fn should_be_xxi_when_convert_21() {
        assert_eq!("XXI", convert(21));
    }

    #[test]
    fn should_be_xxx_when_convert_30() {
        assert_eq!("XXX", convert(30));
    }

    #[test]
    fn should_be_xl_when_convert_40() {
        assert_eq!("XL", convert(40));
    }

    #[test]
    fn should_be_l_when_convert_50() {
        assert_eq!("L", convert(50));
    }

    #[test]
    fn should_be_lc_when_convert_90() {
        assert_eq!("XC", convert(90));
    }

    #[test]
    fn should_be_c_when_convert_100() {
        assert_eq!("C", convert(100));
    }

    #[test]
    fn should_be_cd_when_convert_400() {
        assert_eq!("CD", convert(400));
    }

    #[test]
    fn should_be_d_when_convert_500() {
        assert_eq!("D", convert(500));
    }

    #[test]
    fn should_be_cm_when_convert_900() {
        assert_eq!("CM", convert(900));
    }

    #[test]
    fn should_be_m_when_convert_1000() {
        assert_eq!("M", convert(1000));
    }

    #[test]
    fn should_be_mmxvi_when_convert_2016() {
        assert_eq!("MMXVI", convert(2016));
    }

    #[test]
    fn should_be_cdxcix_when_convert_499() {
        assert_eq!("CDXCIX", convert(499));
    }
}
