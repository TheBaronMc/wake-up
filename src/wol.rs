pub trait Wake {
    fn wake(&self) -> (); 
}

pub fn wake_on_lan(host: &str, port: &u16) -> () {

}