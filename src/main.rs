#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    blog_cms::run_app().await
}
