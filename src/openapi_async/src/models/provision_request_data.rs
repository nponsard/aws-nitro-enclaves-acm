/*
 * NetHSM
 *
 * All endpoints expect exactly the specified JSON. Additional properties will cause a Bad Request Error (400). All HTTP errors contain a JSON structure with an explanation of type string. All <a href=\"https://tools.ietf.org/html/rfc4648#section-4\">base64</a> encoded values are Big Endian.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProvisionRequestData {
    #[serde(rename = "unlockPassphrase")]
    pub unlock_passphrase: String,
    #[serde(rename = "adminPassphrase")]
    pub admin_passphrase: String,
    #[serde(rename = "systemTime")]
    pub system_time: String,
}

impl ProvisionRequestData {
    pub fn new(unlock_passphrase: String, admin_passphrase: String, system_time: String) -> ProvisionRequestData {
        ProvisionRequestData {
            unlock_passphrase,
            admin_passphrase,
            system_time,
        }
    }
}


