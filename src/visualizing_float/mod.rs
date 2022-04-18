const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

/// converts an f32 into its parts by applying the requisite bit shifts and masks
pub fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

pub fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa = 1.0_f32;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}

pub fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

#[cfg(test)]
mod test {
    use super::{to_parts, decode};

    #[test]
    fn test_to_parts() {
        to_parts(123_f32);
        to_parts(-1_f32);
        to_parts(123.23_f32);
        to_parts(0.01_f32);
    }


    #[test]
    fn test_decode() {
        let parts = to_parts(123_f32);
        let decoded = decode(parts.0, parts.1, parts.2);
        println!("decoded: {decoded:?}");
        assert!(false);

    }
}
