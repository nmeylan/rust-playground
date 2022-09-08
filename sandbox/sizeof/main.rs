use std::mem::size_of_val;
use get_size::GetSize;

fn main() {
    let str_vec = vec!["hellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohello", "hellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohello", "hellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohello"];
    println!("a address {}", str_vec[0].as_ptr() as usize);
    println!("b address {}", str_vec[1].as_ptr() as usize);
    println!("The useful size of `str_vec` is {}", size_of_val(&*str_vec));
    println!("get_size is {}", &str_vec.get_size());
    println!("get_heap_size is {}", &str_vec.get_heap_size());

    let str_vec = vec!["hellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohello".to_string(), "hellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohello".to_string(), "hellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohello".to_string()];
    println!("a address {}", str_vec[0].as_ptr() as usize);
    println!("b address {}", str_vec[1].as_ptr() as usize);
    println!("The useful size of `str_vec` is {}", size_of_val(&*str_vec));
    println!("get_size is {}", &str_vec.get_size());
    println!("get_heap_size is {}", &str_vec.get_heap_size());
}