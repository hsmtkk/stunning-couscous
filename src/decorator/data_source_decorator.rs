use super::data_source::DataSource;

pub struct DataSourceDecorator {
    wrappee: dyn DataSource,
}

impl DataSource for DataSourceDecorator {
    fn write_data(&self, data:&str) {
        self.wrappee.write_data(data);
    }

    fn read_data(&self) -> String {
        self.wrappee.read_data()
    }
}