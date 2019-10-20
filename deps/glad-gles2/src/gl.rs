pub use self::enumerations::*;
pub use self::functions::*;
pub use self::types::*;

use std::os::raw;

pub struct FnPtr {
    pub ptr: *const raw::c_void,
    pub is_loaded: bool,
}

impl FnPtr {
    pub fn empty() -> FnPtr {
        FnPtr {
            ptr: FnPtr::not_initialized as *const raw::c_void,
            is_loaded: false,
        }
    }

    pub fn load<F>(&mut self, loadfn: &mut F, name: &'static str)
    where
        F: FnMut(&'static str) -> *const raw::c_void,
    {
        let loaded = loadfn(name);
        if !loaded.is_null() {
            self.ptr = loaded;
            self.is_loaded = true;
        } else {
            self.ptr = FnPtr::not_initialized as *const raw::c_void;
            self.is_loaded = false;
        };
    }

    pub fn aliased(&mut self, other: &FnPtr) {
        if !self.is_loaded && other.is_loaded {
            self.ptr = other.ptr;
            self.is_loaded = other.is_loaded;
        }
    }

    #[inline(never)]
    fn not_initialized() -> ! {
        panic!("gles2: function not initialized")
    }
}

pub mod types {
    #![allow(dead_code, non_snake_case, non_camel_case_types)]

    use std::os::raw;

    pub type GLvoid = raw::c_void;

    pub type GLbyte = raw::c_char;
    pub type GLubyte = raw::c_uchar;
    pub type GLchar = raw::c_char;
    pub type GLboolean = raw::c_uchar;

    pub type GLshort = raw::c_short;
    pub type GLushort = raw::c_ushort;

    pub type GLint = raw::c_int;
    pub type GLuint = raw::c_uint;
    pub type GLint64 = i64;
    pub type GLuint64 = u64;

    pub type GLintptr = isize;
    pub type GLsizeiptr = isize;
    pub type GLintptrARB = isize;
    pub type GLsizeiptrARB = isize;
    pub type GLint64EXT = i64;
    pub type GLuint64EXT = u64;

    pub type GLsizei = GLint;
    pub type GLclampx = raw::c_int;
    pub type GLfixed = GLint;
    pub type GLhalf = raw::c_ushort;
    pub type GLhalfNV = raw::c_ushort;
    pub type GLhalfARB = raw::c_ushort;

    pub type GLenum = raw::c_uint;
    pub type GLbitfield = raw::c_uint;

    pub type GLfloat = raw::c_float;
    pub type GLdouble = raw::c_double;
    pub type GLclampf = raw::c_float;
    pub type GLclampd = raw::c_double;

    pub type GLcharARB = raw::c_char;

    #[cfg(target_os = "macos")]
    pub type GLhandleARB = *const raw::c_void;
    #[cfg(not(target_os = "macos"))]
    pub type GLhandleARB = raw::c_uint;

    pub enum __GLsync {}

    pub type GLsync = *const __GLsync;

    pub enum _cl_context {}

    pub enum _cl_event {}

    pub type GLvdpauSurfaceNV = GLintptr;
    pub type GLeglClientBufferEXT = *const raw::c_void;
    pub type GLeglImageOES = *const raw::c_void;

    pub type GLDEBUGPROC = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut raw::c_void,
    );
    pub type GLDEBUGPROCARB = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut raw::c_void,
    );
    pub type GLDEBUGPROCKHR = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut GLvoid,
    );
    pub type GLDEBUGPROCAMD = extern "system" fn(
        id: GLuint,
        category: GLenum,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut GLvoid,
    );
    pub type GLVULKANPROCNV = extern "system" fn();
}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use super::types::*;
    use std;

    pub const GL_ACTIVE_ATTRIBUTES: std::os::raw::c_uint = 0x8B89;
    pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: std::os::raw::c_uint = 0x8B8A;
    pub const GL_ACTIVE_TEXTURE: std::os::raw::c_uint = 0x84E0;
    pub const GL_ACTIVE_UNIFORMS: std::os::raw::c_uint = 0x8B86;
    pub const GL_ACTIVE_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A36;
    pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: std::os::raw::c_uint = 0x8A35;
    pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: std::os::raw::c_uint = 0x8B87;
    pub const GL_ALIASED_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x846E;
    pub const GL_ALIASED_POINT_SIZE_RANGE: std::os::raw::c_uint = 0x846D;
    pub const GL_ALPHA: std::os::raw::c_uint = 0x1906;
    pub const GL_ALPHA_BITS: std::os::raw::c_uint = 0x0D55;
    pub const GL_ALREADY_SIGNALED: std::os::raw::c_uint = 0x911A;
    pub const GL_ALWAYS: std::os::raw::c_uint = 0x0207;
    pub const GL_ANY_SAMPLES_PASSED: std::os::raw::c_uint = 0x8C2F;
    pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: std::os::raw::c_uint = 0x8D6A;
    pub const GL_ARRAY_BUFFER: std::os::raw::c_uint = 0x8892;
    pub const GL_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8894;
    pub const GL_ATTACHED_SHADERS: std::os::raw::c_uint = 0x8B85;
    pub const GL_BACK: std::os::raw::c_uint = 0x0405;
    pub const GL_BLEND: std::os::raw::c_uint = 0x0BE2;
    pub const GL_BLEND_COLOR: std::os::raw::c_uint = 0x8005;
    pub const GL_BLEND_DST_ALPHA: std::os::raw::c_uint = 0x80CA;
    pub const GL_BLEND_DST_RGB: std::os::raw::c_uint = 0x80C8;
    pub const GL_BLEND_EQUATION: std::os::raw::c_uint = 0x8009;
    pub const GL_BLEND_EQUATION_ALPHA: std::os::raw::c_uint = 0x883D;
    pub const GL_BLEND_EQUATION_RGB: std::os::raw::c_uint = 0x8009;
    pub const GL_BLEND_SRC_ALPHA: std::os::raw::c_uint = 0x80CB;
    pub const GL_BLEND_SRC_RGB: std::os::raw::c_uint = 0x80C9;
    pub const GL_BLUE: std::os::raw::c_uint = 0x1905;
    pub const GL_BLUE_BITS: std::os::raw::c_uint = 0x0D54;
    pub const GL_BOOL: std::os::raw::c_uint = 0x8B56;
    pub const GL_BOOL_VEC2: std::os::raw::c_uint = 0x8B57;
    pub const GL_BOOL_VEC3: std::os::raw::c_uint = 0x8B58;
    pub const GL_BOOL_VEC4: std::os::raw::c_uint = 0x8B59;
    pub const GL_BUFFER_ACCESS_FLAGS: std::os::raw::c_uint = 0x911F;
    pub const GL_BUFFER_KHR: std::os::raw::c_uint = 0x82E0;
    pub const GL_BUFFER_MAPPED: std::os::raw::c_uint = 0x88BC;
    pub const GL_BUFFER_MAP_LENGTH: std::os::raw::c_uint = 0x9120;
    pub const GL_BUFFER_MAP_OFFSET: std::os::raw::c_uint = 0x9121;
    pub const GL_BUFFER_MAP_POINTER: std::os::raw::c_uint = 0x88BD;
    pub const GL_BUFFER_SIZE: std::os::raw::c_uint = 0x8764;
    pub const GL_BUFFER_USAGE: std::os::raw::c_uint = 0x8765;
    pub const GL_BYTE: std::os::raw::c_uint = 0x1400;
    pub const GL_CCW: std::os::raw::c_uint = 0x0901;
    pub const GL_CLAMP_TO_EDGE: std::os::raw::c_uint = 0x812F;
    pub const GL_COLOR: std::os::raw::c_uint = 0x1800;
    pub const GL_COLOR_ATTACHMENT0: std::os::raw::c_uint = 0x8CE0;
    pub const GL_COLOR_ATTACHMENT1: std::os::raw::c_uint = 0x8CE1;
    pub const GL_COLOR_ATTACHMENT10: std::os::raw::c_uint = 0x8CEA;
    pub const GL_COLOR_ATTACHMENT11: std::os::raw::c_uint = 0x8CEB;
    pub const GL_COLOR_ATTACHMENT12: std::os::raw::c_uint = 0x8CEC;
    pub const GL_COLOR_ATTACHMENT13: std::os::raw::c_uint = 0x8CED;
    pub const GL_COLOR_ATTACHMENT14: std::os::raw::c_uint = 0x8CEE;
    pub const GL_COLOR_ATTACHMENT15: std::os::raw::c_uint = 0x8CEF;
    pub const GL_COLOR_ATTACHMENT16: std::os::raw::c_uint = 0x8CF0;
    pub const GL_COLOR_ATTACHMENT17: std::os::raw::c_uint = 0x8CF1;
    pub const GL_COLOR_ATTACHMENT18: std::os::raw::c_uint = 0x8CF2;
    pub const GL_COLOR_ATTACHMENT19: std::os::raw::c_uint = 0x8CF3;
    pub const GL_COLOR_ATTACHMENT2: std::os::raw::c_uint = 0x8CE2;
    pub const GL_COLOR_ATTACHMENT20: std::os::raw::c_uint = 0x8CF4;
    pub const GL_COLOR_ATTACHMENT21: std::os::raw::c_uint = 0x8CF5;
    pub const GL_COLOR_ATTACHMENT22: std::os::raw::c_uint = 0x8CF6;
    pub const GL_COLOR_ATTACHMENT23: std::os::raw::c_uint = 0x8CF7;
    pub const GL_COLOR_ATTACHMENT24: std::os::raw::c_uint = 0x8CF8;
    pub const GL_COLOR_ATTACHMENT25: std::os::raw::c_uint = 0x8CF9;
    pub const GL_COLOR_ATTACHMENT26: std::os::raw::c_uint = 0x8CFA;
    pub const GL_COLOR_ATTACHMENT27: std::os::raw::c_uint = 0x8CFB;
    pub const GL_COLOR_ATTACHMENT28: std::os::raw::c_uint = 0x8CFC;
    pub const GL_COLOR_ATTACHMENT29: std::os::raw::c_uint = 0x8CFD;
    pub const GL_COLOR_ATTACHMENT3: std::os::raw::c_uint = 0x8CE3;
    pub const GL_COLOR_ATTACHMENT30: std::os::raw::c_uint = 0x8CFE;
    pub const GL_COLOR_ATTACHMENT31: std::os::raw::c_uint = 0x8CFF;
    pub const GL_COLOR_ATTACHMENT4: std::os::raw::c_uint = 0x8CE4;
    pub const GL_COLOR_ATTACHMENT5: std::os::raw::c_uint = 0x8CE5;
    pub const GL_COLOR_ATTACHMENT6: std::os::raw::c_uint = 0x8CE6;
    pub const GL_COLOR_ATTACHMENT7: std::os::raw::c_uint = 0x8CE7;
    pub const GL_COLOR_ATTACHMENT8: std::os::raw::c_uint = 0x8CE8;
    pub const GL_COLOR_ATTACHMENT9: std::os::raw::c_uint = 0x8CE9;
    pub const GL_COLOR_BUFFER_BIT: std::os::raw::c_uint = 0x00004000;
    pub const GL_COLOR_CLEAR_VALUE: std::os::raw::c_uint = 0x0C22;
    pub const GL_COLOR_WRITEMASK: std::os::raw::c_uint = 0x0C23;
    pub const GL_COMPARE_REF_TO_TEXTURE: std::os::raw::c_uint = 0x884E;
    pub const GL_COMPARE_R_TO_TEXTURE: std::os::raw::c_uint = 0x884E;
    pub const GL_COMPILE_STATUS: std::os::raw::c_uint = 0x8B81;
    pub const GL_COMPRESSED_R11_EAC: std::os::raw::c_uint = 0x9270;
    pub const GL_COMPRESSED_RG11_EAC: std::os::raw::c_uint = 0x9272;
    pub const GL_COMPRESSED_RGB8_ETC2: std::os::raw::c_uint = 0x9274;
    pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: std::os::raw::c_uint = 0x9276;
    pub const GL_COMPRESSED_RGBA8_ETC2_EAC: std::os::raw::c_uint = 0x9278;
    pub const GL_COMPRESSED_SIGNED_R11_EAC: std::os::raw::c_uint = 0x9271;
    pub const GL_COMPRESSED_SIGNED_RG11_EAC: std::os::raw::c_uint = 0x9273;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: std::os::raw::c_uint = 0x9279;
    pub const GL_COMPRESSED_SRGB8_ETC2: std::os::raw::c_uint = 0x9275;
    pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: std::os::raw::c_uint = 0x9277;
    pub const GL_COMPRESSED_TEXTURE_FORMATS: std::os::raw::c_uint = 0x86A3;
    pub const GL_CONDITION_SATISFIED: std::os::raw::c_uint = 0x911C;
    pub const GL_CONSTANT_ALPHA: std::os::raw::c_uint = 0x8003;
    pub const GL_CONSTANT_COLOR: std::os::raw::c_uint = 0x8001;
    pub const GL_CONTEXT_FLAG_DEBUG_BIT_KHR: std::os::raw::c_uint = 0x00000002;
    pub const GL_COPY_READ_BUFFER: std::os::raw::c_uint = 0x8F36;
    pub const GL_COPY_READ_BUFFER_BINDING: std::os::raw::c_uint = 0x8F36;
    pub const GL_COPY_WRITE_BUFFER: std::os::raw::c_uint = 0x8F37;
    pub const GL_COPY_WRITE_BUFFER_BINDING: std::os::raw::c_uint = 0x8F37;
    pub const GL_CULL_FACE: std::os::raw::c_uint = 0x0B44;
    pub const GL_CULL_FACE_MODE: std::os::raw::c_uint = 0x0B45;
    pub const GL_CURRENT_PROGRAM: std::os::raw::c_uint = 0x8B8D;
    pub const GL_CURRENT_QUERY: std::os::raw::c_uint = 0x8865;
    pub const GL_CURRENT_VERTEX_ATTRIB: std::os::raw::c_uint = 0x8626;
    pub const GL_CW: std::os::raw::c_uint = 0x0900;
    pub const GL_DEBUG_CALLBACK_FUNCTION_KHR: std::os::raw::c_uint = 0x8244;
    pub const GL_DEBUG_CALLBACK_USER_PARAM_KHR: std::os::raw::c_uint = 0x8245;
    pub const GL_DEBUG_GROUP_STACK_DEPTH_KHR: std::os::raw::c_uint = 0x826D;
    pub const GL_DEBUG_LOGGED_MESSAGES_KHR: std::os::raw::c_uint = 0x9145;
    pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_KHR: std::os::raw::c_uint = 0x8243;
    pub const GL_DEBUG_OUTPUT_KHR: std::os::raw::c_uint = 0x92E0;
    pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_KHR: std::os::raw::c_uint = 0x8242;
    pub const GL_DEBUG_SEVERITY_HIGH_KHR: std::os::raw::c_uint = 0x9146;
    pub const GL_DEBUG_SEVERITY_LOW_KHR: std::os::raw::c_uint = 0x9148;
    pub const GL_DEBUG_SEVERITY_MEDIUM_KHR: std::os::raw::c_uint = 0x9147;
    pub const GL_DEBUG_SEVERITY_NOTIFICATION_KHR: std::os::raw::c_uint = 0x826B;
    pub const GL_DEBUG_SOURCE_API_KHR: std::os::raw::c_uint = 0x8246;
    pub const GL_DEBUG_SOURCE_APPLICATION_KHR: std::os::raw::c_uint = 0x824A;
    pub const GL_DEBUG_SOURCE_OTHER_KHR: std::os::raw::c_uint = 0x824B;
    pub const GL_DEBUG_SOURCE_SHADER_COMPILER_KHR: std::os::raw::c_uint = 0x8248;
    pub const GL_DEBUG_SOURCE_THIRD_PARTY_KHR: std::os::raw::c_uint = 0x8249;
    pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_KHR: std::os::raw::c_uint = 0x8247;
    pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR: std::os::raw::c_uint = 0x824D;
    pub const GL_DEBUG_TYPE_ERROR_KHR: std::os::raw::c_uint = 0x824C;
    pub const GL_DEBUG_TYPE_MARKER_KHR: std::os::raw::c_uint = 0x8268;
    pub const GL_DEBUG_TYPE_OTHER_KHR: std::os::raw::c_uint = 0x8251;
    pub const GL_DEBUG_TYPE_PERFORMANCE_KHR: std::os::raw::c_uint = 0x8250;
    pub const GL_DEBUG_TYPE_POP_GROUP_KHR: std::os::raw::c_uint = 0x826A;
    pub const GL_DEBUG_TYPE_PORTABILITY_KHR: std::os::raw::c_uint = 0x824F;
    pub const GL_DEBUG_TYPE_PUSH_GROUP_KHR: std::os::raw::c_uint = 0x8269;
    pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR: std::os::raw::c_uint = 0x824E;
    pub const GL_DECR: std::os::raw::c_uint = 0x1E03;
    pub const GL_DECR_WRAP: std::os::raw::c_uint = 0x8508;
    pub const GL_DELETE_STATUS: std::os::raw::c_uint = 0x8B80;
    pub const GL_DEPTH: std::os::raw::c_uint = 0x1801;
    pub const GL_DEPTH24_STENCIL8: std::os::raw::c_uint = 0x88F0;
    pub const GL_DEPTH32F_STENCIL8: std::os::raw::c_uint = 0x8CAD;
    pub const GL_DEPTH_ATTACHMENT: std::os::raw::c_uint = 0x8D00;
    pub const GL_DEPTH_BITS: std::os::raw::c_uint = 0x0D56;
    pub const GL_DEPTH_BUFFER_BIT: std::os::raw::c_uint = 0x00000100;
    pub const GL_DEPTH_CLEAR_VALUE: std::os::raw::c_uint = 0x0B73;
    pub const GL_DEPTH_COMPONENT: std::os::raw::c_uint = 0x1902;
    pub const GL_DEPTH_COMPONENT16: std::os::raw::c_uint = 0x81A5;
    pub const GL_DEPTH_COMPONENT24: std::os::raw::c_uint = 0x81A6;
    pub const GL_DEPTH_COMPONENT32F: std::os::raw::c_uint = 0x8CAC;
    pub const GL_DEPTH_FUNC: std::os::raw::c_uint = 0x0B74;
    pub const GL_DEPTH_RANGE: std::os::raw::c_uint = 0x0B70;
    pub const GL_DEPTH_STENCIL: std::os::raw::c_uint = 0x84F9;
    pub const GL_DEPTH_STENCIL_ATTACHMENT: std::os::raw::c_uint = 0x821A;
    pub const GL_DEPTH_TEST: std::os::raw::c_uint = 0x0B71;
    pub const GL_DEPTH_WRITEMASK: std::os::raw::c_uint = 0x0B72;
    pub const GL_DITHER: std::os::raw::c_uint = 0x0BD0;
    pub const GL_DONT_CARE: std::os::raw::c_uint = 0x1100;
    pub const GL_DRAW_BUFFER0: std::os::raw::c_uint = 0x8825;
    pub const GL_DRAW_BUFFER1: std::os::raw::c_uint = 0x8826;
    pub const GL_DRAW_BUFFER10: std::os::raw::c_uint = 0x882F;
    pub const GL_DRAW_BUFFER11: std::os::raw::c_uint = 0x8830;
    pub const GL_DRAW_BUFFER12: std::os::raw::c_uint = 0x8831;
    pub const GL_DRAW_BUFFER13: std::os::raw::c_uint = 0x8832;
    pub const GL_DRAW_BUFFER14: std::os::raw::c_uint = 0x8833;
    pub const GL_DRAW_BUFFER15: std::os::raw::c_uint = 0x8834;
    pub const GL_DRAW_BUFFER2: std::os::raw::c_uint = 0x8827;
    pub const GL_DRAW_BUFFER3: std::os::raw::c_uint = 0x8828;
    pub const GL_DRAW_BUFFER4: std::os::raw::c_uint = 0x8829;
    pub const GL_DRAW_BUFFER5: std::os::raw::c_uint = 0x882A;
    pub const GL_DRAW_BUFFER6: std::os::raw::c_uint = 0x882B;
    pub const GL_DRAW_BUFFER7: std::os::raw::c_uint = 0x882C;
    pub const GL_DRAW_BUFFER8: std::os::raw::c_uint = 0x882D;
    pub const GL_DRAW_BUFFER9: std::os::raw::c_uint = 0x882E;
    pub const GL_DRAW_FRAMEBUFFER: std::os::raw::c_uint = 0x8CA9;
    pub const GL_DRAW_FRAMEBUFFER_BINDING: std::os::raw::c_uint = 0x8CA6;
    pub const GL_DST_ALPHA: std::os::raw::c_uint = 0x0304;
    pub const GL_DST_COLOR: std::os::raw::c_uint = 0x0306;
    pub const GL_DYNAMIC_COPY: std::os::raw::c_uint = 0x88EA;
    pub const GL_DYNAMIC_DRAW: std::os::raw::c_uint = 0x88E8;
    pub const GL_DYNAMIC_READ: std::os::raw::c_uint = 0x88E9;
    pub const GL_ELEMENT_ARRAY_BUFFER: std::os::raw::c_uint = 0x8893;
    pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8895;
    pub const GL_EQUAL: std::os::raw::c_uint = 0x0202;
    pub const GL_EXTENSIONS: std::os::raw::c_uint = 0x1F03;
    pub const GL_FALSE: std::os::raw::c_uchar = 0;
    pub const GL_FASTEST: std::os::raw::c_uint = 0x1101;
    pub const GL_FIXED: std::os::raw::c_uint = 0x140C;
    pub const GL_FLOAT: std::os::raw::c_uint = 0x1406;
    pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: std::os::raw::c_uint = 0x8DAD;
    pub const GL_FLOAT_MAT2: std::os::raw::c_uint = 0x8B5A;
    pub const GL_FLOAT_MAT2x3: std::os::raw::c_uint = 0x8B65;
    pub const GL_FLOAT_MAT2x4: std::os::raw::c_uint = 0x8B66;
    pub const GL_FLOAT_MAT3: std::os::raw::c_uint = 0x8B5B;
    pub const GL_FLOAT_MAT3x2: std::os::raw::c_uint = 0x8B67;
    pub const GL_FLOAT_MAT3x4: std::os::raw::c_uint = 0x8B68;
    pub const GL_FLOAT_MAT4: std::os::raw::c_uint = 0x8B5C;
    pub const GL_FLOAT_MAT4x2: std::os::raw::c_uint = 0x8B69;
    pub const GL_FLOAT_MAT4x3: std::os::raw::c_uint = 0x8B6A;
    pub const GL_FLOAT_VEC2: std::os::raw::c_uint = 0x8B50;
    pub const GL_FLOAT_VEC3: std::os::raw::c_uint = 0x8B51;
    pub const GL_FLOAT_VEC4: std::os::raw::c_uint = 0x8B52;
    pub const GL_FRAGMENT_SHADER: std::os::raw::c_uint = 0x8B30;
    pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: std::os::raw::c_uint = 0x8B8B;
    pub const GL_FRAMEBUFFER: std::os::raw::c_uint = 0x8D40;
    pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: std::os::raw::c_uint = 0x8215;
    pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: std::os::raw::c_uint = 0x8214;
    pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: std::os::raw::c_uint = 0x8210;
    pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: std::os::raw::c_uint = 0x8211;
    pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: std::os::raw::c_uint = 0x8216;
    pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: std::os::raw::c_uint = 0x8213;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: std::os::raw::c_uint = 0x8CD1;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: std::os::raw::c_uint = 0x8CD0;
    pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: std::os::raw::c_uint = 0x8212;
    pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: std::os::raw::c_uint = 0x8217;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: std::os::raw::c_uint = 0x8CD3;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: std::os::raw::c_uint = 0x8CD4;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: std::os::raw::c_uint = 0x8CD2;
    pub const GL_FRAMEBUFFER_BINDING: std::os::raw::c_uint = 0x8CA6;
    pub const GL_FRAMEBUFFER_COMPLETE: std::os::raw::c_uint = 0x8CD5;
    pub const GL_FRAMEBUFFER_DEFAULT: std::os::raw::c_uint = 0x8218;
    pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: std::os::raw::c_uint = 0x8CD6;
    pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS: std::os::raw::c_uint = 0x8CD9;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: std::os::raw::c_uint = 0x8CD7;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: std::os::raw::c_uint = 0x8D56;
    pub const GL_FRAMEBUFFER_UNDEFINED: std::os::raw::c_uint = 0x8219;
    pub const GL_FRAMEBUFFER_UNSUPPORTED: std::os::raw::c_uint = 0x8CDD;
    pub const GL_FRONT: std::os::raw::c_uint = 0x0404;
    pub const GL_FRONT_AND_BACK: std::os::raw::c_uint = 0x0408;
    pub const GL_FRONT_FACE: std::os::raw::c_uint = 0x0B46;
    pub const GL_FUNC_ADD: std::os::raw::c_uint = 0x8006;
    pub const GL_FUNC_REVERSE_SUBTRACT: std::os::raw::c_uint = 0x800B;
    pub const GL_FUNC_SUBTRACT: std::os::raw::c_uint = 0x800A;
    pub const GL_GENERATE_MIPMAP_HINT: std::os::raw::c_uint = 0x8192;
    pub const GL_GEQUAL: std::os::raw::c_uint = 0x0206;
    pub const GL_GREATER: std::os::raw::c_uint = 0x0204;
    pub const GL_GREEN: std::os::raw::c_uint = 0x1904;
    pub const GL_GREEN_BITS: std::os::raw::c_uint = 0x0D53;
    pub const GL_HALF_FLOAT: std::os::raw::c_uint = 0x140B;
    pub const GL_HIGH_FLOAT: std::os::raw::c_uint = 0x8DF2;
    pub const GL_HIGH_INT: std::os::raw::c_uint = 0x8DF5;
    pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: std::os::raw::c_uint = 0x8B9B;
    pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: std::os::raw::c_uint = 0x8B9A;
    pub const GL_INCR: std::os::raw::c_uint = 0x1E02;
    pub const GL_INCR_WRAP: std::os::raw::c_uint = 0x8507;
    pub const GL_INFO_LOG_LENGTH: std::os::raw::c_uint = 0x8B84;
    pub const GL_INT: std::os::raw::c_uint = 0x1404;
    pub const GL_INTERLEAVED_ATTRIBS: std::os::raw::c_uint = 0x8C8C;
    pub const GL_INT_2_10_10_10_REV: std::os::raw::c_uint = 0x8D9F;
    pub const GL_INT_SAMPLER_2D: std::os::raw::c_uint = 0x8DCA;
    pub const GL_INT_SAMPLER_2D_ARRAY: std::os::raw::c_uint = 0x8DCF;
    pub const GL_INT_SAMPLER_3D: std::os::raw::c_uint = 0x8DCB;
    pub const GL_INT_SAMPLER_CUBE: std::os::raw::c_uint = 0x8DCC;
    pub const GL_INT_VEC2: std::os::raw::c_uint = 0x8B53;
    pub const GL_INT_VEC3: std::os::raw::c_uint = 0x8B54;
    pub const GL_INT_VEC4: std::os::raw::c_uint = 0x8B55;
    pub const GL_INVALID_ENUM: std::os::raw::c_uint = 0x0500;
    pub const GL_INVALID_FRAMEBUFFER_OPERATION: std::os::raw::c_uint = 0x0506;
    pub const GL_INVALID_INDEX: std::os::raw::c_uint = 0xFFFFFFFF;
    pub const GL_INVALID_OPERATION: std::os::raw::c_uint = 0x0502;
    pub const GL_INVALID_VALUE: std::os::raw::c_uint = 0x0501;
    pub const GL_INVERT: std::os::raw::c_uint = 0x150A;
    pub const GL_KEEP: std::os::raw::c_uint = 0x1E00;
    pub const GL_LEQUAL: std::os::raw::c_uint = 0x0203;
    pub const GL_LESS: std::os::raw::c_uint = 0x0201;
    pub const GL_LINEAR: std::os::raw::c_uint = 0x2601;
    pub const GL_LINEAR_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2703;
    pub const GL_LINEAR_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2701;
    pub const GL_LINES: std::os::raw::c_uint = 0x0001;
    pub const GL_LINE_LOOP: std::os::raw::c_uint = 0x0002;
    pub const GL_LINE_STRIP: std::os::raw::c_uint = 0x0003;
    pub const GL_LINE_WIDTH: std::os::raw::c_uint = 0x0B21;
    pub const GL_LINK_STATUS: std::os::raw::c_uint = 0x8B82;
    pub const GL_LOW_FLOAT: std::os::raw::c_uint = 0x8DF0;
    pub const GL_LOW_INT: std::os::raw::c_uint = 0x8DF3;
    pub const GL_LUMINANCE: std::os::raw::c_uint = 0x1909;
    pub const GL_LUMINANCE_ALPHA: std::os::raw::c_uint = 0x190A;
    pub const GL_MAJOR_VERSION: std::os::raw::c_uint = 0x821B;
    pub const GL_MAP_FLUSH_EXPLICIT_BIT: std::os::raw::c_uint = 0x0010;
    pub const GL_MAP_INVALIDATE_BUFFER_BIT: std::os::raw::c_uint = 0x0008;
    pub const GL_MAP_INVALIDATE_RANGE_BIT: std::os::raw::c_uint = 0x0004;
    pub const GL_MAP_READ_BIT: std::os::raw::c_uint = 0x0001;
    pub const GL_MAP_UNSYNCHRONIZED_BIT: std::os::raw::c_uint = 0x0020;
    pub const GL_MAP_WRITE_BIT: std::os::raw::c_uint = 0x0002;
    pub const GL_MAX: std::os::raw::c_uint = 0x8008;
    pub const GL_MAX_3D_TEXTURE_SIZE: std::os::raw::c_uint = 0x8073;
    pub const GL_MAX_ARRAY_TEXTURE_LAYERS: std::os::raw::c_uint = 0x88FF;
    pub const GL_MAX_COLOR_ATTACHMENTS: std::os::raw::c_uint = 0x8CDF;
    pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8A33;
    pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8B4D;
    pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2E;
    pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8A31;
    pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: std::os::raw::c_uint = 0x851C;
    pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH_KHR: std::os::raw::c_uint = 0x826C;
    pub const GL_MAX_DEBUG_LOGGED_MESSAGES_KHR: std::os::raw::c_uint = 0x9144;
    pub const GL_MAX_DEBUG_MESSAGE_LENGTH_KHR: std::os::raw::c_uint = 0x9143;
    pub const GL_MAX_DRAW_BUFFERS: std::os::raw::c_uint = 0x8824;
    pub const GL_MAX_ELEMENTS_INDICES: std::os::raw::c_uint = 0x80E9;
    pub const GL_MAX_ELEMENTS_VERTICES: std::os::raw::c_uint = 0x80E8;
    pub const GL_MAX_ELEMENT_INDEX: std::os::raw::c_uint = 0x8D6B;
    pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: std::os::raw::c_uint = 0x9125;
    pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2D;
    pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8B49;
    pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: std::os::raw::c_uint = 0x8DFD;
    pub const GL_MAX_LABEL_LENGTH_KHR: std::os::raw::c_uint = 0x82E8;
    pub const GL_MAX_PROGRAM_TEXEL_OFFSET: std::os::raw::c_uint = 0x8905;
    pub const GL_MAX_RENDERBUFFER_SIZE: std::os::raw::c_uint = 0x84E8;
    pub const GL_MAX_SAMPLES: std::os::raw::c_uint = 0x8D57;
    pub const GL_MAX_SERVER_WAIT_TIMEOUT: std::os::raw::c_uint = 0x9111;
    pub const GL_MAX_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8872;
    pub const GL_MAX_TEXTURE_LOD_BIAS: std::os::raw::c_uint = 0x84FD;
    pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: std::os::raw::c_uint = 0x84FF;
    pub const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: std::os::raw::c_uint = 0x84FF;
    pub const GL_MAX_TEXTURE_SIZE: std::os::raw::c_uint = 0x0D33;
    pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: std::os::raw::c_uint = 0x8C8A;
    pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: std::os::raw::c_uint = 0x8C8B;
    pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: std::os::raw::c_uint = 0x8C80;
    pub const GL_MAX_UNIFORM_BLOCK_SIZE: std::os::raw::c_uint = 0x8A30;
    pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: std::os::raw::c_uint = 0x8A2F;
    pub const GL_MAX_VARYING_COMPONENTS: std::os::raw::c_uint = 0x8B4B;
    pub const GL_MAX_VARYING_VECTORS: std::os::raw::c_uint = 0x8DFC;
    pub const GL_MAX_VERTEX_ATTRIBS: std::os::raw::c_uint = 0x8869;
    pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: std::os::raw::c_uint = 0x9122;
    pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8B4C;
    pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2B;
    pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8B4A;
    pub const GL_MAX_VERTEX_UNIFORM_VECTORS: std::os::raw::c_uint = 0x8DFB;
    pub const GL_MAX_VIEWPORT_DIMS: std::os::raw::c_uint = 0x0D3A;
    pub const GL_MEDIUM_FLOAT: std::os::raw::c_uint = 0x8DF1;
    pub const GL_MEDIUM_INT: std::os::raw::c_uint = 0x8DF4;
    pub const GL_MIN: std::os::raw::c_uint = 0x8007;
    pub const GL_MINOR_VERSION: std::os::raw::c_uint = 0x821C;
    pub const GL_MIN_PROGRAM_TEXEL_OFFSET: std::os::raw::c_uint = 0x8904;
    pub const GL_MIRRORED_REPEAT: std::os::raw::c_uint = 0x8370;
    pub const GL_NEAREST: std::os::raw::c_uint = 0x2600;
    pub const GL_NEAREST_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2702;
    pub const GL_NEAREST_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2700;
    pub const GL_NEVER: std::os::raw::c_uint = 0x0200;
    pub const GL_NICEST: std::os::raw::c_uint = 0x1102;
    pub const GL_NONE: std::os::raw::c_uint = 0;
    pub const GL_NOTEQUAL: std::os::raw::c_uint = 0x0205;
    pub const GL_NO_ERROR: std::os::raw::c_uint = 0;
    pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: std::os::raw::c_uint = 0x86A2;
    pub const GL_NUM_EXTENSIONS: std::os::raw::c_uint = 0x821D;
    pub const GL_NUM_PROGRAM_BINARY_FORMATS: std::os::raw::c_uint = 0x87FE;
    pub const GL_NUM_SAMPLE_COUNTS: std::os::raw::c_uint = 0x9380;
    pub const GL_NUM_SHADER_BINARY_FORMATS: std::os::raw::c_uint = 0x8DF9;
    pub const GL_OBJECT_TYPE: std::os::raw::c_uint = 0x9112;
    pub const GL_ONE: std::os::raw::c_uint = 1;
    pub const GL_ONE_MINUS_CONSTANT_ALPHA: std::os::raw::c_uint = 0x8004;
    pub const GL_ONE_MINUS_CONSTANT_COLOR: std::os::raw::c_uint = 0x8002;
    pub const GL_ONE_MINUS_DST_ALPHA: std::os::raw::c_uint = 0x0305;
    pub const GL_ONE_MINUS_DST_COLOR: std::os::raw::c_uint = 0x0307;
    pub const GL_ONE_MINUS_SRC_ALPHA: std::os::raw::c_uint = 0x0303;
    pub const GL_ONE_MINUS_SRC_COLOR: std::os::raw::c_uint = 0x0301;
    pub const GL_OUT_OF_MEMORY: std::os::raw::c_uint = 0x0505;
    pub const GL_PACK_ALIGNMENT: std::os::raw::c_uint = 0x0D05;
    pub const GL_PACK_ROW_LENGTH: std::os::raw::c_uint = 0x0D02;
    pub const GL_PACK_SKIP_PIXELS: std::os::raw::c_uint = 0x0D04;
    pub const GL_PACK_SKIP_ROWS: std::os::raw::c_uint = 0x0D03;
    pub const GL_PIXEL_PACK_BUFFER: std::os::raw::c_uint = 0x88EB;
    pub const GL_PIXEL_PACK_BUFFER_BINDING: std::os::raw::c_uint = 0x88ED;
    pub const GL_PIXEL_UNPACK_BUFFER: std::os::raw::c_uint = 0x88EC;
    pub const GL_PIXEL_UNPACK_BUFFER_BINDING: std::os::raw::c_uint = 0x88EF;
    pub const GL_POINTS: std::os::raw::c_uint = 0x0000;
    pub const GL_POLYGON_OFFSET_FACTOR: std::os::raw::c_uint = 0x8038;
    pub const GL_POLYGON_OFFSET_FILL: std::os::raw::c_uint = 0x8037;
    pub const GL_POLYGON_OFFSET_UNITS: std::os::raw::c_uint = 0x2A00;
    pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: std::os::raw::c_uint = 0x8D69;
    pub const GL_PROGRAM_BINARY_FORMATS: std::os::raw::c_uint = 0x87FF;
    pub const GL_PROGRAM_BINARY_LENGTH: std::os::raw::c_uint = 0x8741;
    pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: std::os::raw::c_uint = 0x8257;
    pub const GL_PROGRAM_KHR: std::os::raw::c_uint = 0x82E2;
    pub const GL_PROGRAM_PIPELINE_KHR: std::os::raw::c_uint = 0x82E4;
    pub const GL_QUERY_KHR: std::os::raw::c_uint = 0x82E3;
    pub const GL_QUERY_RESULT: std::os::raw::c_uint = 0x8866;
    pub const GL_QUERY_RESULT_AVAILABLE: std::os::raw::c_uint = 0x8867;
    pub const GL_R11F_G11F_B10F: std::os::raw::c_uint = 0x8C3A;
    pub const GL_R16F: std::os::raw::c_uint = 0x822D;
    pub const GL_R16I: std::os::raw::c_uint = 0x8233;
    pub const GL_R16UI: std::os::raw::c_uint = 0x8234;
    pub const GL_R32F: std::os::raw::c_uint = 0x822E;
    pub const GL_R32I: std::os::raw::c_uint = 0x8235;
    pub const GL_R32UI: std::os::raw::c_uint = 0x8236;
    pub const GL_R8: std::os::raw::c_uint = 0x8229;
    pub const GL_R8I: std::os::raw::c_uint = 0x8231;
    pub const GL_R8UI: std::os::raw::c_uint = 0x8232;
    pub const GL_R8_SNORM: std::os::raw::c_uint = 0x8F94;
    pub const GL_RASTERIZER_DISCARD: std::os::raw::c_uint = 0x8C89;
    pub const GL_READ_BUFFER: std::os::raw::c_uint = 0x0C02;
    pub const GL_READ_FRAMEBUFFER: std::os::raw::c_uint = 0x8CA8;
    pub const GL_READ_FRAMEBUFFER_BINDING: std::os::raw::c_uint = 0x8CAA;
    pub const GL_RED: std::os::raw::c_uint = 0x1903;
    pub const GL_RED_BITS: std::os::raw::c_uint = 0x0D52;
    pub const GL_RED_INTEGER: std::os::raw::c_uint = 0x8D94;
    pub const GL_RENDERBUFFER: std::os::raw::c_uint = 0x8D41;
    pub const GL_RENDERBUFFER_ALPHA_SIZE: std::os::raw::c_uint = 0x8D53;
    pub const GL_RENDERBUFFER_BINDING: std::os::raw::c_uint = 0x8CA7;
    pub const GL_RENDERBUFFER_BLUE_SIZE: std::os::raw::c_uint = 0x8D52;
    pub const GL_RENDERBUFFER_DEPTH_SIZE: std::os::raw::c_uint = 0x8D54;
    pub const GL_RENDERBUFFER_GREEN_SIZE: std::os::raw::c_uint = 0x8D51;
    pub const GL_RENDERBUFFER_HEIGHT: std::os::raw::c_uint = 0x8D43;
    pub const GL_RENDERBUFFER_INTERNAL_FORMAT: std::os::raw::c_uint = 0x8D44;
    pub const GL_RENDERBUFFER_RED_SIZE: std::os::raw::c_uint = 0x8D50;
    pub const GL_RENDERBUFFER_SAMPLES: std::os::raw::c_uint = 0x8CAB;
    pub const GL_RENDERBUFFER_STENCIL_SIZE: std::os::raw::c_uint = 0x8D55;
    pub const GL_RENDERBUFFER_WIDTH: std::os::raw::c_uint = 0x8D42;
    pub const GL_RENDERER: std::os::raw::c_uint = 0x1F01;
    pub const GL_REPEAT: std::os::raw::c_uint = 0x2901;
    pub const GL_REPLACE: std::os::raw::c_uint = 0x1E01;
    pub const GL_RG: std::os::raw::c_uint = 0x8227;
    pub const GL_RG16F: std::os::raw::c_uint = 0x822F;
    pub const GL_RG16I: std::os::raw::c_uint = 0x8239;
    pub const GL_RG16UI: std::os::raw::c_uint = 0x823A;
    pub const GL_RG32F: std::os::raw::c_uint = 0x8230;
    pub const GL_RG32I: std::os::raw::c_uint = 0x823B;
    pub const GL_RG32UI: std::os::raw::c_uint = 0x823C;
    pub const GL_RG8: std::os::raw::c_uint = 0x822B;
    pub const GL_RG8I: std::os::raw::c_uint = 0x8237;
    pub const GL_RG8UI: std::os::raw::c_uint = 0x8238;
    pub const GL_RG8_SNORM: std::os::raw::c_uint = 0x8F95;
    pub const GL_RGB: std::os::raw::c_uint = 0x1907;
    pub const GL_RGB10_A2: std::os::raw::c_uint = 0x8059;
    pub const GL_RGB10_A2UI: std::os::raw::c_uint = 0x906F;
    pub const GL_RGB16F: std::os::raw::c_uint = 0x881B;
    pub const GL_RGB16I: std::os::raw::c_uint = 0x8D89;
    pub const GL_RGB16UI: std::os::raw::c_uint = 0x8D77;
    pub const GL_RGB32F: std::os::raw::c_uint = 0x8815;
    pub const GL_RGB32I: std::os::raw::c_uint = 0x8D83;
    pub const GL_RGB32UI: std::os::raw::c_uint = 0x8D71;
    pub const GL_RGB565: std::os::raw::c_uint = 0x8D62;
    pub const GL_RGB5_A1: std::os::raw::c_uint = 0x8057;
    pub const GL_RGB8: std::os::raw::c_uint = 0x8051;
    pub const GL_RGB8I: std::os::raw::c_uint = 0x8D8F;
    pub const GL_RGB8UI: std::os::raw::c_uint = 0x8D7D;
    pub const GL_RGB8_SNORM: std::os::raw::c_uint = 0x8F96;
    pub const GL_RGB9_E5: std::os::raw::c_uint = 0x8C3D;
    pub const GL_RGBA: std::os::raw::c_uint = 0x1908;
    pub const GL_RGBA16F: std::os::raw::c_uint = 0x881A;
    pub const GL_RGBA16I: std::os::raw::c_uint = 0x8D88;
    pub const GL_RGBA16UI: std::os::raw::c_uint = 0x8D76;
    pub const GL_RGBA32F: std::os::raw::c_uint = 0x8814;
    pub const GL_RGBA32I: std::os::raw::c_uint = 0x8D82;
    pub const GL_RGBA32UI: std::os::raw::c_uint = 0x8D70;
    pub const GL_RGBA4: std::os::raw::c_uint = 0x8056;
    pub const GL_RGBA8: std::os::raw::c_uint = 0x8058;
    pub const GL_RGBA8I: std::os::raw::c_uint = 0x8D8E;
    pub const GL_RGBA8UI: std::os::raw::c_uint = 0x8D7C;
    pub const GL_RGBA8_SNORM: std::os::raw::c_uint = 0x8F97;
    pub const GL_RGBA_INTEGER: std::os::raw::c_uint = 0x8D99;
    pub const GL_RGB_INTEGER: std::os::raw::c_uint = 0x8D98;
    pub const GL_RG_INTEGER: std::os::raw::c_uint = 0x8228;
    pub const GL_SAMPLER_2D: std::os::raw::c_uint = 0x8B5E;
    pub const GL_SAMPLER_2D_ARRAY: std::os::raw::c_uint = 0x8DC1;
    pub const GL_SAMPLER_2D_ARRAY_SHADOW: std::os::raw::c_uint = 0x8DC4;
    pub const GL_SAMPLER_2D_SHADOW: std::os::raw::c_uint = 0x8B62;
    pub const GL_SAMPLER_3D: std::os::raw::c_uint = 0x8B5F;
    pub const GL_SAMPLER_BINDING: std::os::raw::c_uint = 0x8919;
    pub const GL_SAMPLER_CUBE: std::os::raw::c_uint = 0x8B60;
    pub const GL_SAMPLER_CUBE_SHADOW: std::os::raw::c_uint = 0x8DC5;
    pub const GL_SAMPLER_KHR: std::os::raw::c_uint = 0x82E6;
    pub const GL_SAMPLES: std::os::raw::c_uint = 0x80A9;
    pub const GL_SAMPLE_ALPHA_TO_COVERAGE: std::os::raw::c_uint = 0x809E;
    pub const GL_SAMPLE_BUFFERS: std::os::raw::c_uint = 0x80A8;
    pub const GL_SAMPLE_COVERAGE: std::os::raw::c_uint = 0x80A0;
    pub const GL_SAMPLE_COVERAGE_INVERT: std::os::raw::c_uint = 0x80AB;
    pub const GL_SAMPLE_COVERAGE_VALUE: std::os::raw::c_uint = 0x80AA;
    pub const GL_SCISSOR_BOX: std::os::raw::c_uint = 0x0C10;
    pub const GL_SCISSOR_TEST: std::os::raw::c_uint = 0x0C11;
    pub const GL_SEPARATE_ATTRIBS: std::os::raw::c_uint = 0x8C8D;
    pub const GL_SHADER_BINARY_FORMATS: std::os::raw::c_uint = 0x8DF8;
    pub const GL_SHADER_COMPILER: std::os::raw::c_uint = 0x8DFA;
    pub const GL_SHADER_KHR: std::os::raw::c_uint = 0x82E1;
    pub const GL_SHADER_SOURCE_LENGTH: std::os::raw::c_uint = 0x8B88;
    pub const GL_SHADER_TYPE: std::os::raw::c_uint = 0x8B4F;
    pub const GL_SHADING_LANGUAGE_VERSION: std::os::raw::c_uint = 0x8B8C;
    pub const GL_SHORT: std::os::raw::c_uint = 0x1402;
    pub const GL_SIGNALED: std::os::raw::c_uint = 0x9119;
    pub const GL_SIGNED_NORMALIZED: std::os::raw::c_uint = 0x8F9C;
    pub const GL_SRC_ALPHA: std::os::raw::c_uint = 0x0302;
    pub const GL_SRC_ALPHA_SATURATE: std::os::raw::c_uint = 0x0308;
    pub const GL_SRC_COLOR: std::os::raw::c_uint = 0x0300;
    pub const GL_SRGB: std::os::raw::c_uint = 0x8C40;
    pub const GL_SRGB8: std::os::raw::c_uint = 0x8C41;
    pub const GL_SRGB8_ALPHA8: std::os::raw::c_uint = 0x8C43;
    pub const GL_STACK_OVERFLOW_KHR: std::os::raw::c_uint = 0x0503;
    pub const GL_STACK_UNDERFLOW_KHR: std::os::raw::c_uint = 0x0504;
    pub const GL_STATIC_COPY: std::os::raw::c_uint = 0x88E6;
    pub const GL_STATIC_DRAW: std::os::raw::c_uint = 0x88E4;
    pub const GL_STATIC_READ: std::os::raw::c_uint = 0x88E5;
    pub const GL_STENCIL: std::os::raw::c_uint = 0x1802;
    pub const GL_STENCIL_ATTACHMENT: std::os::raw::c_uint = 0x8D20;
    pub const GL_STENCIL_BACK_FAIL: std::os::raw::c_uint = 0x8801;
    pub const GL_STENCIL_BACK_FUNC: std::os::raw::c_uint = 0x8800;
    pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: std::os::raw::c_uint = 0x8802;
    pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: std::os::raw::c_uint = 0x8803;
    pub const GL_STENCIL_BACK_REF: std::os::raw::c_uint = 0x8CA3;
    pub const GL_STENCIL_BACK_VALUE_MASK: std::os::raw::c_uint = 0x8CA4;
    pub const GL_STENCIL_BACK_WRITEMASK: std::os::raw::c_uint = 0x8CA5;
    pub const GL_STENCIL_BITS: std::os::raw::c_uint = 0x0D57;
    pub const GL_STENCIL_BUFFER_BIT: std::os::raw::c_uint = 0x00000400;
    pub const GL_STENCIL_CLEAR_VALUE: std::os::raw::c_uint = 0x0B91;
    pub const GL_STENCIL_FAIL: std::os::raw::c_uint = 0x0B94;
    pub const GL_STENCIL_FUNC: std::os::raw::c_uint = 0x0B92;
    pub const GL_STENCIL_INDEX8: std::os::raw::c_uint = 0x8D48;
    pub const GL_STENCIL_PASS_DEPTH_FAIL: std::os::raw::c_uint = 0x0B95;
    pub const GL_STENCIL_PASS_DEPTH_PASS: std::os::raw::c_uint = 0x0B96;
    pub const GL_STENCIL_REF: std::os::raw::c_uint = 0x0B97;
    pub const GL_STENCIL_TEST: std::os::raw::c_uint = 0x0B90;
    pub const GL_STENCIL_VALUE_MASK: std::os::raw::c_uint = 0x0B93;
    pub const GL_STENCIL_WRITEMASK: std::os::raw::c_uint = 0x0B98;
    pub const GL_STREAM_COPY: std::os::raw::c_uint = 0x88E2;
    pub const GL_STREAM_DRAW: std::os::raw::c_uint = 0x88E0;
    pub const GL_STREAM_READ: std::os::raw::c_uint = 0x88E1;
    pub const GL_SUBPIXEL_BITS: std::os::raw::c_uint = 0x0D50;
    pub const GL_SYNC_CONDITION: std::os::raw::c_uint = 0x9113;
    pub const GL_SYNC_FENCE: std::os::raw::c_uint = 0x9116;
    pub const GL_SYNC_FLAGS: std::os::raw::c_uint = 0x9115;
    pub const GL_SYNC_FLUSH_COMMANDS_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_SYNC_GPU_COMMANDS_COMPLETE: std::os::raw::c_uint = 0x9117;
    pub const GL_SYNC_STATUS: std::os::raw::c_uint = 0x9114;
    pub const GL_TEXTURE: std::os::raw::c_uint = 0x1702;
    pub const GL_TEXTURE0: std::os::raw::c_uint = 0x84C0;
    pub const GL_TEXTURE1: std::os::raw::c_uint = 0x84C1;
    pub const GL_TEXTURE10: std::os::raw::c_uint = 0x84CA;
    pub const GL_TEXTURE11: std::os::raw::c_uint = 0x84CB;
    pub const GL_TEXTURE12: std::os::raw::c_uint = 0x84CC;
    pub const GL_TEXTURE13: std::os::raw::c_uint = 0x84CD;
    pub const GL_TEXTURE14: std::os::raw::c_uint = 0x84CE;
    pub const GL_TEXTURE15: std::os::raw::c_uint = 0x84CF;
    pub const GL_TEXTURE16: std::os::raw::c_uint = 0x84D0;
    pub const GL_TEXTURE17: std::os::raw::c_uint = 0x84D1;
    pub const GL_TEXTURE18: std::os::raw::c_uint = 0x84D2;
    pub const GL_TEXTURE19: std::os::raw::c_uint = 0x84D3;
    pub const GL_TEXTURE2: std::os::raw::c_uint = 0x84C2;
    pub const GL_TEXTURE20: std::os::raw::c_uint = 0x84D4;
    pub const GL_TEXTURE21: std::os::raw::c_uint = 0x84D5;
    pub const GL_TEXTURE22: std::os::raw::c_uint = 0x84D6;
    pub const GL_TEXTURE23: std::os::raw::c_uint = 0x84D7;
    pub const GL_TEXTURE24: std::os::raw::c_uint = 0x84D8;
    pub const GL_TEXTURE25: std::os::raw::c_uint = 0x84D9;
    pub const GL_TEXTURE26: std::os::raw::c_uint = 0x84DA;
    pub const GL_TEXTURE27: std::os::raw::c_uint = 0x84DB;
    pub const GL_TEXTURE28: std::os::raw::c_uint = 0x84DC;
    pub const GL_TEXTURE29: std::os::raw::c_uint = 0x84DD;
    pub const GL_TEXTURE3: std::os::raw::c_uint = 0x84C3;
    pub const GL_TEXTURE30: std::os::raw::c_uint = 0x84DE;
    pub const GL_TEXTURE31: std::os::raw::c_uint = 0x84DF;
    pub const GL_TEXTURE4: std::os::raw::c_uint = 0x84C4;
    pub const GL_TEXTURE5: std::os::raw::c_uint = 0x84C5;
    pub const GL_TEXTURE6: std::os::raw::c_uint = 0x84C6;
    pub const GL_TEXTURE7: std::os::raw::c_uint = 0x84C7;
    pub const GL_TEXTURE8: std::os::raw::c_uint = 0x84C8;
    pub const GL_TEXTURE9: std::os::raw::c_uint = 0x84C9;
    pub const GL_TEXTURE_2D: std::os::raw::c_uint = 0x0DE1;
    pub const GL_TEXTURE_2D_ARRAY: std::os::raw::c_uint = 0x8C1A;
    pub const GL_TEXTURE_3D: std::os::raw::c_uint = 0x806F;
    pub const GL_TEXTURE_BASE_LEVEL: std::os::raw::c_uint = 0x813C;
    pub const GL_TEXTURE_BINDING_2D: std::os::raw::c_uint = 0x8069;
    pub const GL_TEXTURE_BINDING_2D_ARRAY: std::os::raw::c_uint = 0x8C1D;
    pub const GL_TEXTURE_BINDING_3D: std::os::raw::c_uint = 0x806A;
    pub const GL_TEXTURE_BINDING_CUBE_MAP: std::os::raw::c_uint = 0x8514;
    pub const GL_TEXTURE_COMPARE_FUNC: std::os::raw::c_uint = 0x884D;
    pub const GL_TEXTURE_COMPARE_MODE: std::os::raw::c_uint = 0x884C;
    pub const GL_TEXTURE_CUBE_MAP: std::os::raw::c_uint = 0x8513;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: std::os::raw::c_uint = 0x8516;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: std::os::raw::c_uint = 0x8518;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: std::os::raw::c_uint = 0x851A;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: std::os::raw::c_uint = 0x8515;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: std::os::raw::c_uint = 0x8517;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: std::os::raw::c_uint = 0x8519;
    pub const GL_TEXTURE_IMMUTABLE_FORMAT: std::os::raw::c_uint = 0x912F;
    pub const GL_TEXTURE_IMMUTABLE_LEVELS: std::os::raw::c_uint = 0x82DF;
    pub const GL_TEXTURE_MAG_FILTER: std::os::raw::c_uint = 0x2800;
    pub const GL_TEXTURE_MAX_ANISOTROPY: std::os::raw::c_uint = 0x84FE;
    pub const GL_TEXTURE_MAX_ANISOTROPY_EXT: std::os::raw::c_uint = 0x84FE;
    pub const GL_TEXTURE_MAX_LEVEL: std::os::raw::c_uint = 0x813D;
    pub const GL_TEXTURE_MAX_LOD: std::os::raw::c_uint = 0x813B;
    pub const GL_TEXTURE_MIN_FILTER: std::os::raw::c_uint = 0x2801;
    pub const GL_TEXTURE_MIN_LOD: std::os::raw::c_uint = 0x813A;
    pub const GL_TEXTURE_SWIZZLE_A: std::os::raw::c_uint = 0x8E45;
    pub const GL_TEXTURE_SWIZZLE_B: std::os::raw::c_uint = 0x8E44;
    pub const GL_TEXTURE_SWIZZLE_G: std::os::raw::c_uint = 0x8E43;
    pub const GL_TEXTURE_SWIZZLE_R: std::os::raw::c_uint = 0x8E42;
    pub const GL_TEXTURE_WRAP_R: std::os::raw::c_uint = 0x8072;
    pub const GL_TEXTURE_WRAP_S: std::os::raw::c_uint = 0x2802;
    pub const GL_TEXTURE_WRAP_T: std::os::raw::c_uint = 0x2803;
    pub const GL_TIMEOUT_EXPIRED: std::os::raw::c_uint = 0x911B;
    pub const GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const GL_TRANSFORM_FEEDBACK: std::os::raw::c_uint = 0x8E22;
    pub const GL_TRANSFORM_FEEDBACK_ACTIVE: std::os::raw::c_uint = 0x8E24;
    pub const GL_TRANSFORM_FEEDBACK_BINDING: std::os::raw::c_uint = 0x8E25;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER: std::os::raw::c_uint = 0x8C8E;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE: std::os::raw::c_uint = 0x8E24;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: std::os::raw::c_uint = 0x8C8F;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: std::os::raw::c_uint = 0x8C7F;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED: std::os::raw::c_uint = 0x8E23;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: std::os::raw::c_uint = 0x8C85;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: std::os::raw::c_uint = 0x8C84;
    pub const GL_TRANSFORM_FEEDBACK_PAUSED: std::os::raw::c_uint = 0x8E23;
    pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: std::os::raw::c_uint = 0x8C88;
    pub const GL_TRANSFORM_FEEDBACK_VARYINGS: std::os::raw::c_uint = 0x8C83;
    pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: std::os::raw::c_uint = 0x8C76;
    pub const GL_TRIANGLES: std::os::raw::c_uint = 0x0004;
    pub const GL_TRIANGLE_FAN: std::os::raw::c_uint = 0x0006;
    pub const GL_TRIANGLE_STRIP: std::os::raw::c_uint = 0x0005;
    pub const GL_TRUE: std::os::raw::c_uchar = 1;
    pub const GL_UNIFORM_ARRAY_STRIDE: std::os::raw::c_uint = 0x8A3C;
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: std::os::raw::c_uint = 0x8A42;
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: std::os::raw::c_uint = 0x8A43;
    pub const GL_UNIFORM_BLOCK_BINDING: std::os::raw::c_uint = 0x8A3F;
    pub const GL_UNIFORM_BLOCK_DATA_SIZE: std::os::raw::c_uint = 0x8A40;
    pub const GL_UNIFORM_BLOCK_INDEX: std::os::raw::c_uint = 0x8A3A;
    pub const GL_UNIFORM_BLOCK_NAME_LENGTH: std::os::raw::c_uint = 0x8A41;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: std::os::raw::c_uint = 0x8A46;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: std::os::raw::c_uint = 0x8A44;
    pub const GL_UNIFORM_BUFFER: std::os::raw::c_uint = 0x8A11;
    pub const GL_UNIFORM_BUFFER_BINDING: std::os::raw::c_uint = 0x8A28;
    pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: std::os::raw::c_uint = 0x8A34;
    pub const GL_UNIFORM_BUFFER_SIZE: std::os::raw::c_uint = 0x8A2A;
    pub const GL_UNIFORM_BUFFER_START: std::os::raw::c_uint = 0x8A29;
    pub const GL_UNIFORM_IS_ROW_MAJOR: std::os::raw::c_uint = 0x8A3E;
    pub const GL_UNIFORM_MATRIX_STRIDE: std::os::raw::c_uint = 0x8A3D;
    pub const GL_UNIFORM_NAME_LENGTH: std::os::raw::c_uint = 0x8A39;
    pub const GL_UNIFORM_OFFSET: std::os::raw::c_uint = 0x8A3B;
    pub const GL_UNIFORM_SIZE: std::os::raw::c_uint = 0x8A38;
    pub const GL_UNIFORM_TYPE: std::os::raw::c_uint = 0x8A37;
    pub const GL_UNPACK_ALIGNMENT: std::os::raw::c_uint = 0x0CF5;
    pub const GL_UNPACK_IMAGE_HEIGHT: std::os::raw::c_uint = 0x806E;
    pub const GL_UNPACK_ROW_LENGTH: std::os::raw::c_uint = 0x0CF2;
    pub const GL_UNPACK_SKIP_IMAGES: std::os::raw::c_uint = 0x806D;
    pub const GL_UNPACK_SKIP_PIXELS: std::os::raw::c_uint = 0x0CF4;
    pub const GL_UNPACK_SKIP_ROWS: std::os::raw::c_uint = 0x0CF3;
    pub const GL_UNSIGNALED: std::os::raw::c_uint = 0x9118;
    pub const GL_UNSIGNED_BYTE: std::os::raw::c_uint = 0x1401;
    pub const GL_UNSIGNED_INT: std::os::raw::c_uint = 0x1405;
    pub const GL_UNSIGNED_INT_10F_11F_11F_REV: std::os::raw::c_uint = 0x8C3B;
    pub const GL_UNSIGNED_INT_24_8: std::os::raw::c_uint = 0x84FA;
    pub const GL_UNSIGNED_INT_2_10_10_10_REV: std::os::raw::c_uint = 0x8368;
    pub const GL_UNSIGNED_INT_5_9_9_9_REV: std::os::raw::c_uint = 0x8C3E;
    pub const GL_UNSIGNED_INT_SAMPLER_2D: std::os::raw::c_uint = 0x8DD2;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: std::os::raw::c_uint = 0x8DD7;
    pub const GL_UNSIGNED_INT_SAMPLER_3D: std::os::raw::c_uint = 0x8DD3;
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE: std::os::raw::c_uint = 0x8DD4;
    pub const GL_UNSIGNED_INT_VEC2: std::os::raw::c_uint = 0x8DC6;
    pub const GL_UNSIGNED_INT_VEC3: std::os::raw::c_uint = 0x8DC7;
    pub const GL_UNSIGNED_INT_VEC4: std::os::raw::c_uint = 0x8DC8;
    pub const GL_UNSIGNED_NORMALIZED: std::os::raw::c_uint = 0x8C17;
    pub const GL_UNSIGNED_SHORT: std::os::raw::c_uint = 0x1403;
    pub const GL_UNSIGNED_SHORT_4_4_4_4: std::os::raw::c_uint = 0x8033;
    pub const GL_UNSIGNED_SHORT_5_5_5_1: std::os::raw::c_uint = 0x8034;
    pub const GL_UNSIGNED_SHORT_5_6_5: std::os::raw::c_uint = 0x8363;
    pub const GL_VALIDATE_STATUS: std::os::raw::c_uint = 0x8B83;
    pub const GL_VENDOR: std::os::raw::c_uint = 0x1F00;
    pub const GL_VERSION: std::os::raw::c_uint = 0x1F02;
    pub const GL_VERTEX_ARRAY_BINDING: std::os::raw::c_uint = 0x85B5;
    pub const GL_VERTEX_ARRAY_KHR: std::os::raw::c_uint = 0x8074;
    pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889F;
    pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: std::os::raw::c_uint = 0x88FE;
    pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: std::os::raw::c_uint = 0x8622;
    pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: std::os::raw::c_uint = 0x88FD;
    pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: std::os::raw::c_uint = 0x886A;
    pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: std::os::raw::c_uint = 0x8645;
    pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: std::os::raw::c_uint = 0x8623;
    pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: std::os::raw::c_uint = 0x8624;
    pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: std::os::raw::c_uint = 0x8625;
    pub const GL_VERTEX_SHADER: std::os::raw::c_uint = 0x8B31;
    pub const GL_VIEWPORT: std::os::raw::c_uint = 0x0BA2;
    pub const GL_WAIT_FAILED: std::os::raw::c_uint = 0x911D;
    pub const GL_ZERO: std::os::raw::c_uint = 0;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code)]

    use super::storage;
    use super::types::*;
    use std;
    use std::mem;

    #[inline]
    pub unsafe fn ActiveTexture(texture: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ActiveTexture.ptr)(texture)
    }
    #[inline]
    pub unsafe fn AttachShader(program: GLuint, shader: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::AttachShader.ptr)(
            program, shader,
        )
    }
    #[inline]
    pub unsafe fn BeginQuery(target: GLenum, id: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BeginQuery.ptr)(
            target, id,
        )
    }
    #[inline]
    pub unsafe fn BeginTransformFeedback(primitiveMode: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::BeginTransformFeedback.ptr)(
            primitiveMode,
        )
    }
    #[inline]
    pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(
            storage::BindAttribLocation.ptr,
        )(program, index, name)
    }
    #[inline]
    pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindBuffer.ptr)(
            target, buffer,
        )
    }
    #[inline]
    pub unsafe fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(
            storage::BindBufferBase.ptr,
        )(target, index, buffer)
    }
    #[inline]
    pub unsafe fn BindBufferRange(
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr) -> ()>(
            storage::BindBufferRange.ptr,
        )(target, index, buffer, offset, size)
    }
    #[inline]
    pub unsafe fn BindFramebuffer(target: GLenum, framebuffer: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindFramebuffer.ptr)(
            target,
            framebuffer,
        )
    }
    #[inline]
    pub unsafe fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindRenderbuffer.ptr)(
            target,
            renderbuffer,
        )
    }
    #[inline]
    pub unsafe fn BindSampler(unit: GLuint, sampler: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::BindSampler.ptr)(
            unit, sampler,
        )
    }
    #[inline]
    pub unsafe fn BindTexture(target: GLenum, texture: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindTexture.ptr)(
            target, texture,
        )
    }
    #[inline]
    pub unsafe fn BindTransformFeedback(target: GLenum, id: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            storage::BindTransformFeedback.ptr,
        )(target, id)
    }
    #[inline]
    pub unsafe fn BindVertexArray(array: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::BindVertexArray.ptr)(array)
    }
    #[inline]
    pub unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(
            storage::BlendColor.ptr,
        )(red, green, blue, alpha)
    }
    #[inline]
    pub unsafe fn BlendEquation(mode: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::BlendEquation.ptr)(mode)
    }
    #[inline]
    pub unsafe fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(
            storage::BlendEquationSeparate.ptr,
        )(modeRGB, modeAlpha)
    }
    #[inline]
    pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendFunc.ptr)(
            sfactor, dfactor,
        )
    }
    #[inline]
    pub unsafe fn BlendFuncSeparate(
        sfactorRGB: GLenum,
        dfactorRGB: GLenum,
        sfactorAlpha: GLenum,
        dfactorAlpha: GLenum,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(
            storage::BlendFuncSeparate.ptr,
        )(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
    }
    #[inline]
    pub unsafe fn BlitFramebuffer(
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLbitfield,
                GLenum,
            ) -> (),
        >(storage::BlitFramebuffer.ptr)(
            srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter,
        )
    }
    #[inline]
    pub unsafe fn BufferData(
        target: GLenum,
        size: GLsizeiptr,
        data: *const std::os::raw::c_void,
        usage: GLenum,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizeiptr, *const std::os::raw::c_void, GLenum) -> (),
        >(storage::BufferData.ptr)(target, size, data, usage)
    }
    #[inline]
    pub unsafe fn BufferSubData(
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const std::os::raw::c_void) -> (),
        >(storage::BufferSubData.ptr)(target, offset, size, data)
    }
    #[inline]
    pub unsafe fn CheckFramebufferStatus(target: GLenum) -> GLenum {
        mem::transmute::<_, extern "system" fn(GLenum) -> GLenum>(
            storage::CheckFramebufferStatus.ptr,
        )(target)
    }
    #[inline]
    pub unsafe fn Clear(mask: GLbitfield) -> () {
        mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::Clear.ptr)(mask)
    }
    #[inline]
    pub unsafe fn ClearBufferfi(
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLfloat, GLint) -> ()>(
            storage::ClearBufferfi.ptr,
        )(buffer, drawbuffer, depth, stencil)
    }
    #[inline]
    pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLfloat) -> ()>(
            storage::ClearBufferfv.ptr,
        )(buffer, drawbuffer, value)
    }
    #[inline]
    pub unsafe fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLint) -> ()>(
            storage::ClearBufferiv.ptr,
        )(buffer, drawbuffer, value)
    }
    #[inline]
    pub unsafe fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLuint) -> ()>(
            storage::ClearBufferuiv.ptr,
        )(buffer, drawbuffer, value)
    }
    #[inline]
    pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(
            storage::ClearColor.ptr,
        )(red, green, blue, alpha)
    }
    #[inline]
    pub unsafe fn ClearDepthf(d: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::ClearDepthf.ptr)(d)
    }
    #[inline]
    pub unsafe fn ClearStencil(s: GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::ClearStencil.ptr)(s)
    }
    #[inline]
    pub unsafe fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
        mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum>(
            storage::ClientWaitSync.ptr,
        )(sync, flags, timeout)
    }
    #[inline]
    pub unsafe fn ColorMask(
        red: GLboolean,
        green: GLboolean,
        blue: GLboolean,
        alpha: GLboolean,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(
            storage::ColorMask.ptr,
        )(red, green, blue, alpha)
    }
    #[inline]
    pub unsafe fn CompileShader(shader: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::CompileShader.ptr)(shader)
    }
    #[inline]
    pub unsafe fn CompressedTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLenum,
                GLsizei,
                GLsizei,
                GLint,
                GLsizei,
                *const std::os::raw::c_void,
            ) -> (),
        >(storage::CompressedTexImage2D.ptr)(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            imageSize,
            data,
        )
    }
    #[inline]
    pub unsafe fn CompressedTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLenum,
                GLsizei,
                GLsizei,
                GLsizei,
                GLint,
                GLsizei,
                *const std::os::raw::c_void,
            ) -> (),
        >(storage::CompressedTexImage3D.ptr)(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            imageSize,
            data,
        )
    }
    #[inline]
    pub unsafe fn CompressedTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLenum,
                GLsizei,
                *const std::os::raw::c_void,
            ) -> (),
        >(storage::CompressedTexSubImage2D.ptr)(
            target, level, xoffset, yoffset, width, height, format, imageSize, data,
        )
    }
    #[inline]
    pub unsafe fn CompressedTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLenum,
                GLsizei,
                *const std::os::raw::c_void,
            ) -> (),
        >(storage::CompressedTexSubImage3D.ptr)(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data,
        )
    }
    #[inline]
    pub unsafe fn CopyBufferSubData(
        readTarget: GLenum,
        writeTarget: GLenum,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr) -> ()>(
            storage::CopyBufferSubData.ptr,
        )(readTarget, writeTarget, readOffset, writeOffset, size)
    }
    #[inline]
    pub unsafe fn CopyTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint) -> (),
        >(storage::CopyTexImage2D.ptr)(
            target, level, internalformat, x, y, width, height, border
        )
    }
    #[inline]
    pub unsafe fn CopyTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> (),
        >(storage::CopyTexSubImage2D.ptr)(
            target, level, xoffset, yoffset, x, y, width, height
        )
    }
    #[inline]
    pub unsafe fn CopyTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
            ) -> (),
        >(storage::CopyTexSubImage3D.ptr)(
            target, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    }
    #[inline]
    pub unsafe fn CreateProgram() -> GLuint {
        mem::transmute::<_, extern "system" fn() -> GLuint>(storage::CreateProgram.ptr)()
    }
    #[inline]
    pub unsafe fn CreateShader(type_: GLenum) -> GLuint {
        mem::transmute::<_, extern "system" fn(GLenum) -> GLuint>(storage::CreateShader.ptr)(type_)
    }
    #[inline]
    pub unsafe fn CullFace(mode: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::CullFace.ptr)(mode)
    }
    #[inline]
    pub unsafe fn DebugMessageCallback(
        callback: GLDEBUGPROC,
        userParam: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLDEBUGPROC, *const std::os::raw::c_void) -> ()>(
            storage::DebugMessageCallback.ptr,
        )(callback, userParam)
    }
    #[inline]
    pub unsafe fn DebugMessageCallbackKHR(
        callback: GLDEBUGPROCKHR,
        userParam: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLDEBUGPROCKHR, *const std::os::raw::c_void) -> ()>(
            storage::DebugMessageCallbackKHR.ptr,
        )(callback, userParam)
    }
    #[inline]
    pub unsafe fn DebugMessageControl(
        source: GLenum,
        type_: GLenum,
        severity: GLenum,
        count: GLsizei,
        ids: *const GLuint,
        enabled: GLboolean,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean) -> (),
        >(storage::DebugMessageControl.ptr)(source, type_, severity, count, ids, enabled)
    }
    #[inline]
    pub unsafe fn DebugMessageControlKHR(
        source: GLenum,
        type_: GLenum,
        severity: GLenum,
        count: GLsizei,
        ids: *const GLuint,
        enabled: GLboolean,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean) -> (),
        >(storage::DebugMessageControlKHR.ptr)(source, type_, severity, count, ids, enabled)
    }
    #[inline]
    pub unsafe fn DebugMessageInsert(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        buf: *const GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar) -> (),
        >(storage::DebugMessageInsert.ptr)(source, type_, id, severity, length, buf)
    }
    #[inline]
    pub unsafe fn DebugMessageInsertKHR(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        buf: *const GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar) -> (),
        >(storage::DebugMessageInsertKHR.ptr)(source, type_, id, severity, length, buf)
    }
    #[inline]
    pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            storage::DeleteBuffers.ptr,
        )(n, buffers)
    }
    #[inline]
    pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            storage::DeleteFramebuffers.ptr,
        )(n, framebuffers)
    }
    #[inline]
    pub unsafe fn DeleteProgram(program: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteProgram.ptr)(program)
    }
    #[inline]
    pub unsafe fn DeleteQueries(n: GLsizei, ids: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            storage::DeleteQueries.ptr,
        )(n, ids)
    }
    #[inline]
    pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            storage::DeleteRenderbuffers.ptr,
        )(n, renderbuffers)
    }
    #[inline]
    pub unsafe fn DeleteSamplers(count: GLsizei, samplers: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            storage::DeleteSamplers.ptr,
        )(count, samplers)
    }
    #[inline]
    pub unsafe fn DeleteShader(shader: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteShader.ptr)(shader)
    }
    #[inline]
    pub unsafe fn DeleteSync(sync: GLsync) -> () {
        mem::transmute::<_, extern "system" fn(GLsync) -> ()>(storage::DeleteSync.ptr)(sync)
    }
    #[inline]
    pub unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            storage::DeleteTextures.ptr,
        )(n, textures)
    }
    #[inline]
    pub unsafe fn DeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            storage::DeleteTransformFeedbacks.ptr,
        )(n, ids)
    }
    #[inline]
    pub unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            storage::DeleteVertexArrays.ptr,
        )(n, arrays)
    }
    #[inline]
    pub unsafe fn DepthFunc(func: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DepthFunc.ptr)(func)
    }
    #[inline]
    pub unsafe fn DepthMask(flag: GLboolean) -> () {
        mem::transmute::<_, extern "system" fn(GLboolean) -> ()>(storage::DepthMask.ptr)(flag)
    }
    #[inline]
    pub unsafe fn DepthRangef(n: GLfloat, f: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::DepthRangef.ptr)(
            n, f,
        )
    }
    #[inline]
    pub unsafe fn DetachShader(program: GLuint, shader: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::DetachShader.ptr)(
            program, shader,
        )
    }
    #[inline]
    pub unsafe fn Disable(cap: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Disable.ptr)(cap)
    }
    #[inline]
    pub unsafe fn DisableVertexAttribArray(index: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DisableVertexAttribArray.ptr)(
            index,
        )
    }
    #[inline]
    pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei) -> ()>(
            storage::DrawArrays.ptr,
        )(mode, first, count)
    }
    #[inline]
    pub unsafe fn DrawArraysInstanced(
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei) -> ()>(
            storage::DrawArraysInstanced.ptr,
        )(mode, first, count, instancecount)
    }
    #[inline]
    pub unsafe fn DrawBuffers(n: GLsizei, bufs: *const GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLenum) -> ()>(
            storage::DrawBuffers.ptr,
        )(n, bufs)
    }
    #[inline]
    pub unsafe fn DrawElements(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void) -> (),
        >(storage::DrawElements.ptr)(mode, count, type_, indices)
    }
    #[inline]
    pub unsafe fn DrawElementsInstanced(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const std::os::raw::c_void,
        instancecount: GLsizei,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void, GLsizei) -> (),
        >(storage::DrawElementsInstanced.ptr)(mode, count, type_, indices, instancecount)
    }
    #[inline]
    pub unsafe fn DrawRangeElements(
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLuint,
                GLuint,
                GLsizei,
                GLenum,
                *const std::os::raw::c_void,
            ) -> (),
        >(storage::DrawRangeElements.ptr)(mode, start, end, count, type_, indices)
    }
    #[inline]
    pub unsafe fn Enable(cap: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Enable.ptr)(cap)
    }
    #[inline]
    pub unsafe fn EnableVertexAttribArray(index: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::EnableVertexAttribArray.ptr)(
            index,
        )
    }
    #[inline]
    pub unsafe fn EndQuery(target: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::EndQuery.ptr)(target)
    }
    #[inline]
    pub unsafe fn EndTransformFeedback() -> () {
        mem::transmute::<_, extern "system" fn() -> ()>(storage::EndTransformFeedback.ptr)()
    }
    #[inline]
    pub unsafe fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync {
        mem::transmute::<_, extern "system" fn(GLenum, GLbitfield) -> GLsync>(
            storage::FenceSync.ptr,
        )(condition, flags)
    }
    #[inline]
    pub unsafe fn Finish() -> () {
        mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.ptr)()
    }
    #[inline]
    pub unsafe fn Flush() -> () {
        mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.ptr)()
    }
    #[inline]
    pub unsafe fn FlushMappedBufferRange(
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr) -> ()>(
            storage::FlushMappedBufferRange.ptr,
        )(target, offset, length)
    }
    #[inline]
    pub unsafe fn FramebufferRenderbuffer(
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint) -> ()>(
            storage::FramebufferRenderbuffer.ptr,
        )(target, attachment, renderbuffertarget, renderbuffer)
    }
    #[inline]
    pub unsafe fn FramebufferTexture2D(
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(
            storage::FramebufferTexture2D.ptr,
        )(target, attachment, textarget, texture, level)
    }
    #[inline]
    pub unsafe fn FramebufferTextureLayer(
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint) -> ()>(
            storage::FramebufferTextureLayer.ptr,
        )(target, attachment, texture, level, layer)
    }
    #[inline]
    pub unsafe fn FrontFace(mode: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::FrontFace.ptr)(mode)
    }
    #[inline]
    pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenBuffers.ptr)(
            n, buffers,
        )
    }
    #[inline]
    pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            storage::GenFramebuffers.ptr,
        )(n, framebuffers)
    }
    #[inline]
    pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenQueries.ptr)(
            n, ids,
        )
    }
    #[inline]
    pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            storage::GenRenderbuffers.ptr,
        )(n, renderbuffers)
    }
    #[inline]
    pub unsafe fn GenSamplers(count: GLsizei, samplers: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            storage::GenSamplers.ptr,
        )(count, samplers)
    }
    #[inline]
    pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            storage::GenTextures.ptr,
        )(n, textures)
    }
    #[inline]
    pub unsafe fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            storage::GenTransformFeedbacks.ptr,
        )(n, ids)
    }
    #[inline]
    pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            storage::GenVertexArrays.ptr,
        )(n, arrays)
    }
    #[inline]
    pub unsafe fn GenerateMipmap(target: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::GenerateMipmap.ptr)(target)
    }
    #[inline]
    pub unsafe fn GetActiveAttrib(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLuint,
                GLsizei,
                *mut GLsizei,
                *mut GLint,
                *mut GLenum,
                *mut GLchar,
            ) -> (),
        >(storage::GetActiveAttrib.ptr)(program, index, bufSize, length, size, type_, name)
    }
    #[inline]
    pub unsafe fn GetActiveUniform(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLuint,
                GLsizei,
                *mut GLsizei,
                *mut GLint,
                *mut GLenum,
                *mut GLchar,
            ) -> (),
        >(storage::GetActiveUniform.ptr)(program, index, bufSize, length, size, type_, name)
    }
    #[inline]
    pub unsafe fn GetActiveUniformBlockName(
        program: GLuint,
        uniformBlockIndex: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        uniformBlockName: *mut GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
        >(storage::GetActiveUniformBlockName.ptr)(
            program,
            uniformBlockIndex,
            bufSize,
            length,
            uniformBlockName,
        )
    }
    #[inline]
    pub unsafe fn GetActiveUniformBlockiv(
        program: GLuint,
        uniformBlockIndex: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(
            storage::GetActiveUniformBlockiv.ptr,
        )(program, uniformBlockIndex, pname, params)
    }
    #[inline]
    pub unsafe fn GetActiveUniformsiv(
        program: GLuint,
        uniformCount: GLsizei,
        uniformIndices: *const GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint) -> (),
        >(storage::GetActiveUniformsiv.ptr)(
            program, uniformCount, uniformIndices, pname, params
        )
    }
    #[inline]
    pub unsafe fn GetAttachedShaders(
        program: GLuint,
        maxCount: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint) -> ()>(
            storage::GetAttachedShaders.ptr,
        )(program, maxCount, count, shaders)
    }
    #[inline]
    pub unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(
            storage::GetAttribLocation.ptr,
        )(program, name)
    }
    #[inline]
    pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, *mut GLboolean) -> ()>(
            storage::GetBooleanv.ptr,
        )(pname, data)
    }
    #[inline]
    pub unsafe fn GetBufferParameteri64v(
        target: GLenum,
        pname: GLenum,
        params: *mut GLint64,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint64) -> ()>(
            storage::GetBufferParameteri64v.ptr,
        )(target, pname, params)
    }
    #[inline]
    pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
            storage::GetBufferParameteriv.ptr,
        )(target, pname, params)
    }
    #[inline]
    pub unsafe fn GetBufferPointerv(
        target: GLenum,
        pname: GLenum,
        params: *mut *mut std::os::raw::c_void,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut *mut std::os::raw::c_void) -> ()>(
            storage::GetBufferPointerv.ptr,
        )(target, pname, params)
    }
    #[inline]
    pub unsafe fn GetDebugMessageLog(
        count: GLuint,
        bufSize: GLsizei,
        sources: *mut GLenum,
        types: *mut GLenum,
        ids: *mut GLuint,
        severities: *mut GLenum,
        lengths: *mut GLsizei,
        messageLog: *mut GLchar,
    ) -> GLuint {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLsizei,
                *mut GLenum,
                *mut GLenum,
                *mut GLuint,
                *mut GLenum,
                *mut GLsizei,
                *mut GLchar,
            ) -> GLuint,
        >(storage::GetDebugMessageLog.ptr)(
            count, bufSize, sources, types, ids, severities, lengths, messageLog,
        )
    }
    #[inline]
    pub unsafe fn GetDebugMessageLogKHR(
        count: GLuint,
        bufSize: GLsizei,
        sources: *mut GLenum,
        types: *mut GLenum,
        ids: *mut GLuint,
        severities: *mut GLenum,
        lengths: *mut GLsizei,
        messageLog: *mut GLchar,
    ) -> GLuint {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLsizei,
                *mut GLenum,
                *mut GLenum,
                *mut GLuint,
                *mut GLenum,
                *mut GLsizei,
                *mut GLchar,
            ) -> GLuint,
        >(storage::GetDebugMessageLogKHR.ptr)(
            count, bufSize, sources, types, ids, severities, lengths, messageLog,
        )
    }
    #[inline]
    pub unsafe fn GetError() -> GLenum {
        mem::transmute::<_, extern "system" fn() -> GLenum>(storage::GetError.ptr)()
    }
    #[inline]
    pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>(storage::GetFloatv.ptr)(
            pname, data,
        )
    }
    #[inline]
    pub unsafe fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(
            storage::GetFragDataLocation.ptr,
        )(program, name)
    }
    #[inline]
    pub unsafe fn GetFramebufferAttachmentParameteriv(
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, *mut GLint) -> ()>(
            storage::GetFramebufferAttachmentParameteriv.ptr,
        )(target, attachment, pname, params)
    }
    #[inline]
    pub unsafe fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint64) -> ()>(
            storage::GetInteger64i_v.ptr,
        )(target, index, data)
    }
    #[inline]
    pub unsafe fn GetInteger64v(pname: GLenum, data: *mut GLint64) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, *mut GLint64) -> ()>(
            storage::GetInteger64v.ptr,
        )(pname, data)
    }
    #[inline]
    pub unsafe fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint) -> ()>(
            storage::GetIntegeri_v.ptr,
        )(target, index, data)
    }
    #[inline]
    pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, *mut GLint) -> ()>(storage::GetIntegerv.ptr)(
            pname, data,
        )
    }
    #[inline]
    pub unsafe fn GetInternalformativ(
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
        bufSize: GLsizei,
        params: *mut GLint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint) -> ()>(
            storage::GetInternalformativ.ptr,
        )(target, internalformat, pname, bufSize, params)
    }
    #[inline]
    pub unsafe fn GetObjectLabel(
        identifier: GLenum,
        name: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
        >(storage::GetObjectLabel.ptr)(identifier, name, bufSize, length, label)
    }
    #[inline]
    pub unsafe fn GetObjectLabelKHR(
        identifier: GLenum,
        name: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
        >(storage::GetObjectLabelKHR.ptr)(identifier, name, bufSize, length, label)
    }
    #[inline]
    pub unsafe fn GetObjectPtrLabel(
        ptr: *const std::os::raw::c_void,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                *const std::os::raw::c_void,
                GLsizei,
                *mut GLsizei,
                *mut GLchar,
            ) -> (),
        >(storage::GetObjectPtrLabel.ptr)(ptr, bufSize, length, label)
    }
    #[inline]
    pub unsafe fn GetObjectPtrLabelKHR(
        ptr: *const std::os::raw::c_void,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                *const std::os::raw::c_void,
                GLsizei,
                *mut GLsizei,
                *mut GLchar,
            ) -> (),
        >(storage::GetObjectPtrLabelKHR.ptr)(ptr, bufSize, length, label)
    }
    #[inline]
    pub unsafe fn GetPointerv(pname: GLenum, params: *mut *mut std::os::raw::c_void) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, *mut *mut std::os::raw::c_void) -> ()>(
            storage::GetPointerv.ptr,
        )(pname, params)
    }
    #[inline]
    pub unsafe fn GetPointervKHR(pname: GLenum, params: *mut *mut std::os::raw::c_void) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, *mut *mut std::os::raw::c_void) -> ()>(
            storage::GetPointervKHR.ptr,
        )(pname, params)
    }
    #[inline]
    pub unsafe fn GetProgramBinary(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        binaryFormat: *mut GLenum,
        binary: *mut std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLsizei,
                *mut GLsizei,
                *mut GLenum,
                *mut std::os::raw::c_void,
            ) -> (),
        >(storage::GetProgramBinary.ptr)(program, bufSize, length, binaryFormat, binary)
    }
    #[inline]
    pub unsafe fn GetProgramInfoLog(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(
            storage::GetProgramInfoLog.ptr,
        )(program, bufSize, length, infoLog)
    }
    #[inline]
    pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            storage::GetProgramiv.ptr,
        )(program, pname, params)
    }
    #[inline]
    pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
            storage::GetQueryObjectuiv.ptr,
        )(id, pname, params)
    }
    #[inline]
    pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
            storage::GetQueryiv.ptr,
        )(target, pname, params)
    }
    #[inline]
    pub unsafe fn GetRenderbufferParameteriv(
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
            storage::GetRenderbufferParameteriv.ptr,
        )(target, pname, params)
    }
    #[inline]
    pub unsafe fn GetSamplerParameterfv(
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(
            storage::GetSamplerParameterfv.ptr,
        )(sampler, pname, params)
    }
    #[inline]
    pub unsafe fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            storage::GetSamplerParameteriv.ptr,
        )(sampler, pname, params)
    }
    #[inline]
    pub unsafe fn GetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(
            storage::GetShaderInfoLog.ptr,
        )(shader, bufSize, length, infoLog)
    }
    #[inline]
    pub unsafe fn GetShaderPrecisionFormat(
        shadertype: GLenum,
        precisiontype: GLenum,
        range: *mut GLint,
        precision: *mut GLint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint) -> ()>(
            storage::GetShaderPrecisionFormat.ptr,
        )(shadertype, precisiontype, range, precision)
    }
    #[inline]
    pub unsafe fn GetShaderSource(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(
            storage::GetShaderSource.ptr,
        )(shader, bufSize, length, source)
    }
    #[inline]
    pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            storage::GetShaderiv.ptr,
        )(shader, pname, params)
    }
    #[inline]
    pub unsafe fn GetString(name: GLenum) -> *const GLubyte {
        mem::transmute::<_, extern "system" fn(GLenum) -> *const GLubyte>(storage::GetString.ptr)(
            name,
        )
    }
    #[inline]
    pub unsafe fn GetStringi(name: GLenum, index: GLuint) -> *const GLubyte {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> *const GLubyte>(
            storage::GetStringi.ptr,
        )(name, index)
    }
    #[inline]
    pub unsafe fn GetSynciv(
        sync: GLsync,
        pname: GLenum,
        bufSize: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint) -> (),
        >(storage::GetSynciv.ptr)(sync, pname, bufSize, length, values)
    }
    #[inline]
    pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(
            storage::GetTexParameterfv.ptr,
        )(target, pname, params)
    }
    #[inline]
    pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
            storage::GetTexParameteriv.ptr,
        )(target, pname, params)
    }
    #[inline]
    pub unsafe fn GetTransformFeedbackVarying(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLsizei,
        type_: *mut GLenum,
        name: *mut GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLuint,
                GLsizei,
                *mut GLsizei,
                *mut GLsizei,
                *mut GLenum,
                *mut GLchar,
            ) -> (),
        >(storage::GetTransformFeedbackVarying.ptr)(
            program, index, bufSize, length, size, type_, name,
        )
    }
    #[inline]
    pub unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLuint>(
            storage::GetUniformBlockIndex.ptr,
        )(program, uniformBlockName)
    }
    #[inline]
    pub unsafe fn GetUniformIndices(
        program: GLuint,
        uniformCount: GLsizei,
        uniformNames: *const *const GLchar,
        uniformIndices: *mut GLuint,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint) -> (),
        >(storage::GetUniformIndices.ptr)(
            program, uniformCount, uniformNames, uniformIndices
        )
    }
    #[inline]
    pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(
            storage::GetUniformLocation.ptr,
        )(program, name)
    }
    #[inline]
    pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLfloat) -> ()>(
            storage::GetUniformfv.ptr,
        )(program, location, params)
    }
    #[inline]
    pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLint) -> ()>(
            storage::GetUniformiv.ptr,
        )(program, location, params)
    }
    #[inline]
    pub unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLuint) -> ()>(
            storage::GetUniformuiv.ptr,
        )(program, location, params)
    }
    #[inline]
    pub unsafe fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            storage::GetVertexAttribIiv.ptr,
        )(index, pname, params)
    }
    #[inline]
    pub unsafe fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
            storage::GetVertexAttribIuiv.ptr,
        )(index, pname, params)
    }
    #[inline]
    pub unsafe fn GetVertexAttribPointerv(
        index: GLuint,
        pname: GLenum,
        pointer: *mut *mut std::os::raw::c_void,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut *mut std::os::raw::c_void) -> ()>(
            storage::GetVertexAttribPointerv.ptr,
        )(index, pname, pointer)
    }
    #[inline]
    pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(
            storage::GetVertexAttribfv.ptr,
        )(index, pname, params)
    }
    #[inline]
    pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            storage::GetVertexAttribiv.ptr,
        )(index, pname, params)
    }
    #[inline]
    pub unsafe fn Hint(target: GLenum, mode: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::Hint.ptr)(
            target, mode,
        )
    }
    #[inline]
    pub unsafe fn InvalidateFramebuffer(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLenum) -> ()>(
            storage::InvalidateFramebuffer.ptr,
        )(target, numAttachments, attachments)
    }
    #[inline]
    pub unsafe fn InvalidateSubFramebuffer(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLsizei,
                *const GLenum,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
            ) -> (),
        >(storage::InvalidateSubFramebuffer.ptr)(
            target,
            numAttachments,
            attachments,
            x,
            y,
            width,
            height,
        )
    }
    #[inline]
    pub unsafe fn IsBuffer(buffer: GLuint) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsBuffer.ptr)(buffer)
    }
    #[inline]
    pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::IsEnabled.ptr)(cap)
    }
    #[inline]
    pub unsafe fn IsFramebuffer(framebuffer: GLuint) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsFramebuffer.ptr)(
            framebuffer,
        )
    }
    #[inline]
    pub unsafe fn IsProgram(program: GLuint) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsProgram.ptr)(
            program,
        )
    }
    #[inline]
    pub unsafe fn IsQuery(id: GLuint) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsQuery.ptr)(id)
    }
    #[inline]
    pub unsafe fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsRenderbuffer.ptr)(
            renderbuffer,
        )
    }
    #[inline]
    pub unsafe fn IsSampler(sampler: GLuint) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsSampler.ptr)(
            sampler,
        )
    }
    #[inline]
    pub unsafe fn IsShader(shader: GLuint) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsShader.ptr)(shader)
    }
    #[inline]
    pub unsafe fn IsSync(sync: GLsync) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLsync) -> GLboolean>(storage::IsSync.ptr)(sync)
    }
    #[inline]
    pub unsafe fn IsTexture(texture: GLuint) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsTexture.ptr)(
            texture,
        )
    }
    #[inline]
    pub unsafe fn IsTransformFeedback(id: GLuint) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            storage::IsTransformFeedback.ptr,
        )(id)
    }
    #[inline]
    pub unsafe fn IsVertexArray(array: GLuint) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsVertexArray.ptr)(
            array,
        )
    }
    #[inline]
    pub unsafe fn LineWidth(width: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::LineWidth.ptr)(width)
    }
    #[inline]
    pub unsafe fn LinkProgram(program: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::LinkProgram.ptr)(program)
    }
    #[inline]
    pub unsafe fn MapBufferRange(
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) -> *mut std::os::raw::c_void {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLintptr,
                GLsizeiptr,
                GLbitfield,
            ) -> *mut std::os::raw::c_void,
        >(storage::MapBufferRange.ptr)(target, offset, length, access)
    }
    #[inline]
    pub unsafe fn ObjectLabel(
        identifier: GLenum,
        name: GLuint,
        length: GLsizei,
        label: *const GLchar,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>(
            storage::ObjectLabel.ptr,
        )(identifier, name, length, label)
    }
    #[inline]
    pub unsafe fn ObjectLabelKHR(
        identifier: GLenum,
        name: GLuint,
        length: GLsizei,
        label: *const GLchar,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>(
            storage::ObjectLabelKHR.ptr,
        )(identifier, name, length, label)
    }
    #[inline]
    pub unsafe fn ObjectPtrLabel(
        ptr: *const std::os::raw::c_void,
        length: GLsizei,
        label: *const GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(*const std::os::raw::c_void, GLsizei, *const GLchar) -> (),
        >(storage::ObjectPtrLabel.ptr)(ptr, length, label)
    }
    #[inline]
    pub unsafe fn ObjectPtrLabelKHR(
        ptr: *const std::os::raw::c_void,
        length: GLsizei,
        label: *const GLchar,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(*const std::os::raw::c_void, GLsizei, *const GLchar) -> (),
        >(storage::ObjectPtrLabelKHR.ptr)(ptr, length, label)
    }
    #[inline]
    pub unsafe fn PauseTransformFeedback() -> () {
        mem::transmute::<_, extern "system" fn() -> ()>(storage::PauseTransformFeedback.ptr)()
    }
    #[inline]
    pub unsafe fn PixelStorei(pname: GLenum, param: GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PixelStorei.ptr)(
            pname, param,
        )
    }
    #[inline]
    pub unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::PolygonOffset.ptr)(
            factor, units,
        )
    }
    #[inline]
    pub unsafe fn PopDebugGroup() -> () {
        mem::transmute::<_, extern "system" fn() -> ()>(storage::PopDebugGroup.ptr)()
    }
    #[inline]
    pub unsafe fn PopDebugGroupKHR() -> () {
        mem::transmute::<_, extern "system" fn() -> ()>(storage::PopDebugGroupKHR.ptr)()
    }
    #[inline]
    pub unsafe fn ProgramBinary(
        program: GLuint,
        binaryFormat: GLenum,
        binary: *const std::os::raw::c_void,
        length: GLsizei,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLenum, *const std::os::raw::c_void, GLsizei) -> (),
        >(storage::ProgramBinary.ptr)(program, binaryFormat, binary, length)
    }
    #[inline]
    pub unsafe fn ProgramParameteri(program: GLuint, pname: GLenum, value: GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(
            storage::ProgramParameteri.ptr,
        )(program, pname, value)
    }
    #[inline]
    pub unsafe fn PushDebugGroup(
        source: GLenum,
        id: GLuint,
        length: GLsizei,
        message: *const GLchar,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>(
            storage::PushDebugGroup.ptr,
        )(source, id, length, message)
    }
    #[inline]
    pub unsafe fn PushDebugGroupKHR(
        source: GLenum,
        id: GLuint,
        length: GLsizei,
        message: *const GLchar,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>(
            storage::PushDebugGroupKHR.ptr,
        )(source, id, length, message)
    }
    #[inline]
    pub unsafe fn ReadBuffer(src: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ReadBuffer.ptr)(src)
    }
    #[inline]
    pub unsafe fn ReadPixels(
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *mut std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLenum,
                GLenum,
                *mut std::os::raw::c_void,
            ) -> (),
        >(storage::ReadPixels.ptr)(x, y, width, height, format, type_, pixels)
    }
    #[inline]
    pub unsafe fn ReleaseShaderCompiler() -> () {
        mem::transmute::<_, extern "system" fn() -> ()>(storage::ReleaseShaderCompiler.ptr)()
    }
    #[inline]
    pub unsafe fn RenderbufferStorage(
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, GLsizei) -> ()>(
            storage::RenderbufferStorage.ptr,
        )(target, internalformat, width, height)
    }
    #[inline]
    pub unsafe fn RenderbufferStorageMultisample(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(
            storage::RenderbufferStorageMultisample.ptr,
        )(target, samples, internalformat, width, height)
    }
    #[inline]
    pub unsafe fn ResumeTransformFeedback() -> () {
        mem::transmute::<_, extern "system" fn() -> ()>(storage::ResumeTransformFeedback.ptr)()
    }
    #[inline]
    pub unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean) -> () {
        mem::transmute::<_, extern "system" fn(GLfloat, GLboolean) -> ()>(
            storage::SampleCoverage.ptr,
        )(value, invert)
    }
    #[inline]
    pub unsafe fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>(
            storage::SamplerParameterf.ptr,
        )(sampler, pname, param)
    }
    #[inline]
    pub unsafe fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLfloat) -> ()>(
            storage::SamplerParameterfv.ptr,
        )(sampler, pname, param)
    }
    #[inline]
    pub unsafe fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(
            storage::SamplerParameteri.ptr,
        )(sampler, pname, param)
    }
    #[inline]
    pub unsafe fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
            storage::SamplerParameteriv.ptr,
        )(sampler, pname, param)
    }
    #[inline]
    pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(
            storage::Scissor.ptr,
        )(x, y, width, height)
    }
    #[inline]
    pub unsafe fn ShaderBinary(
        count: GLsizei,
        shaders: *const GLuint,
        binaryformat: GLenum,
        binary: *const std::os::raw::c_void,
        length: GLsizei,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLsizei,
                *const GLuint,
                GLenum,
                *const std::os::raw::c_void,
                GLsizei,
            ) -> (),
        >(storage::ShaderBinary.ptr)(count, shaders, binaryformat, binary, length)
    }
    #[inline]
    pub unsafe fn ShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> (),
        >(storage::ShaderSource.ptr)(shader, count, string, length)
    }
    #[inline]
    pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLuint) -> ()>(
            storage::StencilFunc.ptr,
        )(func, ref_, mask)
    }
    #[inline]
    pub unsafe fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint, GLuint) -> ()>(
            storage::StencilFuncSeparate.ptr,
        )(face, func, ref_, mask)
    }
    #[inline]
    pub unsafe fn StencilMask(mask: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::StencilMask.ptr)(mask)
    }
    #[inline]
    pub unsafe fn StencilMaskSeparate(face: GLenum, mask: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            storage::StencilMaskSeparate.ptr,
        )(face, mask)
    }
    #[inline]
    pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum) -> ()>(
            storage::StencilOp.ptr,
        )(fail, zfail, zpass)
    }
    #[inline]
    pub unsafe fn StencilOpSeparate(
        face: GLenum,
        sfail: GLenum,
        dpfail: GLenum,
        dppass: GLenum,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(
            storage::StencilOpSeparate.ptr,
        )(face, sfail, dpfail, dppass)
    }
    #[inline]
    pub unsafe fn TexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLint,
                GLenum,
                GLenum,
                *const std::os::raw::c_void,
            ) -> (),
        >(storage::TexImage2D.ptr)(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            type_,
            pixels,
        )
    }
    #[inline]
    pub unsafe fn TexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLint,
                GLenum,
                GLenum,
                *const std::os::raw::c_void,
            ) -> (),
        >(storage::TexImage3D.ptr)(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            type_,
            pixels,
        )
    }
    #[inline]
    pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(
            storage::TexParameterf.ptr,
        )(target, pname, param)
    }
    #[inline]
    pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(
            storage::TexParameterfv.ptr,
        )(target, pname, params)
    }
    #[inline]
    pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(
            storage::TexParameteri.ptr,
        )(target, pname, param)
    }
    #[inline]
    pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(
            storage::TexParameteriv.ptr,
        )(target, pname, params)
    }
    #[inline]
    pub unsafe fn TexStorage2D(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(
            storage::TexStorage2D.ptr,
        )(target, levels, internalformat, width, height)
    }
    #[inline]
    pub unsafe fn TexStorage3D(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) -> (),
        >(storage::TexStorage3D.ptr)(target, levels, internalformat, width, height, depth)
    }
    #[inline]
    pub unsafe fn TexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLenum,
                GLenum,
                *const std::os::raw::c_void,
            ) -> (),
        >(storage::TexSubImage2D.ptr)(
            target, level, xoffset, yoffset, width, height, format, type_, pixels,
        )
    }
    #[inline]
    pub unsafe fn TexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLenum,
                GLenum,
                *const std::os::raw::c_void,
            ) -> (),
        >(storage::TexSubImage3D.ptr)(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels,
        )
    }
    #[inline]
    pub unsafe fn TransformFeedbackVaryings(
        program: GLuint,
        count: GLsizei,
        varyings: *const *const GLchar,
        bufferMode: GLenum,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum) -> ()>(
            storage::TransformFeedbackVaryings.ptr,
        )(program, count, varyings, bufferMode)
    }
    #[inline]
    pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLfloat) -> ()>(storage::Uniform1f.ptr)(
            location, v0,
        )
    }
    #[inline]
    pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(
            storage::Uniform1fv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform1i(location: GLint, v0: GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::Uniform1i.ptr)(
            location, v0,
        )
    }
    #[inline]
    pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(
            storage::Uniform1iv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform1ui(location: GLint, v0: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLuint) -> ()>(storage::Uniform1ui.ptr)(
            location, v0,
        )
    }
    #[inline]
    pub unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
            storage::Uniform1uiv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat) -> ()>(
            storage::Uniform2f.ptr,
        )(location, v0, v1)
    }
    #[inline]
    pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(
            storage::Uniform2fv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform2i(location: GLint, v0: GLint, v1: GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Uniform2i.ptr)(
            location, v0, v1,
        )
    }
    #[inline]
    pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(
            storage::Uniform2iv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint) -> ()>(
            storage::Uniform2ui.ptr,
        )(location, v0, v1)
    }
    #[inline]
    pub unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
            storage::Uniform2uiv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat) -> ()>(
            storage::Uniform3f.ptr,
        )(location, v0, v1, v2)
    }
    #[inline]
    pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(
            storage::Uniform3fv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(
            storage::Uniform3i.ptr,
        )(location, v0, v1, v2)
    }
    #[inline]
    pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(
            storage::Uniform3iv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint) -> ()>(
            storage::Uniform3ui.ptr,
        )(location, v0, v1, v2)
    }
    #[inline]
    pub unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
            storage::Uniform3uiv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform4f(
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(
            storage::Uniform4f.ptr,
        )(location, v0, v1, v2, v3)
    }
    #[inline]
    pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(
            storage::Uniform4fv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint) -> ()>(
            storage::Uniform4i.ptr,
        )(location, v0, v1, v2, v3)
    }
    #[inline]
    pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(
            storage::Uniform4iv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn Uniform4ui(
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint) -> ()>(
            storage::Uniform4ui.ptr,
        )(location, v0, v1, v2, v3)
    }
    #[inline]
    pub unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
            storage::Uniform4uiv.ptr,
        )(location, count, value)
    }
    #[inline]
    pub unsafe fn UniformBlockBinding(
        program: GLuint,
        uniformBlockIndex: GLuint,
        uniformBlockBinding: GLuint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
            storage::UniformBlockBinding.ptr,
        )(program, uniformBlockIndex, uniformBlockBinding)
    }
    #[inline]
    pub unsafe fn UniformMatrix2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            storage::UniformMatrix2fv.ptr,
        )(location, count, transpose, value)
    }
    #[inline]
    pub unsafe fn UniformMatrix2x3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            storage::UniformMatrix2x3fv.ptr,
        )(location, count, transpose, value)
    }
    #[inline]
    pub unsafe fn UniformMatrix2x4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            storage::UniformMatrix2x4fv.ptr,
        )(location, count, transpose, value)
    }
    #[inline]
    pub unsafe fn UniformMatrix3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            storage::UniformMatrix3fv.ptr,
        )(location, count, transpose, value)
    }
    #[inline]
    pub unsafe fn UniformMatrix3x2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            storage::UniformMatrix3x2fv.ptr,
        )(location, count, transpose, value)
    }
    #[inline]
    pub unsafe fn UniformMatrix3x4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            storage::UniformMatrix3x4fv.ptr,
        )(location, count, transpose, value)
    }
    #[inline]
    pub unsafe fn UniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            storage::UniformMatrix4fv.ptr,
        )(location, count, transpose, value)
    }
    #[inline]
    pub unsafe fn UniformMatrix4x2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            storage::UniformMatrix4x2fv.ptr,
        )(location, count, transpose, value)
    }
    #[inline]
    pub unsafe fn UniformMatrix4x3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            storage::UniformMatrix4x3fv.ptr,
        )(location, count, transpose, value)
    }
    #[inline]
    pub unsafe fn UnmapBuffer(target: GLenum) -> GLboolean {
        mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::UnmapBuffer.ptr)(
            target,
        )
    }
    #[inline]
    pub unsafe fn UseProgram(program: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::UseProgram.ptr)(program)
    }
    #[inline]
    pub unsafe fn ValidateProgram(program: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::ValidateProgram.ptr)(program)
    }
    #[inline]
    pub unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLfloat) -> ()>(storage::VertexAttrib1f.ptr)(
            index, x,
        )
    }
    #[inline]
    pub unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(
            storage::VertexAttrib1fv.ptr,
        )(index, v)
    }
    #[inline]
    pub unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat) -> ()>(
            storage::VertexAttrib2f.ptr,
        )(index, x, y)
    }
    #[inline]
    pub unsafe fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(
            storage::VertexAttrib2fv.ptr,
        )(index, v)
    }
    #[inline]
    pub unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat) -> ()>(
            storage::VertexAttrib3f.ptr,
        )(index, x, y, z)
    }
    #[inline]
    pub unsafe fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(
            storage::VertexAttrib3fv.ptr,
        )(index, v)
    }
    #[inline]
    pub unsafe fn VertexAttrib4f(
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(
            storage::VertexAttrib4f.ptr,
        )(index, x, y, z, w)
    }
    #[inline]
    pub unsafe fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(
            storage::VertexAttrib4fv.ptr,
        )(index, v)
    }
    #[inline]
    pub unsafe fn VertexAttribDivisor(index: GLuint, divisor: GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            storage::VertexAttribDivisor.ptr,
        )(index, divisor)
    }
    #[inline]
    pub unsafe fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> ()>(
            storage::VertexAttribI4i.ptr,
        )(index, x, y, z, w)
    }
    #[inline]
    pub unsafe fn VertexAttribI4iv(index: GLuint, v: *const GLint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(
            storage::VertexAttribI4iv.ptr,
        )(index, v)
    }
    #[inline]
    pub unsafe fn VertexAttribI4ui(
        index: GLuint,
        x: GLuint,
        y: GLuint,
        z: GLuint,
        w: GLuint,
    ) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint) -> ()>(
            storage::VertexAttribI4ui.ptr,
        )(index, x, y, z, w)
    }
    #[inline]
    pub unsafe fn VertexAttribI4uiv(index: GLuint, v: *const GLuint) -> () {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(
            storage::VertexAttribI4uiv.ptr,
        )(index, v)
    }
    #[inline]
    pub unsafe fn VertexAttribIPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const std::os::raw::c_void) -> (),
        >(storage::VertexAttribIPointer.ptr)(index, size, type_, stride, pointer)
    }
    #[inline]
    pub unsafe fn VertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const std::os::raw::c_void,
    ) -> () {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLint,
                GLenum,
                GLboolean,
                GLsizei,
                *const std::os::raw::c_void,
            ) -> (),
        >(storage::VertexAttribPointer.ptr)(index, size, type_, normalized, stride, pointer)
    }
    #[inline]
    pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () {
        mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(
            storage::Viewport.ptr,
        )(x, y, width, height)
    }
    #[inline]
    pub unsafe fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> () {
        mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> ()>(
            storage::WaitSync.ptr,
        )(sync, flags, timeout)
    }
}

pub mod storage {
    #![allow(non_snake_case, non_upper_case_globals)]

    use super::FnPtr;
    use std::os::raw;

    pub static mut ActiveTexture: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut AttachShader: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BeginQuery: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BeginTransformFeedback: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BindAttribLocation: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BindBuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BindBufferBase: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BindBufferRange: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BindFramebuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BindRenderbuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BindSampler: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BindTexture: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BindTransformFeedback: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BindVertexArray: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BlendColor: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BlendEquation: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BlendEquationSeparate: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BlendFunc: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BlendFuncSeparate: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BlitFramebuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BufferData: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut BufferSubData: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CheckFramebufferStatus: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Clear: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ClearBufferfi: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ClearBufferfv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ClearBufferiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ClearBufferuiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ClearColor: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ClearDepthf: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ClearStencil: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ClientWaitSync: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ColorMask: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CompileShader: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CompressedTexImage2D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CompressedTexImage3D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CompressedTexSubImage2D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CompressedTexSubImage3D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CopyBufferSubData: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CopyTexImage2D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CopyTexSubImage2D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CopyTexSubImage3D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CreateProgram: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CreateShader: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut CullFace: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DebugMessageCallback: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DebugMessageCallbackKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DebugMessageControl: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DebugMessageControlKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DebugMessageInsert: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DebugMessageInsertKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteBuffers: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteFramebuffers: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteProgram: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteQueries: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteRenderbuffers: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteSamplers: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteShader: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteSync: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteTextures: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteTransformFeedbacks: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DeleteVertexArrays: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DepthFunc: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DepthMask: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DepthRangef: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DetachShader: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Disable: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DisableVertexAttribArray: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DrawArrays: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DrawArraysInstanced: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DrawBuffers: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DrawElements: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DrawElementsInstanced: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut DrawRangeElements: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Enable: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut EnableVertexAttribArray: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut EndQuery: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut EndTransformFeedback: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut FenceSync: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Finish: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Flush: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut FlushMappedBufferRange: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut FramebufferRenderbuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut FramebufferTexture2D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut FramebufferTextureLayer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut FrontFace: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GenBuffers: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GenFramebuffers: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GenQueries: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GenRenderbuffers: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GenSamplers: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GenTextures: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GenTransformFeedbacks: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GenVertexArrays: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GenerateMipmap: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetActiveAttrib: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetActiveUniform: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetActiveUniformBlockName: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetActiveUniformBlockiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetActiveUniformsiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetAttachedShaders: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetAttribLocation: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetBooleanv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetBufferParameteri64v: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetBufferParameteriv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetBufferPointerv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetDebugMessageLog: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetDebugMessageLogKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetError: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetFloatv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetFragDataLocation: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetFramebufferAttachmentParameteriv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetInteger64i_v: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetInteger64v: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetIntegeri_v: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetIntegerv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetInternalformativ: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetObjectLabel: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetObjectLabelKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetObjectPtrLabel: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetObjectPtrLabelKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetPointerv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetPointervKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetProgramBinary: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetProgramInfoLog: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetProgramiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetQueryObjectuiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetQueryiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetRenderbufferParameteriv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetSamplerParameterfv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetSamplerParameteriv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetShaderInfoLog: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetShaderPrecisionFormat: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetShaderSource: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetShaderiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetString: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetStringi: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetSynciv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetTexParameterfv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetTexParameteriv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetTransformFeedbackVarying: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetUniformBlockIndex: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetUniformIndices: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetUniformLocation: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetUniformfv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetUniformiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetUniformuiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetVertexAttribIiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetVertexAttribIuiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetVertexAttribPointerv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetVertexAttribfv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut GetVertexAttribiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Hint: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut InvalidateFramebuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut InvalidateSubFramebuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsBuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsEnabled: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsFramebuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsProgram: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsQuery: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsRenderbuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsSampler: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsShader: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsSync: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsTexture: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsTransformFeedback: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut IsVertexArray: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut LineWidth: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut LinkProgram: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut MapBufferRange: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ObjectLabel: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ObjectLabelKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ObjectPtrLabel: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ObjectPtrLabelKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut PauseTransformFeedback: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut PixelStorei: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut PolygonOffset: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut PopDebugGroup: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut PopDebugGroupKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ProgramBinary: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ProgramParameteri: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut PushDebugGroup: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut PushDebugGroupKHR: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ReadBuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ReadPixels: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ReleaseShaderCompiler: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut RenderbufferStorage: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut RenderbufferStorageMultisample: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ResumeTransformFeedback: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut SampleCoverage: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut SamplerParameterf: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut SamplerParameterfv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut SamplerParameteri: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut SamplerParameteriv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Scissor: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ShaderBinary: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ShaderSource: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut StencilFunc: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut StencilFuncSeparate: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut StencilMask: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut StencilMaskSeparate: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut StencilOp: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut StencilOpSeparate: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TexImage2D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TexImage3D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TexParameterf: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TexParameterfv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TexParameteri: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TexParameteriv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TexStorage2D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TexStorage3D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TexSubImage2D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TexSubImage3D: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut TransformFeedbackVaryings: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform1f: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform1fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform1i: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform1iv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform1ui: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform1uiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform2f: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform2fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform2i: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform2iv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform2ui: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform2uiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform3f: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform3fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform3i: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform3iv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform3ui: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform3uiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform4f: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform4fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform4i: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform4iv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform4ui: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Uniform4uiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UniformBlockBinding: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UniformMatrix2fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UniformMatrix2x3fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UniformMatrix2x4fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UniformMatrix3fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UniformMatrix3x2fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UniformMatrix3x4fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UniformMatrix4fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UniformMatrix4x2fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UniformMatrix4x3fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UnmapBuffer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut UseProgram: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut ValidateProgram: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttrib1f: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttrib1fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttrib2f: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttrib2fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttrib3f: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttrib3fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttrib4f: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttrib4fv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttribDivisor: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttribI4i: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttribI4iv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttribI4ui: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttribI4uiv: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttribIPointer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut VertexAttribPointer: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut Viewport: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
    pub static mut WaitSync: FnPtr = FnPtr {
        ptr: FnPtr::not_initialized as *const raw::c_void,
        is_loaded: false,
    };
}

pub fn load<F>(mut loadfn: F)
where
    F: FnMut(&'static str) -> *const raw::c_void,
{
    unsafe {
        storage::ActiveTexture.load(&mut loadfn, "glActiveTexture");
        storage::AttachShader.load(&mut loadfn, "glAttachShader");
        storage::BeginQuery.load(&mut loadfn, "glBeginQuery");
        storage::BeginTransformFeedback.load(&mut loadfn, "glBeginTransformFeedback");
        storage::BindAttribLocation.load(&mut loadfn, "glBindAttribLocation");
        storage::BindBuffer.load(&mut loadfn, "glBindBuffer");
        storage::BindBufferBase.load(&mut loadfn, "glBindBufferBase");
        storage::BindBufferRange.load(&mut loadfn, "glBindBufferRange");
        storage::BindFramebuffer.load(&mut loadfn, "glBindFramebuffer");
        storage::BindRenderbuffer.load(&mut loadfn, "glBindRenderbuffer");
        storage::BindSampler.load(&mut loadfn, "glBindSampler");
        storage::BindTexture.load(&mut loadfn, "glBindTexture");
        storage::BindTransformFeedback.load(&mut loadfn, "glBindTransformFeedback");
        storage::BindVertexArray.load(&mut loadfn, "glBindVertexArray");
        storage::BlendColor.load(&mut loadfn, "glBlendColor");
        storage::BlendEquation.load(&mut loadfn, "glBlendEquation");
        storage::BlendEquationSeparate.load(&mut loadfn, "glBlendEquationSeparate");
        storage::BlendFunc.load(&mut loadfn, "glBlendFunc");
        storage::BlendFuncSeparate.load(&mut loadfn, "glBlendFuncSeparate");
        storage::BlitFramebuffer.load(&mut loadfn, "glBlitFramebuffer");
        storage::BufferData.load(&mut loadfn, "glBufferData");
        storage::BufferSubData.load(&mut loadfn, "glBufferSubData");
        storage::CheckFramebufferStatus.load(&mut loadfn, "glCheckFramebufferStatus");
        storage::Clear.load(&mut loadfn, "glClear");
        storage::ClearBufferfi.load(&mut loadfn, "glClearBufferfi");
        storage::ClearBufferfv.load(&mut loadfn, "glClearBufferfv");
        storage::ClearBufferiv.load(&mut loadfn, "glClearBufferiv");
        storage::ClearBufferuiv.load(&mut loadfn, "glClearBufferuiv");
        storage::ClearColor.load(&mut loadfn, "glClearColor");
        storage::ClearDepthf.load(&mut loadfn, "glClearDepthf");
        storage::ClearStencil.load(&mut loadfn, "glClearStencil");
        storage::ClientWaitSync.load(&mut loadfn, "glClientWaitSync");
        storage::ColorMask.load(&mut loadfn, "glColorMask");
        storage::CompileShader.load(&mut loadfn, "glCompileShader");
        storage::CompressedTexImage2D.load(&mut loadfn, "glCompressedTexImage2D");
        storage::CompressedTexImage3D.load(&mut loadfn, "glCompressedTexImage3D");
        storage::CompressedTexSubImage2D.load(&mut loadfn, "glCompressedTexSubImage2D");
        storage::CompressedTexSubImage3D.load(&mut loadfn, "glCompressedTexSubImage3D");
        storage::CopyBufferSubData.load(&mut loadfn, "glCopyBufferSubData");
        storage::CopyTexImage2D.load(&mut loadfn, "glCopyTexImage2D");
        storage::CopyTexSubImage2D.load(&mut loadfn, "glCopyTexSubImage2D");
        storage::CopyTexSubImage3D.load(&mut loadfn, "glCopyTexSubImage3D");
        storage::CreateProgram.load(&mut loadfn, "glCreateProgram");
        storage::CreateShader.load(&mut loadfn, "glCreateShader");
        storage::CullFace.load(&mut loadfn, "glCullFace");
        storage::DebugMessageCallback.load(&mut loadfn, "glDebugMessageCallback");
        storage::DebugMessageCallbackKHR.load(&mut loadfn, "glDebugMessageCallbackKHR");
        storage::DebugMessageControl.load(&mut loadfn, "glDebugMessageControl");
        storage::DebugMessageControlKHR.load(&mut loadfn, "glDebugMessageControlKHR");
        storage::DebugMessageInsert.load(&mut loadfn, "glDebugMessageInsert");
        storage::DebugMessageInsertKHR.load(&mut loadfn, "glDebugMessageInsertKHR");
        storage::DeleteBuffers.load(&mut loadfn, "glDeleteBuffers");
        storage::DeleteFramebuffers.load(&mut loadfn, "glDeleteFramebuffers");
        storage::DeleteProgram.load(&mut loadfn, "glDeleteProgram");
        storage::DeleteQueries.load(&mut loadfn, "glDeleteQueries");
        storage::DeleteRenderbuffers.load(&mut loadfn, "glDeleteRenderbuffers");
        storage::DeleteSamplers.load(&mut loadfn, "glDeleteSamplers");
        storage::DeleteShader.load(&mut loadfn, "glDeleteShader");
        storage::DeleteSync.load(&mut loadfn, "glDeleteSync");
        storage::DeleteTextures.load(&mut loadfn, "glDeleteTextures");
        storage::DeleteTransformFeedbacks.load(&mut loadfn, "glDeleteTransformFeedbacks");
        storage::DeleteVertexArrays.load(&mut loadfn, "glDeleteVertexArrays");
        storage::DepthFunc.load(&mut loadfn, "glDepthFunc");
        storage::DepthMask.load(&mut loadfn, "glDepthMask");
        storage::DepthRangef.load(&mut loadfn, "glDepthRangef");
        storage::DetachShader.load(&mut loadfn, "glDetachShader");
        storage::Disable.load(&mut loadfn, "glDisable");
        storage::DisableVertexAttribArray.load(&mut loadfn, "glDisableVertexAttribArray");
        storage::DrawArrays.load(&mut loadfn, "glDrawArrays");
        storage::DrawArraysInstanced.load(&mut loadfn, "glDrawArraysInstanced");
        storage::DrawBuffers.load(&mut loadfn, "glDrawBuffers");
        storage::DrawElements.load(&mut loadfn, "glDrawElements");
        storage::DrawElementsInstanced.load(&mut loadfn, "glDrawElementsInstanced");
        storage::DrawRangeElements.load(&mut loadfn, "glDrawRangeElements");
        storage::Enable.load(&mut loadfn, "glEnable");
        storage::EnableVertexAttribArray.load(&mut loadfn, "glEnableVertexAttribArray");
        storage::EndQuery.load(&mut loadfn, "glEndQuery");
        storage::EndTransformFeedback.load(&mut loadfn, "glEndTransformFeedback");
        storage::FenceSync.load(&mut loadfn, "glFenceSync");
        storage::Finish.load(&mut loadfn, "glFinish");
        storage::Flush.load(&mut loadfn, "glFlush");
        storage::FlushMappedBufferRange.load(&mut loadfn, "glFlushMappedBufferRange");
        storage::FramebufferRenderbuffer.load(&mut loadfn, "glFramebufferRenderbuffer");
        storage::FramebufferTexture2D.load(&mut loadfn, "glFramebufferTexture2D");
        storage::FramebufferTextureLayer.load(&mut loadfn, "glFramebufferTextureLayer");
        storage::FrontFace.load(&mut loadfn, "glFrontFace");
        storage::GenBuffers.load(&mut loadfn, "glGenBuffers");
        storage::GenFramebuffers.load(&mut loadfn, "glGenFramebuffers");
        storage::GenQueries.load(&mut loadfn, "glGenQueries");
        storage::GenRenderbuffers.load(&mut loadfn, "glGenRenderbuffers");
        storage::GenSamplers.load(&mut loadfn, "glGenSamplers");
        storage::GenTextures.load(&mut loadfn, "glGenTextures");
        storage::GenTransformFeedbacks.load(&mut loadfn, "glGenTransformFeedbacks");
        storage::GenVertexArrays.load(&mut loadfn, "glGenVertexArrays");
        storage::GenerateMipmap.load(&mut loadfn, "glGenerateMipmap");
        storage::GetActiveAttrib.load(&mut loadfn, "glGetActiveAttrib");
        storage::GetActiveUniform.load(&mut loadfn, "glGetActiveUniform");
        storage::GetActiveUniformBlockName.load(&mut loadfn, "glGetActiveUniformBlockName");
        storage::GetActiveUniformBlockiv.load(&mut loadfn, "glGetActiveUniformBlockiv");
        storage::GetActiveUniformsiv.load(&mut loadfn, "glGetActiveUniformsiv");
        storage::GetAttachedShaders.load(&mut loadfn, "glGetAttachedShaders");
        storage::GetAttribLocation.load(&mut loadfn, "glGetAttribLocation");
        storage::GetBooleanv.load(&mut loadfn, "glGetBooleanv");
        storage::GetBufferParameteri64v.load(&mut loadfn, "glGetBufferParameteri64v");
        storage::GetBufferParameteriv.load(&mut loadfn, "glGetBufferParameteriv");
        storage::GetBufferPointerv.load(&mut loadfn, "glGetBufferPointerv");
        storage::GetDebugMessageLog.load(&mut loadfn, "glGetDebugMessageLog");
        storage::GetDebugMessageLogKHR.load(&mut loadfn, "glGetDebugMessageLogKHR");
        storage::GetError.load(&mut loadfn, "glGetError");
        storage::GetFloatv.load(&mut loadfn, "glGetFloatv");
        storage::GetFragDataLocation.load(&mut loadfn, "glGetFragDataLocation");
        storage::GetFramebufferAttachmentParameteriv
            .load(&mut loadfn, "glGetFramebufferAttachmentParameteriv");
        storage::GetInteger64i_v.load(&mut loadfn, "glGetInteger64i_v");
        storage::GetInteger64v.load(&mut loadfn, "glGetInteger64v");
        storage::GetIntegeri_v.load(&mut loadfn, "glGetIntegeri_v");
        storage::GetIntegerv.load(&mut loadfn, "glGetIntegerv");
        storage::GetInternalformativ.load(&mut loadfn, "glGetInternalformativ");
        storage::GetObjectLabel.load(&mut loadfn, "glGetObjectLabel");
        storage::GetObjectLabelKHR.load(&mut loadfn, "glGetObjectLabelKHR");
        storage::GetObjectPtrLabel.load(&mut loadfn, "glGetObjectPtrLabel");
        storage::GetObjectPtrLabelKHR.load(&mut loadfn, "glGetObjectPtrLabelKHR");
        storage::GetPointerv.load(&mut loadfn, "glGetPointerv");
        storage::GetPointervKHR.load(&mut loadfn, "glGetPointervKHR");
        storage::GetProgramBinary.load(&mut loadfn, "glGetProgramBinary");
        storage::GetProgramInfoLog.load(&mut loadfn, "glGetProgramInfoLog");
        storage::GetProgramiv.load(&mut loadfn, "glGetProgramiv");
        storage::GetQueryObjectuiv.load(&mut loadfn, "glGetQueryObjectuiv");
        storage::GetQueryiv.load(&mut loadfn, "glGetQueryiv");
        storage::GetRenderbufferParameteriv.load(&mut loadfn, "glGetRenderbufferParameteriv");
        storage::GetSamplerParameterfv.load(&mut loadfn, "glGetSamplerParameterfv");
        storage::GetSamplerParameteriv.load(&mut loadfn, "glGetSamplerParameteriv");
        storage::GetShaderInfoLog.load(&mut loadfn, "glGetShaderInfoLog");
        storage::GetShaderPrecisionFormat.load(&mut loadfn, "glGetShaderPrecisionFormat");
        storage::GetShaderSource.load(&mut loadfn, "glGetShaderSource");
        storage::GetShaderiv.load(&mut loadfn, "glGetShaderiv");
        storage::GetString.load(&mut loadfn, "glGetString");
        storage::GetStringi.load(&mut loadfn, "glGetStringi");
        storage::GetSynciv.load(&mut loadfn, "glGetSynciv");
        storage::GetTexParameterfv.load(&mut loadfn, "glGetTexParameterfv");
        storage::GetTexParameteriv.load(&mut loadfn, "glGetTexParameteriv");
        storage::GetTransformFeedbackVarying.load(&mut loadfn, "glGetTransformFeedbackVarying");
        storage::GetUniformBlockIndex.load(&mut loadfn, "glGetUniformBlockIndex");
        storage::GetUniformIndices.load(&mut loadfn, "glGetUniformIndices");
        storage::GetUniformLocation.load(&mut loadfn, "glGetUniformLocation");
        storage::GetUniformfv.load(&mut loadfn, "glGetUniformfv");
        storage::GetUniformiv.load(&mut loadfn, "glGetUniformiv");
        storage::GetUniformuiv.load(&mut loadfn, "glGetUniformuiv");
        storage::GetVertexAttribIiv.load(&mut loadfn, "glGetVertexAttribIiv");
        storage::GetVertexAttribIuiv.load(&mut loadfn, "glGetVertexAttribIuiv");
        storage::GetVertexAttribPointerv.load(&mut loadfn, "glGetVertexAttribPointerv");
        storage::GetVertexAttribfv.load(&mut loadfn, "glGetVertexAttribfv");
        storage::GetVertexAttribiv.load(&mut loadfn, "glGetVertexAttribiv");
        storage::Hint.load(&mut loadfn, "glHint");
        storage::InvalidateFramebuffer.load(&mut loadfn, "glInvalidateFramebuffer");
        storage::InvalidateSubFramebuffer.load(&mut loadfn, "glInvalidateSubFramebuffer");
        storage::IsBuffer.load(&mut loadfn, "glIsBuffer");
        storage::IsEnabled.load(&mut loadfn, "glIsEnabled");
        storage::IsFramebuffer.load(&mut loadfn, "glIsFramebuffer");
        storage::IsProgram.load(&mut loadfn, "glIsProgram");
        storage::IsQuery.load(&mut loadfn, "glIsQuery");
        storage::IsRenderbuffer.load(&mut loadfn, "glIsRenderbuffer");
        storage::IsSampler.load(&mut loadfn, "glIsSampler");
        storage::IsShader.load(&mut loadfn, "glIsShader");
        storage::IsSync.load(&mut loadfn, "glIsSync");
        storage::IsTexture.load(&mut loadfn, "glIsTexture");
        storage::IsTransformFeedback.load(&mut loadfn, "glIsTransformFeedback");
        storage::IsVertexArray.load(&mut loadfn, "glIsVertexArray");
        storage::LineWidth.load(&mut loadfn, "glLineWidth");
        storage::LinkProgram.load(&mut loadfn, "glLinkProgram");
        storage::MapBufferRange.load(&mut loadfn, "glMapBufferRange");
        storage::ObjectLabel.load(&mut loadfn, "glObjectLabel");
        storage::ObjectLabelKHR.load(&mut loadfn, "glObjectLabelKHR");
        storage::ObjectPtrLabel.load(&mut loadfn, "glObjectPtrLabel");
        storage::ObjectPtrLabelKHR.load(&mut loadfn, "glObjectPtrLabelKHR");
        storage::PauseTransformFeedback.load(&mut loadfn, "glPauseTransformFeedback");
        storage::PixelStorei.load(&mut loadfn, "glPixelStorei");
        storage::PolygonOffset.load(&mut loadfn, "glPolygonOffset");
        storage::PopDebugGroup.load(&mut loadfn, "glPopDebugGroup");
        storage::PopDebugGroupKHR.load(&mut loadfn, "glPopDebugGroupKHR");
        storage::ProgramBinary.load(&mut loadfn, "glProgramBinary");
        storage::ProgramParameteri.load(&mut loadfn, "glProgramParameteri");
        storage::PushDebugGroup.load(&mut loadfn, "glPushDebugGroup");
        storage::PushDebugGroupKHR.load(&mut loadfn, "glPushDebugGroupKHR");
        storage::ReadBuffer.load(&mut loadfn, "glReadBuffer");
        storage::ReadPixels.load(&mut loadfn, "glReadPixels");
        storage::ReleaseShaderCompiler.load(&mut loadfn, "glReleaseShaderCompiler");
        storage::RenderbufferStorage.load(&mut loadfn, "glRenderbufferStorage");
        storage::RenderbufferStorageMultisample
            .load(&mut loadfn, "glRenderbufferStorageMultisample");
        storage::ResumeTransformFeedback.load(&mut loadfn, "glResumeTransformFeedback");
        storage::SampleCoverage.load(&mut loadfn, "glSampleCoverage");
        storage::SamplerParameterf.load(&mut loadfn, "glSamplerParameterf");
        storage::SamplerParameterfv.load(&mut loadfn, "glSamplerParameterfv");
        storage::SamplerParameteri.load(&mut loadfn, "glSamplerParameteri");
        storage::SamplerParameteriv.load(&mut loadfn, "glSamplerParameteriv");
        storage::Scissor.load(&mut loadfn, "glScissor");
        storage::ShaderBinary.load(&mut loadfn, "glShaderBinary");
        storage::ShaderSource.load(&mut loadfn, "glShaderSource");
        storage::StencilFunc.load(&mut loadfn, "glStencilFunc");
        storage::StencilFuncSeparate.load(&mut loadfn, "glStencilFuncSeparate");
        storage::StencilMask.load(&mut loadfn, "glStencilMask");
        storage::StencilMaskSeparate.load(&mut loadfn, "glStencilMaskSeparate");
        storage::StencilOp.load(&mut loadfn, "glStencilOp");
        storage::StencilOpSeparate.load(&mut loadfn, "glStencilOpSeparate");
        storage::TexImage2D.load(&mut loadfn, "glTexImage2D");
        storage::TexImage3D.load(&mut loadfn, "glTexImage3D");
        storage::TexParameterf.load(&mut loadfn, "glTexParameterf");
        storage::TexParameterfv.load(&mut loadfn, "glTexParameterfv");
        storage::TexParameteri.load(&mut loadfn, "glTexParameteri");
        storage::TexParameteriv.load(&mut loadfn, "glTexParameteriv");
        storage::TexStorage2D.load(&mut loadfn, "glTexStorage2D");
        storage::TexStorage3D.load(&mut loadfn, "glTexStorage3D");
        storage::TexSubImage2D.load(&mut loadfn, "glTexSubImage2D");
        storage::TexSubImage3D.load(&mut loadfn, "glTexSubImage3D");
        storage::TransformFeedbackVaryings.load(&mut loadfn, "glTransformFeedbackVaryings");
        storage::Uniform1f.load(&mut loadfn, "glUniform1f");
        storage::Uniform1fv.load(&mut loadfn, "glUniform1fv");
        storage::Uniform1i.load(&mut loadfn, "glUniform1i");
        storage::Uniform1iv.load(&mut loadfn, "glUniform1iv");
        storage::Uniform1ui.load(&mut loadfn, "glUniform1ui");
        storage::Uniform1uiv.load(&mut loadfn, "glUniform1uiv");
        storage::Uniform2f.load(&mut loadfn, "glUniform2f");
        storage::Uniform2fv.load(&mut loadfn, "glUniform2fv");
        storage::Uniform2i.load(&mut loadfn, "glUniform2i");
        storage::Uniform2iv.load(&mut loadfn, "glUniform2iv");
        storage::Uniform2ui.load(&mut loadfn, "glUniform2ui");
        storage::Uniform2uiv.load(&mut loadfn, "glUniform2uiv");
        storage::Uniform3f.load(&mut loadfn, "glUniform3f");
        storage::Uniform3fv.load(&mut loadfn, "glUniform3fv");
        storage::Uniform3i.load(&mut loadfn, "glUniform3i");
        storage::Uniform3iv.load(&mut loadfn, "glUniform3iv");
        storage::Uniform3ui.load(&mut loadfn, "glUniform3ui");
        storage::Uniform3uiv.load(&mut loadfn, "glUniform3uiv");
        storage::Uniform4f.load(&mut loadfn, "glUniform4f");
        storage::Uniform4fv.load(&mut loadfn, "glUniform4fv");
        storage::Uniform4i.load(&mut loadfn, "glUniform4i");
        storage::Uniform4iv.load(&mut loadfn, "glUniform4iv");
        storage::Uniform4ui.load(&mut loadfn, "glUniform4ui");
        storage::Uniform4uiv.load(&mut loadfn, "glUniform4uiv");
        storage::UniformBlockBinding.load(&mut loadfn, "glUniformBlockBinding");
        storage::UniformMatrix2fv.load(&mut loadfn, "glUniformMatrix2fv");
        storage::UniformMatrix2x3fv.load(&mut loadfn, "glUniformMatrix2x3fv");
        storage::UniformMatrix2x4fv.load(&mut loadfn, "glUniformMatrix2x4fv");
        storage::UniformMatrix3fv.load(&mut loadfn, "glUniformMatrix3fv");
        storage::UniformMatrix3x2fv.load(&mut loadfn, "glUniformMatrix3x2fv");
        storage::UniformMatrix3x4fv.load(&mut loadfn, "glUniformMatrix3x4fv");
        storage::UniformMatrix4fv.load(&mut loadfn, "glUniformMatrix4fv");
        storage::UniformMatrix4x2fv.load(&mut loadfn, "glUniformMatrix4x2fv");
        storage::UniformMatrix4x3fv.load(&mut loadfn, "glUniformMatrix4x3fv");
        storage::UnmapBuffer.load(&mut loadfn, "glUnmapBuffer");
        storage::UseProgram.load(&mut loadfn, "glUseProgram");
        storage::ValidateProgram.load(&mut loadfn, "glValidateProgram");
        storage::VertexAttrib1f.load(&mut loadfn, "glVertexAttrib1f");
        storage::VertexAttrib1fv.load(&mut loadfn, "glVertexAttrib1fv");
        storage::VertexAttrib2f.load(&mut loadfn, "glVertexAttrib2f");
        storage::VertexAttrib2fv.load(&mut loadfn, "glVertexAttrib2fv");
        storage::VertexAttrib3f.load(&mut loadfn, "glVertexAttrib3f");
        storage::VertexAttrib3fv.load(&mut loadfn, "glVertexAttrib3fv");
        storage::VertexAttrib4f.load(&mut loadfn, "glVertexAttrib4f");
        storage::VertexAttrib4fv.load(&mut loadfn, "glVertexAttrib4fv");
        storage::VertexAttribDivisor.load(&mut loadfn, "glVertexAttribDivisor");
        storage::VertexAttribI4i.load(&mut loadfn, "glVertexAttribI4i");
        storage::VertexAttribI4iv.load(&mut loadfn, "glVertexAttribI4iv");
        storage::VertexAttribI4ui.load(&mut loadfn, "glVertexAttribI4ui");
        storage::VertexAttribI4uiv.load(&mut loadfn, "glVertexAttribI4uiv");
        storage::VertexAttribIPointer.load(&mut loadfn, "glVertexAttribIPointer");
        storage::VertexAttribPointer.load(&mut loadfn, "glVertexAttribPointer");
        storage::Viewport.load(&mut loadfn, "glViewport");
        storage::WaitSync.load(&mut loadfn, "glWaitSync");

        storage::DebugMessageCallback.aliased(&storage::DebugMessageCallbackKHR);
        storage::DebugMessageCallbackKHR.aliased(&storage::DebugMessageCallback);
        storage::DebugMessageControl.aliased(&storage::DebugMessageControlKHR);
        storage::DebugMessageControlKHR.aliased(&storage::DebugMessageControl);
        storage::DebugMessageInsert.aliased(&storage::DebugMessageInsertKHR);
        storage::DebugMessageInsertKHR.aliased(&storage::DebugMessageInsert);
        storage::GetDebugMessageLog.aliased(&storage::GetDebugMessageLogKHR);
        storage::GetDebugMessageLogKHR.aliased(&storage::GetDebugMessageLog);
        storage::GetObjectLabel.aliased(&storage::GetObjectLabelKHR);
        storage::GetObjectLabelKHR.aliased(&storage::GetObjectLabel);
        storage::GetObjectPtrLabel.aliased(&storage::GetObjectPtrLabelKHR);
        storage::GetObjectPtrLabelKHR.aliased(&storage::GetObjectPtrLabel);
        storage::GetPointerv.aliased(&storage::GetPointervKHR);
        storage::GetPointervKHR.aliased(&storage::GetPointerv);
        storage::ObjectLabel.aliased(&storage::ObjectLabelKHR);
        storage::ObjectLabelKHR.aliased(&storage::ObjectLabel);
        storage::ObjectPtrLabel.aliased(&storage::ObjectPtrLabelKHR);
        storage::ObjectPtrLabelKHR.aliased(&storage::ObjectPtrLabel);
        storage::PopDebugGroup.aliased(&storage::PopDebugGroupKHR);
        storage::PopDebugGroupKHR.aliased(&storage::PopDebugGroup);
        storage::PushDebugGroup.aliased(&storage::PushDebugGroupKHR);
        storage::PushDebugGroupKHR.aliased(&storage::PushDebugGroup);
    }
}
