use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("读入数据错误: {0}")]
    ErrorInp(String),

    #[error("未知错误: {0}")]
    ErrorUnk(String),
}