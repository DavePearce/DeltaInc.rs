mod traits;
mod region;
pub mod seq;
pub mod vec;
pub mod lex;
pub mod linear;

pub use traits::{Transformable,PartiallyTransformable};
pub use traits::{Transformer,PartialTransformer};
pub use traits::{Diffable, Incremental};
