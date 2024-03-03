use anyhow::Result;
use db::common::jwt::Claims;
use salvo_core::Depot;
pub fn get_current_user(depot: &mut Depot) -> Result<&Claims> {
    match depot.get::<Claims>("Claims") {
        Ok(v) => Ok(v),
        Err(_) => Err(anyhow::anyhow!("不存在登录用户")),
    }
}
