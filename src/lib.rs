mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("Hello {}", name).as_str());
}


#[wasm_bindgen]
pub struct Resume {
    width: u32,
    height: u32,
    content: String
}

use std::fmt;

impl fmt::Display for Resume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "symbol")?;
        write!(f, "\n")?;
        Ok(())
    }
}

#[wasm_bindgen]
impl Resume {
    pub fn new() -> Resume {
        let width = 64;
        let height = 64;
        let content = "test".to_string();

        //let cells = (0..width * height)
        //    .map(|i| {
        //        if i % 2 == 0 || i % 7 == 0 {
        //            Cell::Alive
        //        } else {
        //            Cell::Dead
        //        }
        //    })
        //    .collect();

        Resume {
            width,
            height,
            content,
        }
    }

    pub fn render(&self) -> String {
        self.content.to_string()
    }

    fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.content = "d1234s".to_string();
        let f = std::fs::File::open("resume.yaml")?;
        let d: String = serde_yaml::from_reader(f)?;
        self.content = "YAML".to_string();
        //println!("Read YAML string: {}", d);
        Ok(())
    }

    pub fn tick(&mut self) {
        //self.content= "next".to_string();
        Self::read(self);
    }
}
