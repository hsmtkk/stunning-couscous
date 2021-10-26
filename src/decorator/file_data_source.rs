use super::data_source::DataSource;

pub struct FileDataSource {
    file_name: String,
}

impl FileDataSource {
    pub fn new(file_name: &str) -> FileDataSource {
        FileDataSource{file_name:file_name.to_string()}
    }
}

impl DataSource for FileDataSource {
    fn write_data(&self, _data:&str) {
        println!("writing data to file");
    }

    fn read_data(&self) -> String {
        println!("reading data from file");
        "read_data".to_string()
    }
}