//sequence iron -> router-> mime -> urlencoded
extern crate iron;
extern crate router;
#[macro_use] extern crate mime;
extern crate urlencoded;
//the sequence of the extern definition matters

use iron::prelude::*;
use iron::status;
use router::Router;
// this definition matters too

use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form);
    router.post("/gcd", post_gcd);

    println!("Iron gcd web server on http: //localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();

}
//finding the greatest common divisor from two numbers
fn gcd(mut n:u64, mut m: u64) -> u64{
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m; m = n; n = t;
        }
        m = m%n;
    }
    n
}
//parsing the post values and process GCD and return the value
fn post_gcd(request: &mut Request) -> IronResult<Response>{
    let mut response = Response::new();

    let hashmap;
    match request.get_ref::<UrlEncodedBody>(){
        Err(e) =>{
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing from data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map)=> {hashmap = map;}
    }

    let unparsed_numbers;
    match hashmap.get("n"){
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has bo 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => {unparsed_numbers = nums;}
    }

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers{
        match u64::from_str(&unparsed) {
            Err(_)=>{
                response.set_mut(status::BadRequest);
                response.set_mut(format!("Value for 'n' parameter not a number: {:?}\n", unparsed));
                return Ok(response);
            }
            Ok(n) => {numbers.push(n);}
        }

        let mut d = numbers[0];
        for m in &numbers[1..]{
            d= gcd(d,*m);
        }

        response.set_mut(status::Ok);
        response.set_mut(mime!(Text/Html; Charset = Utf8));
        response.set_mut(format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n"
                    ,numbers,d));
    }
    Ok(response)
}
//initial format for the web site.
#[allow(unused_variables)]
fn get_form(request: &mut Request) -> IronResult<Response> {

    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>GCD calculator</title>
        <form action= "/gcd" method ="post">
            <input type = "text" name = "n"/>
            <input type = "text" name = "n"/>
            <button type = "submit">Compute GCD</button>
        </form>
    "#);

    Ok(response)
}
