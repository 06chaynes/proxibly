mod forward;
mod settings;

use settings::{Listener, Mode};

use tide::{log, Result};
use tide_rustls::TlsListener;

#[async_std::main]
async fn main() -> Result<()> {
    let settings = settings::Settings::new()?;
    let addr = format!("{}:{}", &settings.listen_address, &settings.listen_port);
    log::with_level(settings.filter_level()?);
    let mut app = tide::new();
    app.with(driftwood::ApacheCombinedLogger);
    match &settings.mode()? {
        Mode::Forward => {
            app.with(forward::Forward::new(&settings));
        }
        Mode::Reverse => {
            unimplemented!()
        }
    }
    match &settings.listener()? {
        Listener::Http => {
            app.listen(&addr).await?;
        }
        Listener::Https => {
            app.listen(
                TlsListener::build()
                    .addrs(&addr)
                    .cert(&settings.https.cert_path)
                    .key(&settings.https.key_path),
            )
            .await?;
        }
        Listener::Acme => {
            unimplemented!()
        }
    }
    Ok(())
}
