use std::ops::Deref;

fn main() {

   let mut  s: Screen=Screen{
       components:vec![]
   };
    let mut  a1=quadrilateral{width:4,length:6,label:"".to_lowercase()};
    s.components.push(Box::new(a1));
    let mut  a2=quadrilateral{width:4,length:5,label:"".to_lowercase()};
    s.components.push(Box::new(a2));
    let mut  a3=quadrilateral{width:4,length:4,label:"".to_lowercase()};
    s.components.push(Box::new(a3));
    let mut  a4=quadrilateral{width:4,length:3,label:"".to_lowercase()};
    s.components.push(Box::new(a4));
    s.run();



}
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value:i32){
        self.list.push(value);
        self.updaverage();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let r=self.list.pop();
        match r {
            Some(value)=>{
                self.updaverage();
                Some(value)
            },
            None=>None,
        }

    }
    pub fn average(&self)->f64{
        self.average
    }
    fn updaverage(&mut self){

        let    n:i32= self.list.len() as i32;
        let total: i32 = self.list.iter().sum();
        self.average= (total / n) as f64;
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
#[derive(Debug)]
pub struct quadrilateral{
    pub width:i32,
    pub length:i32,
    pub label:String,
}
pub trait Draw {
    fn draw(&self);
}
impl Draw for quadrilateral {
    fn draw(& self) {
        if self.length==self.width{
            println!("Equilateral quadrilateral" );
        }else {
            println!("common quadrilateral" );
        }
    }
}


