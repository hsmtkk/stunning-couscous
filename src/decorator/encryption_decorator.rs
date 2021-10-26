use super::data_source::DataSource;
use super::data_source_decorator::DataSourceDecorator;

pub struct EncryptionDecorator {
    dec: Box<DataSourceDecorator>,
}

impl EncryptionDecorator {
    fn new(dec:Box<DataSourceDecorator>) -> EncryptionDecorator {
        EncryptionDecorator{dec}
    }
}

impl DataSource for EncryptionDecorator {
    fn write_data(&self, data:&str) {
        // encrypt
        println!("writing encrypted data");
        self.dec.write_data(data);
    }

    fn read_data(&self) -> String {
        // decrypt
        println!("reading decrypted data");
        self.dec.read_data()
    }
}