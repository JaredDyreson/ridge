use thiserror::Error;

#[derive(Debug, Error)]
pub enum BoundError {
    #[error("out of bounds, got index `({0}, {1})`")]
    Exceed(usize, usize),
}
