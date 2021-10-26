use super::data_source::DataSource;
use super::file_data_source::FileDataSource;

struct Application {}

impl Application {
    fn new() -> Application {
        Application{}
    }

    fn dumbUsageExample(&self) {
        let mut source: impl DataSource;
        source = FileDataSource::new("somefile.dat");
        source.write_data("salary records");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test(){
        let app = super::Application::new();
        app.dumbUsageExample();
    }
}