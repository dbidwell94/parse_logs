#[cfg(test)]
pub mod test_helpers {
    use crate::{Overwrite, Readable};
    use std::io::Error;

    #[derive(Debug)]
    pub struct MockWriter {
        overwrite_calls: usize,
        read_calls: usize,
        data: Vec<u8>,
    }

    impl MockWriter {
        pub fn new() -> Self {
            MockWriter {
                overwrite_calls: 0,
                read_calls: 0,
                data: Vec::new(),
            }
        }

        pub fn get_overwrite_calls(&self) -> &usize {
            &self.overwrite_calls
        }

        pub fn get_read_calls(&self) -> &usize {
            &self.read_calls
        }

        pub fn get_data(&self) -> &Vec<u8> {
            return &self.data;
        }
    }

    impl Overwrite for MockWriter {
        fn overwrite(&mut self, data: &[u8]) -> Result<(), Error> {
            self.overwrite_calls += 1;
            self.data.overwrite(data)
        }
    }

    impl Readable for MockWriter {
        fn read_as_str(&mut self) -> String {
            self.read_calls += 1;
            self.data.read_as_str()
        }
    }
}
