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
    ecma_version: ECMAVersion,
}

impl Config {
    pub fn default() -> Self {
        Self {
            ecma_version: ECMAVersion::TwentyTwentyThree,
        }
    }
}
