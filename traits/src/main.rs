fn main() {
    println!("Hello World");
}

struct People<T ,doings> {
    name : String ,
    task:T
}

trait doings {
    fn done();

    fn progress();

    fn cancelled();
}

impl doings for People {
    fn done() {
        println!("Done Effectively")
    }
    fn progress() {
        println!("In Progess")
    }

    fn cancelled() {
        println!("Cancelled")
    }
}
