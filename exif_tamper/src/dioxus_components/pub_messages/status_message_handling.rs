



#[derive(Default, PartialEq, Clone)]
pub struct Handling {
    pub message: &'static str,
    pub status: Status,
    pub component: Component,
    pub result : Res,
    pub duration: u8,
    pub req: &'static str,
    pub copy: i8
}

#[derive(PartialEq, Clone)]
pub enum Status {
    AllFilesExifReadSuccessfully,
    SomeFilesExifReadSuccessfully,
    MessageEncodedSuccessfully,
    MessageDecodedSuccessfully,
    ProcessingMetaDataFiles,
    Def,
    ErrorInEncodingMessage,
    ErrorInDecodingMessage,
}

#[derive(PartialEq, Clone)]
pub enum Component {
    UserReq,
    UploadFile,
    GetInfos,
    Hide,
    Extract,
    Noni
}

#[derive(PartialEq, Clone)]
pub enum Res {
    Success(&'static str),
    Fail(&'static str),
    Pending(&'static str)
}

impl Default for Status {
    fn default() -> Self {
        Status::Def
    }
}

impl Default for Component {
    fn default() -> Self {
        Component::Noni
    }
}

impl Default for Res {
    fn default() -> Self {
        Res::Pending("Pending...")
    }
}