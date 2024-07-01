use super::*;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)), visibility(pub))]
pub enum Error {
  #[snafu(display("failed to get current directory"))]
  CurrentDir {
    backtrace: Backtrace,
    source: io::Error,
  },
  #[snafu(display("failed to deserialize manifest in package `{path}`"))]
  DeserializeManifest {
    backtrace: Backtrace,
    path: Utf8PathBuf,
    source: ciborium::de::Error<io::Error>,
  },
  #[snafu(display("failed to deserialize YAML package metadata at `{path}`"))]
  DeserializeMetadata {
    backtrace: Backtrace,
    path: Utf8PathBuf,
    source: serde_yaml::Error,
  },
  #[snafu(display("missing `index.html` in `{root}`"))]
  Index {
    backtrace: Backtrace,
    root: Utf8PathBuf,
  },
  #[snafu(display("invalid page filename `{path}`"))]
  InvalidPage {
    backtrace: Backtrace,
    path: Utf8PathBuf,
    source: ParseIntError,
  },
  #[snafu(display("I/O error at `{path}`"))]
  Io {
    backtrace: Backtrace,
    path: Utf8PathBuf,
    source: io::Error,
  },
  #[snafu(display("I/O error copying from `{from}` to `{to}"))]
  IoCopy {
    backtrace: Backtrace,
    from: Utf8PathBuf,
    source: io::Error,
    to: Utf8PathBuf,
  },
  #[snafu(display("missing `metadata.yaml` in `{root}`"))]
  MetadataMissing {
    backtrace: Backtrace,
    root: Utf8PathBuf,
  },
  #[snafu(display("muliple page {page}s"))]
  PageDuplicated { backtrace: Backtrace, page: u64 },
  #[snafu(display("page {page} missing"))]
  PageMissing { backtrace: Backtrace, page: u64 },
  #[snafu(
    display("path contains invalid UTF-8: `{}`", path.display())
  )]
  PathUnicode { backtrace: Backtrace, path: PathBuf },
  #[snafu(display("I/O error initializing async runtime"))]
  Runtime {
    backtrace: Backtrace,
    source: io::Error,
  },
  #[snafu(display("I/O error serving on {address}"))]
  Serve {
    address: SocketAddr,
    backtrace: Backtrace,
    source: io::Error,
  },
  #[snafu(display("unexpected file `{file}` in {ty} package"))]
  UnexpectedFile {
    backtrace: Backtrace,
    file: Utf8PathBuf,
    ty: Type,
  },
  #[snafu(display("failed to walk directory `{root}`"))]
  WalkDir {
    backtrace: Backtrace,
    root: Utf8PathBuf,
    source: walkdir::Error,
  },
}
