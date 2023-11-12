#[derive(Debug)]
enum ECMAVersion {
    SixOrTwentyFifteen,
    TwentySixteen,
    TwentySeventeen,
    TwentyEighteen,
    TwentyNineteen,
    TwentyTwenty,
    TwentyTwentyOne,
    TwentyTwentyTwo,
    TwentyTwentyThree,
}

#[derive(Debug)]
pub struct Config {
    ecmaVersion: ECMAVersion,
}

impl Config {
    pub fn default() -> Self {
        Self {
            ecmaVersion: ECMAVersion::TwentyTwentyThree,
        }
    }
}
