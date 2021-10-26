pub trait DataSource {
    fn write_data(&self, data:&str);
    fn read_data(&self) -> String;
}
