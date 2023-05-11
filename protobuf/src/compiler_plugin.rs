// TODO: move into separate crate
#![doc(hidden)]

use crate::descriptor::FileDescriptorProto;
use crate::plugin::*;
use crate::Message;
use alloc::str;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct GenRequest<'a> {
    pub file_descriptors: &'a [FileDescriptorProto],
    pub files_to_generate: &'a [String],
    pub parameter: &'a str,
}

pub struct GenResult {
    pub name: String,
    pub content: Vec<u8>,
}

#[cfg(feature = "std")]
pub fn plugin_main<F>(gen: F)
where
    F: Fn(&[FileDescriptorProto], &[String]) -> Vec<GenResult>,
{
    plugin_main_2(|r| gen(r.file_descriptors, r.files_to_generate))
}

#[cfg(feature = "std")]
pub fn plugin_main_2<F>(gen: F)
where
    F: Fn(&GenRequest) -> Vec<GenResult>,
{
    let req = CodeGeneratorRequest::parse_from_reader(&mut std::id::stdin()).unwrap();
    let result = gen(&GenRequest {
        file_descriptors: &req.get_proto_file(),
        files_to_generate: &req.get_file_to_generate(),
        parameter: req.get_parameter(),
    });
    let mut resp = CodeGeneratorResponse::new();
    resp.set_file(
        result
            .iter()
            .map(|file| {
                let mut r = CodeGeneratorResponse_File::new();
                r.set_name(file.name.to_string());
                r.set_content(str::from_utf8(file.content.as_ref()).unwrap().to_string());
                r
            })
            .collect(),
    );
    resp.write_to_writer(&mut std::io::stdout()).unwrap();
}
