use bcrypt;

use crate::app::ApiResult;

pub fn encode_password<T: AsRef<str>>(password: T) -> ApiResult<String> {
    Ok(bcrypt::hash(password.as_ref(), bcrypt::DEFAULT_COST)?)
}

pub fn verify_password<T: AsRef<str>>(password: T, hash: &str) -> ApiResult<bool> {
    let ret = bcrypt::verify(password.as_ref(), hash)?;
    Ok(ret)
}
