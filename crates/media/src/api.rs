use super::*;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
  #[snafu(display("deserializing response from {url} failed"))]
  Deserialize {
    url: Url,
    source: ciborium::de::Error<io::Error>,
  },
  #[snafu(display("request to {url} failed"))]
  Request { url: Url, source: reqwest::Error },
  #[snafu(display("response from {url} failed with {status}"))]
  Status { url: Url, status: StatusCode },
}

impl From<Error> for JsValue {
  fn from(error: Error) -> Self {
    js_sys::Error::new(&error.to_string()).into()
  }
}

pub struct Api {
  base: Url,
}

impl Default for Api {
  fn default() -> Self {
    let location = web_sys::window().unwrap().location();
    let mut base = Url::parse(&location.href().unwrap()).unwrap();
    base.set_fragment(None);
    base.set_query(None);
    Self { base }
  }
}

impl Api {
  pub async fn packages(&self) -> Result<BTreeMap<Hash, Manifest>, Error> {
    self.get("api/packages").await
  }

  pub async fn handlers(&self) -> Result<BTreeMap<Target, Hash>, Error> {
    self.get("api/handlers").await
  }

  async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
    let url = self.base.join(path).unwrap();

    let response = reqwest::Client::new()
      .get(url.clone())
      .send()
      .await
      .with_context(|_| Request { url: url.clone() })?;

    let status = response.status();

    ensure!(
      status.is_success(),
      Status {
        status,
        url: url.clone()
      }
    );

    let body = response
      .bytes()
      .await
      .with_context(|_| Request { url: url.clone() })?;

    ciborium::from_reader(Cursor::new(body)).with_context(|_| Deserialize { url: url.clone() })
  }
}