enum WebEvent {
    PageLoad,
    PageUnload,
    keyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}


fn inspect(event:WebEvent){
    match event{
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::keyPress(c)=> println!("pressed '{}'.",c)
    }
}