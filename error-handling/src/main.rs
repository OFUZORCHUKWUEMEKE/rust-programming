use std::fs::File;
use std::io::ErrorKind;

// fn main(){
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result{
//         Ok(file)=>file,
//         Err(error)=>panic!("Problem opening the file: {:?}",error)
//     }
// }


// In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in debugging.

fn main(){
    let greeting_file = File::open("hello.txt");

    // let greeting = File::open("hello.txt").unwrap()

    let greeting = File::open("hello.txt").expect("hello.txt should be included in this project").

    let greeting_file_result = match greeting_file{
        Ok(file)=>file,
        Err(error)=> match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc)=>fc,
                Error(err)=>panic!("Problem creating the file : {:?}",e)
            },
            other_error=>{
                panic!("problem opening the file : {:?}",other_error)
            }
        }
    }
}

