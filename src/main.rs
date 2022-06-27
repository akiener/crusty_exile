pub mod api;

#[tokio::main]
async fn main() {
    let skill_gem = api::ninja::SkillGem::fetch_or_read_cache().await;
    let a = 1;
    println!("a: {}", a);
    println!("Hello, world!");
}
