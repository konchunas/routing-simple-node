include!("./transfer_capnp.rs");
use capnp::serialize_packed;
use capnp::message;
use std::io;

pub fn create_example(amount: u64) -> io::Result<Vec<u8>>
{
    let mut message = message::Builder::new_default();
    {
        let mut transfer = message.init_root::<transfer::Builder>();
        // Setters
        transfer.set_to(1234);
        transfer.set_from(4142);
        transfer.set_amount(amount);
        transfer.set_seed(1);
    }
    let mut buffer: Vec<u8> = Vec::new();
    let result = serialize_packed::write_message(&mut buffer, &mut message);
    match result {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}