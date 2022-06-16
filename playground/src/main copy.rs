
#![allow(dead_code,unused_imports,unused_variables)]
use std::vec;
use rand::Rng;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, Ordering};

#[derive(Debug,Default,Clone,Copy)]
struct FreeLocations{
    size: u64,
    offset: u64,
}

impl Ord for FreeLocations {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.size).cmp( &other.size)
    }
}

impl PartialOrd for FreeLocations {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.size.cmp(&other.size))
    }
    // fn lt(&self,other: &Self) -> bool {
    //     matches!(self.size.partial_cmp(&other.size), Some(Less))
    // }
    // fn le(&self, other: &Self) -> bool {
    //      !matches!(self.size.partial_cmp(&other.size), None | Some(Greater))
    // }
    // fn gt(&self, other: &Self) -> bool {
    //     matches!(self.size.partial_cmp(&other.size), Some(Greater))
    // }
    // fn ge(&self, other: &Self) -> bool {
    //     matches!(self.size.partial_cmp(&other.size), Some(Greater | Equal))
    // }
}

impl PartialEq for FreeLocations {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl Eq for FreeLocations { }


fn fill_vector(max:u64) -> Vec<FreeLocations> {
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();
    v.push( FreeLocations{
        size:20,
        offset: 0,
    });

     for i in 1..max {
        
        let last_v = v.pop().unwrap();
        v.push(last_v);
        
        // if i == 500000 {
        //     v.push( FreeLocations{
        //         size: 500 as u64,
        //         offset: 500 as u64,
        //     });
        // }
        // else{
            v.push( FreeLocations{
                size: last_v.size +  rng.gen_range(0..50) as u64,
                offset: rng.gen_range(0..600) as u64,
            });
        // }
           
     }
   v

}

fn find_bn(vec:  &Vec<FreeLocations>, size:u64) ->Option<FreeLocations>{
   let p = match vec.binary_search_by(|i| i.size.cmp(&size))  {
        Ok(pos) => { println!(" Found : {:?}", pos);  Some(vec[pos])} // element already in vector @ `pos` 
        Err(err) => {  println!(" Not Found : {:?}", err); None } 
 
    };
 p
}

fn find_ord(vec:  &Vec<FreeLocations>, size:u64) ->Option<FreeLocations>{

     let a:Vec<FreeLocations>= vec.iter().filter(|x| x.size == size).cloned().collect();
     if a.len()>0  { Some(a[0]) }
     else { None }
  
}



fn insert_bn(loc:  FreeLocations, vector:  &mut Vec<FreeLocations>) {

    let pos = vector.binary_search_by(|i| i.size.cmp(&loc.size)).unwrap_or_else(|e| e);
    vector.insert(pos, loc);

}



fn main() {




    let mut vector:Vec<FreeLocations> = vec![];
    vector.push( FreeLocations{
        size: 20,
        offset: 3,
    });

    vector.push( FreeLocations{
        size: 30,
        offset: 40,
    });

    vector.push( FreeLocations{
        size: 40,
        offset: 60,
    });
   // println!("Vector  : {:?}", vector);
  
    let new_elem = FreeLocations{
        size:5,
        offset:430
    };

    insert_bn(new_elem,&mut vector);


    println!("New Vector  : {:?}", vector);
    println!("Len  : {:?}", vector.len());
    println!("pos 4  : {:?}", vector[3]);
  println!("Found {:?}", find_bn(&vector, 2));


    // vector =fill_vector(1000000);
  //  let v = vector.clone();
   //   println!("Vector f : {:?}", v[500000]);
    // println!("Vector: {:?}", vector);
 //   println!("Found {:?}", find_bn(&vector, v[500000].size).unwrap());
  // println!("Found {:?}", find_ord( &vector, v[500000].size).unwrap());
    // match vector.binary_search_by(|i| i.size.cmp(&500))  {
    //     Ok(pos) => {println!("pos: {:?}", pos)} // element already in vector @ `pos` 
    //     Err(err) =>    println!("Error: {:?}", err) ,
 
    // }
   // println!("Vec: {:?}", fill_vector(5));
    
}