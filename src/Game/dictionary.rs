// we may need to derive from many sources...
// we may have to make from a url
// we may have to make from a local file


use std::path::{Iter, Path};
use std::vec::IntoIter;
// if a url gets passed, we need to be able to handle downloading and parsing
// if its a local file, itd be nice to attempt to parse the file from many source types
// and remake it under
use http::Uri;

struct Dictionary(Vec<String>);

impl IntoIterator for Dictionary {  // Sugar
    type Item = String;
    type IntoIter = IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TryFrom<Uri> for Dictionary {
    fn try_from(value: Uri) -> Result<Self, Self::Error> {
        todo!()
    }
    type Error = ();
}

impl TryFrom<Path> for Dictionary {
    type Error = ();
    fn try_from(value: Box<Path>) -> Result<Self, Self::Error> {
        todo!()
    }
}