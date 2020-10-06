
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let tokenlib = PyModule::from_code(py, r#"
import tokenlib
def make_token(x, y):
    return tokenlib.make_token(x, secret=y)
"#, "main.py", "main").map_err(|e| {
            e.print_and_set_sys_last_vars(py);
            e
        })?;

        let thedict = [("user_id", 42)].into_py_dict(py);
        let result = match tokenlib.call1("make_token", (thedict, "asdf",)) {
            Err(e) => {
                e.print_and_set_sys_last_vars(py);
                return Err(e)
            }
            Ok(x) => x.extract::<String>().unwrap(),
        };
        //assert_eq!(result, false);
        println!("result {:?}", result);
        Ok(())
    })
}

