fn main() {
    let mut result = false;
    let toto: Result<i32, &str> = Err("failure");

    let mut res: Result<i32, String> = (||{
        toto?;
        return Ok(0 as i32);
    })();
    if res.is_err() {
            res = (||{
            println!("success block 2");
            return Ok(1 as i32);
        })();
    }
    if res.is_ok() {

        println!("{}", res.unwrap());
    }
    println!("end main");
}
