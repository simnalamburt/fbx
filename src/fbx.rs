pub struct Document {
    pub fbx_version: usize,
    pub creator: String,
    pub creation_time_stamp: usize,
}

impl Document {
    pub fn new() -> Document {
        Document {
            fbx_version: 0,
            creator: "".to_string(),
            creation_time_stamp: 0,
        }
    }
}
