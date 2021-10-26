use super::data_source::DataSource;
use super::data_source_decorator::DataSourceDecorator;

pub struct CompressionDecorator {
    dec: Box<DataSourceDecorator>,
}

impl CompressionDecorator {
    fn new(dec:Box<DataSourceDecorator>) -> CompressionDecorator {
        CompressionDecorator{dec}
    }
}

impl DataSource for CompressionDecorator {
    fn write_data(&self, data:&str) {
        // compress
        println!("writing compressed data");
        self.dec.write_data(data);
    }

    fn read_data(&self) -> String {
        // decompress
        println!("reading decompressed data");
        self.dec.read_data()
    }
}