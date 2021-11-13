pub fn load_env() {
    dotenv::from_filename("test.env").unwrap();
}
