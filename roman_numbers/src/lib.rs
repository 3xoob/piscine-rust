#[derive(Debug, Clone, PartialEq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RomanNumber {
    digits: Vec<RomanDigit>,
}

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("Invalid Roman digit value"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber {
                digits: vec![RomanDigit::Nulla],
            };
        }

        let mut result = Vec::new();
        let roman_values = [
            (1000, RomanDigit::M),
            (900, RomanDigit::C),
            (500, RomanDigit::D),
            (400, RomanDigit::C),
            (100, RomanDigit::C),
            (90, RomanDigit::X),
            (50, RomanDigit::L),
            (40, RomanDigit::X),
            (10, RomanDigit::X),
            (9, RomanDigit::I),
            (5, RomanDigit::V),
            (4, RomanDigit::I),
            (1, RomanDigit::I),
        ];

        for &(value, ref digit) in roman_values.iter() {
            while num >= value {
                if value == 900 {
                    result.push(RomanDigit::C);
                    result.push(RomanDigit::M);
                } else if value == 400 {
                    result.push(RomanDigit::C);
                    result.push(RomanDigit::D);
                } else if value == 90 {
                    result.push(RomanDigit::X);
                    result.push(RomanDigit::C);
                } else if value == 40 {
                    result.push(RomanDigit::X);
                    result.push(RomanDigit::L);
                } else if value == 9 {
                    result.push(RomanDigit::I);
                    result.push(RomanDigit::X);
                } else if value == 4 {
                    result.push(RomanDigit::I);
                    result.push(RomanDigit::V);
                } else {
                    result.push(digit.clone());
                }
                num -= value;
            }
        }

        RomanNumber { digits: result }
    }
}
