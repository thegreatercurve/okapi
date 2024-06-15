#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    ecma_version: ECMAVersion,
}
