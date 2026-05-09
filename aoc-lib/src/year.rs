pub trait AOCYear {
    fn num_days(&self) -> u8;
    fn run_day(&self, day: u8, input: &str);
}