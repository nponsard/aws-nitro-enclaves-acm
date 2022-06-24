/*
 * NetHSM
 *
 * All endpoints expect exactly the specified JSON. Additional properties will cause a Bad Request Error (400). All HTTP errors contain a JSON structure with an explanation of type string. All <a href=\"https://tools.ietf.org/html/rfc4648#section-4\">base64</a> encoded values are Big Endian.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignMode {
    #[serde(rename = "PKCS1")]
    PKCS1,
    #[serde(rename = "PSS_MD5")]
    PSSMD5,
    #[serde(rename = "PSS_SHA1")]
    PSSSHA1,
    #[serde(rename = "PSS_SHA224")]
    PSSSHA224,
    #[serde(rename = "PSS_SHA256")]
    PSSSHA256,
    #[serde(rename = "PSS_SHA384")]
    PSSSHA384,
    #[serde(rename = "PSS_SHA512")]
    PSSSHA512,
    #[serde(rename = "EdDSA")]
    EdDSA,
    #[serde(rename = "ECDSA")]
    ECDSA,

}

impl ToString for SignMode {
    fn to_string(&self) -> String {
        match self {
            Self::PKCS1 => String::from("PKCS1"),
            Self::PSSMD5 => String::from("PSS_MD5"),
            Self::PSSSHA1 => String::from("PSS_SHA1"),
            Self::PSSSHA224 => String::from("PSS_SHA224"),
            Self::PSSSHA256 => String::from("PSS_SHA256"),
            Self::PSSSHA384 => String::from("PSS_SHA384"),
            Self::PSSSHA512 => String::from("PSS_SHA512"),
            Self::EdDSA => String::from("EdDSA"),
            Self::ECDSA => String::from("ECDSA"),
        }
    }
}

impl Default for SignMode {
    fn default() -> SignMode {
        Self::PKCS1
    }
}




