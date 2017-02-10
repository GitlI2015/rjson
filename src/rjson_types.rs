

#[derive(Debug)]
pub enum rjson_type {
    RJ_NULL,
    RJ_FALSE,
    RJ_TRUE,
    RJ_NUMBER,
    RJ_STRING,
    RJ_ARRAY,
    RJ_OBJECT,
}

#[derive(Debug)]
pub struct rjson_value {
    pub rj_type: rjson_type,
}
