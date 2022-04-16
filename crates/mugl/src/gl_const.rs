//! All WebGL core and extension constants.
//! See: <https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Constants>

#![allow(non_upper_case_globals)]

/* WebGL 1.0 constants */

/* ClearBufferMask */
pub const DEPTH_BUFFER_BIT: u32 = 0x00000100;
pub const STENCIL_BUFFER_BIT: u32 = 0x00000400;
pub const COLOR_BUFFER_BIT: u32 = 0x00004000;

/* BeginMode */
pub const POINTS: u32 = 0x0000;
pub const LINES: u32 = 0x0001;
pub const LINE_LOOP: u32 = 0x0002;
pub const LINE_STRIP: u32 = 0x0003;
pub const TRIANGLES: u32 = 0x0004;
pub const TRIANGLE_STRIP: u32 = 0x0005;
pub const TRIANGLE_FAN: u32 = 0x0006;

/* AlphaFunction (not supported in ES20) */
/*      NEVER */
/*      LESS */
/*      EQUAL */
/*      LEQUAL */
/*      GREATER */
/*      NOTEQUAL */
/*      GEQUAL */
/*      ALWAYS */

/* BlendingFactorDest */
pub const ZERO: u32 = 0;
pub const ONE: u32 = 1;
pub const SRC_COLOR: u32 = 0x0300;
pub const ONE_MINUS_SRC_COLOR: u32 = 0x0301;
pub const SRC_ALPHA: u32 = 0x0302;
pub const ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
pub const DST_ALPHA: u32 = 0x0304;
pub const ONE_MINUS_DST_ALPHA: u32 = 0x0305;

/* BlendingFactorSrc */
/*      ZERO */
/*      ONE */
pub const DST_COLOR: u32 = 0x0306;
pub const ONE_MINUS_DST_COLOR: u32 = 0x0307;
pub const SRC_ALPHA_SATURATE: u32 = 0x0308;
/*      SRC_ALPHA */
/*      ONE_MINUS_SRC_ALPHA */
/*      DST_ALPHA */
/*      ONE_MINUS_DST_ALPHA */

/* BlendEquationSeparate */
pub const FUNC_ADD: u32 = 0x8006;
pub const BLEND_EQUATION: u32 = 0x8009;
pub const BLEND_EQUATION_RGB: u32 = 0x8009; /* same as BLEND_EQUATION */
pub const BLEND_EQUATION_ALPHA: u32 = 0x883D;

/* BlendSubtract */
pub const FUNC_SUBTRACT: u32 = 0x800A;
pub const FUNC_REVERSE_SUBTRACT: u32 = 0x800B;

/* Separate Blend Functions */
pub const BLEND_DST_RGB: u32 = 0x80C8;
pub const BLEND_SRC_RGB: u32 = 0x80C9;
pub const BLEND_DST_ALPHA: u32 = 0x80CA;
pub const BLEND_SRC_ALPHA: u32 = 0x80CB;
pub const CONSTANT_COLOR: u32 = 0x8001;
pub const ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;
pub const CONSTANT_ALPHA: u32 = 0x8003;
pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;
pub const BLEND_COLOR: u32 = 0x8005;

/* Buffer Objects */
pub const ARRAY_BUFFER: u32 = 0x8892;
pub const ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
pub const ARRAY_BUFFER_BINDING: u32 = 0x8894;
pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = 0x8895;

pub const STREAM_DRAW: u32 = 0x88E0;
pub const STATIC_DRAW: u32 = 0x88E4;
pub const DYNAMIC_DRAW: u32 = 0x88E8;

pub const BUFFER_SIZE: u32 = 0x8764;
pub const BUFFER_USAGE: u32 = 0x8765;

pub const CURRENT_VERTEX_ATTRIB: u32 = 0x8626;

/* CullFaceMode */
pub const FRONT: u32 = 0x0404;
pub const BACK: u32 = 0x0405;
pub const FRONT_AND_BACK: u32 = 0x0408;

/* DepthFunction */
/*      NEVER */
/*      LESS */
/*      EQUAL */
/*      LEQUAL */
/*      GREATER */
/*      NOTEQUAL */
/*      GEQUAL */
/*      ALWAYS */

/* EnableCap */
/* TEXTURE_2D */
pub const CULL_FACE: u32 = 0x0B44;
pub const BLEND: u32 = 0x0BE2;
pub const DITHER: u32 = 0x0BD0;
pub const STENCIL_TEST: u32 = 0x0B90;
pub const DEPTH_TEST: u32 = 0x0B71;
pub const SCISSOR_TEST: u32 = 0x0C11;
pub const POLYGON_OFFSET_FILL: u32 = 0x8037;
pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809E;
pub const SAMPLE_COVERAGE: u32 = 0x80A0;

/* ErrorCode */
pub const NO_ERROR: u32 = 0;
pub const INVALID_ENUM: u32 = 0x0500;
pub const INVALID_VALUE: u32 = 0x0501;
pub const INVALID_OPERATION: u32 = 0x0502;
pub const OUT_OF_MEMORY: u32 = 0x0505;

/* FrontFaceDirection */
pub const CW: u32 = 0x0900;
pub const CCW: u32 = 0x0901;

/* GetPName */
pub const LINE_WIDTH: u32 = 0x0B21;
pub const ALIASED_POINT_SIZE_RANGE: u32 = 0x846D;
pub const ALIASED_LINE_WIDTH_RANGE: u32 = 0x846E;
pub const CULL_FACE_MODE: u32 = 0x0B45;
pub const FRONT_FACE: u32 = 0x0B46;
pub const DEPTH_RANGE: u32 = 0x0B70;
pub const DEPTH_WRITEMASK: u32 = 0x0B72;
pub const DEPTH_CLEAR_VALUE: u32 = 0x0B73;
pub const DEPTH_FUNC: u32 = 0x0B74;
pub const STENCIL_CLEAR_VALUE: u32 = 0x0B91;
pub const STENCIL_FUNC: u32 = 0x0B92;
pub const STENCIL_FAIL: u32 = 0x0B94;
pub const STENCIL_PASS_DEPTH_FAIL: u32 = 0x0B95;
pub const STENCIL_PASS_DEPTH_PASS: u32 = 0x0B96;
pub const STENCIL_REF: u32 = 0x0B97;
pub const STENCIL_VALUE_MASK: u32 = 0x0B93;
pub const STENCIL_WRITEMASK: u32 = 0x0B98;
pub const STENCIL_BACK_FUNC: u32 = 0x8800;
pub const STENCIL_BACK_FAIL: u32 = 0x8801;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 0x8802;
pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = 0x8803;
pub const STENCIL_BACK_REF: u32 = 0x8CA3;
pub const STENCIL_BACK_VALUE_MASK: u32 = 0x8CA4;
pub const STENCIL_BACK_WRITEMASK: u32 = 0x8CA5;
pub const VIEWPORT: u32 = 0x0BA2;
pub const SCISSOR_BOX: u32 = 0x0C10;
/*      SCISSOR_TEST */
pub const COLOR_CLEAR_VALUE: u32 = 0x0C22;
pub const COLOR_WRITEMASK: u32 = 0x0C23;
pub const UNPACK_ALIGNMENT: u32 = 0x0CF5;
pub const PACK_ALIGNMENT: u32 = 0x0D05;
pub const MAX_TEXTURE_SIZE: u32 = 0x0D33;
pub const MAX_VIEWPORT_DIMS: u32 = 0x0D3A;
pub const SUBPIXEL_BITS: u32 = 0x0D50;
pub const RED_BITS: u32 = 0x0D52;
pub const GREEN_BITS: u32 = 0x0D53;
pub const BLUE_BITS: u32 = 0x0D54;
pub const ALPHA_BITS: u32 = 0x0D55;
pub const DEPTH_BITS: u32 = 0x0D56;
pub const STENCIL_BITS: u32 = 0x0D57;
pub const POLYGON_OFFSET_UNITS: u32 = 0x2A00;
/*      POLYGON_OFFSET_FILL */
pub const POLYGON_OFFSET_FACTOR: u32 = 0x8038;
pub const TEXTURE_BINDING_2D: u32 = 0x8069;
pub const SAMPLE_BUFFERS: u32 = 0x80A8;
pub const SAMPLES: u32 = 0x80A9;
pub const SAMPLE_COVERAGE_VALUE: u32 = 0x80AA;
pub const SAMPLE_COVERAGE_INVERT: u32 = 0x80AB;

/* GetTextureParameter */
/*      TEXTURE_MAG_FILTER */
/*      TEXTURE_MIN_FILTER */
/*      TEXTURE_WRAP_S */
/*      TEXTURE_WRAP_T */

pub const COMPRESSED_TEXTURE_FORMATS: u32 = 0x86A3;

/* HintMode */
pub const DONT_CARE: u32 = 0x1100;
pub const FASTEST: u32 = 0x1101;
pub const NICEST: u32 = 0x1102;

/* HintTarget */
pub const GENERATE_MIPMAP_HINT: u32 = 0x8192;

/* DataType */
pub const BYTE: u32 = 0x1400;
pub const UNSIGNED_BYTE: u32 = 0x1401;
pub const SHORT: u32 = 0x1402;
pub const UNSIGNED_SHORT: u32 = 0x1403;
pub const INT: u32 = 0x1404;
pub const UNSIGNED_INT: u32 = 0x1405;
pub const FLOAT: u32 = 0x1406;

/* PixelFormat */
pub const DEPTH_COMPONENT: u32 = 0x1902;
pub const ALPHA: u32 = 0x1906;
pub const RGB: u32 = 0x1907;
pub const RGBA: u32 = 0x1908;
pub const LUMINANCE: u32 = 0x1909;
pub const LUMINANCE_ALPHA: u32 = 0x190A;

/* PixelType */
/*      UNSIGNED_BYTE */
pub const UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
pub const UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
pub const UNSIGNED_SHORT_5_6_5: u32 = 0x8363;

/* Shaders */
pub const FRAGMENT_SHADER: u32 = 0x8B30;
pub const VERTEX_SHADER: u32 = 0x8B31;
pub const MAX_VERTEX_ATTRIBS: u32 = 0x8869;
pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = 0x8DFB;
pub const MAX_VARYING_VECTORS: u32 = 0x8DFC;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8B4D;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 0x8B4C;
pub const MAX_TEXTURE_IMAGE_UNITS: u32 = 0x8872;
pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 0x8DFD;
pub const SHADER_TYPE: u32 = 0x8B4F;
pub const DELETE_STATUS: u32 = 0x8B80;
pub const LINK_STATUS: u32 = 0x8B82;
pub const VALIDATE_STATUS: u32 = 0x8B83;
pub const ATTACHED_SHADERS: u32 = 0x8B85;
pub const ACTIVE_UNIFORMS: u32 = 0x8B86;
pub const ACTIVE_ATTRIBUTES: u32 = 0x8B89;
pub const SHADING_LANGUAGE_VERSION: u32 = 0x8B8C;
pub const CURRENT_PROGRAM: u32 = 0x8B8D;

/* StencilFunction */
pub const NEVER: u32 = 0x0200;
pub const LESS: u32 = 0x0201;
pub const EQUAL: u32 = 0x0202;
pub const LEQUAL: u32 = 0x0203;
pub const GREATER: u32 = 0x0204;
pub const NOTEQUAL: u32 = 0x0205;
pub const GEQUAL: u32 = 0x0206;
pub const ALWAYS: u32 = 0x0207;

/* StencilOp */
/*      ZERO */
pub const KEEP: u32 = 0x1E00;
pub const REPLACE: u32 = 0x1E01;
pub const INCR: u32 = 0x1E02;
pub const DECR: u32 = 0x1E03;
pub const INVERT: u32 = 0x150A;
pub const INCR_WRAP: u32 = 0x8507;
pub const DECR_WRAP: u32 = 0x8508;

/* StringName */
pub const VENDOR: u32 = 0x1F00;
pub const RENDERER: u32 = 0x1F01;
pub const VERSION: u32 = 0x1F02;

/* TextureMagFilter */
pub const NEAREST: u32 = 0x2600;
pub const LINEAR: u32 = 0x2601;

/* TextureMinFilter */
/*      NEAREST */
/*      LINEAR */
pub const NEAREST_MIPMAP_NEAREST: u32 = 0x2700;
pub const LINEAR_MIPMAP_NEAREST: u32 = 0x2701;
pub const NEAREST_MIPMAP_LINEAR: u32 = 0x2702;
pub const LINEAR_MIPMAP_LINEAR: u32 = 0x2703;

/* TextureParameterName */
pub const TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const TEXTURE_WRAP_S: u32 = 0x2802;
pub const TEXTURE_WRAP_T: u32 = 0x2803;

/* TextureTarget */
pub const TEXTURE_2D: u32 = 0x0DE1;
pub const TEXTURE: u32 = 0x1702;

pub const TEXTURE_CUBE_MAP: u32 = 0x8513;
pub const TEXTURE_BINDING_CUBE_MAP: u32 = 0x8514;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 0x8515;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 0x8516;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 0x8517;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 0x8518;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 0x8519;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 0x851A;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851C;

/* TextureUnit */
pub const TEXTURE0: u32 = 0x84C0;
pub const TEXTURE1: u32 = 0x84C1;
pub const TEXTURE2: u32 = 0x84C2;
pub const TEXTURE3: u32 = 0x84C3;
pub const TEXTURE4: u32 = 0x84C4;
pub const TEXTURE5: u32 = 0x84C5;
pub const TEXTURE6: u32 = 0x84C6;
pub const TEXTURE7: u32 = 0x84C7;
pub const TEXTURE8: u32 = 0x84C8;
pub const TEXTURE9: u32 = 0x84C9;
pub const TEXTURE10: u32 = 0x84CA;
pub const TEXTURE11: u32 = 0x84CB;
pub const TEXTURE12: u32 = 0x84CC;
pub const TEXTURE13: u32 = 0x84CD;
pub const TEXTURE14: u32 = 0x84CE;
pub const TEXTURE15: u32 = 0x84CF;
pub const TEXTURE16: u32 = 0x84D0;
pub const TEXTURE17: u32 = 0x84D1;
pub const TEXTURE18: u32 = 0x84D2;
pub const TEXTURE19: u32 = 0x84D3;
pub const TEXTURE20: u32 = 0x84D4;
pub const TEXTURE21: u32 = 0x84D5;
pub const TEXTURE22: u32 = 0x84D6;
pub const TEXTURE23: u32 = 0x84D7;
pub const TEXTURE24: u32 = 0x84D8;
pub const TEXTURE25: u32 = 0x84D9;
pub const TEXTURE26: u32 = 0x84DA;
pub const TEXTURE27: u32 = 0x84DB;
pub const TEXTURE28: u32 = 0x84DC;
pub const TEXTURE29: u32 = 0x84DD;
pub const TEXTURE30: u32 = 0x84DE;
pub const TEXTURE31: u32 = 0x84DF;
pub const ACTIVE_TEXTURE: u32 = 0x84E0;

/* TextureWrapMode */
pub const REPEAT: u32 = 0x2901;
pub const CLAMP_TO_EDGE: u32 = 0x812F;
pub const MIRRORED_REPEAT: u32 = 0x8370;

/* Uniform Types */
pub const FLOAT_VEC2: u32 = 0x8B50;
pub const FLOAT_VEC3: u32 = 0x8B51;
pub const FLOAT_VEC4: u32 = 0x8B52;
pub const INT_VEC2: u32 = 0x8B53;
pub const INT_VEC3: u32 = 0x8B54;
pub const INT_VEC4: u32 = 0x8B55;
pub const BOOL: u32 = 0x8B56;
pub const BOOL_VEC2: u32 = 0x8B57;
pub const BOOL_VEC3: u32 = 0x8B58;
pub const BOOL_VEC4: u32 = 0x8B59;
pub const FLOAT_MAT2: u32 = 0x8B5A;
pub const FLOAT_MAT3: u32 = 0x8B5B;
pub const FLOAT_MAT4: u32 = 0x8B5C;
pub const SAMPLER_2D: u32 = 0x8B5E;
pub const SAMPLER_CUBE: u32 = 0x8B60;

/* Vertex Arrays */
pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 0x8622;
pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = 0x8623;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 0x8624;
pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = 0x8625;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 0x886A;
pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = 0x8645;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 0x889F;

/* Read Format */
pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 = 0x8B9A;
pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 0x8B9B;

/* Shader Source */
pub const COMPILE_STATUS: u32 = 0x8B81;

/* Shader Precision-Specified Types */
pub const LOW_FLOAT: u32 = 0x8DF0;
pub const MEDIUM_FLOAT: u32 = 0x8DF1;
pub const HIGH_FLOAT: u32 = 0x8DF2;
pub const LOW_INT: u32 = 0x8DF3;
pub const MEDIUM_INT: u32 = 0x8DF4;
pub const HIGH_INT: u32 = 0x8DF5;

/* Framebuffer Object. */
pub const FRAMEBUFFER: u32 = 0x8D40;
pub const RENDERBUFFER: u32 = 0x8D41;

pub const RGBA4: u32 = 0x8056;
pub const RGB5_A1: u32 = 0x8057;
pub const RGB565: u32 = 0x8D62;
pub const DEPTH_COMPONENT16: u32 = 0x81A5;
pub const STENCIL_INDEX: u32 = 0x1901;
pub const STENCIL_INDEX8: u32 = 0x8D48;
pub const DEPTH_STENCIL: u32 = 0x84F9;

pub const RENDERBUFFER_WIDTH: u32 = 0x8D42;
pub const RENDERBUFFER_HEIGHT: u32 = 0x8D43;
pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = 0x8D44;
pub const RENDERBUFFER_RED_SIZE: u32 = 0x8D50;
pub const RENDERBUFFER_GREEN_SIZE: u32 = 0x8D51;
pub const RENDERBUFFER_BLUE_SIZE: u32 = 0x8D52;
pub const RENDERBUFFER_ALPHA_SIZE: u32 = 0x8D53;
pub const RENDERBUFFER_DEPTH_SIZE: u32 = 0x8D54;
pub const RENDERBUFFER_STENCIL_SIZE: u32 = 0x8D55;

pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 0x8CD0;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 0x8CD1;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 0x8CD2;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 0x8CD3;

pub const COLOR_ATTACHMENT0: u32 = 0x8CE0;
pub const DEPTH_ATTACHMENT: u32 = 0x8D00;
pub const STENCIL_ATTACHMENT: u32 = 0x8D20;
pub const DEPTH_STENCIL_ATTACHMENT: u32 = 0x821A;

pub const NONE: u32 = 0;

pub const FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 0x8CD6;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 0x8CD7;
pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: u32 = 0x8CD9;
pub const FRAMEBUFFER_UNSUPPORTED: u32 = 0x8CDD;

pub const FRAMEBUFFER_BINDING: u32 = 0x8CA6;
pub const RENDERBUFFER_BINDING: u32 = 0x8CA7;
pub const MAX_RENDERBUFFER_SIZE: u32 = 0x84E8;

pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 0x0506;

/* WebGL-specific enums */
pub const UNPACK_FLIP_Y_WEBGL: u32 = 0x9240;
pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 0x9241;
pub const CONTEXT_LOST_WEBGL: u32 = 0x9242;
pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 = 0x9243;
pub const BROWSER_DEFAULT_WEBGL: u32 = 0x9244;

/* WebGL 2.0 constants */

pub const READ_BUFFER: u32 = 0x0C02;
pub const UNPACK_ROW_LENGTH: u32 = 0x0CF2;
pub const UNPACK_SKIP_ROWS: u32 = 0x0CF3;
pub const UNPACK_SKIP_PIXELS: u32 = 0x0CF4;
pub const PACK_ROW_LENGTH: u32 = 0x0D02;
pub const PACK_SKIP_ROWS: u32 = 0x0D03;
pub const PACK_SKIP_PIXELS: u32 = 0x0D04;
pub const COLOR: u32 = 0x1800;
pub const DEPTH: u32 = 0x1801;
pub const STENCIL: u32 = 0x1802;
pub const RED: u32 = 0x1903;
pub const RGB8: u32 = 0x8051;
pub const RGBA8: u32 = 0x8058;
pub const RGB10_A2: u32 = 0x8059;
pub const TEXTURE_BINDING_3D: u32 = 0x806A;
pub const UNPACK_SKIP_IMAGES: u32 = 0x806D;
pub const UNPACK_IMAGE_HEIGHT: u32 = 0x806E;
pub const TEXTURE_3D: u32 = 0x806F;
pub const TEXTURE_WRAP_R: u32 = 0x8072;
pub const MAX_3D_TEXTURE_SIZE: u32 = 0x8073;
pub const UNSIGNED_INT_2_10_10_10_REV: u32 = 0x8368;
pub const MAX_ELEMENTS_VERTICES: u32 = 0x80E8;
pub const MAX_ELEMENTS_INDICES: u32 = 0x80E9;
pub const TEXTURE_MIN_LOD: u32 = 0x813A;
pub const TEXTURE_MAX_LOD: u32 = 0x813B;
pub const TEXTURE_BASE_LEVEL: u32 = 0x813C;
pub const TEXTURE_MAX_LEVEL: u32 = 0x813D;
pub const MIN: u32 = 0x8007;
pub const MAX: u32 = 0x8008;
pub const DEPTH_COMPONENT24: u32 = 0x81A6;
pub const MAX_TEXTURE_LOD_BIAS: u32 = 0x84FD;
pub const TEXTURE_COMPARE_MODE: u32 = 0x884C;
pub const TEXTURE_COMPARE_FUNC: u32 = 0x884D;
pub const CURRENT_QUERY: u32 = 0x8865;
pub const QUERY_RESULT: u32 = 0x8866;
pub const QUERY_RESULT_AVAILABLE: u32 = 0x8867;
pub const STREAM_READ: u32 = 0x88E1;
pub const STREAM_COPY: u32 = 0x88E2;
pub const STATIC_READ: u32 = 0x88E5;
pub const STATIC_COPY: u32 = 0x88E6;
pub const DYNAMIC_READ: u32 = 0x88E9;
pub const DYNAMIC_COPY: u32 = 0x88EA;
pub const MAX_DRAW_BUFFERS: u32 = 0x8824;
pub const DRAW_BUFFER0: u32 = 0x8825;
pub const DRAW_BUFFER1: u32 = 0x8826;
pub const DRAW_BUFFER2: u32 = 0x8827;
pub const DRAW_BUFFER3: u32 = 0x8828;
pub const DRAW_BUFFER4: u32 = 0x8829;
pub const DRAW_BUFFER5: u32 = 0x882A;
pub const DRAW_BUFFER6: u32 = 0x882B;
pub const DRAW_BUFFER7: u32 = 0x882C;
pub const DRAW_BUFFER8: u32 = 0x882D;
pub const DRAW_BUFFER9: u32 = 0x882E;
pub const DRAW_BUFFER10: u32 = 0x882F;
pub const DRAW_BUFFER11: u32 = 0x8830;
pub const DRAW_BUFFER12: u32 = 0x8831;
pub const DRAW_BUFFER13: u32 = 0x8832;
pub const DRAW_BUFFER14: u32 = 0x8833;
pub const DRAW_BUFFER15: u32 = 0x8834;
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8B49;
pub const MAX_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8B4A;
pub const SAMPLER_3D: u32 = 0x8B5F;
pub const SAMPLER_2D_SHADOW: u32 = 0x8B62;
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: u32 = 0x8B8B;
pub const PIXEL_PACK_BUFFER: u32 = 0x88EB;
pub const PIXEL_UNPACK_BUFFER: u32 = 0x88EC;
pub const PIXEL_PACK_BUFFER_BINDING: u32 = 0x88ED;
pub const PIXEL_UNPACK_BUFFER_BINDING: u32 = 0x88EF;
pub const FLOAT_MAT2x3: u32 = 0x8B65;
pub const FLOAT_MAT2x4: u32 = 0x8B66;
pub const FLOAT_MAT3x2: u32 = 0x8B67;
pub const FLOAT_MAT3x4: u32 = 0x8B68;
pub const FLOAT_MAT4x2: u32 = 0x8B69;
pub const FLOAT_MAT4x3: u32 = 0x8B6A;
pub const SRGB: u32 = 0x8C40;
pub const SRGB8: u32 = 0x8C41;
pub const SRGB8_ALPHA8: u32 = 0x8C43;
pub const COMPARE_REF_TO_TEXTURE: u32 = 0x884E;
pub const RGBA32F: u32 = 0x8814;
pub const RGB32F: u32 = 0x8815;
pub const RGBA16F: u32 = 0x881A;
pub const RGB16F: u32 = 0x881B;
pub const VERTEX_ATTRIB_ARRAY_INTEGER: u32 = 0x88FD;
pub const MAX_ARRAY_TEXTURE_LAYERS: u32 = 0x88FF;
pub const MIN_PROGRAM_TEXEL_OFFSET: u32 = 0x8904;
pub const MAX_PROGRAM_TEXEL_OFFSET: u32 = 0x8905;
pub const MAX_VARYING_COMPONENTS: u32 = 0x8B4B;
pub const TEXTURE_2D_ARRAY: u32 = 0x8C1A;
pub const TEXTURE_BINDING_2D_ARRAY: u32 = 0x8C1D;
pub const R11F_G11F_B10F: u32 = 0x8C3A;
pub const UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8C3B;
pub const RGB9_E5: u32 = 0x8C3D;
pub const UNSIGNED_INT_5_9_9_9_REV: u32 = 0x8C3E;
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: u32 = 0x8C7F;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 = 0x8C80;
pub const TRANSFORM_FEEDBACK_VARYINGS: u32 = 0x8C83;
pub const TRANSFORM_FEEDBACK_BUFFER_START: u32 = 0x8C84;
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: u32 = 0x8C85;
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: u32 = 0x8C88;
pub const RASTERIZER_DISCARD: u32 = 0x8C89;
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: u32 = 0x8C8A;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: u32 = 0x8C8B;
pub const INTERLEAVED_ATTRIBS: u32 = 0x8C8C;
pub const SEPARATE_ATTRIBS: u32 = 0x8C8D;
pub const TRANSFORM_FEEDBACK_BUFFER: u32 = 0x8C8E;
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: u32 = 0x8C8F;
pub const RGBA32UI: u32 = 0x8D70;
pub const RGB32UI: u32 = 0x8D71;
pub const RGBA16UI: u32 = 0x8D76;
pub const RGB16UI: u32 = 0x8D77;
pub const RGBA8UI: u32 = 0x8D7C;
pub const RGB8UI: u32 = 0x8D7D;
pub const RGBA32I: u32 = 0x8D82;
pub const RGB32I: u32 = 0x8D83;
pub const RGBA16I: u32 = 0x8D88;
pub const RGB16I: u32 = 0x8D89;
pub const RGBA8I: u32 = 0x8D8E;
pub const RGB8I: u32 = 0x8D8F;
pub const RED_INTEGER: u32 = 0x8D94;
pub const RGB_INTEGER: u32 = 0x8D98;
pub const RGBA_INTEGER: u32 = 0x8D99;
pub const SAMPLER_2D_ARRAY: u32 = 0x8DC1;
pub const SAMPLER_2D_ARRAY_SHADOW: u32 = 0x8DC4;
pub const SAMPLER_CUBE_SHADOW: u32 = 0x8DC5;
pub const UNSIGNED_INT_VEC2: u32 = 0x8DC6;
pub const UNSIGNED_INT_VEC3: u32 = 0x8DC7;
pub const UNSIGNED_INT_VEC4: u32 = 0x8DC8;
pub const INT_SAMPLER_2D: u32 = 0x8DCA;
pub const INT_SAMPLER_3D: u32 = 0x8DCB;
pub const INT_SAMPLER_CUBE: u32 = 0x8DCC;
pub const INT_SAMPLER_2D_ARRAY: u32 = 0x8DCF;
pub const UNSIGNED_INT_SAMPLER_2D: u32 = 0x8DD2;
pub const UNSIGNED_INT_SAMPLER_3D: u32 = 0x8DD3;
pub const UNSIGNED_INT_SAMPLER_CUBE: u32 = 0x8DD4;
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: u32 = 0x8DD7;
pub const DEPTH_COMPONENT32F: u32 = 0x8CAC;
pub const DEPTH32F_STENCIL8: u32 = 0x8CAD;
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: u32 = 0x8DAD;
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: u32 = 0x8210;
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: u32 = 0x8211;
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: u32 = 0x8212;
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: u32 = 0x8213;
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: u32 = 0x8214;
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: u32 = 0x8215;
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: u32 = 0x8216;
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: u32 = 0x8217;
pub const FRAMEBUFFER_DEFAULT: u32 = 0x8218;
pub const UNSIGNED_INT_24_8: u32 = 0x84FA;
pub const DEPTH24_STENCIL8: u32 = 0x88F0;
pub const UNSIGNED_NORMALIZED: u32 = 0x8C17;
pub const DRAW_FRAMEBUFFER_BINDING: u32 = 0x8CA6; /* Same as FRAMEBUFFER_BINDING */
pub const READ_FRAMEBUFFER: u32 = 0x8CA8;
pub const DRAW_FRAMEBUFFER: u32 = 0x8CA9;
pub const READ_FRAMEBUFFER_BINDING: u32 = 0x8CAA;
pub const RENDERBUFFER_SAMPLES: u32 = 0x8CAB;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: u32 = 0x8CD4;
pub const MAX_COLOR_ATTACHMENTS: u32 = 0x8CDF;
pub const COLOR_ATTACHMENT1: u32 = 0x8CE1;
pub const COLOR_ATTACHMENT2: u32 = 0x8CE2;
pub const COLOR_ATTACHMENT3: u32 = 0x8CE3;
pub const COLOR_ATTACHMENT4: u32 = 0x8CE4;
pub const COLOR_ATTACHMENT5: u32 = 0x8CE5;
pub const COLOR_ATTACHMENT6: u32 = 0x8CE6;
pub const COLOR_ATTACHMENT7: u32 = 0x8CE7;
pub const COLOR_ATTACHMENT8: u32 = 0x8CE8;
pub const COLOR_ATTACHMENT9: u32 = 0x8CE9;
pub const COLOR_ATTACHMENT10: u32 = 0x8CEA;
pub const COLOR_ATTACHMENT11: u32 = 0x8CEB;
pub const COLOR_ATTACHMENT12: u32 = 0x8CEC;
pub const COLOR_ATTACHMENT13: u32 = 0x8CED;
pub const COLOR_ATTACHMENT14: u32 = 0x8CEE;
pub const COLOR_ATTACHMENT15: u32 = 0x8CEF;
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: u32 = 0x8D56;
pub const MAX_SAMPLES: u32 = 0x8D57;
pub const HALF_FLOAT: u32 = 0x140B;
pub const RG: u32 = 0x8227;
pub const RG_INTEGER: u32 = 0x8228;
pub const R8: u32 = 0x8229;
pub const RG8: u32 = 0x822B;
pub const R16F: u32 = 0x822D;
pub const R32F: u32 = 0x822E;
pub const RG16F: u32 = 0x822F;
pub const RG32F: u32 = 0x8230;
pub const R8I: u32 = 0x8231;
pub const R8UI: u32 = 0x8232;
pub const R16I: u32 = 0x8233;
pub const R16UI: u32 = 0x8234;
pub const R32I: u32 = 0x8235;
pub const R32UI: u32 = 0x8236;
pub const RG8I: u32 = 0x8237;
pub const RG8UI: u32 = 0x8238;
pub const RG16I: u32 = 0x8239;
pub const RG16UI: u32 = 0x823A;
pub const RG32I: u32 = 0x823B;
pub const RG32UI: u32 = 0x823C;
pub const VERTEX_ARRAY_BINDING: u32 = 0x85B5;
pub const R8_SNORM: u32 = 0x8F94;
pub const RG8_SNORM: u32 = 0x8F95;
pub const RGB8_SNORM: u32 = 0x8F96;
pub const RGBA8_SNORM: u32 = 0x8F97;
pub const SIGNED_NORMALIZED: u32 = 0x8F9C;
pub const COPY_READ_BUFFER: u32 = 0x8F36;
pub const COPY_WRITE_BUFFER: u32 = 0x8F37;
pub const COPY_READ_BUFFER_BINDING: u32 = 0x8F36; /* Same as COPY_READ_BUFFER */
pub const COPY_WRITE_BUFFER_BINDING: u32 = 0x8F37; /* Same as COPY_WRITE_BUFFER */
pub const UNIFORM_BUFFER: u32 = 0x8A11;
pub const UNIFORM_BUFFER_BINDING: u32 = 0x8A28;
pub const UNIFORM_BUFFER_START: u32 = 0x8A29;
pub const UNIFORM_BUFFER_SIZE: u32 = 0x8A2A;
pub const MAX_VERTEX_UNIFORM_BLOCKS: u32 = 0x8A2B;
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: u32 = 0x8A2D;
pub const MAX_COMBINED_UNIFORM_BLOCKS: u32 = 0x8A2E;
pub const MAX_UNIFORM_BUFFER_BINDINGS: u32 = 0x8A2F;
pub const MAX_UNIFORM_BLOCK_SIZE: u32 = 0x8A30;
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8A31;
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8A33;
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: u32 = 0x8A34;
pub const ACTIVE_UNIFORM_BLOCKS: u32 = 0x8A36;
pub const UNIFORM_TYPE: u32 = 0x8A37;
pub const UNIFORM_SIZE: u32 = 0x8A38;
pub const UNIFORM_BLOCK_INDEX: u32 = 0x8A3A;
pub const UNIFORM_OFFSET: u32 = 0x8A3B;
pub const UNIFORM_ARRAY_STRIDE: u32 = 0x8A3C;
pub const UNIFORM_MATRIX_STRIDE: u32 = 0x8A3D;
pub const UNIFORM_IS_ROW_MAJOR: u32 = 0x8A3E;
pub const UNIFORM_BLOCK_BINDING: u32 = 0x8A3F;
pub const UNIFORM_BLOCK_DATA_SIZE: u32 = 0x8A40;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: u32 = 0x8A42;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: u32 = 0x8A43;
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: u32 = 0x8A44;
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: u32 = 0x8A46;
pub const INVALID_INDEX: u32 = 0xFFFFFFFF;
pub const MAX_VERTEX_OUTPUT_COMPONENTS: u32 = 0x9122;
pub const MAX_FRAGMENT_INPUT_COMPONENTS: u32 = 0x9125;
pub const MAX_SERVER_WAIT_TIMEOUT: u32 = 0x9111;
pub const OBJECT_TYPE: u32 = 0x9112;
pub const SYNC_CONDITION: u32 = 0x9113;
pub const SYNC_STATUS: u32 = 0x9114;
pub const SYNC_FLAGS: u32 = 0x9115;
pub const SYNC_FENCE: u32 = 0x9116;
pub const SYNC_GPU_COMMANDS_COMPLETE: u32 = 0x9117;
pub const UNSIGNALED: u32 = 0x9118;
pub const SIGNALED: u32 = 0x9119;
pub const ALREADY_SIGNALED: u32 = 0x911A;
pub const TIMEOUT_EXPIRED: u32 = 0x911B;
pub const CONDITION_SATISFIED: u32 = 0x911C;
pub const WAIT_FAILED: u32 = 0x911D;
pub const SYNC_FLUSH_COMMANDS_BIT: u32 = 0x00000001;
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: u32 = 0x88FE;
pub const ANY_SAMPLES_PASSED: u32 = 0x8C2F;
pub const ANY_SAMPLES_PASSED_CONSERVATIVE: u32 = 0x8D6A;
pub const SAMPLER_BINDING: u32 = 0x8919;
pub const RGB10_A2UI: u32 = 0x906F;
pub const INT_2_10_10_10_REV: u32 = 0x8D9F;
pub const TRANSFORM_FEEDBACK: u32 = 0x8E22;
pub const TRANSFORM_FEEDBACK_PAUSED: u32 = 0x8E23;
pub const TRANSFORM_FEEDBACK_ACTIVE: u32 = 0x8E24;
pub const TRANSFORM_FEEDBACK_BINDING: u32 = 0x8E25;
pub const TEXTURE_IMMUTABLE_FORMAT: u32 = 0x912F;
pub const MAX_ELEMENT_INDEX: u32 = 0x8D6B;
pub const TEXTURE_IMMUTABLE_LEVELS: u32 = 0x82DF;

pub const TIMEOUT_IGNORED: i32 = -1;

/* WebGL-specific enums */
pub const MAX_CLIENT_WAIT_TIMEOUT_WEBGL: u32 = 0x9247;

/* WebGL extension constants */

/* ANGLE_instanced_arrays  */
pub const VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: u32 = 0x88FE;

/* EXT_blend_minmax */
pub const MIN_EXT: u32 = 0x8007;
pub const MAX_EXT: u32 = 0x8008;

/* EXT_clip_cull_distance */
pub const MAX_CLIP_DISTANCES_EXT: u32 = 0x0D32;
pub const MAX_CULL_DISTANCES_EXT: u32 = 0x82F9;
pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES_EXT: u32 = 0x82FA;

pub const CLIP_DISTANCE0_EXT: u32 = 0x3000;
pub const CLIP_DISTANCE1_EXT: u32 = 0x3001;
pub const CLIP_DISTANCE2_EXT: u32 = 0x3002;
pub const CLIP_DISTANCE3_EXT: u32 = 0x3003;
pub const CLIP_DISTANCE4_EXT: u32 = 0x3004;
pub const CLIP_DISTANCE5_EXT: u32 = 0x3005;
pub const CLIP_DISTANCE6_EXT: u32 = 0x3006;
pub const CLIP_DISTANCE7_EXT: u32 = 0x3007;

/* EXT_color_buffer_half_float */
pub const RGBA16F_EXT: u32 = 0x881A;
pub const RGB16F_EXT: u32 = 0x881B;
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: u32 = 0x8211;
pub const UNSIGNED_NORMALIZED_EXT: u32 = 0x8C17;

/* EXT_disjoint_timer_query */
pub const QUERY_COUNTER_BITS_EXT: u32 = 0x8864;
pub const CURRENT_QUERY_EXT: u32 = 0x8865;
pub const QUERY_RESULT_EXT: u32 = 0x8866;
pub const QUERY_RESULT_AVAILABLE_EXT: u32 = 0x8867;
pub const TIME_ELAPSED_EXT: u32 = 0x88BF;
pub const TIMESTAMP_EXT: u32 = 0x8E28;
pub const GPU_DISJOINT_EXT: u32 = 0x8FBB;

/* EXT_disjoint_timer_query_webgl2 */
/*
pub const QUERY_COUNTER_BITS_EXT  : u32 = 0x8864;
pub const TIME_ELAPSED_EXT  : u32 = 0x88BF;
pub const TIMESTAMP_EXT  : u32 = 0x8E28;
pub const GPU_DISJOINT_EXT  : u32 = 0x8FBB;
*/

/* EXT_sRGB */
pub const SRGB_EXT: u32 = 0x8C40;
pub const SRGB_ALPHA_EXT: u32 = 0x8C42;
pub const SRGB8_ALPHA8_EXT: u32 = 0x8C43;
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: u32 = 0x8210;

/* EXT_texture_compression_bptc */
pub const COMPRESSED_RGBA_BPTC_UNORM_EXT: u32 = 0x8E8C;
pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT: u32 = 0x8E8D;
pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT: u32 = 0x8E8E;
pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT: u32 = 0x8E8F;

/* EXT_texture_compression_rgtc */
pub const COMPRESSED_RED_RGTC1_EXT: u32 = 0x8DBB;
pub const COMPRESSED_SIGNED_RED_RGTC1_EXT: u32 = 0x8DBC;
pub const COMPRESSED_RED_GREEN_RGTC2_EXT: u32 = 0x8DBD;
pub const COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT: u32 = 0x8DBE;

/* EXT_texture_filter_anisotropic */
pub const TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FF;

/* EXT_texture_norm16 */
pub const R16_EXT: u32 = 0x822A;
pub const RG16_EXT: u32 = 0x822C;
pub const RGB16_EXT: u32 = 0x8054;
pub const RGBA16_EXT: u32 = 0x805B;
pub const R16_SNORM_EXT: u32 = 0x8F98;
pub const RG16_SNORM_EXT: u32 = 0x8F99;
pub const RGB16_SNORM_EXT: u32 = 0x8F9A;
pub const RGBA16_SNORM_EXT: u32 = 0x8F9B;

/* KHR_parallel_shader_compile */
pub const COMPLETION_STATUS_KHR: u32 = 0x91B1;

/* OES_standard_derivatives */
pub const FRAGMENT_SHADER_DERIVATIVE_HINT_OES: u32 = 0x8B8B;

/* OES_texture_half_float */
pub const HALF_FLOAT_OES: u32 = 0x8D61;

/* OES_vertex_array_object */
pub const VERTEX_ARRAY_BINDING_OES: u32 = 0x85B5;

/* OVR_multiview2 */
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_NUM_VIEWS_OVR: u32 = 0x9630;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_BASE_VIEW_INDEX_OVR: u32 = 0x9632;
pub const MAX_VIEWS_OVR: u32 = 0x9631;
pub const FRAMEBUFFER_INCOMPLETE_VIEW_TARGETS_OVR: u32 = 0x9633;

/* WEBGL_blend_equation_advanced_coherent  */
pub const MULTIPLY: u32 = 0x9294;
pub const SCREEN: u32 = 0x9295;
pub const OVERLAY: u32 = 0x9296;
pub const DARKEN: u32 = 0x9297;
pub const LIGHTEN: u32 = 0x9298;
pub const COLORDODGE: u32 = 0x9299;
pub const COLORBURN: u32 = 0x929A;
pub const HARDLIGHT: u32 = 0x929B;
pub const SOFTLIGHT: u32 = 0x929C;
pub const DIFFERENCE: u32 = 0x929E;
pub const EXCLUSION: u32 = 0x92A0;
pub const HSL_HUE: u32 = 0x92AD;
pub const HSL_SATURATION: u32 = 0x92AE;
pub const HSL_COLOR: u32 = 0x92AF;
pub const HSL_LUMINOSITY: u32 = 0x92B0;

/* WEBGL_color_buffer_float */
pub const RGBA32F_EXT: u32 = 0x8814;
/*
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT  : u32 = 0x8211;
pub const UNSIGNED_NORMALIZED_EXT  : u32 = 0x8C17;
*/

/* WEBGL_compressed_texture_astc */
/* Compressed Texture Format */
pub const COMPRESSED_RGBA_ASTC_4x4_KHR: u32 = 0x93B0;
pub const COMPRESSED_RGBA_ASTC_5x4_KHR: u32 = 0x93B1;
pub const COMPRESSED_RGBA_ASTC_5x5_KHR: u32 = 0x93B2;
pub const COMPRESSED_RGBA_ASTC_6x5_KHR: u32 = 0x93B3;
pub const COMPRESSED_RGBA_ASTC_6x6_KHR: u32 = 0x93B4;
pub const COMPRESSED_RGBA_ASTC_8x5_KHR: u32 = 0x93B5;
pub const COMPRESSED_RGBA_ASTC_8x6_KHR: u32 = 0x93B6;
pub const COMPRESSED_RGBA_ASTC_8x8_KHR: u32 = 0x93B7;
pub const COMPRESSED_RGBA_ASTC_10x5_KHR: u32 = 0x93B8;
pub const COMPRESSED_RGBA_ASTC_10x6_KHR: u32 = 0x93B9;
pub const COMPRESSED_RGBA_ASTC_10x8_KHR: u32 = 0x93BA;
pub const COMPRESSED_RGBA_ASTC_10x10_KHR: u32 = 0x93BB;
pub const COMPRESSED_RGBA_ASTC_12x10_KHR: u32 = 0x93BC;
pub const COMPRESSED_RGBA_ASTC_12x12_KHR: u32 = 0x93BD;

pub const COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: u32 = 0x93D0;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: u32 = 0x93D1;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: u32 = 0x93D2;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: u32 = 0x93D3;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: u32 = 0x93D4;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: u32 = 0x93D5;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: u32 = 0x93D6;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: u32 = 0x93D7;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: u32 = 0x93D8;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: u32 = 0x93D9;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: u32 = 0x93DA;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: u32 = 0x93DB;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: u32 = 0x93DC;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: u32 = 0x93DD;

/* WEBGL_compressed_texture_etc */
/* Compressed Texture Formats */
pub const COMPRESSED_R11_EAC: u32 = 0x9270;
pub const COMPRESSED_SIGNED_R11_EAC: u32 = 0x9271;
pub const COMPRESSED_RG11_EAC: u32 = 0x9272;
pub const COMPRESSED_SIGNED_RG11_EAC: u32 = 0x9273;
pub const COMPRESSED_RGB8_ETC2: u32 = 0x9274;
pub const COMPRESSED_SRGB8_ETC2: u32 = 0x9275;
pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 0x9276;
pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 0x9277;
pub const COMPRESSED_RGBA8_ETC2_EAC: u32 = 0x9278;
pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: u32 = 0x9279;

/* WEBGL_compressed_texture_etc1 */
/* Compressed Texture Format */
pub const COMPRESSED_RGB_ETC1_WEBGL: u32 = 0x8D64;

/* WEBGL_compressed_texture_pvrtc */
/* Compressed Texture Formats */
pub const COMPRESSED_RGB_PVRTC_4BPPV1_IMG: u32 = 0x8C00;
pub const COMPRESSED_RGB_PVRTC_2BPPV1_IMG: u32 = 0x8C01;
pub const COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: u32 = 0x8C02;
pub const COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: u32 = 0x8C03;

/* WEBGL_compressed_texture_s3tc */
/* Compressed Texture Formats */
pub const COMPRESSED_RGB_S3TC_DXT1_EXT: u32 = 0x83F0;
pub const COMPRESSED_RGBA_S3TC_DXT1_EXT: u32 = 0x83F1;
pub const COMPRESSED_RGBA_S3TC_DXT3_EXT: u32 = 0x83F2;
pub const COMPRESSED_RGBA_S3TC_DXT5_EXT: u32 = 0x83F3;

/* WEBGL_compressed_texture_s3tc_srgb */
/* Compressed Texture Formats */
pub const COMPRESSED_SRGB_S3TC_DXT1_EXT: u32 = 0x8C4C;
pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: u32 = 0x8C4D;
pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: u32 = 0x8C4E;
pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: u32 = 0x8C4F;

/* WEBGL_debug_renderer_info */
pub const UNMASKED_VENDOR_WEBGL: u32 = 0x9245;
pub const UNMASKED_RENDERER_WEBGL: u32 = 0x9246;

/* WEBGL_depth_texture */
pub const UNSIGNED_INT_24_8_WEBGL: u32 = 0x84FA;

/* WEBGL_draw_buffers */
pub const COLOR_ATTACHMENT0_WEBGL: u32 = 0x8CE0;
pub const COLOR_ATTACHMENT1_WEBGL: u32 = 0x8CE1;
pub const COLOR_ATTACHMENT2_WEBGL: u32 = 0x8CE2;
pub const COLOR_ATTACHMENT3_WEBGL: u32 = 0x8CE3;
pub const COLOR_ATTACHMENT4_WEBGL: u32 = 0x8CE4;
pub const COLOR_ATTACHMENT5_WEBGL: u32 = 0x8CE5;
pub const COLOR_ATTACHMENT6_WEBGL: u32 = 0x8CE6;
pub const COLOR_ATTACHMENT7_WEBGL: u32 = 0x8CE7;
pub const COLOR_ATTACHMENT8_WEBGL: u32 = 0x8CE8;
pub const COLOR_ATTACHMENT9_WEBGL: u32 = 0x8CE9;
pub const COLOR_ATTACHMENT10_WEBGL: u32 = 0x8CEA;
pub const COLOR_ATTACHMENT11_WEBGL: u32 = 0x8CEB;
pub const COLOR_ATTACHMENT12_WEBGL: u32 = 0x8CEC;
pub const COLOR_ATTACHMENT13_WEBGL: u32 = 0x8CED;
pub const COLOR_ATTACHMENT14_WEBGL: u32 = 0x8CEE;
pub const COLOR_ATTACHMENT15_WEBGL: u32 = 0x8CEF;

pub const DRAW_BUFFER0_WEBGL: u32 = 0x8825;
pub const DRAW_BUFFER1_WEBGL: u32 = 0x8826;
pub const DRAW_BUFFER2_WEBGL: u32 = 0x8827;
pub const DRAW_BUFFER3_WEBGL: u32 = 0x8828;
pub const DRAW_BUFFER4_WEBGL: u32 = 0x8829;
pub const DRAW_BUFFER5_WEBGL: u32 = 0x882A;
pub const DRAW_BUFFER6_WEBGL: u32 = 0x882B;
pub const DRAW_BUFFER7_WEBGL: u32 = 0x882C;
pub const DRAW_BUFFER8_WEBGL: u32 = 0x882D;
pub const DRAW_BUFFER9_WEBGL: u32 = 0x882E;
pub const DRAW_BUFFER10_WEBGL: u32 = 0x882F;
pub const DRAW_BUFFER11_WEBGL: u32 = 0x8830;
pub const DRAW_BUFFER12_WEBGL: u32 = 0x8831;
pub const DRAW_BUFFER13_WEBGL: u32 = 0x8832;
pub const DRAW_BUFFER14_WEBGL: u32 = 0x8833;
pub const DRAW_BUFFER15_WEBGL: u32 = 0x8834;

pub const MAX_COLOR_ATTACHMENTS_WEBGL: u32 = 0x8CDF;
pub const MAX_DRAW_BUFFERS_WEBGL: u32 = 0x8824;
