pub type FormulaTrace = Vec<String>;

pub fn join_trace(steps: &[String]) -> String {
    steps.join("\n")
}

pub fn num(value: f64) -> String {
    format!("{value:.2}")
}
