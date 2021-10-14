#[derive(Debug, Clone)]
pub struct ShaderSource<'a> {
    pub sources: &'a [(&'a str, &'a [u8])],
}

impl<'a> ShaderSource<'a> {
    pub fn get_source(&self, api: &str) -> Option<&[u8]> {
        self.sources
            .iter()
            .find(|&&(id, _)| id == api)
            .map(|(_, data)| *data)
    }
}
