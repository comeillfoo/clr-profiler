use crate::{
    ffi::{mdMethodDef, HRESULT, mdTypeDef},
    MethodProps, TypeDefProps,
};

pub trait MetadataImportTrait {
    fn get_method_props(&self, mb: mdMethodDef) -> Result<MethodProps, HRESULT>;
    fn get_typedef_props(&self, mb: mdTypeDef) -> Result<TypeDefProps, HRESULT>;
}
