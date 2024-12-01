use std::fs;
use std::path::PathBuf;

pub struct Input {
    pub prob_id: u8,
    pub dataset: PathBuf,
}

impl Input {
    pub fn init(prob_id: u8, dataset: String) -> Option<Input> {
        return Some(Input {
            prob_id,
            dataset: PathBuf::from(format!("./inputs/{prob_id}_{dataset}")),
        });
    }

    pub fn load_content(&self) -> Option<String> {
        match fs::read_to_string(self.dataset.clone()) {
            Ok(content) => Some(content),
            Err(_) => None,
        };
        return None;
    }
}
