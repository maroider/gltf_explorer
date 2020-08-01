use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

const GLTF_FILE_EXTENSIONS: &[&str] = &["glb", "gltf"];

pub fn pick_and_import() -> iced::Subscription<PickAndImport> {
    super::dialog::dialog(native_dialog::OpenSingleFile {
        dir: None,
        filter: Some(GLTF_FILE_EXTENSIONS),
    })
    .map(|res| match res {
        Ok(path) => {
            if let Some(path) = path {
                let path = PathBuf::from(path);
                match gltf::import(&path) {
                    Ok((document, buffers, images)) => {
                        PickAndImport::Finished(Arc::new(Document {
                            path,
                            document,
                            buffers,
                            images,
                        }))
                    }
                    Err(err) => PickAndImport::Errored(err.to_string()),
                }
            } else {
                PickAndImport::NoFileSelected
            }
        }
        Err(err) => PickAndImport::Errored(err.to_string()),
    })
}

#[derive(Debug, Clone)]
pub enum PickAndImport {
    NoFileSelected,
    Finished(Arc<Document>),
    Errored(String),
}

#[derive(Debug, Clone)]
pub struct Document {
    pub path: PathBuf,
    pub document: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

impl Document {
    pub fn import<P: AsRef<Path>>(file: P) -> Result<Arc<Self>, String> {
        gltf::import(file.as_ref())
            .map(|(document, buffers, images)| {
                Arc::new(Document {
                    path: file.as_ref().to_path_buf(),
                    document,
                    buffers,
                    images,
                })
            })
            .map_err(|err| err.to_string())
    }
}
