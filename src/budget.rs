// budget.rs
pub struct Budget {
    pub income: f64,
    pub expenses: f64,
}

impl Budget {
    pub fn new() -> Self {
        Budget {
            income: 0.0,
            expenses: 0.0,
        }
    }

    pub fn add_income(&mut self, amount: f64) {
        self.income += amount;
    }

    pub fn add_expense(&mut self, amount: f64) {
        self.expenses += amount;
    }

    pub fn balance(&self) -> f64 {
        self.income - self.expenses
    }
}
