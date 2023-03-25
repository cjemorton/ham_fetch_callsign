extern crate reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.head("https://apc-cap.ic.gc.ca/datafiles/amateur_delim.zip")
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}
