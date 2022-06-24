# \DefaultApi

All URIs are relative to *https://nethsmdemo.nitrokey.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**config_backup_passphrase_put**](DefaultApi.md#config_backup_passphrase_put) | **PUT** /config/backup-passphrase | 
[**config_logging_get**](DefaultApi.md#config_logging_get) | **GET** /config/logging | 
[**config_logging_put**](DefaultApi.md#config_logging_put) | **PUT** /config/logging | 
[**config_network_get**](DefaultApi.md#config_network_get) | **GET** /config/network | 
[**config_network_put**](DefaultApi.md#config_network_put) | **PUT** /config/network | 
[**config_time_get**](DefaultApi.md#config_time_get) | **GET** /config/time | 
[**config_time_put**](DefaultApi.md#config_time_put) | **PUT** /config/time | 
[**config_tls_cert_pem_get**](DefaultApi.md#config_tls_cert_pem_get) | **GET** /config/tls/cert.pem | 
[**config_tls_cert_pem_put**](DefaultApi.md#config_tls_cert_pem_put) | **PUT** /config/tls/cert.pem | 
[**config_tls_csr_pem_post**](DefaultApi.md#config_tls_csr_pem_post) | **POST** /config/tls/csr.pem | 
[**config_tls_public_pem_get**](DefaultApi.md#config_tls_public_pem_get) | **GET** /config/tls/public.pem | 
[**config_unattended_boot_get**](DefaultApi.md#config_unattended_boot_get) | **GET** /config/unattended-boot | 
[**config_unattended_boot_put**](DefaultApi.md#config_unattended_boot_put) | **PUT** /config/unattended-boot | 
[**config_unlock_passphrase_put**](DefaultApi.md#config_unlock_passphrase_put) | **PUT** /config/unlock-passphrase | 
[**health_alive_get**](DefaultApi.md#health_alive_get) | **GET** /health/alive | 
[**health_ready_get**](DefaultApi.md#health_ready_get) | **GET** /health/ready | 
[**health_state_get**](DefaultApi.md#health_state_get) | **GET** /health/state | 
[**info_get**](DefaultApi.md#info_get) | **GET** /info | 
[**keys_generate_post**](DefaultApi.md#keys_generate_post) | **POST** /keys/generate | 
[**keys_get**](DefaultApi.md#keys_get) | **GET** /keys | 
[**keys_key_id_cert_delete**](DefaultApi.md#keys_key_id_cert_delete) | **DELETE** /keys/{KeyID}/cert | 
[**keys_key_id_cert_get**](DefaultApi.md#keys_key_id_cert_get) | **GET** /keys/{KeyID}/cert | 
[**keys_key_id_cert_put**](DefaultApi.md#keys_key_id_cert_put) | **PUT** /keys/{KeyID}/cert | 
[**keys_key_id_csr_pem_post**](DefaultApi.md#keys_key_id_csr_pem_post) | **POST** /keys/{KeyID}/csr.pem | 
[**keys_key_id_decrypt_post**](DefaultApi.md#keys_key_id_decrypt_post) | **POST** /keys/{KeyID}/decrypt | 
[**keys_key_id_delete**](DefaultApi.md#keys_key_id_delete) | **DELETE** /keys/{KeyID} | 
[**keys_key_id_get**](DefaultApi.md#keys_key_id_get) | **GET** /keys/{KeyID} | 
[**keys_key_id_public_pem_get**](DefaultApi.md#keys_key_id_public_pem_get) | **GET** /keys/{KeyID}/public.pem | 
[**keys_key_id_put**](DefaultApi.md#keys_key_id_put) | **PUT** /keys/{KeyID} | 
[**keys_key_id_sign_post**](DefaultApi.md#keys_key_id_sign_post) | **POST** /keys/{KeyID}/sign | 
[**keys_post**](DefaultApi.md#keys_post) | **POST** /keys | 
[**lock_post**](DefaultApi.md#lock_post) | **POST** /lock | 
[**metrics_get**](DefaultApi.md#metrics_get) | **GET** /metrics | 
[**provision_post**](DefaultApi.md#provision_post) | **POST** /provision | 
[**random_post**](DefaultApi.md#random_post) | **POST** /random | 
[**system_backup_post**](DefaultApi.md#system_backup_post) | **POST** /system/backup | 
[**system_cancel_update_post**](DefaultApi.md#system_cancel_update_post) | **POST** /system/cancel-update | 
[**system_commit_update_post**](DefaultApi.md#system_commit_update_post) | **POST** /system/commit-update | 
[**system_info_get**](DefaultApi.md#system_info_get) | **GET** /system/info | 
[**system_reboot_post**](DefaultApi.md#system_reboot_post) | **POST** /system/reboot | 
[**system_reset_post**](DefaultApi.md#system_reset_post) | **POST** /system/reset | 
[**system_restore_post**](DefaultApi.md#system_restore_post) | **POST** /system/restore | 
[**system_shutdown_post**](DefaultApi.md#system_shutdown_post) | **POST** /system/shutdown | 
[**system_update_post**](DefaultApi.md#system_update_post) | **POST** /system/update | 
[**unlock_post**](DefaultApi.md#unlock_post) | **POST** /unlock | 
[**users_get**](DefaultApi.md#users_get) | **GET** /users | 
[**users_post**](DefaultApi.md#users_post) | **POST** /users | 
[**users_user_id_delete**](DefaultApi.md#users_user_id_delete) | **DELETE** /users/{UserID} | 
[**users_user_id_get**](DefaultApi.md#users_user_id_get) | **GET** /users/{UserID} | 
[**users_user_id_passphrase_post**](DefaultApi.md#users_user_id_passphrase_post) | **POST** /users/{UserID}/passphrase | 
[**users_user_id_put**](DefaultApi.md#users_user_id_put) | **PUT** /users/{UserID} | 



## config_backup_passphrase_put

> config_backup_passphrase_put(body)


Update the backup passphrase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**BackupPassphraseConfig**](BackupPassphraseConfig.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_logging_get

> crate::models::LoggingConfig config_logging_get()


Get logging configuration. Protocol is always syslog over UDP. Configurable are IP adress and port, log level.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoggingConfig**](LoggingConfig.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_logging_put

> config_logging_put(body)


Configure log level and destination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**LoggingConfig**](LoggingConfig.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_network_get

> crate::models::NetworkConfig config_network_get()


Get network configuration. IP address, netmask, router.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::NetworkConfig**](NetworkConfig.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_network_put

> config_network_put(body)


Configure network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**NetworkConfig**](NetworkConfig.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_time_get

> crate::models::TimeConfig config_time_get()


Get system time.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TimeConfig**](TimeConfig.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_time_put

> config_time_put(body)


Configure system time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**TimeConfig**](TimeConfig.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_tls_cert_pem_get

> String config_tls_cert_pem_get()


Get certificate for NetHSMs https API.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_tls_cert_pem_put

> config_tls_cert_pem_put(body)


Set certificate for NetHSMs https API e.g. to replace self-signed intital certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_tls_csr_pem_post

> String config_tls_csr_pem_post(body)


Get NetHSM certificate signing request e.g. to replace self-signed intital certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**DistinguishedName**](DistinguishedName.md)> |  |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_tls_public_pem_get

> String config_tls_public_pem_get()


Get public key for NetHSMs https API.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_unattended_boot_get

> crate::models::UnattendedBootConfig config_unattended_boot_get()


Read unattended boot configuration: is it on or off?

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UnattendedBootConfig**](UnattendedBootConfig.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_unattended_boot_put

> config_unattended_boot_put(body)


Configure unattended boot: switch it on or off (flip the switch).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**UnattendedBootConfig**](UnattendedBootConfig.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_unlock_passphrase_put

> config_unlock_passphrase_put(body)


Update the unlock passphrase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**UnlockPassphraseConfig**](UnlockPassphraseConfig.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_alive_get

> health_alive_get()


Retrieve wether NetHSM is alive (powered up). This corresponds to the state <i>Locked</i> or <i>Unprovisioned</i>.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_ready_get

> health_ready_get()


Retrieve wether NetHSM is alive and ready to take traffic. This corresponds to the state <i>Operational</i>.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_state_get

> crate::models::HealthStateData health_state_get()


Retrieve the state of NetHSM.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthStateData**](HealthStateData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info_get

> crate::models::InfoData info_get()


Information about the vendor and product.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InfoData**](InfoData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_generate_post

> keys_generate_post(body)


Generate a pair of public and private key and store it in NetHSM. KeyID is optional as a parameter and will be generated by NetHSM if not present.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**KeyGenerateRequestData**](KeyGenerateRequestData.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_get

> Vec<crate::models::KeyItem> keys_get()


Get a list of the identifiers of all keys that are currently stored in NetHSM. Separate requests need to be made to request the individual key data.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::KeyItem>**](KeyItem.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_key_id_cert_delete

> keys_key_id_cert_delete(key_id)


Delete the certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_key_id_cert_get

> String keys_key_id_cert_get(key_id)


Retrieve stored certificate. The content-type header will display the media type of the stored data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_key_id_cert_put

> keys_key_id_cert_put(key_id, body)


Store a certificate. Maximum size 1MB. The content-type header provides the media type. Only application/json, application/x-pem-file, application/x-x509-ca-cert, application/octet-stream, text/plain and application/pgp-keys is allowed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_key_id_csr_pem_post

> String keys_key_id_csr_pem_post(key_id, body)


Retrieve a certificate signing request in PEM format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |
**body** | Option<[**DistinguishedName**](DistinguishedName.md)> |  |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_key_id_decrypt_post

> crate::models::DecryptData keys_key_id_decrypt_post(key_id, body)


Decrypt an encrypted message with the secret key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |
**body** | Option<[**DecryptRequestData**](DecryptRequestData.md)> |  |  |

### Return type

[**crate::models::DecryptData**](DecryptData.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_key_id_delete

> keys_key_id_delete(key_id)


Delete a pair of public and private key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_key_id_get

> crate::models::PublicKey keys_key_id_get(key_id)


Retrieve the public key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |

### Return type

[**crate::models::PublicKey**](PublicKey.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_key_id_public_pem_get

> String keys_key_id_public_pem_get(key_id)


Retrieve public key in PEM format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_key_id_put

> keys_key_id_put(key_id, body)


Import a private key into NetHSM and store it under the <i>KeyID</i> path. The public key will be automatically derived.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |
**body** | Option<[**PrivateKey**](PrivateKey.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_key_id_sign_post

> crate::models::SignData keys_key_id_sign_post(key_id, body)


Sign a message with the secret key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |
**body** | Option<[**SignRequestData**](SignRequestData.md)> |  |  |

### Return type

[**crate::models::SignData**](SignData.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keys_post

> crate::models::KeyItem keys_post(mechanisms, body)


Import a private key into NetHSM and let NetHSM generate a KeyID. The public key will be automatically derived.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mechanisms** | Option<[**Vec<crate::models::KeyMechanism>**](crate::models::KeyMechanism.md)> |  |  |
**body** | Option<[**PrivateKey**](PrivateKey.md)> |  |  |

### Return type

[**crate::models::KeyItem**](KeyItem.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_post

> lock_post()


Brings an <i>Operational</i> NetHSM into <i>Locked</i> state.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metrics_get

> serde_json::Value metrics_get()


Get metrics. Precondition: NetHSM is <i>Operational</i> and a <b>R-Metrics</b> can be authenticated.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## provision_post

> provision_post(body)


Initial provisioning, only available in <i>Unprovisioned</i> state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ProvisionRequestData**](ProvisionRequestData.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## random_post

> crate::models::RandomData random_post(body)


Retrieve cryptographically strong random bytes from NetHSM. Precondition: NetHSM is <i>Operational</i> and a <b>R-Operator</b> can be authenticated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**RandomRequestData**](RandomRequestData.md)> |  |  |

### Return type

[**crate::models::RandomData**](RandomData.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_backup_post

> system_backup_post()


Back up the key store to a backup file.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_cancel_update_post

> system_cancel_update_post()


Cancel update of NetHSM software.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_commit_update_post

> system_commit_update_post()


Commit update of NetHSM software.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_info_get

> crate::models::SystemInfo system_info_get()


Get detailed system information, including firmware version, system software version, hardware version.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SystemInfo**](SystemInfo.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_reboot_post

> system_reboot_post()


Reboot NetHSM.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_reset_post

> system_reset_post()


Reset NetHSM to factory settings.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_restore_post

> system_restore_post(backup_passphrase, system_time, body)


Restore the key store from a backup file. Only available in <i>Unprovisioned</i> state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backup_passphrase** | **String** |  | [required] |
**system_time** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_shutdown_post

> system_shutdown_post()


Shut down NetHSM.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_update_post

> crate::models::SystemUpdateData system_update_post(body)


Update NetHSM software.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::SystemUpdateData**](SystemUpdateData.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_post

> unlock_post(body)


Brings a <i>Locked</i> NetHSM into <i>Operational</i> state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**UnlockRequestData**](UnlockRequestData.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get

> Vec<crate::models::UserItem> users_get()


Get a list of all user ids that have accounts on NetHSM.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::UserItem>**](UserItem.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_post

> users_post(body)


Create a new user on NetHSM. The user-ID is generated by NetHSM.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**UserPostData**](UserPostData.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_delete

> users_user_id_delete(user_id)


Delete a user from keyfender.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_get

> crate::models::UserData users_user_id_get(user_id)


Get user info: name and role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::UserData**](UserData.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_passphrase_post

> users_user_id_passphrase_post(user_id, body)


Update the passphrase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**body** | Option<[**UserPassphrasePostData**](UserPassphrasePostData.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_put

> users_user_id_put(user_id, body)


Create a user on keyfender.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**body** | Option<[**UserPostData**](UserPostData.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

