pub struct Tag {
    pub tags: Vec<String>
}

impl Tag {
    pub fn has_tag(&self, tag: String) -> bool {
        self.tags.contains(&tag)
    }

    pub fn new(tag: String) -> Tag {
        Tag {
            tags: vec![tag]
        }
    }
}