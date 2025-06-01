#[cfg(test)]
mod test {
    use env_file_reader::read_file;

    #[test]
    fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        println!("API: {api_key}, SECRET: {secret}");
    }
}
