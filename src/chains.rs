use std::collections::HashMap;


pub trait Step {
    fn execute(&self, input: &str) -> String;
}

pub struct Chain {
    steps: Vec<Box<dyn Step>>
}

impl Chain {
    pub fn new() -> Self {
        Chain {
            steps: Vec::new(),
        }
    }
    pub fn add_steps(mut self, step: Box<dyn Step>) -> Self{
        self.steps.push(step);
        self
    }
    pub fn execute(&self, input: &str)-> String{
        let mut output = input.to_string();
        for step in &self.steps {
            output = step.execute(&output);
        }
        output
    }
    
}

pub struct Prompt {
    template: String,
    variables: HashMap<String,String>
}

impl Prompt {
    pub fn new(template: String) -> Self{
        let mut variables  = HashMap::new();
        Prompt{
            template,
            variables,
        }
    }

    pub fn add_variable(mut self, key:String,value:String)-> Self{
        self.variables.insert(key.to_string(), value.to_string());
        self
    }
    pub fn render(&self)-> String{
        let mut rendered = self.template.clone();
        for (key, value) in &self.variables {
            rendered = rendered.replace(&format!("{{{{{}}}}}", key), value);
        }
        rendered
    }
}

impl Step for Prompt {
     fn execute(&self, _input: &str) -> String {
        self.render()
    }
}

// llm struct should be implemented here.
// should probably have a url, temp, api key and model
pub struct LLM; // just a dummy!

impl Step for LLM {
     fn execute(&self, input: &str) -> String {
        format!("OutputL {}", input)
    }
    
}


//might need a parser for the output here. not sure yet
// serde crate does take care of a huge part out outputs
