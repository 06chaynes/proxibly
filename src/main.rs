mod settings;
mod transparent;

use tide::{log, Result};

#[async_std::main]
async fn main() -> Result<()> {
    let settings = settings::Settings::new()?;
    let addr = format!("{}:{}", &settings.listen_address, &settings.listen_port);
    log::with_level(settings.filter_level()?);
    let mut app = tide::new();
    app.with(driftwood::ApacheCombinedLogger);
    app.with(transparent::Transparent::new(settings));
    app.listen(&addr).await?;
    Ok(())
}
