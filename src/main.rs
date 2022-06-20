use std::collections::HashMap;


fn main() {
    let mut ds:School=School::new();
    ds.add(7,"Alice");
    ds.add(5, "Bob");
    ds.add(6, "LiLi");
    ds.add(7,"Petu" );
    let s=ds.grade(7);
    let x=ds.grades();
    println!("{:?}",s);
    println!("{:?}",x);
    //sortName(s);
   
}

#[derive(Debug)]
pub struct School {
    pub students: HashMap<String,u32>
}
impl  School {
    pub fn new()->School{
        School { students:(HashMap::new()) }
    }
    pub fn add(&mut self, grade: u32, student: &str) {
         self.students.insert(student.to_string(),grade);
    }


    pub fn grades(&self) ->Vec<u32>{
        let mut vec=Vec::new();
        for i in &self.students {
            vec.push(*i.1);
           }
           vec.sort();
           vec.dedup();
           return vec;
        }

    pub fn grade(&self, grade: u32) -> Vec<&String> {
        let mut vec=Vec::new();
        for i in &self.students {
            if i.1==&grade
            {
                vec.push(i.0);
            }
        }
        vec.sort();
        return vec;
    }
}

fn sort(mut v:Vec<u32>)
{
   let l=v.len();
    for i in 0..(l-1) {
        for j in  1..l{
            if v[i] > v[j] {
                let mut tg = v[i];
                v[i] = v[j];
                v[j]=tg;
            }
        }
    }
    println!("{:?}",v);
}
fn sortName(mut v:Vec<&String>)
{
    
}