pub mod libgen;

pub mod waste {
    pub struct WasteFolderProperties {
        pub parent: String,
        pub info: String,
        pub file: String,
        pub is_removable: bool,
    }

    impl WasteFolderProperties {
        pub fn new() -> Self {
            Self {
                parent: String::new(),
                info: String::new(),
                file: String::new(),
                is_removable: false,
            }
        }
    }
}
