use qr_generator as qr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Hello World";
    let data = qr::data_from_bytes(text.as_bytes(), 13, 8)?;
    let generator = qr::create_generator(13)?;
    let ec = qr::create_ec_codewords(&data, &generator)?;
    let all_codewords = [&data[..], &ec[..]].concat();
    println!("{:?}", all_codewords);
    Ok(())
}
