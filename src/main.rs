mod settings;
mod transparent;

use tide::Result;

#[async_std::main]
async fn main() -> Result<()> {
    let settings = settings::Settings::new()?;
    let addr = format!("{}:{}", &settings.listen_address, &settings.listen_port);
    let mut app = tide::new();
    app.with(driftwood::DevLogger);
    app.with(transparent::Transparent::new(settings));
    app.listen(&addr).await?;
    Ok(())
}
