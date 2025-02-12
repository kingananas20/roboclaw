mod roboclaw;
use pyo3::prelude::*;
use roboclaw::RoboClaw;

#[pymodule]
fn roboclaw_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RoboClaw>()?;
    Ok(())
} 

#[cfg(test)]
mod tests {
    use crate::roboclaw::*;

    #[test]
    fn crc() {
        let mut crc: Crc16 = Crc16::new();

        crc.update(255);

        assert_eq!(crc.get(), 7920);
        
        crc.clear();
        crc.update(255);
        assert_eq!(crc.get(), 7920);
    }
}
