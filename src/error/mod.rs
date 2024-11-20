use gl::types::GLenum;
use thiserror::Error;

#[macro_export]
macro_rules! misc_error {
    ($e:expr) => {
        Error::Misc(format!($e))
    };
}
pub use misc_error;

pub type Result<T> = std::result::Result<T, Error>;
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("no vertex object is currently bound")]
    NoBoundVertexObject,

    #[error("specified vertex object is not currently bound")]
    VertexObjectNotBound,

    #[error("no program is currently in use")]
    NoActiveProgram,

    #[error("found index {index} but only {num_vertices} vertices")]
    InvalidIndex { index: u32, num_vertices: i32 },

    #[error("number of {attr_name_a} vertex attributes should match number of {attr_name_b}")]
    AttributeValuesMistmatch {
        attr_name_a: &'static str,
        attr_name_b: &'static str,
    },

    #[error("failed to query location of {0} attribute")]
    AttributeLocNotFound(String),

    #[error("failed to query location of {0} uniform")]
    UniformLocNotFound(String),

    #[error("an error occurred while linking program: {0}")]
    ProgramLink(String),

    #[error("an error occurred while compiling {shader_name}: {reason}")]
    ShaderCompile { shader_name: String, reason: String },

    #[error("all texture units are currently active")]
    MaxActiveTextures,

    #[error("expected object desciptor kind for type requested")]
    UnexpectedObjectDescriptorKind,

    #[error("use of object desciptor that has already been invalidated")]
    InvalidObjectDescriptor,

    #[error("an OpenGL error occurred: {0}")]
    GLError(&'static str),

    #[error("invalid vertex object configuration: {0}")]
    InvalidVertexObject(String),

    #[error("no corresponding texture unit for value {0}")]
    InvalidTextureUnit(usize),

    #[error("{0}")]
    Boxed(BoxError),

    #[error("{0}")]
    Misc(String),
}

impl Error {
    pub fn boxed(error: impl Into<BoxError>) -> Self {
        Self::Boxed(error.into())
    }
}

pub fn gl_check_err() -> Result<()> {
    let error = unsafe { gl::GetError() };
    if error == gl::NO_ERROR {
        return Ok(());
    }
    Err(Error::GLError(error_gl_enum_display(error)))
}

fn error_gl_enum_display(gl_enum: GLenum) -> &'static str {
    match gl_enum {
        gl::NO_ERROR => "GL_NO_ERROR: No error has been recorded.",
        gl::INVALID_ENUM => {
            "GL_INVALID_ENUM: An unacceptable value is specified for an enumerated argument."
        }
        gl::INVALID_VALUE => "GL_INVALID_VALUE: A numeric argument is out of range.",
        gl::INVALID_OPERATION => {
            "GL_INVALID_OPERATION: The specified operation is not allowed in the current state."
        }
        gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW: This command would cause a stack overflow.",
        gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW: This command would cause a stack underflow.",
        gl::OUT_OF_MEMORY => {
            "GL_OUT_OF_MEMORY: There is not enough memory left to execute the command."
        }
        gl::INVALID_FRAMEBUFFER_OPERATION => {
            "GL_INVALID_FRAMEBUFFER_OPERATION: The framebuffer object is not complete."
        }
        _ => "unknown error",
    }
}
