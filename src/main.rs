use ctext::Error;

fn main() -> Result<(), Error> {
    let mut args = std::env::args();

    match (args.nth(1), /*remaining_args=*/ args.len()) {
        (Some(arg), 0) if arg == "--version" => {
            println!("ctext {:?}", std::env::var("CTEXT_VERSION"))
        }
        _ => return Err(Error::TooManyArguments(std::env::args().count())),
    }
    Ok(())
}
