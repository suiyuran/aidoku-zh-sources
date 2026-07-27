use super::*;
use net::Charset;

pub fn change_charset() -> Result<()> {
	let charset = Charset::from_settings()?;
	Url::ChangeCharset(charset).request()?.send()?;

	Ok(())
}
