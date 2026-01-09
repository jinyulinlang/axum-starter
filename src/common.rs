use crate::serde::deserialize_number;
use serde::{Deserialize, Serialize};

const DEFAULT_PAGE_NO: u64 = 1;
const DEFAULT_PAGE_SIZE: u64 = 15;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BasePageDTO {
    #[serde(default = "default_page_no", deserialize_with = "deserialize_number")]
    pub page: u64,

    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub size: u64,
}

fn default_page_no() -> u64 {
    DEFAULT_PAGE_NO
}
fn default_page_size() -> u64 {
    DEFAULT_PAGE_SIZE
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PageInfoData<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub size: u64,
}

impl<T> PageInfoData<T> {
    pub fn new(list: Vec<T>, total: u64, page: u64, size: u64) -> Self {
        Self {
            list,
            total,
            page,
            size,
        }
    }
    pub fn from_pagination(pagination: BasePageDTO, total: u64, list: Vec<T>) -> Self {
        Self::new(list, total, pagination.page, pagination.size)
    }
}
