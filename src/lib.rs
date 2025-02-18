#[allow(unused_imports)]
#[allow(dead_code)]

mod roboclaw;
use pyo3::prelude::*;
use roboclaw::{RoboClaw, Motor};

#[pymodule]
fn roboclaw_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RoboClaw>()?;
    m.add_class::<Motor>()?;
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

    #[test]
    fn encoder() {
        let read_result: Vec<u32> = vec![12_000_000, 0b10000000];
        let mut encoder_value: i64 = 1_000_000_000;
        
        encoder_value = calculate_encoder(encoder_value, read_result);

        println!("{}", encoder_value);  
        assert!(true);
    }
}
