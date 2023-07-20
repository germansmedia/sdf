#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
//#![allow(dead_code)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Opaque { _unused: [u8; 0], }

#[cfg(system="linux")]
mod linux;
#[cfg(system="linux")]
pub use linux::*;

#[cfg(system="windows")]
mod windows;
#[cfg(system="windows")]
pub use windows::*;

#[cfg(system="android")]
mod android;
#[cfg(system="android")]
pub use android::*;

pub type VkBool32 = u32;
pub const VK_FALSE: u32 = 0;
pub const VK_TRUE: u32 = 1;

pub const VK_WHOLE_SIZE: i32 = -1;

pub const VK_KHR_SURFACE_EXTENSION_NAME: &'static [u8; 15usize] = b"VK_KHR_surface\0";
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &'static [u8; 19usize] = b"VK_KHR_xcb_surface\0";
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &'static [u8; 17usize] = b"VK_KHR_swapchain\0";

pub type VkStructureType = ::std::os::raw::c_uint;
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = 0;
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = 1;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = 2;
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = 3;
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = 4;
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = 5;
pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = 8;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: VkStructureType = 9;
pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = 12;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: VkStructureType = 15;
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType = 16;
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType = 18;
pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = 29;
pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: VkStructureType = 30;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType = 32;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: VkStructureType = 33;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType = 34;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: VkStructureType = 35;
pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType = 39;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType = 40;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = 42;
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = 1000001000;
pub const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR: VkStructureType = 1000001001;
pub const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000005000;
//pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO: VkStructureType = 1000207002;

pub type VkResult = ::std::os::raw::c_int;
pub const VK_SUCCESS: VkResult = 0;
pub const VK_NOT_READY: VkResult = 1;
pub const VK_TIMEOUT: VkResult = 2;
pub const VK_EVENT_SET: VkResult = 3;
pub const VK_EVENT_RESET: VkResult = 4;
pub const VK_INCOMPLETE: VkResult = 5;
pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = -1;
pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = -2;
pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = -3;
pub const VK_ERROR_DEVICE_LOST: VkResult = -4;
pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = -5;
pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = -6;
pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = -7;
pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = -8;
pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = -9;
pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = -10;
pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = -11;
pub const VK_ERROR_FRAGMENTED_POOL: VkResult = -12;
pub const VK_ERROR_UNKNOWN: VkResult = -13;
pub const VK_ERROR_OUT_OF_POOL_MEMORY: VkResult = -1000069000;
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE: VkResult = -1000072003;
pub const VK_ERROR_FRAGMENTATION: VkResult = -1000161000;
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: VkResult = -1000257000;
pub const VK_ERROR_SURFACE_LOST_KHR: VkResult = -1000000000;
pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult = -1000000001;
pub const VK_SUBOPTIMAL_KHR: VkResult = 1000001003;
pub const VK_ERROR_OUT_OF_DATE_KHR: VkResult = -1000001004;
pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: VkResult = -1000003001;
pub const VK_ERROR_VALIDATION_FAILED_EXT: VkResult = -1000011001;
pub const VK_ERROR_INVALID_SHADER_NV: VkResult = -1000012000;
pub const VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: VkResult = -1000158000;
pub const VK_ERROR_NOT_PERMITTED_EXT: VkResult = -1000174001;
pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: VkResult = -1000255000;

pub type VkFormat = ::std::os::raw::c_uint;
pub const VK_FORMAT_UNDEFINED: VkFormat = 0;
pub const VK_FORMAT_R4G4_UNORM_PACK8: VkFormat = 1;
pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: VkFormat = 2;
pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: VkFormat = 3;
pub const VK_FORMAT_R5G6B5_UNORM_PACK16: VkFormat = 4;
pub const VK_FORMAT_B5G6R5_UNORM_PACK16: VkFormat = 5;
pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: VkFormat = 6;
pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: VkFormat = 7;
pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: VkFormat = 8;
pub const VK_FORMAT_R8_UNORM: VkFormat = 9;
pub const VK_FORMAT_R8_SNORM: VkFormat = 10;
pub const VK_FORMAT_R8_USCALED: VkFormat = 11;
pub const VK_FORMAT_R8_SSCALED: VkFormat = 12;
pub const VK_FORMAT_R8_UINT: VkFormat = 13;
pub const VK_FORMAT_R8_SINT: VkFormat = 14;
pub const VK_FORMAT_R8_SRGB: VkFormat = 15;
pub const VK_FORMAT_R8G8_UNORM: VkFormat = 16;
pub const VK_FORMAT_R8G8_SNORM: VkFormat = 17;
pub const VK_FORMAT_R8G8_USCALED: VkFormat = 18;
pub const VK_FORMAT_R8G8_SSCALED: VkFormat = 19;
pub const VK_FORMAT_R8G8_UINT: VkFormat = 20;
pub const VK_FORMAT_R8G8_SINT: VkFormat = 21;
pub const VK_FORMAT_R8G8_SRGB: VkFormat = 22;
pub const VK_FORMAT_R8G8B8_UNORM: VkFormat = 23;
pub const VK_FORMAT_R8G8B8_SNORM: VkFormat = 24;
pub const VK_FORMAT_R8G8B8_USCALED: VkFormat = 25;
pub const VK_FORMAT_R8G8B8_SSCALED: VkFormat = 26;
pub const VK_FORMAT_R8G8B8_UINT: VkFormat = 27;
pub const VK_FORMAT_R8G8B8_SINT: VkFormat = 28;
pub const VK_FORMAT_R8G8B8_SRGB: VkFormat = 29;
pub const VK_FORMAT_B8G8R8_UNORM: VkFormat = 30;
pub const VK_FORMAT_B8G8R8_SNORM: VkFormat = 31;
pub const VK_FORMAT_B8G8R8_USCALED: VkFormat = 32;
pub const VK_FORMAT_B8G8R8_SSCALED: VkFormat = 33;
pub const VK_FORMAT_B8G8R8_UINT: VkFormat = 34;
pub const VK_FORMAT_B8G8R8_SINT: VkFormat = 35;
pub const VK_FORMAT_B8G8R8_SRGB: VkFormat = 36;
pub const VK_FORMAT_R8G8B8A8_UNORM: VkFormat = 37;
pub const VK_FORMAT_R8G8B8A8_SNORM: VkFormat = 38;
pub const VK_FORMAT_R8G8B8A8_USCALED: VkFormat = 39;
pub const VK_FORMAT_R8G8B8A8_SSCALED: VkFormat = 40;
pub const VK_FORMAT_R8G8B8A8_UINT: VkFormat = 41;
pub const VK_FORMAT_R8G8B8A8_SINT: VkFormat = 42;
pub const VK_FORMAT_R8G8B8A8_SRGB: VkFormat = 43;
pub const VK_FORMAT_B8G8R8A8_UNORM: VkFormat = 44;
pub const VK_FORMAT_B8G8R8A8_SNORM: VkFormat = 45;
pub const VK_FORMAT_B8G8R8A8_USCALED: VkFormat = 46;
pub const VK_FORMAT_B8G8R8A8_SSCALED: VkFormat = 47;
pub const VK_FORMAT_B8G8R8A8_UINT: VkFormat = 48;
pub const VK_FORMAT_B8G8R8A8_SINT: VkFormat = 49;
pub const VK_FORMAT_B8G8R8A8_SRGB: VkFormat = 50;
pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: VkFormat = 51;
pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: VkFormat = 52;
pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: VkFormat = 53;
pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: VkFormat = 54;
pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: VkFormat = 55;
pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: VkFormat = 56;
pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: VkFormat = 57;
pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: VkFormat = 58;
pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: VkFormat = 59;
pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: VkFormat = 60;
pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: VkFormat = 61;
pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: VkFormat = 62;
pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: VkFormat = 63;
pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: VkFormat = 64;
pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: VkFormat = 65;
pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: VkFormat = 66;
pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: VkFormat = 67;
pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: VkFormat = 68;
pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: VkFormat = 69;
pub const VK_FORMAT_R16_UNORM: VkFormat = 70;
pub const VK_FORMAT_R16_SNORM: VkFormat = 71;
pub const VK_FORMAT_R16_USCALED: VkFormat = 72;
pub const VK_FORMAT_R16_SSCALED: VkFormat = 73;
pub const VK_FORMAT_R16_UINT: VkFormat = 74;
pub const VK_FORMAT_R16_SINT: VkFormat = 75;
pub const VK_FORMAT_R16_SFLOAT: VkFormat = 76;
pub const VK_FORMAT_R16G16_UNORM: VkFormat = 77;
pub const VK_FORMAT_R16G16_SNORM: VkFormat = 78;
pub const VK_FORMAT_R16G16_USCALED: VkFormat = 79;
pub const VK_FORMAT_R16G16_SSCALED: VkFormat = 80;
pub const VK_FORMAT_R16G16_UINT: VkFormat = 81;
pub const VK_FORMAT_R16G16_SINT: VkFormat = 82;
pub const VK_FORMAT_R16G16_SFLOAT: VkFormat = 83;
pub const VK_FORMAT_R16G16B16_UNORM: VkFormat = 84;
pub const VK_FORMAT_R16G16B16_SNORM: VkFormat = 85;
pub const VK_FORMAT_R16G16B16_USCALED: VkFormat = 86;
pub const VK_FORMAT_R16G16B16_SSCALED: VkFormat = 87;
pub const VK_FORMAT_R16G16B16_UINT: VkFormat = 88;
pub const VK_FORMAT_R16G16B16_SINT: VkFormat = 89;
pub const VK_FORMAT_R16G16B16_SFLOAT: VkFormat = 90;
pub const VK_FORMAT_R16G16B16A16_UNORM: VkFormat = 91;
pub const VK_FORMAT_R16G16B16A16_SNORM: VkFormat = 92;
pub const VK_FORMAT_R16G16B16A16_USCALED: VkFormat = 93;
pub const VK_FORMAT_R16G16B16A16_SSCALED: VkFormat = 94;
pub const VK_FORMAT_R16G16B16A16_UINT: VkFormat = 95;
pub const VK_FORMAT_R16G16B16A16_SINT: VkFormat = 96;
pub const VK_FORMAT_R16G16B16A16_SFLOAT: VkFormat = 97;
pub const VK_FORMAT_R32_UINT: VkFormat = 98;
pub const VK_FORMAT_R32_SINT: VkFormat = 99;
pub const VK_FORMAT_R32_SFLOAT: VkFormat = 100;
pub const VK_FORMAT_R32G32_UINT: VkFormat = 101;
pub const VK_FORMAT_R32G32_SINT: VkFormat = 102;
pub const VK_FORMAT_R32G32_SFLOAT: VkFormat = 103;
pub const VK_FORMAT_R32G32B32_UINT: VkFormat = 104;
pub const VK_FORMAT_R32G32B32_SINT: VkFormat = 105;
pub const VK_FORMAT_R32G32B32_SFLOAT: VkFormat = 106;
pub const VK_FORMAT_R32G32B32A32_UINT: VkFormat = 107;
pub const VK_FORMAT_R32G32B32A32_SINT: VkFormat = 108;
pub const VK_FORMAT_R32G32B32A32_SFLOAT: VkFormat = 109;
pub const VK_FORMAT_R64_UINT: VkFormat = 110;
pub const VK_FORMAT_R64_SINT: VkFormat = 111;
pub const VK_FORMAT_R64_SFLOAT: VkFormat = 112;
pub const VK_FORMAT_R64G64_UINT: VkFormat = 113;
pub const VK_FORMAT_R64G64_SINT: VkFormat = 114;
pub const VK_FORMAT_R64G64_SFLOAT: VkFormat = 115;
pub const VK_FORMAT_R64G64B64_UINT: VkFormat = 116;
pub const VK_FORMAT_R64G64B64_SINT: VkFormat = 117;
pub const VK_FORMAT_R64G64B64_SFLOAT: VkFormat = 118;
pub const VK_FORMAT_R64G64B64A64_UINT: VkFormat = 119;
pub const VK_FORMAT_R64G64B64A64_SINT: VkFormat = 120;
pub const VK_FORMAT_R64G64B64A64_SFLOAT: VkFormat = 121;
pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: VkFormat = 122;
pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: VkFormat = 123;
pub const VK_FORMAT_D16_UNORM: VkFormat = 124;
pub const VK_FORMAT_X8_D24_UNORM_PACK32: VkFormat = 125;
pub const VK_FORMAT_D32_SFLOAT: VkFormat = 126;
pub const VK_FORMAT_S8_UINT: VkFormat = 127;
pub const VK_FORMAT_D16_UNORM_S8_UINT: VkFormat = 128;
pub const VK_FORMAT_D24_UNORM_S8_UINT: VkFormat = 129;
pub const VK_FORMAT_D32_SFLOAT_S8_UINT: VkFormat = 130;
pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: VkFormat = 131;
pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: VkFormat = 132;
pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: VkFormat = 133;
pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: VkFormat = 134;
pub const VK_FORMAT_BC2_UNORM_BLOCK: VkFormat = 135;
pub const VK_FORMAT_BC2_SRGB_BLOCK: VkFormat = 136;
pub const VK_FORMAT_BC3_UNORM_BLOCK: VkFormat = 137;
pub const VK_FORMAT_BC3_SRGB_BLOCK: VkFormat = 138;
pub const VK_FORMAT_BC4_UNORM_BLOCK: VkFormat = 139;
pub const VK_FORMAT_BC4_SNORM_BLOCK: VkFormat = 140;
pub const VK_FORMAT_BC5_UNORM_BLOCK: VkFormat = 141;
pub const VK_FORMAT_BC5_SNORM_BLOCK: VkFormat = 142;
pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: VkFormat = 143;
pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: VkFormat = 144;
pub const VK_FORMAT_BC7_UNORM_BLOCK: VkFormat = 145;
pub const VK_FORMAT_BC7_SRGB_BLOCK: VkFormat = 146;
pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: VkFormat = 147;
pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: VkFormat = 148;
pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: VkFormat = 149;
pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: VkFormat = 150;
pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: VkFormat = 151;
pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: VkFormat = 152;
pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: VkFormat = 153;
pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: VkFormat = 154;
pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: VkFormat = 155;
pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: VkFormat = 156;
pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: VkFormat = 157;
pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: VkFormat = 158;
pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: VkFormat = 159;
pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: VkFormat = 160;
pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: VkFormat = 161;
pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: VkFormat = 162;
pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: VkFormat = 163;
pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: VkFormat = 164;
pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: VkFormat = 165;
pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: VkFormat = 166;
pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: VkFormat = 167;
pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: VkFormat = 168;
pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: VkFormat = 169;
pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: VkFormat = 170;
pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: VkFormat = 171;
pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: VkFormat = 172;
pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: VkFormat = 173;
pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: VkFormat = 174;
pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: VkFormat = 175;
pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: VkFormat = 176;
pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: VkFormat = 177;
pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: VkFormat = 178;
pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: VkFormat = 179;
pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: VkFormat = 180;
pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: VkFormat = 181;
pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: VkFormat = 182;
pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: VkFormat = 183;
pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: VkFormat = 184;
pub const VK_FORMAT_G8B8G8R8_422_UNORM: VkFormat = 1000156000;
pub const VK_FORMAT_B8G8R8G8_422_UNORM: VkFormat = 1000156001;
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM: VkFormat = 1000156002;
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM: VkFormat = 1000156003;
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM: VkFormat = 1000156004;
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM: VkFormat = 1000156005;
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM: VkFormat = 1000156006;
pub const VK_FORMAT_R10X6_UNORM_PACK16: VkFormat = 1000156007;
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: VkFormat = 1000156008;
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: VkFormat = 1000156009;
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: VkFormat = 1000156010;
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: VkFormat = 1000156011;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: VkFormat = 1000156012;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: VkFormat = 1000156013;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: VkFormat = 1000156014;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: VkFormat = 1000156015;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: VkFormat = 1000156016;
pub const VK_FORMAT_R12X4_UNORM_PACK16: VkFormat = 1000156017;
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: VkFormat = 1000156018;
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: VkFormat = 1000156019;
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: VkFormat = 1000156020;
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: VkFormat = 1000156021;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: VkFormat = 1000156022;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: VkFormat = 1000156023;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: VkFormat = 1000156024;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: VkFormat = 1000156025;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: VkFormat = 1000156026;
pub const VK_FORMAT_G16B16G16R16_422_UNORM: VkFormat = 1000156027;
pub const VK_FORMAT_B16G16R16G16_422_UNORM: VkFormat = 1000156028;
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM: VkFormat = 1000156029;
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM: VkFormat = 1000156030;
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM: VkFormat = 1000156031;
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM: VkFormat = 1000156032;
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM: VkFormat = 1000156033;
pub const VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG: VkFormat = 1000054000;
pub const VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG: VkFormat = 1000054001;
pub const VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG: VkFormat = 1000054002;
pub const VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG: VkFormat = 1000054003;
pub const VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG: VkFormat = 1000054004;
pub const VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG: VkFormat = 1000054005;
pub const VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG: VkFormat = 1000054006;
pub const VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG: VkFormat = 1000054007;
pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK_EXT: VkFormat = 1000066000;
pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK_EXT: VkFormat = 1000066001;
pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK_EXT: VkFormat = 1000066002;
pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK_EXT: VkFormat = 1000066003;
pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK_EXT: VkFormat = 1000066004;
pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK_EXT: VkFormat = 1000066005;
pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK_EXT: VkFormat = 1000066006;
pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK_EXT: VkFormat = 1000066007;
pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK_EXT: VkFormat = 1000066008;
pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK_EXT: VkFormat = 1000066009;
pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK_EXT: VkFormat = 1000066010;
pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK_EXT: VkFormat = 1000066011;
pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK_EXT: VkFormat = 1000066012;
pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK_EXT: VkFormat = 1000066013;
pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT: VkFormat = 1000340000;
pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT: VkFormat = 1000340001;

pub type VkColorSpaceKHR = ::std::os::raw::c_uint;
pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = 0;
pub const VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT: VkColorSpaceKHR = 1000104001;
pub const VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT: VkColorSpaceKHR = 1000104002;
pub const VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT: VkColorSpaceKHR = 1000104003;
pub const VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT: VkColorSpaceKHR = 1000104004;
pub const VK_COLOR_SPACE_BT709_LINEAR_EXT: VkColorSpaceKHR = 1000104005;
pub const VK_COLOR_SPACE_BT709_NONLINEAR_EXT: VkColorSpaceKHR = 1000104006;
pub const VK_COLOR_SPACE_BT2020_LINEAR_EXT: VkColorSpaceKHR = 1000104007;
pub const VK_COLOR_SPACE_HDR10_ST2084_EXT: VkColorSpaceKHR = 1000104008;
pub const VK_COLOR_SPACE_DOLBYVISION_EXT: VkColorSpaceKHR = 1000104009;
pub const VK_COLOR_SPACE_HDR10_HLG_EXT: VkColorSpaceKHR = 1000104010;
pub const VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT: VkColorSpaceKHR = 1000104011;
pub const VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT: VkColorSpaceKHR = 1000104012;
pub const VK_COLOR_SPACE_PASS_THROUGH_EXT: VkColorSpaceKHR = 1000104013;
pub const VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT: VkColorSpaceKHR = 1000104014;
pub const VK_COLOR_SPACE_DISPLAY_NATIVE_AMD: VkColorSpaceKHR = 1000213000;

pub type VkInstance = *mut Opaque;
pub type VkPhysicalDevice = *mut Opaque;
pub type VkDevice = *mut Opaque;
pub type VkQueue = *mut Opaque;
pub type VkCommandPool = *mut Opaque;
pub type VkDescriptorPool = *mut Opaque;
pub type VkSurfaceKHR = *mut Opaque;
pub type VkSwapchainKHR = *mut Opaque;
pub type VkImage = *mut Opaque;
pub type VkImageView = *mut Opaque;
pub type VkFramebuffer = *mut Opaque;
pub type VkRenderPass = *mut Opaque;
pub type VkCommandBuffer = *mut Opaque;
pub type VkPipeline = *mut Opaque;
pub type VkPipelineLayout = *mut Opaque;
pub type VkShaderModule = *mut Opaque;
pub type VkPipelineCache = *mut Opaque;
pub type VkSampler = *mut Opaque;
pub type VkDescriptorSetLayout = *mut Opaque;
pub type VkSemaphore = *mut Opaque;
pub type VkFence = *mut Opaque;
pub type VkDescriptorSet = *mut Opaque;
pub type VkBufferView = *mut Opaque;
pub type VkBuffer = *mut Opaque;
pub type VkDeviceMemory = *mut Opaque;

pub type VkSystemAllocationScope = ::std::os::raw::c_uint;

pub type VkInternalAllocationType = ::std::os::raw::c_uint;

pub type VkPhysicalDeviceType = ::std::os::raw::c_uint;

pub type VkDeviceSize = u64;

pub type PFN_vkAllocationFunction = ::std::option::Option<unsafe extern "C" fn(pUserData: *mut ::std::os::raw::c_void,size: size_t,alignment: size_t,allocationScope: VkSystemAllocationScope) -> *mut ::std::os::raw::c_void>;
pub type PFN_vkFreeFunction = ::std::option::Option<unsafe extern "C" fn(pUserData: *mut ::std::os::raw::c_void,pMemory: *mut ::std::os::raw::c_void)>;
pub type PFN_vkInternalAllocationNotification = ::std::option::Option<unsafe extern "C" fn(pUserData: *mut ::std::os::raw::c_void,size: size_t,allocationType: VkInternalAllocationType,allocationScope: VkSystemAllocationScope)>;
pub type PFN_vkInternalFreeNotification = ::std::option::Option<unsafe extern "C" fn(pUserData: *mut ::std::os::raw::c_void,size: size_t,allocationType: VkInternalAllocationType,allocationScope: VkSystemAllocationScope)>;
pub type PFN_vkReallocationFunction = ::std::option::Option<unsafe extern "C" fn(pUserData: *mut ::std::os::raw::c_void,pOriginal: *mut ::std::os::raw::c_void,size: size_t,alignment: size_t,allocationScope: VkSystemAllocationScope) -> *mut ::std::os::raw::c_void>;

pub type VkInstanceCreateFlags = u32;

pub type VkSampleCountFlags = u32;

pub type VkQueueFlags = u32;
pub type VkQueueFlagBits = ::std::os::raw::c_uint;
pub const VK_QUEUE_GRAPHICS_BIT: VkQueueFlagBits = 1;
pub const VK_QUEUE_COMPUTE_BIT: VkQueueFlagBits = 2;
pub const VK_QUEUE_TRANSFER_BIT: VkQueueFlagBits = 4;
pub const VK_QUEUE_SPARSE_BINDING_BIT: VkQueueFlagBits = 8;

pub type VkDeviceQueueCreateFlags = u32;

pub type VkDeviceCreateFlags = u32;

pub type VkCommandPoolCreateFlags = u32;
pub type VkCommandPoolCreateFlagBits = ::std::os::raw::c_uint;
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlagBits = 2;

pub type VkDescriptorPoolCreateFlags = u32;
pub type VkDescriptorPoolCreateFlagBits = ::std::os::raw::c_uint;
pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: VkDescriptorPoolCreateFlagBits = 1;

pub type VkDescriptorType = ::std::os::raw::c_uint;
pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: VkDescriptorType = 3;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: VkDescriptorType = 6;

pub type VkMemoryPropertyFlags = u32;
pub type VkMemoryPropertyFlagBits = ::std::os::raw::c_uint;
pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlagBits = 1;
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlagBits = 2;
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlagBits = 4;
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlagBits = 8;
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlagBits = 16;
pub const VK_MEMORY_PROPERTY_PROTECTED_BIT: VkMemoryPropertyFlagBits = 32;

pub type VkMemoryHeapFlags = u32;

pub type VkCompositeAlphaFlagsKHR = u32;
pub type VkSurfaceTransformFlagsKHR = u32;
pub type VkSurfaceTransformFlagBitsKHR = ::std::os::raw::c_uint;

pub type VkImageUsageFlags = u32;
pub type VkImageUsageFlagBits = ::std::os::raw::c_uint;
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlagBits = 2;
pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlagBits = 8;
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlagBits = 16;

pub type VkSwapchainCreateFlagsKHR = u32;

pub type VkSharingMode = ::std::os::raw::c_uint;
pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = 0;

pub type VkCompositeAlphaFlagBitsKHR = ::std::os::raw::c_uint;
pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: VkCompositeAlphaFlagBitsKHR = 1;

pub type VkPresentModeKHR = ::std::os::raw::c_uint;
pub const VK_PRESENT_MODE_FIFO_KHR: VkPresentModeKHR = 2;

pub type VkImageViewCreateFlags = u32;

pub type VkImageViewType = ::std::os::raw::c_uint;
pub const VK_IMAGE_VIEW_TYPE_2D: VkImageViewType = 1;

pub type VkComponentSwizzle = ::std::os::raw::c_uint;
pub const VK_COMPONENT_SWIZZLE_IDENTITY: VkComponentSwizzle = 0;

pub type VkImageAspectFlags = u32;
pub type VkImageAspectFlagBits = ::std::os::raw::c_uint;
pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlagBits = 1;

pub type VkFramebufferCreateFlags = u32;

pub type VkXcbSurfaceCreateFlagsKHR = u32;

pub type VkImageLayout = ::std::os::raw::c_uint;
pub const VK_IMAGE_LAYOUT_GENERAL: VkImageLayout = 1;

pub type VkPipelineBindPoint = ::std::os::raw::c_uint;
pub const VK_PIPELINE_BIND_POINT_COMPUTE: VkPipelineBindPoint = 1;

pub type VkPipelineStageFlags = u32;
pub type VkPipelineStageFlagBits = ::std::os::raw::c_uint;
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits = 1024;

pub type VkCommandBufferLevel = ::std::os::raw::c_uint;
pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: VkCommandBufferLevel = 0;

pub type VkPipelineCreateFlags = u32;

pub type VkShaderStageFlags = u32;
pub type VkShaderStageFlagBits = ::std::os::raw::c_uint;
pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlagBits = 32;

pub type VkPipelineShaderStageCreateFlags = u32;

pub type VkDescriptorSetLayoutCreateFlags = u32;

pub type VkPipelineLayoutCreateFlags = u32;

pub type VkShaderModuleCreateFlags = u32;

pub type VkFenceCreateFlags = u32;

pub type VkSemaphoreCreateFlags = u32;

pub type VkCommandBufferUsageFlags = u32;

pub type VkQueryControlFlags = u32;

pub type VkQueryPipelineStatisticFlags = u32;

pub type VkSemaphoreWaitFlags = u32;

pub type VkCommandBufferResetFlags = u32;

pub type VkBufferCreateFlags = u32;

pub type VkBufferUsageFlags = u32;
pub type VkBufferUsageFlagBits = ::std::os::raw::c_uint;
pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkBufferUsageFlagBits = 16;

pub type VkMemoryMapFlags = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkAllocationCallbacks {
    pub pUserData: *mut ::std::os::raw::c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub pApplicationName: *const ::std::os::raw::c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const ::std::os::raw::c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkInstanceCreateFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const ::std::os::raw::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: VkDeviceSize,
    pub sparseAddressSpaceSize: VkDeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3usize],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3usize],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: f32,
    pub maxSamplerAnisotropy: f32,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2usize],
    pub viewportBoundsRange: [f32; 2usize],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: size_t,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: f32,
    pub maxInterpolationOffset: f32,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: VkSampleCountFlags,
    pub framebufferDepthSampleCounts: VkSampleCountFlags,
    pub framebufferStencilSampleCounts: VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
    pub sampledImageDepthSampleCounts: VkSampleCountFlags,
    pub sampledImageStencilSampleCounts: VkSampleCountFlags,
    pub storageImageSampleCounts: VkSampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: VkBool32,
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2usize],
    pub lineWidthRange: [f32; 2usize],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
    pub strictLines: VkBool32,
    pub standardSampleLocations: VkBool32,
    pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    pub nonCoherentAtomSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: [::std::os::raw::c_char; 256usize],
    pub pipelineCacheUUID: [u8; 16usize],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceFeatures {
    pub robustBufferAccess: VkBool32,
    pub fullDrawIndexUint32: VkBool32,
    pub imageCubeArray: VkBool32,
    pub independentBlend: VkBool32,
    pub geometryShader: VkBool32,
    pub tessellationShader: VkBool32,
    pub sampleRateShading: VkBool32,
    pub dualSrcBlend: VkBool32,
    pub logicOp: VkBool32,
    pub multiDrawIndirect: VkBool32,
    pub drawIndirectFirstInstance: VkBool32,
    pub depthClamp: VkBool32,
    pub depthBiasClamp: VkBool32,
    pub fillModeNonSolid: VkBool32,
    pub depthBounds: VkBool32,
    pub wideLines: VkBool32,
    pub largePoints: VkBool32,
    pub alphaToOne: VkBool32,
    pub multiViewport: VkBool32,
    pub samplerAnisotropy: VkBool32,
    pub textureCompressionETC2: VkBool32,
    pub textureCompressionASTC_LDR: VkBool32,
    pub textureCompressionBC: VkBool32,
    pub occlusionQueryPrecise: VkBool32,
    pub pipelineStatisticsQuery: VkBool32,
    pub vertexPipelineStoresAndAtomics: VkBool32,
    pub fragmentStoresAndAtomics: VkBool32,
    pub shaderTessellationAndGeometryPointSize: VkBool32,
    pub shaderImageGatherExtended: VkBool32,
    pub shaderStorageImageExtendedFormats: VkBool32,
    pub shaderStorageImageMultisample: VkBool32,
    pub shaderStorageImageReadWithoutFormat: VkBool32,
    pub shaderStorageImageWriteWithoutFormat: VkBool32,
    pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
    pub shaderSampledImageArrayDynamicIndexing: VkBool32,
    pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageImageArrayDynamicIndexing: VkBool32,
    pub shaderClipDistance: VkBool32,
    pub shaderCullDistance: VkBool32,
    pub shaderFloat64: VkBool32,
    pub shaderInt64: VkBool32,
    pub shaderInt16: VkBool32,
    pub shaderResourceResidency: VkBool32,
    pub shaderResourceMinLod: VkBool32,
    pub sparseBinding: VkBool32,
    pub sparseResidencyBuffer: VkBool32,
    pub sparseResidencyImage2D: VkBool32,
    pub sparseResidencyImage3D: VkBool32,
    pub sparseResidency2Samples: VkBool32,
    pub sparseResidency4Samples: VkBool32,
    pub sparseResidency8Samples: VkBool32,
    pub sparseResidency16Samples: VkBool32,
    pub sparseResidencyAliased: VkBool32,
    pub variableMultisampleRate: VkBool32,
    pub inheritedQueries: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkDeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const ::std::os::raw::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const ::std::os::raw::c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorPoolSize {
    pub type_: VkDescriptorType,
    pub descriptorCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkDescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const VkDescriptorPoolSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; 32usize],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: VkSurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: VkFormat,
    pub imageColorSpace: VkColorSpaceKHR,
    pub imageExtent: VkExtent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: VkImageUsageFlags,
    pub imageSharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: VkSurfaceTransformFlagBitsKHR,
    pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
    pub presentMode: VkPresentModeKHR,
    pub clipped: VkBool32,
    pub oldSwapchain: VkSwapchainKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkImageViewCreateFlags,
    pub image: VkImage,
    pub viewType: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageSubresourceRange {
    pub aspectMask: VkImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFramebufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkFramebufferCreateFlags,
    pub renderPass: VkRenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut Opaque,
    pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkComputePipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlagBits,
    pub module: VkShaderModule,
    pub pName: *const ::std::os::raw::c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: size_t,
    pub pData: *const ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: VkShaderStageFlags,
    pub pImmutableSamplers: *const VkSampler,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkPipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPushConstantRange {
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub pWaitDstStageMask: *const VkPipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const VkCommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkShaderModuleCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub codeSize: size_t,
    pub pCode: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkFenceCreateFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkSemaphoreCreateFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandBufferInheritanceInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusionQueryEnable: VkBool32,
    pub queryFlags: VkQueryControlFlags,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWriteDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub pImageInfo: *const VkDescriptorImageInfo,
    pub pBufferInfo: *const VkDescriptorBufferInfo,
    pub pTexelBufferView: *const VkBufferView,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCopyDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub srcSet: VkDescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSemaphoreWaitInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkSemaphoreWaitFlags,
    pub semaphoreCount: u32,
    pub pSemaphores: *const VkSemaphore,
    pub pValues: *const u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut VkResult,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeIndex: u32,
}

extern "C" { pub fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo,pAllocator: *const VkAllocationCallbacks,pInstance: *mut VkInstance) -> VkResult; }
extern "C" { pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkEnumeratePhysicalDevices(instance: VkInstance,pPhysicalDeviceCount: *mut u32,pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult; }
extern "C" { pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice,pProperties: *mut VkPhysicalDeviceProperties); }
extern "C" { pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice,pQueueFamilyPropertyCount: *mut u32,pQueueFamilyProperties: *mut VkQueueFamilyProperties); }
extern "C" { pub fn vkCreateDevice(physicalDevice: VkPhysicalDevice,pCreateInfo: *const VkDeviceCreateInfo,pAllocator: *const VkAllocationCallbacks,pDevice: *mut VkDevice) -> VkResult; }
extern "C" { pub fn vkGetDeviceQueue(device: VkDevice,queueFamilyIndex: u32,queueIndex: u32,pQueue: *mut VkQueue); }
extern "C" { pub fn vkCreateCommandPool(device: VkDevice,pCreateInfo: *const VkCommandPoolCreateInfo,pAllocator: *const VkAllocationCallbacks,pCommandPool: *mut VkCommandPool) -> VkResult; }
extern "C" { pub fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkCreateDescriptorPool(device: VkDevice,pCreateInfo: *const VkDescriptorPoolCreateInfo,pAllocator: *const VkAllocationCallbacks,pDescriptorPool: *mut VkDescriptorPool) -> VkResult; }
extern "C" { pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice,pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties); }
extern "C" { pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: VkPhysicalDevice,surface: VkSurfaceKHR,pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult; }
extern "C" { pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: VkPhysicalDevice,surface: VkSurfaceKHR,pSurfaceFormatCount: *mut u32,pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult; }
extern "C" { pub fn vkCreateSwapchainKHR(device: VkDevice,pCreateInfo: *const VkSwapchainCreateInfoKHR,pAllocator: *const VkAllocationCallbacks,pSwapchain: *mut VkSwapchainKHR) -> VkResult; }
extern "C" { pub fn vkGetSwapchainImagesKHR(device: VkDevice,swapchain: VkSwapchainKHR,pSwapchainImageCount: *mut u32,pSwapchainImages: *mut VkImage) -> VkResult; }
extern "C" { pub fn vkDestroySwapchainKHR(device: VkDevice,swapchain: VkSwapchainKHR,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkCreateImageView(device: VkDevice,pCreateInfo: *const VkImageViewCreateInfo,pAllocator: *const VkAllocationCallbacks,pView: *mut VkImageView) -> VkResult; }
extern "C" { pub fn vkDestroyImageView(device: VkDevice,imageView: VkImageView,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkCreateXcbSurfaceKHR(instance: VkInstance,pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,pAllocator: *const VkAllocationCallbacks,pSurface: *mut VkSurfaceKHR) -> VkResult; }
extern "C" { pub fn vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: VkPhysicalDevice,queueFamilyIndex: u32,surface: VkSurfaceKHR,pSupported: *mut VkBool32) -> VkResult; }
extern "C" { pub fn vkDestroySurfaceKHR(instance: VkInstance,surface: VkSurfaceKHR,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkAllocateCommandBuffers(device: VkDevice,pAllocateInfo: *const VkCommandBufferAllocateInfo,pCommandBuffers: *mut VkCommandBuffer) -> VkResult; }
extern "C" { pub fn vkCreateComputePipelines(device: VkDevice,pipelineCache: VkPipelineCache,createInfoCount: u32,pCreateInfos: *const VkComputePipelineCreateInfo,pAllocator: *const VkAllocationCallbacks,pPipelines: *mut VkPipeline) -> VkResult; }
extern "C" { pub fn vkCreateDescriptorSetLayout(device: VkDevice,pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,pAllocator: *const VkAllocationCallbacks,pSetLayout: *mut VkDescriptorSetLayout) -> VkResult; }
extern "C" { pub fn vkCreatePipelineLayout(device: VkDevice,pCreateInfo: *const VkPipelineLayoutCreateInfo,pAllocator: *const VkAllocationCallbacks,pPipelineLayout: *mut VkPipelineLayout) -> VkResult; }
extern "C" { pub fn vkQueueSubmit(queue: VkQueue,submitCount: u32,pSubmits: *const VkSubmitInfo,fence: VkFence) -> VkResult; }
extern "C" { pub fn vkCreateShaderModule(device: VkDevice,pCreateInfo: *const VkShaderModuleCreateInfo,pAllocator: *const VkAllocationCallbacks,pShaderModule: *mut VkShaderModule) -> VkResult; }
extern "C" { pub fn vkCreateFence(device: VkDevice,pCreateInfo: *const VkFenceCreateInfo,pAllocator: *const VkAllocationCallbacks,pFence: *mut VkFence) -> VkResult; }
extern "C" { pub fn vkCreateSemaphore(device: VkDevice,pCreateInfo: *const VkSemaphoreCreateInfo,pAllocator: *const VkAllocationCallbacks,pSemaphore: *mut VkSemaphore) -> VkResult; }
extern "C" { pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer,pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult; }
extern "C" { pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult; }
extern "C" { pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer,pipelineBindPoint: VkPipelineBindPoint,pipeline: VkPipeline); }
extern "C" { pub fn vkCmdBindDescriptorSets(commandBuffer: VkCommandBuffer,pipelineBindPoint: VkPipelineBindPoint,layout: VkPipelineLayout,firstSet: u32,descriptorSetCount: u32,pDescriptorSets: *const VkDescriptorSet,dynamicOffsetCount: u32,pDynamicOffsets: *const u32); }
extern "C" { pub fn vkCmdDispatch(commandBuffer: VkCommandBuffer,groupCountX: u32,groupCountY: u32,groupCountZ: u32); }
extern "C" { pub fn vkFreeCommandBuffers(device: VkDevice,commandPool: VkCommandPool,commandBufferCount: u32,pCommandBuffers: *const VkCommandBuffer); }
extern "C" { pub fn vkDestroyPipeline(device: VkDevice,pipeline: VkPipeline,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkDestroyShaderModule(device: VkDevice,shaderModule: VkShaderModule,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkUpdateDescriptorSets(device: VkDevice,descriptorWriteCount: u32,pDescriptorWrites: *const VkWriteDescriptorSet,descriptorCopyCount: u32,pDescriptorCopies: *const VkCopyDescriptorSet); }
extern "C" { pub fn vkWaitForFences(device: VkDevice,fenceCount: u32,pFences: *const VkFence,waitAll: VkBool32,timeout: u64) -> VkResult; }
extern "C" { pub fn vkDestroyFence(device: VkDevice,fence: VkFence,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkAllocateDescriptorSets(device: VkDevice,pAllocateInfo: *const VkDescriptorSetAllocateInfo,pDescriptorSets: *mut VkDescriptorSet) -> VkResult; }
extern "C" { pub fn vkDestroyPipelineLayout(device: VkDevice,pipelineLayout: VkPipelineLayout,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkDestroyDescriptorSetLayout(device: VkDevice,descriptorSetLayout: VkDescriptorSetLayout,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkDestroySemaphore(device: VkDevice,semaphore: VkSemaphore,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkAcquireNextImageKHR(device: VkDevice,swapchain: VkSwapchainKHR,timeout: u64,semaphore: VkSemaphore,fence: VkFence,pImageIndex: *mut u32) -> VkResult; }
extern "C" { pub fn vkQueuePresentKHR(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult; }
extern "C" { pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer,flags: VkCommandBufferResetFlags) -> VkResult; }
extern "C" { pub fn vkFreeDescriptorSets(device: VkDevice,descriptorPool: VkDescriptorPool,descriptorSetCount: u32,pDescriptorSets: *const VkDescriptorSet) -> VkResult; }
extern "C" { pub fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult; }
extern "C" { pub fn vkCreateBuffer(device: VkDevice,pCreateInfo: *const VkBufferCreateInfo,pAllocator: *const VkAllocationCallbacks,pBuffer: *mut VkBuffer) -> VkResult; }
extern "C" { pub fn vkDestroyBuffer(device: VkDevice,buffer: VkBuffer,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkAllocateMemory(device: VkDevice,pAllocateInfo: *const VkMemoryAllocateInfo,pAllocator: *const VkAllocationCallbacks,pMemory: *mut VkDeviceMemory) -> VkResult; }
extern "C" { pub fn vkFreeMemory(device: VkDevice,memory: VkDeviceMemory,pAllocator: *const VkAllocationCallbacks); }
extern "C" { pub fn vkMapMemory(device: VkDevice,memory: VkDeviceMemory,offset: VkDeviceSize,size: VkDeviceSize,flags: VkMemoryMapFlags,ppData: *mut *mut ::std::os::raw::c_void) -> VkResult; }
extern "C" { pub fn vkBindBufferMemory(device: VkDevice,buffer: VkBuffer,memory: VkDeviceMemory,memoryOffset: VkDeviceSize) -> VkResult; }
