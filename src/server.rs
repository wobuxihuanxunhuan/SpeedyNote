use warp::Filter;
use std::net::SocketAddr;
use serde_json::json;

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    
    // 健康检查端点
    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&json!({
                "status": "healthy",
                "service": "SpeedyNote",
                "version": "0.1.0"
            }))
        });
    
    // API端点
    let api_route = warp::path("api")
        .and(warp::path("notes"))
        .and(warp::get())
        .map(|| {
            warp::reply::json(&json!({
                "notes": [],
                "message": "API端点准备就绪"
            }))
        });
    
    // 静态文件服务（用于Web界面）
    let static_files = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("./dist/index.html"));
    
    let static_assets = warp::path("static")
        .and(warp::fs::dir("./dist"));
    
    // 组合所有路由
    let routes = health_route
        .or(api_route)
        .or(static_files)
        .or(static_assets)
        .with(warp::cors().allow_any_origin());
    
    println!("🚀 SpeedyNote HTTP服务器启动在端口 {}", port);
    println!("🌐 访问地址: http://localhost:{}", port);
    println!("🔍 健康检查: http://localhost:{}/health", port);
    
    warp::serve(routes)
        .run(addr)
        .await;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_health_endpoint() {
        let health_route = warp::path("health")
            .and(warp::get())
            .map(|| {
                warp::reply::json(&json!({
                    "status": "healthy",
                    "service": "SpeedyNote",
                    "version": "0.1.0"
                }))
            });
        
        let resp = warp::test::request()
            .method("GET")
            .path("/health")
            .reply(&health_route)
            .await;
        
        assert_eq!(resp.status(), 200);
        assert!(resp.body().contains("healthy"));
    }
}