use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::{
        client::ClientInfo,
        ctx::ReqCtx,
        jwt::Claims,
        res::{ListData, PageParams},
    },
    db_conn,
    system::{
        entities::{prelude::SysOperLog, sys_oper_log},
        models::sys_oper_log::{OperationLogData, SysOperLogDeleteReq, SysOperLogSearchReq},
        prelude::SysOperLogModel,
    },
    DB,
};
use sea_orm::{
    sea_query::Table, ActiveValue::Set, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::system::check_user_online;

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SysOperLogSearchReq,
) -> Result<ListData<SysOperLogModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysOperLog::find();
    if let Some(x) = req.title {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::Title.eq(x));
        }
    }
    if let Some(x) = req.oper_name {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::OperName.contains(&x));
        }
    }

    if let Some(x) = req.operator_type {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::OperatorType.eq(x));
        }
    }

    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + " 00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_oper_log::Column::OperTime.gte(t));
        }
    }
    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + " 23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_oper_log::Column::OperTime.lte(t));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s
        .order_by_desc(sys_oper_log::Column::OperTime)
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;

    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };
    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: SysOperLogDeleteReq) -> Result<String> {
    let mut s = SysOperLog::delete_many();

    s = s.filter(sys_oper_log::Column::OperId.is_in(delete_req.oper_log_ids));

    // 开始删除
    let d = s.exec(db).await.map_err(|e| anyhow!(e.to_string()))?;

    match d.rows_affected {
        0 => Err(anyhow!("你要删除的日志不存在".to_string(),)),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// delete 完全删除
pub async fn clean(db: &DatabaseConnection) -> Result<String> {
    let stmt = Table::truncate().table(sys_oper_log::Entity).to_owned();
    let db_backend = db.get_database_backend();
    db.execute(db_backend.build(&stmt)).await?;
    Ok("日志清空成功".to_string())
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, oper_id: String) -> Result<SysOperLogModel> {
    let s = SysOperLog::find()
        .filter(sys_oper_log::Column::OperId.eq(oper_id))
        .one(db)
        .await?;

    let res = match s {
        Some(m) => m,
        None => return Err(anyhow!("没有找到数据")),
    };
    Ok(res)
}

/// 添加日志到数据库
pub async fn add_to_db(
    user: &Claims,
    req_ctx: &ReqCtx,
    uc: &ClientInfo,
    log: OperationLogData,
    api_name: String,
    duration: i64,
) -> Result<String> {
    // 获取用户
    let db = DB.get_or_init(db_conn).await;
    // 获取用户的登录相关信息
    let (_, v) = check_user_online(Some(db), user.token_id.clone()).await;
    let uo = match v {
        None => return Err(anyhow!("用户无登录信息")),
        Some(x) => x,
    };

    let operator_type = match log.req_method.as_str() {
        "GET" => "1",    // 查询
        "POST" => "2",   // 新增
        "PUT" => "3",    // 修改
        "DELETE" => "4", // 删除
        _ => "0",        // 其他
    };

    // 设置操作信息
    let active_model = sys_oper_log::ActiveModel {
        oper_id: Set(scru128::new_string()),
        time_id: Set(log.time_id),
        title: Set(api_name),
        business_type: Set("".to_string()),
        method: Set(req_ctx.path.clone()),
        request_method: Set(log.req_method),
        operator_type: Set(operator_type.to_string()),
        oper_name: Set(user.name.clone()),
        dept_name: Set(uo.dept_name),
        oper_url: Set(log.req_url),
        oper_ip: Set(uc.net.ip.clone()),
        oper_location: Set(uc.net.location.clone()),
        oper_param: Set(if log.req_data.len() > 10000 {
            "数据太长不保存".to_string()
        } else {
            log.req_data
        }),
        path_param: Set(req_ctx.path_params.clone()),
        json_result: Set(if log.res_body.len() > 65535 {
            "数据太长不保存".to_string()
        } else {
            log.res_body
        }),
        status: Set("1".to_string()),
        error_msg: Set("".to_string()),
        duration: Set(duration),
        oper_time: Set(Local::now().naive_local()),
    };
    SysOperLog::insert(active_model)
        .exec(db)
        .await
        .expect("oper_log_add error");
    Ok("日志数据库添加成功".to_string())
}
