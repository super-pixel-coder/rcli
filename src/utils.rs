pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn std::io::Read>> {
    let reader: Box<dyn std::io::Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    Ok(reader)
}
