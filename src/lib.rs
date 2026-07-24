pub mod app;

pub mod math {
    #[derive(Debug)]
    pub enum FunctionFamily {
        Polynomial,
        Trigonometric,
        Exponential,
        Logarithmic,
        Rational,
        Radical,
        None,
    }

    #[derive(Debug)]
    pub enum ProblemType {
        Function(FunctionFamily),
        Limit,
        Differential,
        None,
    }

    impl Default for ProblemType {
        fn default() -> Self {
            Self::None
        }
    }

    #[derive(Debug, Default)]
    pub struct Problem {
        pub t: ProblemType,
        pub expression: Expression,
    }

    #[derive(Debug)]
    pub struct Equation {
        pub expression: Expression,
    }

    #[derive(Debug, Default)]
    pub struct Expression {
        pub content: String,
    }
}
