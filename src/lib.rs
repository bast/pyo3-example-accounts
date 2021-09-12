use pyo3::prelude::*;

#[pyclass]
struct Account {
    balance: f64,
}

#[pymethods]
impl Account {
    #[new]
    fn new(balance: f64) -> Self {
        Account { balance }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

#[pymodule]
fn accounts(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_class::<Account>()?;

    Ok(())
}
