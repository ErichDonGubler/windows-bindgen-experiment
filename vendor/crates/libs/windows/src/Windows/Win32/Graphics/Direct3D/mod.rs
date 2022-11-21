#[cfg(feature = "Win32_Graphics_Direct3D_Dxc")]
pub mod Dxc;
#[cfg(feature = "Win32_Graphics_Direct3D_Fxc")]
pub mod Fxc;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
pub struct ID3DBlob(::windows::core::IUnknown);
impl ID3DBlob {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        (::windows::core::Vtable::vtable(self).GetBufferPointer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows::core::Vtable::vtable(self).GetBufferSize)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3DBlob, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3DBlob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3DBlob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DBlob {}
impl ::core::fmt::Debug for ID3DBlob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DBlob").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3DBlob {}
unsafe impl ::core::marker::Sync for ID3DBlob {}
unsafe impl ::windows::core::Vtable for ID3DBlob {
    type Vtable = ID3DBlob_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3DBlob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ba5fb08_5195_40e2_ac58_0d989c3a0102);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3DBlob_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBufferPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
pub struct ID3DDestructionNotifier(::windows::core::IUnknown);
impl ID3DDestructionNotifier {
    pub unsafe fn RegisterDestructionCallback(&self, callbackfn: PFN_DESTRUCTION_CALLBACK, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterDestructionCallback)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(callbackfn), ::core::mem::transmute(pdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterDestructionCallback(&self, callbackid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterDestructionCallback)(::windows::core::Vtable::as_raw(self), callbackid).ok()
    }
}
::windows::core::interface_hierarchy!(ID3DDestructionNotifier, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3DDestructionNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3DDestructionNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DDestructionNotifier {}
impl ::core::fmt::Debug for ID3DDestructionNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DDestructionNotifier").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3DDestructionNotifier {}
unsafe impl ::core::marker::Sync for ID3DDestructionNotifier {}
unsafe impl ::windows::core::Vtable for ID3DDestructionNotifier {
    type Vtable = ID3DDestructionNotifier_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3DDestructionNotifier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa06eb39a_50da_425b_8c31_4eecd6c270f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3DDestructionNotifier_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterDestructionCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callbackfn: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pcallbackid: *mut u32) -> ::windows::core::HRESULT,
    pub UnregisterDestructionCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callbackid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
pub struct ID3DInclude(::std::ptr::NonNull<::std::ffi::c_void>);
impl ID3DInclude {
    pub unsafe fn Open<'a, P0>(&self, includetype: D3D_INCLUDE_TYPE, pfilename: P0, pparentdata: *const ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Vtable::vtable(self).Open)(::windows::core::Vtable::as_raw(self), includetype, pfilename.into(), ::core::mem::transmute(pparentdata), ::core::mem::transmute(ppdata), ::core::mem::transmute(pbytes)).ok()
    }
    pub unsafe fn Close(&self, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
}
impl ::core::clone::Clone for ID3DInclude {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3DInclude {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DInclude {}
impl ::core::fmt::Debug for ID3DInclude {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DInclude").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3DInclude {}
unsafe impl ::core::marker::Sync for ID3DInclude {}
unsafe impl ::windows::core::Vtable for ID3DInclude {
    type Vtable = ID3DInclude_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3DInclude_Vtbl {
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includetype: D3D_INCLUDE_TYPE, pfilename: ::windows::core::PCSTR, pparentdata: *const ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_COMPONENT_MASK_W: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_COMPONENT_MASK_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_COMPONENT_MASK_Y: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_COMPONENT_MASK_Z: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_1_DEFAULT_MAX_ANISOTROPY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_1_IA_PRIMITIVE_MAX_COUNT: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_1_MAX_TEXTURE_REPEAT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_1_REQ_TEXTURE1D_U_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_1_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_1_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_1_REQ_TEXTURECUBE_DIMENSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_1_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_2_IA_PRIMITIVE_MAX_COUNT: u32 = 1048575u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_2_MAX_TEXTURE_REPEAT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_3_MAX_TEXTURE_REPEAT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_3_REQ_TEXTURE1D_U_DIMENSION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_3_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_3_REQ_TEXTURECUBE_DIMENSION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FL9_3_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_11_1_DOUBLE_EXTENSIONS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_11_1_SHADER_EXTENSIONS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_64_UAVS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_ATOMIC_INT64_ON_DESCRIPTOR_HEAP_RESOURCE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_ATOMIC_INT64_ON_GROUP_SHARED: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_ATOMIC_INT64_ON_TYPED_RESOURCE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_BARYCENTRICS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_COMPUTE_SHADERS_PLUS_RAW_AND_STRUCTURED_BUFFERS_VIA_SHADER_4_X: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_DERIVATIVES_IN_MESH_AND_AMPLIFICATION_SHADERS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_DOUBLES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_INNER_COVERAGE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_INT64_OPS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_LEVEL_9_COMPARISON_FILTERING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_MINIMUM_PRECISION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_NATIVE_16BIT_OPS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_RAYTRACING_TIER_1_1: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_RESOURCE_DESCRIPTOR_HEAP_INDEXING: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_ROVS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_SAMPLER_DESCRIPTOR_HEAP_INDEXING: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_SAMPLER_FEEDBACK: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_SHADING_RATE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_STENCIL_REF: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_TILED_RESOURCES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_TYPED_UAV_LOAD_ADDITIONAL_FORMATS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_UAVS_AT_EVERY_STAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_VIEWPORT_AND_RT_ARRAY_INDEX_FROM_ANY_SHADER_FEEDING_RASTERIZER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_VIEW_ID: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_WAVE_MMA: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SHADER_FEATURE_WAVE_OPS: u32 = 16384u32;
pub const D3D_TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c0f29e3_3f5f_4d35_84c9_bc0983b62c28);
pub const D3D_TEXTURE_LAYOUT_ROW_MAJOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5dc234f_72bb_4bec_9705_8cf258df6b6c);
pub const WKPDID_CommentStringW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0149dc0_90e8_4ec8_8144_e900ad266bb2);
pub const WKPDID_D3D12UniqueObjectId: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b39de15_ec04_4bae_ba4d_8cef79fc04c1);
pub const WKPDID_D3DDebugObjectName: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x429b8c22_9188_4b0c_8742_acb0bf85c200);
pub const WKPDID_D3DDebugObjectNameW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cca5fd8_921f_42c8_8566_70caf2a9b741);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_CBUFFER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_CT_CBUFFER: D3D_CBUFFER_TYPE = D3D_CBUFFER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_CT_TBUFFER: D3D_CBUFFER_TYPE = D3D_CBUFFER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_CT_INTERFACE_POINTERS: D3D_CBUFFER_TYPE = D3D_CBUFFER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_CT_RESOURCE_BIND_INFO: D3D_CBUFFER_TYPE = D3D_CBUFFER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_CT_CBUFFER: D3D_CBUFFER_TYPE = D3D_CBUFFER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_CT_TBUFFER: D3D_CBUFFER_TYPE = D3D_CBUFFER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_CT_CBUFFER: D3D_CBUFFER_TYPE = D3D_CBUFFER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_CT_TBUFFER: D3D_CBUFFER_TYPE = D3D_CBUFFER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_CT_INTERFACE_POINTERS: D3D_CBUFFER_TYPE = D3D_CBUFFER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_CT_RESOURCE_BIND_INFO: D3D_CBUFFER_TYPE = D3D_CBUFFER_TYPE(3i32);
impl ::core::marker::Copy for D3D_CBUFFER_TYPE {}
impl ::core::clone::Clone for D3D_CBUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_CBUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_CBUFFER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_CBUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_CBUFFER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_DRIVER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_DRIVER_TYPE_UNKNOWN: D3D_DRIVER_TYPE = D3D_DRIVER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_DRIVER_TYPE_HARDWARE: D3D_DRIVER_TYPE = D3D_DRIVER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_DRIVER_TYPE_REFERENCE: D3D_DRIVER_TYPE = D3D_DRIVER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_DRIVER_TYPE_NULL: D3D_DRIVER_TYPE = D3D_DRIVER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_DRIVER_TYPE_SOFTWARE: D3D_DRIVER_TYPE = D3D_DRIVER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_DRIVER_TYPE_WARP: D3D_DRIVER_TYPE = D3D_DRIVER_TYPE(5i32);
impl ::core::marker::Copy for D3D_DRIVER_TYPE {}
impl ::core::clone::Clone for D3D_DRIVER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_DRIVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_DRIVER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_DRIVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_DRIVER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_FEATURE_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_1_0_CORE: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(4096i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_9_1: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(37120i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_9_2: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(37376i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_9_3: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(37632i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_10_0: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(40960i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_10_1: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(41216i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_11_0: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(45056i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_11_1: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(45312i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_12_0: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(49152i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_12_1: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(49408i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_FEATURE_LEVEL_12_2: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(49664i32);
impl ::core::marker::Copy for D3D_FEATURE_LEVEL {}
impl ::core::clone::Clone for D3D_FEATURE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_FEATURE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_FEATURE_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_FEATURE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_FEATURE_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_INCLUDE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INCLUDE_LOCAL: D3D_INCLUDE_TYPE = D3D_INCLUDE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INCLUDE_SYSTEM: D3D_INCLUDE_TYPE = D3D_INCLUDE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_INCLUDE_LOCAL: D3D_INCLUDE_TYPE = D3D_INCLUDE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_INCLUDE_SYSTEM: D3D_INCLUDE_TYPE = D3D_INCLUDE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INCLUDE_FORCE_DWORD: D3D_INCLUDE_TYPE = D3D_INCLUDE_TYPE(2147483647i32);
impl ::core::marker::Copy for D3D_INCLUDE_TYPE {}
impl ::core::clone::Clone for D3D_INCLUDE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_INCLUDE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_INCLUDE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_INCLUDE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_INCLUDE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_INTERPOLATION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INTERPOLATION_UNDEFINED: D3D_INTERPOLATION_MODE = D3D_INTERPOLATION_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INTERPOLATION_CONSTANT: D3D_INTERPOLATION_MODE = D3D_INTERPOLATION_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INTERPOLATION_LINEAR: D3D_INTERPOLATION_MODE = D3D_INTERPOLATION_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INTERPOLATION_LINEAR_CENTROID: D3D_INTERPOLATION_MODE = D3D_INTERPOLATION_MODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE: D3D_INTERPOLATION_MODE = D3D_INTERPOLATION_MODE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_CENTROID: D3D_INTERPOLATION_MODE = D3D_INTERPOLATION_MODE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INTERPOLATION_LINEAR_SAMPLE: D3D_INTERPOLATION_MODE = D3D_INTERPOLATION_MODE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_SAMPLE: D3D_INTERPOLATION_MODE = D3D_INTERPOLATION_MODE(7i32);
impl ::core::marker::Copy for D3D_INTERPOLATION_MODE {}
impl ::core::clone::Clone for D3D_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_INTERPOLATION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_MIN_PRECISION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_MIN_PRECISION_DEFAULT: D3D_MIN_PRECISION = D3D_MIN_PRECISION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_MIN_PRECISION_FLOAT_16: D3D_MIN_PRECISION = D3D_MIN_PRECISION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_MIN_PRECISION_FLOAT_2_8: D3D_MIN_PRECISION = D3D_MIN_PRECISION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_MIN_PRECISION_RESERVED: D3D_MIN_PRECISION = D3D_MIN_PRECISION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_MIN_PRECISION_SINT_16: D3D_MIN_PRECISION = D3D_MIN_PRECISION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_MIN_PRECISION_UINT_16: D3D_MIN_PRECISION = D3D_MIN_PRECISION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_MIN_PRECISION_ANY_16: D3D_MIN_PRECISION = D3D_MIN_PRECISION(240i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_MIN_PRECISION_ANY_10: D3D_MIN_PRECISION = D3D_MIN_PRECISION(241i32);
impl ::core::marker::Copy for D3D_MIN_PRECISION {}
impl ::core::clone::Clone for D3D_MIN_PRECISION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_MIN_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_MIN_PRECISION {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_MIN_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_MIN_PRECISION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_NAME(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_UNDEFINED: D3D_NAME = D3D_NAME(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_POSITION: D3D_NAME = D3D_NAME(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_CLIP_DISTANCE: D3D_NAME = D3D_NAME(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_CULL_DISTANCE: D3D_NAME = D3D_NAME(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_RENDER_TARGET_ARRAY_INDEX: D3D_NAME = D3D_NAME(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_VIEWPORT_ARRAY_INDEX: D3D_NAME = D3D_NAME(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_VERTEX_ID: D3D_NAME = D3D_NAME(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_PRIMITIVE_ID: D3D_NAME = D3D_NAME(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_INSTANCE_ID: D3D_NAME = D3D_NAME(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_IS_FRONT_FACE: D3D_NAME = D3D_NAME(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_SAMPLE_INDEX: D3D_NAME = D3D_NAME(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_FINAL_QUAD_EDGE_TESSFACTOR: D3D_NAME = D3D_NAME(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_FINAL_QUAD_INSIDE_TESSFACTOR: D3D_NAME = D3D_NAME(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_FINAL_TRI_EDGE_TESSFACTOR: D3D_NAME = D3D_NAME(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_FINAL_TRI_INSIDE_TESSFACTOR: D3D_NAME = D3D_NAME(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_FINAL_LINE_DETAIL_TESSFACTOR: D3D_NAME = D3D_NAME(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_FINAL_LINE_DENSITY_TESSFACTOR: D3D_NAME = D3D_NAME(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_BARYCENTRICS: D3D_NAME = D3D_NAME(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_SHADINGRATE: D3D_NAME = D3D_NAME(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_CULLPRIMITIVE: D3D_NAME = D3D_NAME(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_TARGET: D3D_NAME = D3D_NAME(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_DEPTH: D3D_NAME = D3D_NAME(65i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_COVERAGE: D3D_NAME = D3D_NAME(66i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_DEPTH_GREATER_EQUAL: D3D_NAME = D3D_NAME(67i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_DEPTH_LESS_EQUAL: D3D_NAME = D3D_NAME(68i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_STENCIL_REF: D3D_NAME = D3D_NAME(69i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_NAME_INNER_COVERAGE: D3D_NAME = D3D_NAME(70i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_UNDEFINED: D3D_NAME = D3D_NAME(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_POSITION: D3D_NAME = D3D_NAME(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_CLIP_DISTANCE: D3D_NAME = D3D_NAME(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_CULL_DISTANCE: D3D_NAME = D3D_NAME(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_RENDER_TARGET_ARRAY_INDEX: D3D_NAME = D3D_NAME(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_VIEWPORT_ARRAY_INDEX: D3D_NAME = D3D_NAME(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_VERTEX_ID: D3D_NAME = D3D_NAME(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_PRIMITIVE_ID: D3D_NAME = D3D_NAME(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_INSTANCE_ID: D3D_NAME = D3D_NAME(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_IS_FRONT_FACE: D3D_NAME = D3D_NAME(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_SAMPLE_INDEX: D3D_NAME = D3D_NAME(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_TARGET: D3D_NAME = D3D_NAME(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_DEPTH: D3D_NAME = D3D_NAME(65i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_NAME_COVERAGE: D3D_NAME = D3D_NAME(66i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_NAME_FINAL_QUAD_EDGE_TESSFACTOR: D3D_NAME = D3D_NAME(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_NAME_FINAL_QUAD_INSIDE_TESSFACTOR: D3D_NAME = D3D_NAME(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_NAME_FINAL_TRI_EDGE_TESSFACTOR: D3D_NAME = D3D_NAME(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_NAME_FINAL_TRI_INSIDE_TESSFACTOR: D3D_NAME = D3D_NAME(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_NAME_FINAL_LINE_DETAIL_TESSFACTOR: D3D_NAME = D3D_NAME(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_NAME_FINAL_LINE_DENSITY_TESSFACTOR: D3D_NAME = D3D_NAME(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_NAME_DEPTH_GREATER_EQUAL: D3D_NAME = D3D_NAME(67i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_NAME_DEPTH_LESS_EQUAL: D3D_NAME = D3D_NAME(68i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_NAME_STENCIL_REF: D3D_NAME = D3D_NAME(69i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_NAME_INNER_COVERAGE: D3D_NAME = D3D_NAME(70i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D12_NAME_BARYCENTRICS: D3D_NAME = D3D_NAME(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D12_NAME_SHADINGRATE: D3D_NAME = D3D_NAME(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D12_NAME_CULLPRIMITIVE: D3D_NAME = D3D_NAME(25i32);
impl ::core::marker::Copy for D3D_NAME {}
impl ::core::clone::Clone for D3D_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_NAME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_NAME {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_NAME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_PARAMETER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PF_NONE: D3D_PARAMETER_FLAGS = D3D_PARAMETER_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PF_IN: D3D_PARAMETER_FLAGS = D3D_PARAMETER_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PF_OUT: D3D_PARAMETER_FLAGS = D3D_PARAMETER_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PF_FORCE_DWORD: D3D_PARAMETER_FLAGS = D3D_PARAMETER_FLAGS(2147483647i32);
impl ::core::marker::Copy for D3D_PARAMETER_FLAGS {}
impl ::core::clone::Clone for D3D_PARAMETER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_PARAMETER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_PARAMETER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_PARAMETER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_PARAMETER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_PRIMITIVE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_UNDEFINED: D3D_PRIMITIVE = D3D_PRIMITIVE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_POINT: D3D_PRIMITIVE = D3D_PRIMITIVE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_LINE: D3D_PRIMITIVE = D3D_PRIMITIVE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TRIANGLE: D3D_PRIMITIVE = D3D_PRIMITIVE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_LINE_ADJ: D3D_PRIMITIVE = D3D_PRIMITIVE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TRIANGLE_ADJ: D3D_PRIMITIVE = D3D_PRIMITIVE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_1_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_2_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_3_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_4_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_5_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_6_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_7_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_8_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_9_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_10_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_11_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_12_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_13_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_14_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_15_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_16_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_17_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_18_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_19_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_20_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_21_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_22_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_23_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_24_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_25_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_26_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_27_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_28_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_29_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_30_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_31_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_32_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_UNDEFINED: D3D_PRIMITIVE = D3D_PRIMITIVE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_POINT: D3D_PRIMITIVE = D3D_PRIMITIVE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_LINE: D3D_PRIMITIVE = D3D_PRIMITIVE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TRIANGLE: D3D_PRIMITIVE = D3D_PRIMITIVE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_LINE_ADJ: D3D_PRIMITIVE = D3D_PRIMITIVE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TRIANGLE_ADJ: D3D_PRIMITIVE = D3D_PRIMITIVE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_UNDEFINED: D3D_PRIMITIVE = D3D_PRIMITIVE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_POINT: D3D_PRIMITIVE = D3D_PRIMITIVE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_LINE: D3D_PRIMITIVE = D3D_PRIMITIVE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TRIANGLE: D3D_PRIMITIVE = D3D_PRIMITIVE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_LINE_ADJ: D3D_PRIMITIVE = D3D_PRIMITIVE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TRIANGLE_ADJ: D3D_PRIMITIVE = D3D_PRIMITIVE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_1_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_2_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_3_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_4_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_5_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_6_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_7_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_8_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_9_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_10_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_11_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_12_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_13_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_14_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_15_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_16_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_17_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_18_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_19_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_20_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_21_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_22_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_23_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_24_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_25_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_26_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_27_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_28_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_29_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_30_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_31_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_32_CONTROL_POINT_PATCH: D3D_PRIMITIVE = D3D_PRIMITIVE(39i32);
impl ::core::marker::Copy for D3D_PRIMITIVE {}
impl ::core::clone::Clone for D3D_PRIMITIVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_PRIMITIVE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_PRIMITIVE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_PRIMITIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_PRIMITIVE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_PRIMITIVE_TOPOLOGY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_UNDEFINED: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_POINTLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_LINELIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_LINESTRIP: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(43i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(44i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(45i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(46i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(47i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(49i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(50i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(51i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(52i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(53i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(54i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(55i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(56i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(57i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(58i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(59i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(60i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(61i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(62i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(63i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TOPOLOGY_UNDEFINED: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TOPOLOGY_POINTLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TOPOLOGY_LINELIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TOPOLOGY_LINESTRIP: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLELIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TOPOLOGY_LINELIST_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_UNDEFINED: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_POINTLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_LINELIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_LINELIST_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(43i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(44i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(45i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(46i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(47i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(49i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(50i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(51i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(52i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(53i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(54i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(55i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(56i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(57i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(58i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(59i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(60i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(61i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(62i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(63i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY = D3D_PRIMITIVE_TOPOLOGY(64i32);
impl ::core::marker::Copy for D3D_PRIMITIVE_TOPOLOGY {}
impl ::core::clone::Clone for D3D_PRIMITIVE_TOPOLOGY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_PRIMITIVE_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_PRIMITIVE_TOPOLOGY {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_PRIMITIVE_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_PRIMITIVE_TOPOLOGY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_REGISTER_COMPONENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_REGISTER_COMPONENT_UNKNOWN: D3D_REGISTER_COMPONENT_TYPE = D3D_REGISTER_COMPONENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_REGISTER_COMPONENT_UINT32: D3D_REGISTER_COMPONENT_TYPE = D3D_REGISTER_COMPONENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_REGISTER_COMPONENT_SINT32: D3D_REGISTER_COMPONENT_TYPE = D3D_REGISTER_COMPONENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_REGISTER_COMPONENT_FLOAT32: D3D_REGISTER_COMPONENT_TYPE = D3D_REGISTER_COMPONENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_REGISTER_COMPONENT_UNKNOWN: D3D_REGISTER_COMPONENT_TYPE = D3D_REGISTER_COMPONENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_REGISTER_COMPONENT_UINT32: D3D_REGISTER_COMPONENT_TYPE = D3D_REGISTER_COMPONENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_REGISTER_COMPONENT_SINT32: D3D_REGISTER_COMPONENT_TYPE = D3D_REGISTER_COMPONENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_REGISTER_COMPONENT_FLOAT32: D3D_REGISTER_COMPONENT_TYPE = D3D_REGISTER_COMPONENT_TYPE(3i32);
impl ::core::marker::Copy for D3D_REGISTER_COMPONENT_TYPE {}
impl ::core::clone::Clone for D3D_REGISTER_COMPONENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_REGISTER_COMPONENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_REGISTER_COMPONENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_REGISTER_COMPONENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_REGISTER_COMPONENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_RESOURCE_RETURN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_RETURN_TYPE_UNORM: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_RETURN_TYPE_SNORM: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_RETURN_TYPE_SINT: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_RETURN_TYPE_UINT: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_RETURN_TYPE_FLOAT: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_RETURN_TYPE_MIXED: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_RETURN_TYPE_DOUBLE: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_RETURN_TYPE_CONTINUED: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_RETURN_TYPE_UNORM: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_RETURN_TYPE_SNORM: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_RETURN_TYPE_SINT: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_RETURN_TYPE_UINT: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_RETURN_TYPE_FLOAT: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_RETURN_TYPE_MIXED: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_RETURN_TYPE_UNORM: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_RETURN_TYPE_SNORM: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_RETURN_TYPE_SINT: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_RETURN_TYPE_UINT: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_RETURN_TYPE_FLOAT: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_RETURN_TYPE_MIXED: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_RETURN_TYPE_DOUBLE: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_RETURN_TYPE_CONTINUED: D3D_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE(8i32);
impl ::core::marker::Copy for D3D_RESOURCE_RETURN_TYPE {}
impl ::core::clone::Clone for D3D_RESOURCE_RETURN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_RESOURCE_RETURN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_RESOURCE_RETURN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_RESOURCE_RETURN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_RESOURCE_RETURN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_SHADER_CBUFFER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_CBF_USERPACKED: D3D_SHADER_CBUFFER_FLAGS = D3D_SHADER_CBUFFER_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_CBF_USERPACKED: D3D_SHADER_CBUFFER_FLAGS = D3D_SHADER_CBUFFER_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_CBF_FORCE_DWORD: D3D_SHADER_CBUFFER_FLAGS = D3D_SHADER_CBUFFER_FLAGS(2147483647i32);
impl ::core::marker::Copy for D3D_SHADER_CBUFFER_FLAGS {}
impl ::core::clone::Clone for D3D_SHADER_CBUFFER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_SHADER_CBUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_SHADER_CBUFFER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_SHADER_CBUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_CBUFFER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_SHADER_INPUT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIF_USERPACKED: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIF_COMPARISON_SAMPLER: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIF_TEXTURE_COMPONENT_0: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIF_TEXTURE_COMPONENT_1: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIF_TEXTURE_COMPONENTS: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIF_UNUSED: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SIF_USERPACKED: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SIF_COMPARISON_SAMPLER: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SIF_TEXTURE_COMPONENT_0: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SIF_TEXTURE_COMPONENT_1: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SIF_TEXTURE_COMPONENTS: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIF_FORCE_DWORD: D3D_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS(2147483647i32);
impl ::core::marker::Copy for D3D_SHADER_INPUT_FLAGS {}
impl ::core::clone::Clone for D3D_SHADER_INPUT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_SHADER_INPUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_SHADER_INPUT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_SHADER_INPUT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_INPUT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_SHADER_INPUT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_CBUFFER: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_TBUFFER: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_TEXTURE: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_SAMPLER: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_UAV_RWTYPED: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_STRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_UAV_RWSTRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_BYTEADDRESS: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_UAV_RWBYTEADDRESS: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_UAV_APPEND_STRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_UAV_CONSUME_STRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_RTACCELERATIONSTRUCTURE: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SIT_UAV_FEEDBACKTEXTURE: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SIT_CBUFFER: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SIT_TBUFFER: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SIT_TEXTURE: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SIT_SAMPLER: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SIT_UAV_RWTYPED: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SIT_STRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SIT_UAV_RWSTRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SIT_BYTEADDRESS: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SIT_UAV_RWBYTEADDRESS: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SIT_UAV_APPEND_STRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SIT_UAV_CONSUME_STRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SIT_UAV_RWSTRUCTURED_WITH_COUNTER: D3D_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE(11i32);
impl ::core::marker::Copy for D3D_SHADER_INPUT_TYPE {}
impl ::core::clone::Clone for D3D_SHADER_INPUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_SHADER_INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_SHADER_INPUT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_SHADER_INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_INPUT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_SHADER_VARIABLE_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVC_SCALAR: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVC_VECTOR: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVC_MATRIX_ROWS: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVC_MATRIX_COLUMNS: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVC_OBJECT: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVC_STRUCT: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVC_INTERFACE_CLASS: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVC_INTERFACE_POINTER: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVC_SCALAR: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVC_VECTOR: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVC_MATRIX_ROWS: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVC_MATRIX_COLUMNS: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVC_OBJECT: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVC_STRUCT: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVC_INTERFACE_CLASS: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVC_INTERFACE_POINTER: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVC_FORCE_DWORD: D3D_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS(2147483647i32);
impl ::core::marker::Copy for D3D_SHADER_VARIABLE_CLASS {}
impl ::core::clone::Clone for D3D_SHADER_VARIABLE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_SHADER_VARIABLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_SHADER_VARIABLE_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_SHADER_VARIABLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_VARIABLE_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_SHADER_VARIABLE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVF_USERPACKED: D3D_SHADER_VARIABLE_FLAGS = D3D_SHADER_VARIABLE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVF_USED: D3D_SHADER_VARIABLE_FLAGS = D3D_SHADER_VARIABLE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVF_INTERFACE_POINTER: D3D_SHADER_VARIABLE_FLAGS = D3D_SHADER_VARIABLE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVF_INTERFACE_PARAMETER: D3D_SHADER_VARIABLE_FLAGS = D3D_SHADER_VARIABLE_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVF_USERPACKED: D3D_SHADER_VARIABLE_FLAGS = D3D_SHADER_VARIABLE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVF_USED: D3D_SHADER_VARIABLE_FLAGS = D3D_SHADER_VARIABLE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVF_INTERFACE_POINTER: D3D_SHADER_VARIABLE_FLAGS = D3D_SHADER_VARIABLE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVF_INTERFACE_PARAMETER: D3D_SHADER_VARIABLE_FLAGS = D3D_SHADER_VARIABLE_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVF_FORCE_DWORD: D3D_SHADER_VARIABLE_FLAGS = D3D_SHADER_VARIABLE_FLAGS(2147483647i32);
impl ::core::marker::Copy for D3D_SHADER_VARIABLE_FLAGS {}
impl ::core::clone::Clone for D3D_SHADER_VARIABLE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_SHADER_VARIABLE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_SHADER_VARIABLE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_SHADER_VARIABLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_VARIABLE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_SHADER_VARIABLE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_VOID: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_BOOL: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_INT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_FLOAT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_STRING: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TEXTURE: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TEXTURE1D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TEXTURE2D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TEXTURE3D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TEXTURECUBE: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_SAMPLER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_SAMPLER1D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_SAMPLER2D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_SAMPLER3D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_SAMPLERCUBE: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_PIXELSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_VERTEXSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_PIXELFRAGMENT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_VERTEXFRAGMENT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_UINT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_UINT8: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_GEOMETRYSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_RASTERIZER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_DEPTHSTENCIL: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_BLEND: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_CBUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TBUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TEXTURE1DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TEXTURE2DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_RENDERTARGETVIEW: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_DEPTHSTENCILVIEW: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TEXTURE2DMS: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TEXTURE2DMSARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_TEXTURECUBEARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_HULLSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_DOMAINSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_INTERFACE_POINTER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_COMPUTESHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_DOUBLE: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_RWTEXTURE1D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_RWTEXTURE1DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_RWTEXTURE2D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_RWTEXTURE2DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(43i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_RWTEXTURE3D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(44i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_RWBUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(45i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_BYTEADDRESS_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(46i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_RWBYTEADDRESS_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(47i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_STRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_RWSTRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(49i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_APPEND_STRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(50i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_CONSUME_STRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(51i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_MIN8FLOAT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(52i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_MIN10FLOAT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(53i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_MIN16FLOAT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(54i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_MIN12INT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(55i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_MIN16INT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(56i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_MIN16UINT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(57i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_INT16: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(58i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_UINT16: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(59i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_FLOAT16: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(60i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_INT64: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(61i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_UINT64: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(62i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_VOID: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_BOOL: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_INT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_FLOAT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_STRING: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TEXTURE: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TEXTURE1D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TEXTURE2D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TEXTURE3D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TEXTURECUBE: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_SAMPLER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_SAMPLER1D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_SAMPLER2D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_SAMPLER3D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_SAMPLERCUBE: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_PIXELSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_VERTEXSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_PIXELFRAGMENT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_VERTEXFRAGMENT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_UINT: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_UINT8: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_GEOMETRYSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_RASTERIZER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_DEPTHSTENCIL: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_BLEND: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_CBUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TBUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TEXTURE1DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TEXTURE2DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_RENDERTARGETVIEW: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_DEPTHSTENCILVIEW: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TEXTURE2DMS: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TEXTURE2DMSARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SVT_TEXTURECUBEARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_HULLSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_DOMAINSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_INTERFACE_POINTER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_COMPUTESHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_DOUBLE: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_RWTEXTURE1D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_RWTEXTURE1DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_RWTEXTURE2D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_RWTEXTURE2DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(43i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_RWTEXTURE3D: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(44i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_RWBUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(45i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_BYTEADDRESS_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(46i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_RWBYTEADDRESS_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(47i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_STRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_RWSTRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(49i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_APPEND_STRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(50i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SVT_CONSUME_STRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(51i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SVT_FORCE_DWORD: D3D_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE(2147483647i32);
impl ::core::marker::Copy for D3D_SHADER_VARIABLE_TYPE {}
impl ::core::clone::Clone for D3D_SHADER_VARIABLE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_SHADER_VARIABLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_SHADER_VARIABLE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_SHADER_VARIABLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_VARIABLE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_SRV_DIMENSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_UNKNOWN: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_BUFFER: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_TEXTURE1D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_TEXTURE1DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_TEXTURE2D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_TEXTURE2DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_TEXTURE2DMS: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_TEXTURE3D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_TEXTURECUBE: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_TEXTURECUBEARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_SRV_DIMENSION_BUFFEREX: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SRV_DIMENSION_UNKNOWN: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SRV_DIMENSION_BUFFER: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SRV_DIMENSION_TEXTURE1D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SRV_DIMENSION_TEXTURE1DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SRV_DIMENSION_TEXTURE2D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SRV_DIMENSION_TEXTURE2DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SRV_DIMENSION_TEXTURE2DMS: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SRV_DIMENSION_TEXTURE3D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_SRV_DIMENSION_TEXTURECUBE: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_UNKNOWN: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_BUFFER: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_TEXTURE1D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_TEXTURE1DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_TEXTURE2D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_TEXTURE2DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_TEXTURE2DMS: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_TEXTURE3D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_TEXTURECUBE: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D10_1_SRV_DIMENSION_TEXTURECUBEARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_UNKNOWN: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_BUFFER: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_TEXTURE1D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_TEXTURE1DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_TEXTURE2D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_TEXTURE2DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_TEXTURE2DMS: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_TEXTURE3D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_TEXTURECUBE: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_TEXTURECUBEARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_SRV_DIMENSION_BUFFEREX: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION(11i32);
impl ::core::marker::Copy for D3D_SRV_DIMENSION {}
impl ::core::clone::Clone for D3D_SRV_DIMENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_SRV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_SRV_DIMENSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_SRV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SRV_DIMENSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_TESSELLATOR_DOMAIN(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_DOMAIN_UNDEFINED: D3D_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_DOMAIN_ISOLINE: D3D_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_DOMAIN_TRI: D3D_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_DOMAIN_QUAD: D3D_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_DOMAIN_UNDEFINED: D3D_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_DOMAIN_ISOLINE: D3D_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_DOMAIN_TRI: D3D_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_DOMAIN_QUAD: D3D_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN(3i32);
impl ::core::marker::Copy for D3D_TESSELLATOR_DOMAIN {}
impl ::core::clone::Clone for D3D_TESSELLATOR_DOMAIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_TESSELLATOR_DOMAIN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_TESSELLATOR_DOMAIN {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_TESSELLATOR_DOMAIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_TESSELLATOR_DOMAIN").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_TESSELLATOR_OUTPUT_PRIMITIVE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_OUTPUT_UNDEFINED: D3D_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_OUTPUT_POINT: D3D_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_OUTPUT_LINE: D3D_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW: D3D_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW: D3D_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_OUTPUT_UNDEFINED: D3D_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_OUTPUT_POINT: D3D_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_OUTPUT_LINE: D3D_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_OUTPUT_TRIANGLE_CW: D3D_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_OUTPUT_TRIANGLE_CCW: D3D_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE(4i32);
impl ::core::marker::Copy for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {}
impl ::core::clone::Clone for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_TESSELLATOR_OUTPUT_PRIMITIVE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_TESSELLATOR_PARTITIONING(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_PARTITIONING_UNDEFINED: D3D_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_PARTITIONING_INTEGER: D3D_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_PARTITIONING_POW2: D3D_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD: D3D_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN: D3D_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_PARTITIONING_UNDEFINED: D3D_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_PARTITIONING_INTEGER: D3D_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_PARTITIONING_POW2: D3D_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD: D3D_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub const D3D11_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN: D3D_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING(4i32);
impl ::core::marker::Copy for D3D_TESSELLATOR_PARTITIONING {}
impl ::core::clone::Clone for D3D_TESSELLATOR_PARTITIONING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_TESSELLATOR_PARTITIONING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_TESSELLATOR_PARTITIONING {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_TESSELLATOR_PARTITIONING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_TESSELLATOR_PARTITIONING").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub struct D3DVECTOR {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ::core::marker::Copy for D3DVECTOR {}
impl ::core::clone::Clone for D3DVECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DVECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVECTOR").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DVECTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DVECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl ::core::cmp::Eq for D3DVECTOR {}
impl ::core::default::Default for D3DVECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub struct D3D_SHADER_MACRO {
    pub Name: ::windows::core::PCSTR,
    pub Definition: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for D3D_SHADER_MACRO {}
impl ::core::clone::Clone for D3D_SHADER_MACRO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D_SHADER_MACRO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D_SHADER_MACRO").field("Name", &self.Name).field("Definition", &self.Definition).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D_SHADER_MACRO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D_SHADER_MACRO {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Definition == other.Definition
    }
}
impl ::core::cmp::Eq for D3D_SHADER_MACRO {}
impl ::core::default::Default for D3D_SHADER_MACRO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
pub type PFN_DESTRUCTION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pdata: *mut ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");