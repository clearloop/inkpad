#[actix_web::main]
async fn main() {
    ceres_proxy::run().await.unwrap();
}
