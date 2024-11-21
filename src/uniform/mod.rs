use gl::types::{
    GLdouble,
    GLfloat,
    GLint,
};
use nalgebra_glm as glm;

#[derive(Debug)]
pub struct Uniform {
    pub name: String,
    pub kind: UniformType,

    // Only used for matrix uniforms
    pub transpose: bool,
}

#[derive(Debug)]
pub enum UniformType {
    D(GLdouble),
    F(GLfloat),
    I(GLint),
    D2(glm::TVec2<GLdouble>),
    F2(glm::TVec2<GLfloat>),
    I2(glm::TVec2<GLint>),
    D3(glm::TVec3<GLdouble>),
    F3(glm::TVec3<GLfloat>),
    I3(glm::TVec3<GLint>),
    D4(glm::TVec4<GLdouble>),
    F4(glm::TVec4<GLfloat>),
    I4(glm::TVec4<GLint>),
    DMat2(glm::TMat2<GLdouble>),
    FMat2(glm::TMat2<GLfloat>),
    DMat3(glm::TMat3<GLdouble>),
    FMat3(glm::TMat3<GLfloat>),
    DMat4(glm::TMat4<GLdouble>),
    FMat4(glm::TMat4<GLfloat>),
}

impl Uniform {
    pub fn new(name: &str, kind: UniformType, transpose: bool) -> Self {
        Self { name: name.to_string(), kind, transpose }
    }

    pub fn new_1d(name: &str, value: GLdouble) -> Self {
        Self::new(name, UniformType::D(value), false)
    }

    pub fn new_1f(name: &str, value: GLfloat) -> Self {
        Self::new(name, UniformType::F(value), false)
    }

    pub fn new_1i(name: &str, value: GLint) -> Self {
        Self::new(name, UniformType::I(value), false)
    }

    pub fn new_2d(name: &str, value: glm::TVec2<GLdouble>) -> Self {
        Self::new(name, UniformType::D2(value), false)
    }

    pub fn new_2f(name: &str, value: glm::TVec2<GLfloat>) -> Self {
        Self::new(name, UniformType::F2(value), false)
    }

    pub fn new_2i(name: &str, value: glm::TVec2<GLint>) -> Self {
        Self::new(name, UniformType::I2(value), false)
    }

    pub fn new_3d(name: &str, value: glm::TVec3<GLdouble>) -> Self {
        Self::new(name, UniformType::D3(value), false)
    }

    pub fn new_3f(name: &str, value: glm::TVec3<GLfloat>) -> Self {
        Self::new(name, UniformType::F3(value), false)
    }

    pub fn new_3i(name: &str, value: glm::TVec3<GLint>) -> Self {
        Self::new(name, UniformType::I3(value), false)
    }

    pub fn new_4d(name: &str, value: glm::TVec4<GLdouble>) -> Self {
        Self::new(name, UniformType::D4(value), false)
    }

    pub fn new_4f(name: &str, value: glm::TVec4<GLfloat>) -> Self {
        Self::new(name, UniformType::F4(value), false)
    }

    pub fn new_4i(name: &str, value: glm::TVec4<GLint>) -> Self {
        Self::new(name, UniformType::I4(value), false)
    }

    pub fn new_mat2dv(name: &str, value: glm::TMat2<GLdouble>, transpose: bool) -> Self {
        Self::new(name, UniformType::DMat2(value), transpose)
    }

    pub fn new_mat2fv(name: &str, value: glm::TMat2<GLfloat>, transpose: bool) -> Self {
        Self::new(name, UniformType::FMat2(value), transpose)
    }

    pub fn new_mat3dv(name: &str, value: glm::TMat3<GLdouble>, transpose: bool) -> Self {
        Self::new(name, UniformType::DMat3(value), transpose)
    }

    pub fn new_mat3fv(name: &str, value: glm::TMat3<GLfloat>, transpose: bool) -> Self {
        Self::new(name, UniformType::FMat3(value), transpose)
    }

    pub fn new_mat4dv(name: &str, value: glm::TMat4<GLdouble>, transpose: bool) -> Self {
        Self::new(name, UniformType::DMat4(value), transpose)
    }

    pub fn new_mat4fv(name: &str, value: glm::TMat4<GLfloat>, transpose: bool) -> Self {
        Self::new(name, UniformType::FMat4(value), transpose)
    }

    pub fn default_1d(name: &str) -> Self {
        Self::new(name, UniformType::D(0.0), false)
    }

    pub fn default_1f(name: &str) -> Self {
        Self::new(name, UniformType::F(0.0), false)
    }

    pub fn default_1i(name: &str) -> Self {
        Self::new(name, UniformType::I(0), false)
    }

    pub fn default_2d(name: &str) -> Self {
        Self::new(name, UniformType::D2(glm::TVec2::<GLdouble>::default()), false)
    }

    pub fn default_2f(name: &str) -> Self {
        Self::new(name, UniformType::F2(glm::TVec2::<GLfloat>::default()), false)
    }

    pub fn default_2i(name: &str) -> Self {
        Self::new(name, UniformType::I2(glm::TVec2::<GLint>::default()), false)
    }

    pub fn default_3d(name: &str) -> Self {
        Self::new(name, UniformType::D3(glm::TVec3::<GLdouble>::default()), false)
    }

    pub fn default_3f(name: &str) -> Self {
        Self::new(name, UniformType::F3(glm::TVec3::<GLfloat>::default()), false)
    }

    pub fn default_3i(name: &str) -> Self {
        Self::new(name, UniformType::I3(glm::TVec3::<GLint>::default()), false)
    }

    pub fn default_4d(name: &str) -> Self {
        Self::new(name, UniformType::D4(glm::TVec4::<GLdouble>::default()), false)
    }

    pub fn default_4f(name: &str) -> Self {
        Self::new(name, UniformType::F4(glm::TVec4::<GLfloat>::default()), false)
    }

    pub fn default_4i(name: &str) -> Self {
        Self::new(name, UniformType::I4(glm::TVec4::<GLint>::default()), false)
    }

    pub fn default_mat2d(name: &str, transpose: bool) -> Self {
        Self::new(name, UniformType::DMat2(glm::TMat2::<GLdouble>::identity()), transpose)
    }

    pub fn default_mat2f(name: &str, transpose: bool) -> Self {
        Self::new(name, UniformType::FMat2(glm::TMat2::<GLfloat>::identity()), transpose)
    }

    pub fn default_mat3d(name: &str, transpose: bool) -> Self {
        Self::new(name, UniformType::DMat3(glm::TMat3::<GLdouble>::identity()), transpose)
    }

    pub fn default_mat3f(name: &str, transpose: bool) -> Self {
        Self::new(name, UniformType::FMat3(glm::TMat3::<GLfloat>::identity()), transpose)
    }

    pub fn default_mat4d(name: &str, transpose: bool) -> Self {
        Self::new(name, UniformType::DMat4(glm::TMat4::<GLdouble>::identity()), transpose)
    }

    pub fn default_mat4f(name: &str, transpose: bool) -> Self {
        Self::new(name, UniformType::FMat4(glm::TMat4::<GLfloat>::identity()), transpose)
    }
}
