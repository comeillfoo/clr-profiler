#![allow(non_snake_case)]
use crate::ffi::{
    int, mdCustomAttribute, mdEvent, mdFieldDef, mdInterfaceImpl, mdMemberRef, mdMethodDef,
    mdModule, mdModuleRef, mdParamDef, mdPermission, mdProperty, mdSignature, mdString, mdToken,
    mdTypeDef, mdTypeRef, mdTypeSpec, Unknown, BOOL, COR_FIELD_OFFSET, DWORD, GUID, HCORENUM,
    HRESULT, LPCWSTR, MDUTF8CSTR, PCCOR_SIGNATURE, REFIID, ULONG, UVCP_CONSTANT, WCHAR,
};
use std::ffi::c_void;

#[repr(C)]
pub struct IMetaDataImport<T> {
    pub CloseEnum: unsafe extern "system" fn(this: &T, hEnum: HCORENUM) -> (),
    pub CountEnum:
        unsafe extern "system" fn(this: &T, hEnum: HCORENUM, pulCount: *mut ULONG) -> HRESULT,
    pub ResetEnum:
        unsafe extern "system" fn(this: &T, hEnum: HCORENUM, ulPos: *const ULONG) -> HRESULT,
    pub EnumTypeDefs: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rTypeDefs: *const mdTypeDef,
        cMax: ULONG,
        pcTypeDefs: *mut ULONG,
    ) -> HRESULT,
    pub EnumInterfaceImpls: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        td: mdTypeDef,
        rImpls: *mut mdInterfaceImpl,
        cMax: ULONG,
        pcImpls: *mut ULONG,
    ) -> HRESULT,
    pub EnumTypeRefs: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rTypeRefs: *mut mdTypeRef,
        cMax: ULONG,
        pcTypeRefs: *mut ULONG,
    ) -> HRESULT,
    pub FindTypeDefByName: unsafe extern "system" fn(
        this: &T,
        szTypeDef: LPCWSTR,
        tkEnclosingClass: mdToken,
        ptd: *mut mdTypeDef,
    ) -> HRESULT,
    pub GetScopeProps: unsafe extern "system" fn(
        this: &T,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        pmvid: *mut GUID,
    ) -> HRESULT,
    pub GetModuleFromScope: unsafe extern "system" fn(this: &T, pmd: *mut mdModule) -> HRESULT,
    pub GetTypeDefProps: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        szTypeDef: *mut WCHAR,
        cchTypeDef: ULONG,
        pchTypeDef: *mut ULONG,
        pdwTypeDefFlags: *mut DWORD,
        ptkExtends: *mut mdToken,
    ) -> HRESULT,
    pub GetInterfaceImplProps: unsafe extern "system" fn(
        this: &T,
        iiImpl: mdInterfaceImpl,
        pClass: *mut mdTypeDef,
        ptkIface: *mut mdToken,
    ) -> HRESULT,
    pub GetTypeRefProps: unsafe extern "system" fn(
        this: &T,
        tr: mdTypeRef,
        ptkResolutionScope: *mut mdToken,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
    ) -> HRESULT,
    pub ResolveTypeRef: unsafe extern "system" fn(
        this: &T,
        tr: mdTypeRef,
        riid: REFIID,
        ppIScope: *mut *mut Unknown, // TODO: What actual class here?
        ptd: *mut mdTypeDef,
    ) -> HRESULT,
    pub EnumMembers: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        rMembers: *mut mdToken,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumMembersWithName: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        szName: LPCWSTR,
        rMembers: *mut mdToken,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumMethods: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        rMethods: *mut mdMethodDef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumMethodsWithName: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        szName: LPCWSTR,
        rMethods: *mut mdMethodDef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumFields: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        rFields: *mut mdFieldDef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumFieldsWithName: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        szName: LPCWSTR,
        rFields: *mut mdFieldDef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumParams: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        mb: mdMethodDef,
        rParams: *mut mdParamDef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumMemberRefs: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        tkParent: mdToken,
        rMemberRefs: *mut mdMemberRef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumMethodImpls: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        td: mdTypeDef,
        rMethodBody: *mut mdToken,
        rMethodDecl: *mut mdToken,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumPermissionSets: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        tk: mdToken,
        dwActions: DWORD,
        rPermission: *mut mdPermission,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub FindMember: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        szName: LPCWSTR,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmb: *mut mdToken,
    ) -> HRESULT,
    pub FindMethod: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        szName: LPCWSTR,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmb: *mut mdMethodDef,
    ) -> HRESULT,
    pub FindField: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        szName: LPCWSTR,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmb: *mut mdFieldDef,
    ) -> HRESULT,
    pub FindMemberRef: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        szName: LPCWSTR,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmb: *mut mdMemberRef,
    ) -> HRESULT,
    pub GetMethodProps: unsafe extern "system" fn(
        this: &T,
        mb: mdMethodDef,
        pClass: *mut mdTypeDef,
        szMethod: *mut WCHAR,
        cchMethod: ULONG,
        pchMethod: *mut ULONG,
        pdwAttr: *mut DWORD,
        ppvSigBlob: *mut PCCOR_SIGNATURE,
        pcbSigBlob: *mut ULONG,
        pulCodeRVA: *mut ULONG,
        pdwImplFlags: *mut DWORD,
    ) -> HRESULT,
    pub GetMemberRefProps: unsafe extern "system" fn(
        this: &T,
        mr: mdMemberRef,
        ptk: *mut mdToken,
        szMember: *mut WCHAR,
        cchMember: ULONG,
        pchMember: *mut ULONG,
        ppvSigBlob: *mut PCCOR_SIGNATURE,
        pbSig: *mut ULONG,
    ) -> HRESULT,
    pub EnumProperties: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        td: mdTypeDef,
        rProperties: *mut mdProperty,
        cMax: ULONG,
        pcProperties: *mut ULONG,
    ) -> HRESULT,
    pub EnumEvents: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        td: mdTypeDef,
        rEvents: *mut mdEvent,
        cMax: ULONG,
        pcEvents: *mut ULONG,
    ) -> HRESULT,
    pub GetEventProps: unsafe extern "system" fn(
        this: &T,
        ev: mdEvent,
        pClass: *mut mdTypeDef,
        szEvent: *mut WCHAR,
        cchEvent: ULONG,
        pchEvent: *mut ULONG,
        pdwEventFlags: *mut DWORD,
        ptkEventType: *mut mdToken,
        pmdAddOn: *mut mdMethodDef,
        pmdRemoveOn: *mut mdMethodDef,
        pmdFire: *mut mdMethodDef,
        rmdOtherMethod: *mut mdMethodDef,
        cMax: ULONG,
        pcOtherMethod: *mut ULONG,
    ) -> HRESULT,
    pub EnumMethodSemantics: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        mb: mdMethodDef,
        rEventProp: *mut mdToken,
        cMax: ULONG,
        pcEventProp: *mut ULONG,
    ) -> HRESULT,
    pub GetMethodSemantics: unsafe extern "system" fn(
        this: &T,
        mb: mdMethodDef,
        tkEventProp: mdToken,
        pdwSemanticsFlags: *mut DWORD,
    ) -> HRESULT,
    pub GetClassLayout: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        pdwPackSize: *mut DWORD,
        rFieldOffset: *mut COR_FIELD_OFFSET,
        cMax: ULONG,
        pcFieldOffset: *mut ULONG,
        pulClassSize: *mut ULONG,
    ) -> HRESULT,
    pub GetFieldMarshal: unsafe extern "system" fn(
        this: &T,
        tk: mdToken,
        ppvNativeType: *mut PCCOR_SIGNATURE,
        pcbNativeType: *mut ULONG,
    ) -> HRESULT,
    pub GetRVA: unsafe extern "system" fn(
        this: &T,
        tk: mdToken,
        pulCodeRVA: *mut ULONG,
        pdwImplFlags: *mut DWORD,
    ) -> HRESULT,
    pub GetPermissionSetProps: unsafe extern "system" fn(
        this: &T,
        pm: mdPermission,
        pdwAction: *mut DWORD,
        ppvPermission: *mut *mut c_void,
        pcbPermission: *mut ULONG,
    ) -> HRESULT,
    pub GetSigFromToken: unsafe extern "system" fn(
        this: &T,
        mdSig: mdSignature,
        ppvSig: *mut PCCOR_SIGNATURE,
        pcbSig: *mut ULONG,
    ) -> HRESULT,
    pub GetModuleRefProps: unsafe extern "system" fn(
        this: &T,
        mur: mdModuleRef,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
    ) -> HRESULT,
    pub EnumModuleRefs: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rModuleRefs: *mut mdModuleRef,
        cmax: ULONG,
        pcModuleRefs: *mut ULONG,
    ) -> HRESULT,
    pub GetTypeSpecFromToken: unsafe extern "system" fn(
        this: &T,
        typespec: mdTypeSpec,
        ppvSig: *mut PCCOR_SIGNATURE,
        pcbSig: *mut ULONG,
    ) -> HRESULT,
    pub GetNameFromToken: unsafe extern "system" fn(
        this: &T,
        tk: mdToken,
        pszUtf8NamePtr: *mut MDUTF8CSTR,
    ) -> HRESULT,
    pub EnumUnresolvedMethods: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rMethods: *mut mdToken,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub GetUserString: unsafe extern "system" fn(
        this: &T,
        stk: mdString,
        szString: *mut WCHAR,
        cchString: ULONG,
        pchString: *mut ULONG,
    ) -> HRESULT,
    pub GetPinvokeMap: unsafe extern "system" fn(
        this: &T,
        tk: mdToken,
        pdwMappingFlags: *mut DWORD,
        szImportName: *mut WCHAR,
        cchImportName: ULONG,
        pchImportName: *mut ULONG,
        pmrImportDLL: *mut mdModuleRef,
    ) -> HRESULT,
    pub EnumSignatures: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rSignatures: *mut mdSignature,
        cMax: ULONG,
        pcSignatures: *mut ULONG,
    ) -> HRESULT,
    pub EnumTypeSpecs: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rTypeSpecs: *mut mdTypeSpec,
        cMax: ULONG,
        pcTypeSpecs: *mut ULONG,
    ) -> HRESULT,
    pub EnumUserStrings: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rStrings: *mut mdString,
        cMax: ULONG,
        pcStrings: *mut ULONG,
    ) -> HRESULT,
    pub GetParamForMethodIndex: unsafe extern "system" fn(
        this: &T,
        md: mdMethodDef,
        ulParamSeq: ULONG,
        ppd: *mut mdParamDef,
    ) -> HRESULT,
    pub EnumCustomAttributes: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        tk: mdToken,
        tkType: mdToken,
        rCustomAttributes: *mut mdCustomAttribute,
        cMax: ULONG,
        pcCustomAttributes: *mut ULONG,
    ) -> HRESULT,
    pub GetCustomAttributeProps: unsafe extern "system" fn(
        this: &T,
        cv: mdCustomAttribute,
        ptkObj: *mut mdToken,
        ptkType: *mut mdToken,
        ppBlob: *mut *mut c_void,
        pcbSize: *mut ULONG,
    ) -> HRESULT,
    pub FindTypeRef: unsafe extern "system" fn(
        this: &T,
        tkResolutionScope: mdToken,
        szName: LPCWSTR,
        ptr: *mut mdTypeRef,
    ) -> HRESULT,
    pub GetMemberProps: unsafe extern "system" fn(
        this: &T,
        mb: mdToken,
        pClass: *mut mdTypeDef,
        szMember: *mut WCHAR,
        cchMember: ULONG,
        pchMember: *mut ULONG,
        pdwAttr: *mut DWORD,
        ppvSigBlob: *mut PCCOR_SIGNATURE,
        pcbSigBlob: *mut ULONG,
        pulCodeRVA: *mut ULONG,
        pdwImplFlags: *mut DWORD,
        pdwCPlusTypeFlag: *mut DWORD,
        ppValue: *mut UVCP_CONSTANT,
        pcchValue: *mut ULONG,
    ) -> HRESULT,
    pub GetFieldProps: unsafe extern "system" fn(
        this: &T,
        mb: mdToken,
        pClass: *mut mdTypeDef,
        szField: *mut WCHAR,
        cchField: ULONG,
        pchField: *mut ULONG,
        pdwAttr: *mut DWORD,
        ppvSigBlob: *mut PCCOR_SIGNATURE,
        pcbSigBlob: *mut ULONG,
        pdwCPlusTypeFlag: *mut DWORD,
        ppValue: *mut UVCP_CONSTANT,
        pcchValue: *mut ULONG,
    ) -> HRESULT,
    pub GetPropertyProps: unsafe extern "system" fn(
        this: &T,
        prop: mdProperty,
        pClass: *mut mdTypeDef,
        szProperty: *mut WCHAR,
        cchProperty: ULONG,
        pchProperty: *mut ULONG,
        pdwPropFlags: *mut DWORD,
        ppvSig: *mut PCCOR_SIGNATURE,
        pbSig: *mut ULONG,
        pdwCPlusTypeFlag: *mut DWORD,
        ppDefaultValue: *mut UVCP_CONSTANT,
        pcchDefaultValue: *mut ULONG,
        pmdSetter: *mut mdMethodDef,
        pmdGetter: *mut mdMethodDef,
        rmdOtherMethod: *mut mdMethodDef,
        cMax: ULONG,
        pcOtherMethod: *mut ULONG,
    ) -> HRESULT,
    pub GetParamProps: unsafe extern "system" fn(
        this: &T,
        tk: mdParamDef,
        pmd: *mut mdMethodDef,
        pulSequence: *mut ULONG,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        pdwAttr: *mut DWORD,
        pdwCPlusTypeFlag: *mut DWORD,
        ppValue: *mut UVCP_CONSTANT,
        pcchValue: *mut ULONG,
    ) -> HRESULT,
    pub GetCustomAttributeByName: unsafe extern "system" fn(
        this: &T,
        tkObj: mdToken,
        szName: LPCWSTR,
        ppData: *mut *mut c_void,
        pcbData: *mut ULONG,
    ) -> HRESULT,
    pub IsValidToken: unsafe extern "system" fn(this: &T, tk: mdToken) -> BOOL,
    pub GetNestedClassProps: unsafe extern "system" fn(
        this: &T,
        tdNestedClass: mdTypeDef,
        ptdEnclosingClass: *mut mdTypeDef,
    ) -> HRESULT,
    pub GetNativeCallConvFromSig: unsafe extern "system" fn(
        this: &T,
        pvSig: *const c_void,
        cbSig: ULONG,
        pCallConv: *mut ULONG,
    ) -> HRESULT,
    pub IsGlobal: unsafe extern "system" fn(this: &T, pd: mdToken, pbGlobal: *mut int) -> HRESULT,
}

impl IMetaDataImport<()> {
    // 7DAC8207-D3AE-4C75-9B67-92801A497D44
    pub const IID: GUID = GUID {
        data1: 0x7DAC8207,
        data2: 0xD3AE,
        data3: 0x4C75,
        data4: [0x9B, 0x67, 0x92, 0x80, 0x1A, 0x49, 0x7D, 0x44],
    };
}
