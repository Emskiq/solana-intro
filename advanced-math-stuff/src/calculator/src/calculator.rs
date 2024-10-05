use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorInstructions {
    operation: u8,
    operating_value: u32,
}

impl CalculatorInstructions {
    pub fn evaluate(&self, val: u32) -> u32 {
        match &self.operation {
            1 => return val + &self.operating_value,
            2 => return val - &self.operating_value,
            3 => return val * &self.operating_value,
            4 => return val / &self.operating_value,
            _ => return val * 0,
        }
    }
}
