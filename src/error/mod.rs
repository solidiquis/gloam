use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("no model is currently bound to the context")]
    NoBoundModel,

    #[error("number of values for {0} attribute should be evenly divisible by component size")]
    ComponentSizeValuesMistmatch(&'static str),

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

    #[error("{0}")]
    Boxed(BoxError),

    #[error("{0}")]
    Misc(&'static str),
}

impl Error {
    pub fn boxed(error: impl Into<BoxError>) -> Self {
        Self::Boxed(error.into())
    }
}
