use axum::{extract::Path, routing, Router};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper, Debug)]
#[pg_mapper(table = "account")]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub balance: i32,
}
pub struct CreateAccount {
    pub username: String,
    pub balance: i32,
}

/// 数据库配置
fn get_cfg() -> deadpool_postgres::Config {
    let mut cfg = deadpool_postgres::Config::new();
    cfg.user = Some("axum_rs".to_string()); //数据库用户名
    cfg.password = Some("axum.rs".to_string()); //数据库密码
    cfg.dbname = Some("axum_rs".to_string()); //数据库名称
    cfg.host = Some("pg.axum.rs".to_string()); // 数据库主机
    cfg.port = Some(5432); //数据库端口
    cfg
}
/// 从连接池中获取数据库连接
async fn get_client() -> Result<deadpool_postgres::Client, String> {
    // 通过配置文件创建连接池
    let pool = get_cfg()
        .create_pool(tokio_postgres::NoTls)
        .map_err(|err| err.to_string())?;
    // 从连接池中获取数据库连接
    pool.get().await.map_err(|err| err.to_string())
}
/// 插入数据
async fn insert(Path(username): Path<String>) -> Result<&'static str, String> {
    let create_user = CreateAccount {
        username,
        balance: 0,
    };
    let client = get_client().await?;
    let stmt = client
        .prepare("INSERT INTO account (username, balance) VALUES ($1, $2)")
        .await
        .map_err(|err| err.to_string())?;
    let rows = client
        .execute(&stmt, &[&create_user.username, &create_user.balance])
        .await
        .map_err(|err| err.to_string())?;
    if rows < 1 {
        return Err("Insert account failed".to_string());
    }
    Ok("Successfully insert account")
}

/// 修改账户余额
async fn update(Path((id, balance)): Path<(i32, i32)>) -> Result<&'static str, String> {
    let client = get_client().await?;
    let stmt = client
        .prepare("UPDATE account SET balance=$1 WHERE id=$2")
        .await
        .map_err(|err| err.to_string())?;
    let rows = client
        .execute(&stmt, &[&balance, &id])
        .await
        .map_err(|err| err.to_string())?;
    if rows < 1 {
        return Err("Update account failed".to_string());
    }
    Ok("Successfully update account")
}

/// 删除账户
async fn delete(Path(id): Path<i32>) -> Result<&'static str, String> {
    let client = get_client().await?;
    let stmt = client
        .prepare("DELETE FROM account WHERE id=$1")
        .await
        .map_err(|err| err.to_string())?;
    let rows = client
        .execute(&stmt, &[&id])
        .await
        .map_err(|err| err.to_string())?;
    if rows < 1 {
        return Err("Delete account failed".to_string());
    }
    Ok("Successfully delete account")
}

/// 所有账户
async fn list() -> Result<String, String> {
    let client = get_client().await?;
    let stmt = client
        .prepare("SELECT id,username,balance FROM account ORDER BY id DESC")
        .await
        .map_err(|err| err.to_string())?;
    let account_list = client
        .query(&stmt, &[])
        .await
        .map_err(|err| err.to_string())?
        .iter()
        .map(|row| Account::from_row_ref(&row).unwrap())
        .collect::<Vec<Account>>();
    let mut output = Vec::with_capacity(account_list.len());
    for account in account_list.iter() {
        output.push(format!("{:?}", account));
    }
    Ok(output.join("\n"))
}
async fn find(Path(id): Path<i32>) -> Result<String, String> {
    let client = get_client().await?;
    let stmt = client
        .prepare("SELECT id,username,balance FROM account WHERE id=$1 ORDER BY id DESC LIMIT 1")
        .await
        .map_err(|err| err.to_string())?;
    let account = client
        .query(&stmt, &[&id])
        .await
        .map_err(|err| err.to_string())?
        .iter()
        .map(|row| Account::from_row_ref(&row).unwrap())
        .collect::<Vec<Account>>()
        .pop()
        .ok_or(format!("Couldn't find account #{}", id))?;
    Ok(format!("{:?}", account))
}
/// 使用事务在账户之间进行转账
async fn transfer(
    Path((from_id, to_id, balance)): Path<(i32, i32, i32)>,
) -> Result<&'static str, String> {
    let mut client = get_client().await?;
    let tx = client.transaction().await.map_err(|err| err.to_string())?;

    // 修改出账记录
    let stmt = tx
        .prepare("UPDATE account SET balance=balance-$1 WHERE id=$2 AND balance>=$1")
        .await
        .map_err(|err| err.to_string())?;
    match tx.execute(&stmt, &[&balance, &from_id]).await {
        Ok(_rows) if _rows > 0 => {
            // 检查受影响的行数
            // 如果大于零表示账户存在并且余额足够
            // 不必做其余操作，等待最终的事务提交
        }
        _ => {
            // 回滚事务
            tx.rollback().await.map_err(|err| err.to_string())?;
            // 提前结束函数，将错误信息返回
            return Err("Step 1 failed".to_string());
        }
    };

    // 修改入账记录
    let stmt = tx
        .prepare("UPDATE account SET balance=balance+$1 WHERE id=$2")
        .await
        .map_err(|err| err.to_string())?;
    match tx.execute(&stmt, &[&balance, &to_id]).await {
        Ok(_rows) if _rows > 0 => {
            // 检查受影响的行数
            // 如果大于零表示入账记录修改成功
            // 不必做其余操作，等待最终的事务提交
        }
        _ => {
            // 回滚事务
            tx.rollback().await.map_err(|err| err.to_string())?;
            // 提前结束函数，将错误信息返回
            return Err("Step 2 failed".to_string());
        }
    };

    // 提交事务
    tx.commit().await.map_err(|err| err.to_string())?;
    Ok("Successfully transfer")
}
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(list))
        .route("/find/:id", routing::get(find))
        .route("/insert/:username", routing::get(insert))
        .route("/update/:id/:balance", routing::get(update))
        .route("/delete/:id", routing::get(delete))
        .route("/transfer/:from_id/:to_id/:balance", routing::get(transfer));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
