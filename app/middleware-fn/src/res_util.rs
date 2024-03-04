use std::fmt::Debug;

use salvo::prelude::*;
use salvo_core::{Depot, Writer};
use serde::Serialize;

/// 数据统一返回格式
#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: Option<i32>,
    pub data: Option<T>,
    pub msg: Option<String>,
}

#[async_trait]
impl<T> Writer for Res<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        let json_string = serde_json::to_string(&self).unwrap_or_default();
        depot.insert("res_v", json_string);
        res.render(Json(&self));
    }
}

impl<T: Serialize> Res<T> {
    pub fn with_data(data: T) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some("success".to_string()),
        }
    }
    pub fn with_err(err: &str) -> Self {
        Self {
            code: Some(500),
            data: None,
            msg: Some(err.to_string()),
        }
    }
    pub fn with_msg(msg: &str) -> Self {
        Self {
            code: Some(200),
            data: None,
            msg: Some(msg.to_string()),
        }
    }
}
