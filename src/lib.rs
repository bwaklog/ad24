use std::fs;
use std::path::PathBuf;

pub struct Input {
    pub prob_id: u8,
    pub dataset: PathBuf,
    pub content: String,
}

impl Input {
    pub fn init(prob_id: u8, dataset: String) -> Option<Input> {
        let path = PathBuf::from(format!("./inputs/{prob_id}_{dataset}"));
        Some(Input {
            prob_id,
            dataset: PathBuf::from(format!("./inputs/{prob_id}_{dataset}")),
            content: fs::read_to_string(path).unwrap(),
        })
    }

    pub fn load_content(&self) -> Option<String> {
        match fs::read_to_string(self.dataset.clone()) {
            Ok(content) => Some(content),
            Err(_) => None,
        };
        None
    }
}
