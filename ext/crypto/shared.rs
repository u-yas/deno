use std::borrow::Cow;

use deno_core::error::custom_error;
use deno_core::error::AnyError;
use deno_core::ZeroCopyBuf;
use serde::Deserialize;
use serde::Serialize;

pub const RSA_ENCRYPTION_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("1.2.840.113549.1.1.1");
pub const SHA1_RSA_ENCRYPTION_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("1.2.840.113549.1.1.5");
pub const SHA256_RSA_ENCRYPTION_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("1.2.840.113549.1.1.11");
pub const SHA384_RSA_ENCRYPTION_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("1.2.840.113549.1.1.12");
pub const SHA512_RSA_ENCRYPTION_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("1.2.840.113549.1.1.13");
pub const RSASSA_PSS_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("1.2.840.113549.1.1.10");
pub const ID_SHA1_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("1.3.14.3.2.26");
pub const ID_SHA256_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("2.16.840.1.101.3.4.2.1");
pub const ID_SHA384_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("2.16.840.1.101.3.4.2.2");
pub const ID_SHA512_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("2.16.840.1.101.3.4.2.3");
pub const ID_MFG1: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("1.2.840.113549.1.1.8");
pub const RSAES_OAEP_OID: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("1.2.840.113549.1.1.7");
pub const ID_P_SPECIFIED: rsa::pkcs8::ObjectIdentifier =
  rsa::pkcs8::ObjectIdentifier::new("1.2.840.113549.1.1.9");

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum ShaHash {
  #[serde(rename = "SHA-1")]
  Sha1,
  #[serde(rename = "SHA-256")]
  Sha256,
  #[serde(rename = "SHA-384")]
  Sha384,
  #[serde(rename = "SHA-512")]
  Sha512,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum EcNamedCurve {
  #[serde(rename = "P-256")]
  P256,
  #[serde(rename = "P-384")]
  P384,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "data")]
pub enum RawKeyData {
  Secret(ZeroCopyBuf),
  Private(ZeroCopyBuf),
  Public(ZeroCopyBuf),
}

pub fn data_error(msg: impl Into<Cow<'static, str>>) -> AnyError {
  custom_error("DOMExceptionDataError", msg)
}

pub fn not_supported_error(msg: impl Into<Cow<'static, str>>) -> AnyError {
  custom_error("DOMExceptionNotSupportedError", msg)
}

pub fn unsupported_format() -> AnyError {
  not_supported_error("unsupported format")
}