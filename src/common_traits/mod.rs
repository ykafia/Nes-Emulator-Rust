pub trait Byte_IO{
    fn byte_write(&self,address:u8,data:u8) -> ();
    fn byte_read(&self,address:u8,b_read_only:Option<bool>) -> u8; //b_read_only.unwrap_or(false)

}
