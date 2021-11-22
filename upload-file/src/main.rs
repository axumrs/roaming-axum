use axum::{
    extract::{ContentLengthLimit, Multipart},
    http::HeaderMap,
    response::Html,
    routing, Router,
};

/// 允许上传的大小
const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 10; // 10MB

/// 上传表单
async fn upload_file() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
            <meta charset="utf-8">
                <title>上传文件</title>
            </head>
            <body>
                <form action="/upload" method="post" enctype="multipart/form-data">
                    <label>
                        上传文件：
                        <input type="file" name="axum_rs_file">
                    </label>
                    <button type="submit">上传文件</button>
                </form>
            </body>
        </html>
        "#,
    )
}

/// 上传操作
async fn upload_file_action(
    ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, { MAX_UPLOAD_SIZE }>,
) -> Result<(HeaderMap, String), String> {
    if let Some(file) = multipart.next_field().await.unwrap() {
        let filename = file.file_name().unwrap().to_string(); // 上传的文件名
        let data = file.bytes().await.unwrap(); // 上传的文件的内容

        // 保存上传的文件
        std::fs::write(&filename, &data).map_err(|err| err.to_string())?;

        return cn(format!(
            "【上传的文件】文件名：{:?}, 文件大小：{}",
            filename,
            data.len()
        ))
        .await;
    }
    cn(String::from("没有上传文件")).await
}

/// 中文响应
async fn cn(msg: String) -> Result<(HeaderMap, String), String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/plain;charset=utf-8".parse().unwrap(),
    );
    Ok((headers, msg))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/upload",
        routing::get(upload_file).post(upload_file_action),
    );

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
