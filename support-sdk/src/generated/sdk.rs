// The contents of this file are generated; do not modify them.

#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{ClientHooks, OperationInfo, RequestBuilderExt, encode_path};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}

        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    /// `AccessGroupForApiPermissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "name",
    ///    "permissions",
    ///    "updated_at"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/Permissions_for_ApiPermissions"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AccessGroupForApiPermissions {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForAccessGroupId,
        pub name: ::std::string::String,
        pub permissions: PermissionsForApiPermissions,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl AccessGroupForApiPermissions {
        pub fn builder() -> builder::AccessGroupForApiPermissions {
            Default::default()
        }
    }

    /// `AccessGroupId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum AccessGroupId {}

    /// `AccessGroupUpdateParamsForApiPermissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "permissions"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/Permissions_for_ApiPermissions"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AccessGroupUpdateParamsForApiPermissions {
        pub name: ::std::string::String,
        pub permissions: PermissionsForApiPermissions,
    }

    impl AccessGroupUpdateParamsForApiPermissions {
        pub fn builder() -> builder::AccessGroupUpdateParamsForApiPermissions {
            Default::default()
        }
    }

    /// `AddGroupBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "group_id"
    ///  ],
    ///  "properties": {
    ///    "group_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AddGroupBody {
        pub group_id: TypedUuidForAccessGroupId,
    }

    impl AddGroupBody {
        pub fn builder() -> builder::AddGroupBody {
            Default::default()
        }
    }

    /// `AddMagicLinkRedirectBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "redirect_uri"
    ///  ],
    ///  "properties": {
    ///    "redirect_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AddMagicLinkRedirectBody {
        pub redirect_uri: ::std::string::String,
    }

    impl AddMagicLinkRedirectBody {
        pub fn builder() -> builder::AddMagicLinkRedirectBody {
            Default::default()
        }
    }

    /// `AddOAuthClientRedirectBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "redirect_uri"
    ///  ],
    ///  "properties": {
    ///    "redirect_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AddOAuthClientRedirectBody {
        pub redirect_uri: ::std::string::String,
    }

    impl AddOAuthClientRedirectBody {
        pub fn builder() -> builder::AddOAuthClientRedirectBody {
            Default::default()
        }
    }

    /// `ApiKeyCreateParamsForApiPermissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "expires_at"
    ///  ],
    ///  "properties": {
    ///    "expires_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "permissions": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref":
    /// "#/components/schemas/Permissions_for_ApiPermissions"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiKeyCreateParamsForApiPermissions {
        pub expires_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub permissions: ::std::option::Option<PermissionsForApiPermissions>,
    }

    impl ApiKeyCreateParamsForApiPermissions {
        pub fn builder() -> builder::ApiKeyCreateParamsForApiPermissions {
            Default::default()
        }
    }

    /// `ApiKeyId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum ApiKeyId {}

    /// `ApiKeyResponseForApiPermissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///    },
    ///    "permissions": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref":
    /// "#/components/schemas/Permissions_for_ApiPermissions"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiKeyResponseForApiPermissions {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: TypedUuidForApiKeyId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub permissions: ::std::option::Option<PermissionsForApiPermissions>,
    }

    impl ApiKeyResponseForApiPermissions {
        pub fn builder() -> builder::ApiKeyResponseForApiPermissions {
            Default::default()
        }
    }

    /// `ApiPermissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "oneOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "CreateApiUser",
    ///        "GetApiUserSelf",
    ///        "GetApiUsersAssigned",
    ///        "GetApiUsersAll",
    ///        "ManageApiUsersAssigned",
    ///        "ManageApiUsersAll",
    ///        "CreateApiKeySelf",
    ///        "CreateApiKeyAssigned",
    ///        "CreateApiKeyAll",
    ///        "GetApiKeysAssigned",
    ///        "GetApiKeysAll",
    ///        "ManageApiKeysAssigned",
    ///        "ManageApiKeysAll",
    ///        "CreateUserApiProviderLinkToken",
    ///        "CreateGroup",
    ///        "GetGroupsJoined",
    ///        "GetGroupsAll",
    ///        "ManageGroupsAssigned",
    ///        "ManageGroupsAll",
    ///        "ManageGroupMembershipsAssigned",
    ///        "ManageGroupMembershipsAll",
    ///        "CreateMapper",
    ///        "GetMappersAll",
    ///        "ManageMappersAssigned",
    ///        "ManageMappersAll",
    ///        "CreateOAuthClient",
    ///        "GetOAuthClientsAssigned",
    ///        "GetOAuthClientsAll",
    ///        "ManageOAuthClientsAssigned",
    ///        "ManageOAuthClientsAll",
    ///        "CreateMagicLinkClient",
    ///        "GetMagicLinkClientsAssigned",
    ///        "GetMagicLinkClientsAll",
    ///        "ManageMagicLinkClientsAssigned",
    ///        "ManageMagicLinkClientsAll",
    ///        "CreateAccessToken",
    ///        "RetrieveRemoteAccessToken",
    ///        "GetSagasAll",
    ///        "ManageSagasAll"
    ///      ]
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetApiUser"
    ///      ],
    ///      "properties": {
    ///        "GetApiUser": {
    ///          "$ref": "#/components/schemas/TypedUuidForUserId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetApiUsers"
    ///      ],
    ///      "properties": {
    ///        "GetApiUsers": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForUserId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageApiUser"
    ///      ],
    ///      "properties": {
    ///        "ManageApiUser": {
    ///          "$ref": "#/components/schemas/TypedUuidForUserId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageApiUsers"
    ///      ],
    ///      "properties": {
    ///        "ManageApiUsers": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForUserId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "CreateApiKey"
    ///      ],
    ///      "properties": {
    ///        "CreateApiKey": {
    ///          "$ref": "#/components/schemas/TypedUuidForUserId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetApiKey"
    ///      ],
    ///      "properties": {
    ///        "GetApiKey": {
    ///          "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetApiKeys"
    ///      ],
    ///      "properties": {
    ///        "GetApiKeys": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageApiKey"
    ///      ],
    ///      "properties": {
    ///        "ManageApiKey": {
    ///          "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageApiKeys"
    ///      ],
    ///      "properties": {
    ///        "ManageApiKeys": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetGroup"
    ///      ],
    ///      "properties": {
    ///        "GetGroup": {
    ///          "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageGroup"
    ///      ],
    ///      "properties": {
    ///        "ManageGroup": {
    ///          "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageGroups"
    ///      ],
    ///      "properties": {
    ///        "ManageGroups": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageGroupMembership"
    ///      ],
    ///      "properties": {
    ///        "ManageGroupMembership": {
    ///          "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageGroupMemberships"
    ///      ],
    ///      "properties": {
    ///        "ManageGroupMemberships": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageMapper"
    ///      ],
    ///      "properties": {
    ///        "ManageMapper": {
    ///          "$ref": "#/components/schemas/TypedUuidForMapperId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageMappers"
    ///      ],
    ///      "properties": {
    ///        "ManageMappers": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForMapperId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetOAuthClient"
    ///      ],
    ///      "properties": {
    ///        "GetOAuthClient": {
    ///          "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetOAuthClients"
    ///      ],
    ///      "properties": {
    ///        "GetOAuthClients": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageOAuthClient"
    ///      ],
    ///      "properties": {
    ///        "ManageOAuthClient": {
    ///          "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageOAuthClients"
    ///      ],
    ///      "properties": {
    ///        "ManageOAuthClients": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetMagicLinkClient"
    ///      ],
    ///      "properties": {
    ///        "GetMagicLinkClient": {
    ///          "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetMagicLinkClients"
    ///      ],
    ///      "properties": {
    ///        "GetMagicLinkClients": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageMagicLinkClient"
    ///      ],
    ///      "properties": {
    ///        "ManageMagicLinkClient": {
    ///          "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageMagicLinkClients"
    ///      ],
    ///      "properties": {
    ///        "ManageMagicLinkClients": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Unsupported"
    ///      ],
    ///      "properties": {
    ///        "Unsupported": {}

    ///      },
    ///      "additionalProperties": false
    ///    }

    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub enum ApiPermissions {
        CreateApiUser,
        GetApiUserSelf,
        GetApiUsersAssigned,
        GetApiUsersAll,
        ManageApiUsersAssigned,
        ManageApiUsersAll,
        CreateApiKeySelf,
        CreateApiKeyAssigned,
        CreateApiKeyAll,
        GetApiKeysAssigned,
        GetApiKeysAll,
        ManageApiKeysAssigned,
        ManageApiKeysAll,
        CreateUserApiProviderLinkToken,
        CreateGroup,
        GetGroupsJoined,
        GetGroupsAll,
        ManageGroupsAssigned,
        ManageGroupsAll,
        ManageGroupMembershipsAssigned,
        ManageGroupMembershipsAll,
        CreateMapper,
        GetMappersAll,
        ManageMappersAssigned,
        ManageMappersAll,
        CreateOAuthClient,
        GetOAuthClientsAssigned,
        GetOAuthClientsAll,
        ManageOAuthClientsAssigned,
        ManageOAuthClientsAll,
        CreateMagicLinkClient,
        GetMagicLinkClientsAssigned,
        GetMagicLinkClientsAll,
        ManageMagicLinkClientsAssigned,
        ManageMagicLinkClientsAll,
        CreateAccessToken,
        RetrieveRemoteAccessToken,
        GetSagasAll,
        ManageSagasAll,
        GetApiUser(TypedUuidForUserId),
        GetApiUsers(Vec<TypedUuidForUserId>),
        ManageApiUser(TypedUuidForUserId),
        ManageApiUsers(Vec<TypedUuidForUserId>),
        CreateApiKey(TypedUuidForUserId),
        GetApiKey(TypedUuidForApiKeyId),
        GetApiKeys(Vec<TypedUuidForApiKeyId>),
        ManageApiKey(TypedUuidForApiKeyId),
        ManageApiKeys(Vec<TypedUuidForApiKeyId>),
        GetGroup(TypedUuidForAccessGroupId),
        ManageGroup(TypedUuidForAccessGroupId),
        ManageGroups(Vec<TypedUuidForAccessGroupId>),
        ManageGroupMembership(TypedUuidForAccessGroupId),
        ManageGroupMemberships(Vec<TypedUuidForAccessGroupId>),
        ManageMapper(TypedUuidForMapperId),
        ManageMappers(Vec<TypedUuidForMapperId>),
        GetOAuthClient(TypedUuidForOAuthClientId),
        GetOAuthClients(Vec<TypedUuidForOAuthClientId>),
        ManageOAuthClient(TypedUuidForOAuthClientId),
        ManageOAuthClients(Vec<TypedUuidForOAuthClientId>),
        GetMagicLinkClient(TypedUuidForMagicLinkId),
        GetMagicLinkClients(Vec<TypedUuidForMagicLinkId>),
        ManageMagicLinkClient(TypedUuidForMagicLinkId),
        ManageMagicLinkClients(Vec<TypedUuidForMagicLinkId>),
        Unsupported(::serde_json::Value),
    }

    impl ::std::convert::From<TypedUuidForMapperId> for ApiPermissions {
        fn from(value: TypedUuidForMapperId) -> Self {
            Self::ManageMapper(value)
        }
    }

    impl ::std::convert::From<Vec<TypedUuidForMapperId>> for ApiPermissions {
        fn from(value: Vec<TypedUuidForMapperId>) -> Self {
            Self::ManageMappers(value)
        }
    }

    impl ::std::convert::From<::serde_json::Value> for ApiPermissions {
        fn from(value: ::serde_json::Value) -> Self {
            Self::Unsupported(value)
        }
    }

    /// `ApiUserContactEmail`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "email",
    ///    "id",
    ///    "updated_at",
    ///    "user_id"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserProviderId"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "user_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserId"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserContactEmail {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub email: ::std::string::String,
        pub id: TypedUuidForUserProviderId,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub user_id: TypedUuidForUserId,
    }

    impl ApiUserContactEmail {
        pub fn builder() -> builder::ApiUserContactEmail {
            Default::default()
        }
    }

    /// `ApiUserEmailUpdateParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "email"
    ///  ],
    ///  "properties": {
    ///    "email": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserEmailUpdateParams {
        pub email: ::std::string::String,
    }

    impl ApiUserEmailUpdateParams {
        pub fn builder() -> builder::ApiUserEmailUpdateParams {
            Default::default()
        }
    }

    /// `ApiUserForApiPermissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "groups",
    ///    "id",
    ///    "permissions",
    ///    "updated_at"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "groups": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserId"
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/Permissions_for_ApiPermissions"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserForApiPermissions {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub groups: Vec<TypedUuidForAccessGroupId>,
        pub id: TypedUuidForUserId,
        pub permissions: PermissionsForApiPermissions,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ApiUserForApiPermissions {
        pub fn builder() -> builder::ApiUserForApiPermissions {
            Default::default()
        }
    }

    /// `ApiUserLinkRequestPayload`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "user_id"
    ///  ],
    ///  "properties": {
    ///    "user_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserId"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserLinkRequestPayload {
        pub user_id: TypedUuidForUserId,
    }

    impl ApiUserLinkRequestPayload {
        pub fn builder() -> builder::ApiUserLinkRequestPayload {
            Default::default()
        }
    }

    /// `ApiUserLinkRequestResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "token"
    ///  ],
    ///  "properties": {
    ///    "token": {
    ///      "$ref": "#/components/schemas/SecretString"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserLinkRequestResponse {
        pub token: SecretString,
    }

    impl ApiUserLinkRequestResponse {
        pub fn builder() -> builder::ApiUserLinkRequestResponse {
            Default::default()
        }
    }

    /// `ApiUserProvider`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "display_names",
    ///    "emails",
    ///    "id",
    ///    "provider",
    ///    "provider_id",
    ///    "updated_at",
    ///    "user_id"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "display_names": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }

    ///    },
    ///    "emails": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }

    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserProviderId"
    ///    },
    ///    "provider": {
    ///      "type": "string"
    ///    },
    ///    "provider_id": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "user_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserId"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserProvider {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub display_names: ::std::vec::Vec<::std::string::String>,
        pub emails: ::std::vec::Vec<::std::string::String>,
        pub id: TypedUuidForUserProviderId,
        pub provider: ::std::string::String,
        pub provider_id: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub user_id: TypedUuidForUserId,
    }

    impl ApiUserProvider {
        pub fn builder() -> builder::ApiUserProvider {
            Default::default()
        }
    }

    /// `ApiUserProviderLinkPayload`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "token"
    ///  ],
    ///  "properties": {
    ///    "token": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserProviderLinkPayload {
        pub token: ::std::string::String,
    }

    impl ApiUserProviderLinkPayload {
        pub fn builder() -> builder::ApiUserProviderLinkPayload {
            Default::default()
        }
    }

    /// `ApiUserUpdateParamsForApiPermissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "group_ids",
    ///    "permissions"
    ///  ],
    ///  "properties": {
    ///    "group_ids": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/Permissions_for_ApiPermissions"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserUpdateParamsForApiPermissions {
        pub group_ids: Vec<TypedUuidForAccessGroupId>,
        pub permissions: PermissionsForApiPermissions,
    }

    impl ApiUserUpdateParamsForApiPermissions {
        pub fn builder() -> builder::ApiUserUpdateParamsForApiPermissions {
            Default::default()
        }
    }

    /// `CreateMapper`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "rule"
    ///  ],
    ///  "properties": {
    ///    "max_activations": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "rule": {}

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct CreateMapper {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_activations: ::std::option::Option<i32>,
        pub name: ::std::string::String,
        pub rule: ::serde_json::Value,
    }

    impl CreateMapper {
        pub fn builder() -> builder::CreateMapper {
            Default::default()
        }
    }

    /// Request body for initiating a device authorization flow. The client
    /// sends its `client_id` and an optional `scope`. The API server proxies
    /// the device authorization request to the upstream provider and tracks it
    /// as a login attempt.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "description": "Request body for initiating a device authorization
    /// flow. The client sends its `client_id` and an optional `scope`. The API
    /// server proxies the device authorization request to the upstream provider
    /// and tracks it as a login attempt.",
    ///  "type": "object",
    ///  "required": [
    ///    "client_id"
    ///  ],
    ///  "properties": {
    ///    "client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///    },
    ///    "scope": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct DeviceAuthorizationRequest {
        pub client_id: TypedUuidForOAuthClientId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scope: ::std::option::Option<::std::string::String>,
    }

    impl DeviceAuthorizationRequest {
        pub fn builder() -> builder::DeviceAuthorizationRequest {
            Default::default()
        }
    }

    /// Request body for the device token exchange. The client polls this
    /// endpoint with the device_code received from the authorization step.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "description": "Request body for the device token exchange. The client
    /// polls this endpoint with the device_code received from the authorization
    /// step.",
    ///  "type": "object",
    ///  "required": [
    ///    "client_id",
    ///    "device_code",
    ///    "grant_type"
    ///  ],
    ///  "properties": {
    ///    "client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///    },
    ///    "device_code": {
    ///      "type": "string"
    ///    },
    ///    "grant_type": {
    ///      "type": "string"
    ///    },
    ///    "pkce_verifier": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct DeviceTokenExchangeRequest {
        pub client_id: TypedUuidForOAuthClientId,
        pub device_code: ::std::string::String,
        pub grant_type: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pkce_verifier: ::std::option::Option<::std::string::String>,
    }

    impl DeviceTokenExchangeRequest {
        pub fn builder() -> builder::DeviceTokenExchangeRequest {
            Default::default()
        }
    }

    /// Error information from a response.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "description": "Error information from a response.",
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "request_id"
    ///  ],
    ///  "properties": {
    ///    "error_code": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "request_id": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Error {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error_code: ::std::option::Option<::std::string::String>,
        pub message: ::std::string::String,
        pub request_id: ::std::string::String,
    }

    impl Error {
        pub fn builder() -> builder::Error {
            Default::default()
        }
    }

    /// `GetUserResponseForApiPermissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "info",
    ///    "providers"
    ///  ],
    ///  "properties": {
    ///    "info": {
    ///      "$ref": "#/components/schemas/ApiUser_for_ApiPermissions"
    ///    },
    ///    "providers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ApiUserProvider"
    ///      }

    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct GetUserResponseForApiPermissions {
        pub info: ApiUserForApiPermissions,
        pub providers: ::std::vec::Vec<ApiUserProvider>,
    }

    impl GetUserResponseForApiPermissions {
        pub fn builder() -> builder::GetUserResponseForApiPermissions {
            Default::default()
        }
    }

    /// `InitialApiKeyResponseForApiPermissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "key"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///    },
    ///    "key": {
    ///      "$ref": "#/components/schemas/SecretString"
    ///    },
    ///    "permissions": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref":
    /// "#/components/schemas/Permissions_for_ApiPermissions"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InitialApiKeyResponseForApiPermissions {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: TypedUuidForApiKeyId,
        pub key: SecretString,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub permissions: ::std::option::Option<PermissionsForApiPermissions>,
    }

    impl InitialApiKeyResponseForApiPermissions {
        pub fn builder() -> builder::InitialApiKeyResponseForApiPermissions {
            Default::default()
        }
    }

    /// `InitialMagicLinkSecretResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "key"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkSecretId"
    ///    },
    ///    "key": {
    ///      "$ref": "#/components/schemas/SecretString"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InitialMagicLinkSecretResponse {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: TypedUuidForMagicLinkSecretId,
        pub key: SecretString,
    }

    impl InitialMagicLinkSecretResponse {
        pub fn builder() -> builder::InitialMagicLinkSecretResponse {
            Default::default()
        }
    }

    /// `InitialOAuthClientSecretResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "key"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthSecretId"
    ///    },
    ///    "key": {
    ///      "$ref": "#/components/schemas/SecretString"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InitialOAuthClientSecretResponse {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: TypedUuidForOAuthSecretId,
        pub key: SecretString,
    }

    impl InitialOAuthClientSecretResponse {
        pub fn builder() -> builder::InitialOAuthClientSecretResponse {
            Default::default()
        }
    }

    /// `Jwk`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "e",
    ///    "kid",
    ///    "kty",
    ///    "n",
    ///    "use"
    ///  ],
    ///  "properties": {
    ///    "e": {
    ///      "type": "string"
    ///    },
    ///    "kid": {
    ///      "type": "string"
    ///    },
    ///    "kty": {
    ///      "type": "string"
    ///    },
    ///    "n": {
    ///      "type": "string"
    ///    },
    ///    "use": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Jwk {
        pub e: ::std::string::String,
        pub kid: ::std::string::String,
        pub kty: ::std::string::String,
        pub n: ::std::string::String,
        #[serde(rename = "use")]
        pub use_: ::std::string::String,
    }

    impl Jwk {
        pub fn builder() -> builder::Jwk {
            Default::default()
        }
    }

    /// `Jwks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "keys"
    ///  ],
    ///  "properties": {
    ///    "keys": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Jwk"
    ///      }

    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Jwks {
        pub keys: ::std::vec::Vec<Jwk>,
    }

    impl Jwks {
        pub fn builder() -> builder::Jwks {
            Default::default()
        }
    }

    /// `MagicLink`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "redirect_uris",
    ///    "secrets"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///    },
    ///    "redirect_uris": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MagicLinkRedirectUri"
    ///      }

    ///    },
    ///    "secrets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MagicLinkSecret"
    ///      }

    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLink {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForMagicLinkId,
        pub redirect_uris: ::std::vec::Vec<MagicLinkRedirectUri>,
        pub secrets: ::std::vec::Vec<MagicLinkSecret>,
    }

    impl MagicLink {
        pub fn builder() -> builder::MagicLink {
            Default::default()
        }
    }

    /// `MagicLinkAttemptId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum MagicLinkAttemptId {}

    /// `MagicLinkExchangeRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "attempt_id",
    ///    "recipient",
    ///    "secret"
    ///  ],
    ///  "properties": {
    ///    "attempt_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkAttemptId"
    ///    },
    ///    "recipient": {
    ///      "type": "string"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkExchangeRequest {
        pub attempt_id: TypedUuidForMagicLinkAttemptId,
        pub recipient: ::std::string::String,
        pub secret: ::std::string::String,
    }

    impl MagicLinkExchangeRequest {
        pub fn builder() -> builder::MagicLinkExchangeRequest {
            Default::default()
        }
    }

    /// `MagicLinkExchangeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "access_token",
    ///    "expires_in",
    ///    "token_type"
    ///  ],
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "expires_in": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "token_type": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkExchangeResponse {
        pub access_token: ::std::string::String,
        pub expires_in: i64,
        pub token_type: ::std::string::String,
    }

    impl MagicLinkExchangeResponse {
        pub fn builder() -> builder::MagicLinkExchangeResponse {
            Default::default()
        }
    }

    /// `MagicLinkId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum MagicLinkId {}

    /// `MagicLinkMedium`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "email"
    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    pub enum MagicLinkMedium {
        #[serde(rename = "email")]
        Email,
    }

    impl ::std::fmt::Display for MagicLinkMedium {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Email => f.write_str("email"),
            }
        }
    }

    impl ::std::str::FromStr for MagicLinkMedium {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "email" => Ok(Self::Email),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MagicLinkMedium {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MagicLinkMedium {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MagicLinkMedium {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// `MagicLinkRedirectUri`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "magic_link_client_id",
    ///    "redirect_uri"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkRedirectUriId"
    ///    },
    ///    "magic_link_client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///    },
    ///    "redirect_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkRedirectUri {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForMagicLinkRedirectUriId,
        pub magic_link_client_id: TypedUuidForMagicLinkId,
        pub redirect_uri: ::std::string::String,
    }

    impl MagicLinkRedirectUri {
        pub fn builder() -> builder::MagicLinkRedirectUri {
            Default::default()
        }
    }

    /// `MagicLinkRedirectUriId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum MagicLinkRedirectUriId {}

    /// `MagicLinkSecret`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "magic_link_client_id",
    ///    "secret_signature"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkSecretId"
    ///    },
    ///    "magic_link_client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///    },
    ///    "secret_signature": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkSecret {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForMagicLinkSecretId,
        pub magic_link_client_id: TypedUuidForMagicLinkId,
        pub secret_signature: ::std::string::String,
    }

    impl MagicLinkSecret {
        pub fn builder() -> builder::MagicLinkSecret {
            Default::default()
        }
    }

    /// `MagicLinkSecretId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum MagicLinkSecretId {}

    /// `MagicLinkSendRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "expires_in",
    ///    "medium",
    ///    "recipient",
    ///    "redirect_uri",
    ///    "secret"
    ///  ],
    ///  "properties": {
    ///    "expires_in": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "medium": {
    ///      "$ref": "#/components/schemas/MagicLinkMedium"
    ///    },
    ///    "recipient": {
    ///      "type": "string"
    ///    },
    ///    "redirect_uri": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    },
    ///    "scope": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkSendRequest {
        pub expires_in: i64,
        pub medium: MagicLinkMedium,
        pub recipient: ::std::string::String,
        pub redirect_uri: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scope: ::std::option::Option<::std::string::String>,
        pub secret: ::std::string::String,
    }

    impl MagicLinkSendRequest {
        pub fn builder() -> builder::MagicLinkSendRequest {
            Default::default()
        }
    }

    /// `MagicLinkSendResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "attempt_id"
    ///  ],
    ///  "properties": {
    ///    "attempt_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkAttemptId"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkSendResponse {
        pub attempt_id: TypedUuidForMagicLinkAttemptId,
    }

    impl MagicLinkSendResponse {
        pub fn builder() -> builder::MagicLinkSendResponse {
            Default::default()
        }
    }

    /// `Mapper`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "name",
    ///    "rule",
    ///    "updated_at"
    ///  ],
    ///  "properties": {
    ///    "activations": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "depleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMapperId"
    ///    },
    ///    "max_activations": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "rule": {},
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Mapper {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub activations: ::std::option::Option<i32>,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub depleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForMapperId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_activations: ::std::option::Option<i32>,
        pub name: ::std::string::String,
        pub rule: ::serde_json::Value,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl Mapper {
        pub fn builder() -> builder::Mapper {
            Default::default()
        }
    }

    /// `MapperId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum MapperId {}

    /// `OAuthAuthzCodeExchangeBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "grant_type",
    ///    "pkce_verifier",
    ///    "redirect_uri"
    ///  ],
    ///  "properties": {
    ///    "client_id": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "client_secret": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/SecretString"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "grant_type": {
    ///      "type": "string"
    ///    },
    ///    "pkce_verifier": {
    ///      "description": "PKCE code verifier (RFC 7636). Required for all
    /// authorization code exchanges.",
    ///      "type": "string"
    ///    },
    ///    "redirect_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthAuthzCodeExchangeBody {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub client_id: ::std::option::Option<TypedUuidForOAuthClientId>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub client_secret: ::std::option::Option<SecretString>,
        pub code: ::std::string::String,
        pub grant_type: ::std::string::String,
        /// PKCE code verifier (RFC 7636). Required for all authorization code
        /// exchanges.
        pub pkce_verifier: ::std::string::String,
        pub redirect_uri: ::std::string::String,
    }

    impl OAuthAuthzCodeExchangeBody {
        pub fn builder() -> builder::OAuthAuthzCodeExchangeBody {
            Default::default()
        }
    }

    /// `OAuthAuthzCodeExchangeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "access_token",
    ///    "expires_in",
    ///    "scope",
    ///    "token_type"
    ///  ],
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "expires_in": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "idp_token": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "scope": {
    ///      "description": "The scope granted to the access token (RFC 6749
    /// §5.1).",
    ///      "type": "string"
    ///    },
    ///    "token_type": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthAuthzCodeExchangeResponse {
        pub access_token: ::std::string::String,
        pub expires_in: i64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub idp_token: ::std::option::Option<::std::string::String>,
        /// The scope granted to the access token (RFC 6749 §5.1).
        pub scope: ::std::string::String,
        pub token_type: ::std::string::String,
    }

    impl OAuthAuthzCodeExchangeResponse {
        pub fn builder() -> builder::OAuthAuthzCodeExchangeResponse {
            Default::default()
        }
    }

    /// `OAuthClient`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "redirect_uris",
    ///    "secrets"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///    },
    ///    "redirect_uris": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OAuthClientRedirectUri"
    ///      }

    ///    },
    ///    "secrets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OAuthClientSecret"
    ///      }

    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthClient {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForOAuthClientId,
        pub redirect_uris: ::std::vec::Vec<OAuthClientRedirectUri>,
        pub secrets: ::std::vec::Vec<OAuthClientSecret>,
    }

    impl OAuthClient {
        pub fn builder() -> builder::OAuthClient {
            Default::default()
        }
    }

    /// `OAuthClientId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum OAuthClientId {}

    /// `OAuthClientRedirectUri`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "oauth_client_id",
    ///    "redirect_uri"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthRedirectUriId"
    ///    },
    ///    "oauth_client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///    },
    ///    "redirect_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthClientRedirectUri {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForOAuthRedirectUriId,
        pub oauth_client_id: TypedUuidForOAuthClientId,
        pub redirect_uri: ::std::string::String,
    }

    impl OAuthClientRedirectUri {
        pub fn builder() -> builder::OAuthClientRedirectUri {
            Default::default()
        }
    }

    /// `OAuthClientSecret`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "oauth_client_id",
    ///    "secret_signature"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthSecretId"
    ///    },
    ///    "oauth_client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///    },
    ///    "secret_signature": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthClientSecret {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForOAuthSecretId,
        pub oauth_client_id: TypedUuidForOAuthClientId,
        pub secret_signature: ::std::string::String,
    }

    impl OAuthClientSecret {
        pub fn builder() -> builder::OAuthClientSecret {
            Default::default()
        }
    }

    /// `OAuthProviderAuthorizationCodeInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "auth_url_endpoint",
    ///    "redirect_endpoint",
    ///    "token_endpoint",
    ///    "token_endpoint_content_type"
    ///  ],
    ///  "properties": {
    ///    "auth_url_endpoint": {
    ///      "type": "string"
    ///    },
    ///    "redirect_endpoint": {
    ///      "type": "string"
    ///    },
    ///    "token_endpoint": {
    ///      "type": "string"
    ///    },
    ///    "token_endpoint_content_type": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthProviderAuthorizationCodeInfo {
        pub auth_url_endpoint: ::std::string::String,
        pub redirect_endpoint: ::std::string::String,
        pub token_endpoint: ::std::string::String,
        pub token_endpoint_content_type: ::std::string::String,
    }

    impl OAuthProviderAuthorizationCodeInfo {
        pub fn builder() -> builder::OAuthProviderAuthorizationCodeInfo {
            Default::default()
        }
    }

    /// `OAuthProviderAuthorizationCodePkceInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "client_id",
    ///    "proxy_port",
    ///    "redirect_endpoint",
    ///    "web"
    ///  ],
    ///  "properties": {
    ///    "client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///    },
    ///    "proxy_port": {
    ///      "type": "integer",
    ///      "format": "uint16",
    ///      "minimum": 0.0
    ///    },
    ///    "redirect_endpoint": {
    ///      "type": "string"
    ///    },
    ///    "web": {
    ///      "$ref": "#/components/schemas/OAuthProviderAuthorizationCodeInfo"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthProviderAuthorizationCodePkceInfo {
        pub client_id: TypedUuidForOAuthClientId,
        pub proxy_port: u16,
        pub redirect_endpoint: ::std::string::String,
        pub web: OAuthProviderAuthorizationCodeInfo,
    }

    impl OAuthProviderAuthorizationCodePkceInfo {
        pub fn builder() -> builder::OAuthProviderAuthorizationCodePkceInfo {
            Default::default()
        }
    }

    /// `OAuthProviderDeviceInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "auth_url_endpoint",
    ///    "client_id",
    ///    "token_endpoint"
    ///  ],
    ///  "properties": {
    ///    "auth_url_endpoint": {
    ///      "type": "string"
    ///    },
    ///    "client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///    },
    ///    "token_endpoint": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthProviderDeviceInfo {
        pub auth_url_endpoint: ::std::string::String,
        pub client_id: TypedUuidForOAuthClientId,
        pub token_endpoint: ::std::string::String,
    }

    impl OAuthProviderDeviceInfo {
        pub fn builder() -> builder::OAuthProviderDeviceInfo {
            Default::default()
        }
    }

    /// `OAuthProviderName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "github",
    ///    "google",
    ///    "zendesk"
    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    pub enum OAuthProviderName {
        #[serde(rename = "github")]
        Github,
        #[serde(rename = "google")]
        Google,
        #[serde(rename = "zendesk")]
        Zendesk,
    }

    impl ::std::fmt::Display for OAuthProviderName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Github => f.write_str("github"),
                Self::Google => f.write_str("google"),
                Self::Zendesk => f.write_str("zendesk"),
            }
        }
    }

    impl ::std::str::FromStr for OAuthProviderName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "github" => Ok(Self::Github),
                "google" => Ok(Self::Google),
                "zendesk" => Ok(Self::Zendesk),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OAuthProviderName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OAuthProviderName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OAuthProviderName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// `OAuthRedirectUriId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum OAuthRedirectUriId {}

    /// `OAuthSecretId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum OAuthSecretId {}

    /// `OpenIdConfiguration`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "jwks_uri"
    ///  ],
    ///  "properties": {
    ///    "jwks_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OpenIdConfiguration {
        pub jwks_uri: ::std::string::String,
    }

    impl OpenIdConfiguration {
        pub fn builder() -> builder::OpenIdConfiguration {
            Default::default()
        }
    }

    /// `PermissionsForApiPermissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/ApiPermissions"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct PermissionsForApiPermissions(pub ::std::vec::Vec<ApiPermissions>);
    impl ::std::ops::Deref for PermissionsForApiPermissions {
        type Target = ::std::vec::Vec<ApiPermissions>;
        fn deref(&self) -> &::std::vec::Vec<ApiPermissions> {
            &self.0
        }
    }

    impl ::std::convert::From<PermissionsForApiPermissions> for ::std::vec::Vec<ApiPermissions> {
        fn from(value: PermissionsForApiPermissions) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::vec::Vec<ApiPermissions>> for PermissionsForApiPermissions {
        fn from(value: ::std::vec::Vec<ApiPermissions>) -> Self {
            Self(value)
        }
    }

    /// `SecretString`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct SecretString(pub ::std::string::String);
    impl ::std::ops::Deref for SecretString {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<SecretString> for ::std::string::String {
        fn from(value: SecretString) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::string::String> for SecretString {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for SecretString {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for SecretString {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForAccessGroupId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/AccessGroupId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForAccessGroupId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForAccessGroupId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForAccessGroupId> for ::uuid::Uuid {
        fn from(value: TypedUuidForAccessGroupId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForAccessGroupId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForAccessGroupId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForAccessGroupId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForAccessGroupId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForAccessGroupId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForApiKeyId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/ApiKeyId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForApiKeyId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForApiKeyId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForApiKeyId> for ::uuid::Uuid {
        fn from(value: TypedUuidForApiKeyId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForApiKeyId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForApiKeyId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForApiKeyId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForApiKeyId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForApiKeyId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForMagicLinkAttemptId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/MagicLinkAttemptId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForMagicLinkAttemptId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForMagicLinkAttemptId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForMagicLinkAttemptId> for ::uuid::Uuid {
        fn from(value: TypedUuidForMagicLinkAttemptId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMagicLinkAttemptId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForMagicLinkAttemptId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForMagicLinkAttemptId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForMagicLinkAttemptId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForMagicLinkAttemptId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForMagicLinkId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/MagicLinkId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForMagicLinkId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForMagicLinkId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForMagicLinkId> for ::uuid::Uuid {
        fn from(value: TypedUuidForMagicLinkId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMagicLinkId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForMagicLinkId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForMagicLinkId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForMagicLinkId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForMagicLinkId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForMagicLinkRedirectUriId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/MagicLinkRedirectUriId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForMagicLinkRedirectUriId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForMagicLinkRedirectUriId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForMagicLinkRedirectUriId> for ::uuid::Uuid {
        fn from(value: TypedUuidForMagicLinkRedirectUriId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMagicLinkRedirectUriId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForMagicLinkRedirectUriId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForMagicLinkRedirectUriId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForMagicLinkRedirectUriId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForMagicLinkRedirectUriId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForMagicLinkSecretId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/MagicLinkSecretId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForMagicLinkSecretId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForMagicLinkSecretId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForMagicLinkSecretId> for ::uuid::Uuid {
        fn from(value: TypedUuidForMagicLinkSecretId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMagicLinkSecretId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForMagicLinkSecretId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForMagicLinkSecretId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForMagicLinkSecretId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForMagicLinkSecretId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForMapperId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/MapperId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForMapperId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForMapperId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForMapperId> for ::uuid::Uuid {
        fn from(value: TypedUuidForMapperId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMapperId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForMapperId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForMapperId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForMapperId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForMapperId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForOAuthClientId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/OAuthClientId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForOAuthClientId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForOAuthClientId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForOAuthClientId> for ::uuid::Uuid {
        fn from(value: TypedUuidForOAuthClientId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForOAuthClientId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForOAuthClientId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForOAuthClientId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForOAuthClientId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForOAuthClientId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForOAuthRedirectUriId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/OAuthRedirectUriId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForOAuthRedirectUriId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForOAuthRedirectUriId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForOAuthRedirectUriId> for ::uuid::Uuid {
        fn from(value: TypedUuidForOAuthRedirectUriId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForOAuthRedirectUriId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForOAuthRedirectUriId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForOAuthRedirectUriId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForOAuthRedirectUriId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForOAuthRedirectUriId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForOAuthSecretId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/OAuthSecretId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForOAuthSecretId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForOAuthSecretId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForOAuthSecretId> for ::uuid::Uuid {
        fn from(value: TypedUuidForOAuthSecretId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForOAuthSecretId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForOAuthSecretId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForOAuthSecretId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForOAuthSecretId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForOAuthSecretId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForUserId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/UserId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForUserId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForUserId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForUserId> for ::uuid::Uuid {
        fn from(value: TypedUuidForUserId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForUserId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForUserId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForUserId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForUserId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForUserId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForUserProviderId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid",
    ///  "x-rust-type": {
    ///    "crate": "newtype-uuid",
    ///    "parameters": [
    ///      {
    ///        "$ref": "#/components/schemas/UserProviderId"
    ///      }

    ///    ],
    ///    "path": "newtype_uuid::TypedUuid",
    ///    "version": "1"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForUserProviderId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForUserProviderId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForUserProviderId> for ::uuid::Uuid {
        fn from(value: TypedUuidForUserProviderId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForUserProviderId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForUserProviderId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForUserProviderId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForUserProviderId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForUserProviderId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `UserId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum UserId {}

    /// `UserProviderId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// false
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(deny_unknown_fields)]
    pub enum UserProviderId {}

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct AccessGroupForApiPermissions {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForAccessGroupId, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            permissions:
                ::std::result::Result<super::PermissionsForApiPermissions, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for AccessGroupForApiPermissions {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                }
            }
        }

        impl AccessGroupForApiPermissions {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForAccessGroupId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PermissionsForApiPermissions>,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {e}"));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<AccessGroupForApiPermissions> for super::AccessGroupForApiPermissions {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccessGroupForApiPermissions,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    name: value.name?,
                    permissions: value.permissions?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl ::std::convert::From<super::AccessGroupForApiPermissions> for AccessGroupForApiPermissions {
            fn from(value: super::AccessGroupForApiPermissions) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    name: Ok(value.name),
                    permissions: Ok(value.permissions),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AccessGroupUpdateParamsForApiPermissions {
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            permissions:
                ::std::result::Result<super::PermissionsForApiPermissions, ::std::string::String>,
        }

        impl ::std::default::Default for AccessGroupUpdateParamsForApiPermissions {
            fn default() -> Self {
                Self {
                    name: Err("no value supplied for name".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl AccessGroupUpdateParamsForApiPermissions {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PermissionsForApiPermissions>,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<AccessGroupUpdateParamsForApiPermissions>
            for super::AccessGroupUpdateParamsForApiPermissions
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccessGroupUpdateParamsForApiPermissions,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    name: value.name?,
                    permissions: value.permissions?,
                })
            }
        }

        impl ::std::convert::From<super::AccessGroupUpdateParamsForApiPermissions>
            for AccessGroupUpdateParamsForApiPermissions
        {
            fn from(value: super::AccessGroupUpdateParamsForApiPermissions) -> Self {
                Self {
                    name: Ok(value.name),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddGroupBody {
            group_id:
                ::std::result::Result<super::TypedUuidForAccessGroupId, ::std::string::String>,
        }

        impl ::std::default::Default for AddGroupBody {
            fn default() -> Self {
                Self {
                    group_id: Err("no value supplied for group_id".to_string()),
                }
            }
        }

        impl AddGroupBody {
            pub fn group_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForAccessGroupId>,
                T::Error: ::std::fmt::Display,
            {
                self.group_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for group_id: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<AddGroupBody> for super::AddGroupBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AddGroupBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    group_id: value.group_id?,
                })
            }
        }

        impl ::std::convert::From<super::AddGroupBody> for AddGroupBody {
            fn from(value: super::AddGroupBody) -> Self {
                Self {
                    group_id: Ok(value.group_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddMagicLinkRedirectBody {
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for AddMagicLinkRedirectBody {
            fn default() -> Self {
                Self {
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl AddMagicLinkRedirectBody {
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for redirect_uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<AddMagicLinkRedirectBody> for super::AddMagicLinkRedirectBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AddMagicLinkRedirectBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl ::std::convert::From<super::AddMagicLinkRedirectBody> for AddMagicLinkRedirectBody {
            fn from(value: super::AddMagicLinkRedirectBody) -> Self {
                Self {
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddOAuthClientRedirectBody {
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for AddOAuthClientRedirectBody {
            fn default() -> Self {
                Self {
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl AddOAuthClientRedirectBody {
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for redirect_uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<AddOAuthClientRedirectBody> for super::AddOAuthClientRedirectBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AddOAuthClientRedirectBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl ::std::convert::From<super::AddOAuthClientRedirectBody> for AddOAuthClientRedirectBody {
            fn from(value: super::AddOAuthClientRedirectBody) -> Self {
                Self {
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiKeyCreateParamsForApiPermissions {
            expires_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            permissions: ::std::result::Result<
                ::std::option::Option<super::PermissionsForApiPermissions>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ApiKeyCreateParamsForApiPermissions {
            fn default() -> Self {
                Self {
                    expires_at: Err("no value supplied for expires_at".to_string()),
                    permissions: Ok(Default::default()),
                }
            }
        }

        impl ApiKeyCreateParamsForApiPermissions {
            pub fn expires_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.expires_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::PermissionsForApiPermissions>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiKeyCreateParamsForApiPermissions>
            for super::ApiKeyCreateParamsForApiPermissions
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiKeyCreateParamsForApiPermissions,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    expires_at: value.expires_at?,
                    permissions: value.permissions?,
                })
            }
        }

        impl ::std::convert::From<super::ApiKeyCreateParamsForApiPermissions>
            for ApiKeyCreateParamsForApiPermissions
        {
            fn from(value: super::ApiKeyCreateParamsForApiPermissions) -> Self {
                Self {
                    expires_at: Ok(value.expires_at),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiKeyResponseForApiPermissions {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForApiKeyId, ::std::string::String>,
            permissions: ::std::result::Result<
                ::std::option::Option<super::PermissionsForApiPermissions>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ApiKeyResponseForApiPermissions {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    permissions: Ok(Default::default()),
                }
            }
        }

        impl ApiKeyResponseForApiPermissions {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForApiKeyId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::PermissionsForApiPermissions>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiKeyResponseForApiPermissions>
            for super::ApiKeyResponseForApiPermissions
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiKeyResponseForApiPermissions,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    permissions: value.permissions?,
                })
            }
        }

        impl ::std::convert::From<super::ApiKeyResponseForApiPermissions>
            for ApiKeyResponseForApiPermissions
        {
            fn from(value: super::ApiKeyResponseForApiPermissions) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserContactEmail {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<super::TypedUuidForUserProviderId, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            user_id: ::std::result::Result<super::TypedUuidForUserId, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserContactEmail {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    email: Err("no value supplied for email".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                    user_id: Err("no value supplied for user_id".to_string()),
                }
            }
        }

        impl ApiUserContactEmail {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserProviderId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserId>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserContactEmail> for super::ApiUserContactEmail {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserContactEmail,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    email: value.email?,
                    id: value.id?,
                    updated_at: value.updated_at?,
                    user_id: value.user_id?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserContactEmail> for ApiUserContactEmail {
            fn from(value: super::ApiUserContactEmail) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    email: Ok(value.email),
                    id: Ok(value.id),
                    updated_at: Ok(value.updated_at),
                    user_id: Ok(value.user_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserEmailUpdateParams {
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserEmailUpdateParams {
            fn default() -> Self {
                Self {
                    email: Err("no value supplied for email".to_string()),
                }
            }
        }

        impl ApiUserEmailUpdateParams {
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserEmailUpdateParams> for super::ApiUserEmailUpdateParams {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserEmailUpdateParams,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    email: value.email?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserEmailUpdateParams> for ApiUserEmailUpdateParams {
            fn from(value: super::ApiUserEmailUpdateParams) -> Self {
                Self {
                    email: Ok(value.email),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserForApiPermissions {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            groups:
                ::std::result::Result<Vec<super::TypedUuidForAccessGroupId>, ::std::string::String>,
            id: ::std::result::Result<super::TypedUuidForUserId, ::std::string::String>,
            permissions:
                ::std::result::Result<super::PermissionsForApiPermissions, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ApiUserForApiPermissions {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    groups: Err("no value supplied for groups".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                }
            }
        }

        impl ApiUserForApiPermissions {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn groups<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<Vec<super::TypedUuidForAccessGroupId>>,
                T::Error: ::std::fmt::Display,
            {
                self.groups = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for groups: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PermissionsForApiPermissions>,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {e}"));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserForApiPermissions> for super::ApiUserForApiPermissions {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserForApiPermissions,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    groups: value.groups?,
                    id: value.id?,
                    permissions: value.permissions?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserForApiPermissions> for ApiUserForApiPermissions {
            fn from(value: super::ApiUserForApiPermissions) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    groups: Ok(value.groups),
                    id: Ok(value.id),
                    permissions: Ok(value.permissions),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserLinkRequestPayload {
            user_id: ::std::result::Result<super::TypedUuidForUserId, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserLinkRequestPayload {
            fn default() -> Self {
                Self {
                    user_id: Err("no value supplied for user_id".to_string()),
                }
            }
        }

        impl ApiUserLinkRequestPayload {
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserId>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserLinkRequestPayload> for super::ApiUserLinkRequestPayload {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserLinkRequestPayload,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    user_id: value.user_id?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserLinkRequestPayload> for ApiUserLinkRequestPayload {
            fn from(value: super::ApiUserLinkRequestPayload) -> Self {
                Self {
                    user_id: Ok(value.user_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserLinkRequestResponse {
            token: ::std::result::Result<super::SecretString, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserLinkRequestResponse {
            fn default() -> Self {
                Self {
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl ApiUserLinkRequestResponse {
            pub fn token<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SecretString>,
                T::Error: ::std::fmt::Display,
            {
                self.token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserLinkRequestResponse> for super::ApiUserLinkRequestResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserLinkRequestResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    token: value.token?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserLinkRequestResponse> for ApiUserLinkRequestResponse {
            fn from(value: super::ApiUserLinkRequestResponse) -> Self {
                Self {
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserProvider {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            display_names: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            emails: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForUserProviderId, ::std::string::String>,
            provider: ::std::result::Result<::std::string::String, ::std::string::String>,
            provider_id: ::std::result::Result<::std::string::String, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            user_id: ::std::result::Result<super::TypedUuidForUserId, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserProvider {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    display_names: Err("no value supplied for display_names".to_string()),
                    emails: Err("no value supplied for emails".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    provider: Err("no value supplied for provider".to_string()),
                    provider_id: Err("no value supplied for provider_id".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                    user_id: Err("no value supplied for user_id".to_string()),
                }
            }
        }

        impl ApiUserProvider {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn display_names<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.display_names = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for display_names: {e}"));
                self
            }
            pub fn emails<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.emails = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for emails: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserProviderId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn provider<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.provider = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider: {e}"));
                self
            }
            pub fn provider_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.provider_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider_id: {e}"));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserId>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserProvider> for super::ApiUserProvider {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserProvider,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    display_names: value.display_names?,
                    emails: value.emails?,
                    id: value.id?,
                    provider: value.provider?,
                    provider_id: value.provider_id?,
                    updated_at: value.updated_at?,
                    user_id: value.user_id?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserProvider> for ApiUserProvider {
            fn from(value: super::ApiUserProvider) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    display_names: Ok(value.display_names),
                    emails: Ok(value.emails),
                    id: Ok(value.id),
                    provider: Ok(value.provider),
                    provider_id: Ok(value.provider_id),
                    updated_at: Ok(value.updated_at),
                    user_id: Ok(value.user_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserProviderLinkPayload {
            token: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserProviderLinkPayload {
            fn default() -> Self {
                Self {
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl ApiUserProviderLinkPayload {
            pub fn token<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserProviderLinkPayload> for super::ApiUserProviderLinkPayload {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserProviderLinkPayload,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    token: value.token?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserProviderLinkPayload> for ApiUserProviderLinkPayload {
            fn from(value: super::ApiUserProviderLinkPayload) -> Self {
                Self {
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserUpdateParamsForApiPermissions {
            group_ids:
                ::std::result::Result<Vec<super::TypedUuidForAccessGroupId>, ::std::string::String>,
            permissions:
                ::std::result::Result<super::PermissionsForApiPermissions, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserUpdateParamsForApiPermissions {
            fn default() -> Self {
                Self {
                    group_ids: Err("no value supplied for group_ids".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl ApiUserUpdateParamsForApiPermissions {
            pub fn group_ids<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<Vec<super::TypedUuidForAccessGroupId>>,
                T::Error: ::std::fmt::Display,
            {
                self.group_ids = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for group_ids: {e}"));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PermissionsForApiPermissions>,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserUpdateParamsForApiPermissions>
            for super::ApiUserUpdateParamsForApiPermissions
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserUpdateParamsForApiPermissions,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    group_ids: value.group_ids?,
                    permissions: value.permissions?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserUpdateParamsForApiPermissions>
            for ApiUserUpdateParamsForApiPermissions
        {
            fn from(value: super::ApiUserUpdateParamsForApiPermissions) -> Self {
                Self {
                    group_ids: Ok(value.group_ids),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateMapper {
            max_activations:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            rule: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        }

        impl ::std::default::Default for CreateMapper {
            fn default() -> Self {
                Self {
                    max_activations: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    rule: Err("no value supplied for rule".to_string()),
                }
            }
        }

        impl CreateMapper {
            pub fn max_activations<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.max_activations = value.try_into().map_err(|e| {
                    format!("error converting supplied value for max_activations: {e}")
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn rule<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.rule = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rule: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<CreateMapper> for super::CreateMapper {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CreateMapper,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    max_activations: value.max_activations?,
                    name: value.name?,
                    rule: value.rule?,
                })
            }
        }

        impl ::std::convert::From<super::CreateMapper> for CreateMapper {
            fn from(value: super::CreateMapper) -> Self {
                Self {
                    max_activations: Ok(value.max_activations),
                    name: Ok(value.name),
                    rule: Ok(value.rule),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeviceAuthorizationRequest {
            client_id:
                ::std::result::Result<super::TypedUuidForOAuthClientId, ::std::string::String>,
            scope: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for DeviceAuthorizationRequest {
            fn default() -> Self {
                Self {
                    client_id: Err("no value supplied for client_id".to_string()),
                    scope: Ok(Default::default()),
                }
            }
        }

        impl DeviceAuthorizationRequest {
            pub fn client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthClientId>,
                T::Error: ::std::fmt::Display,
            {
                self.client_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for client_id: {e}"));
                self
            }
            pub fn scope<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.scope = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for scope: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<DeviceAuthorizationRequest> for super::DeviceAuthorizationRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DeviceAuthorizationRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    client_id: value.client_id?,
                    scope: value.scope?,
                })
            }
        }

        impl ::std::convert::From<super::DeviceAuthorizationRequest> for DeviceAuthorizationRequest {
            fn from(value: super::DeviceAuthorizationRequest) -> Self {
                Self {
                    client_id: Ok(value.client_id),
                    scope: Ok(value.scope),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeviceTokenExchangeRequest {
            client_id:
                ::std::result::Result<super::TypedUuidForOAuthClientId, ::std::string::String>,
            device_code: ::std::result::Result<::std::string::String, ::std::string::String>,
            grant_type: ::std::result::Result<::std::string::String, ::std::string::String>,
            pkce_verifier: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for DeviceTokenExchangeRequest {
            fn default() -> Self {
                Self {
                    client_id: Err("no value supplied for client_id".to_string()),
                    device_code: Err("no value supplied for device_code".to_string()),
                    grant_type: Err("no value supplied for grant_type".to_string()),
                    pkce_verifier: Ok(Default::default()),
                }
            }
        }

        impl DeviceTokenExchangeRequest {
            pub fn client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthClientId>,
                T::Error: ::std::fmt::Display,
            {
                self.client_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for client_id: {e}"));
                self
            }
            pub fn device_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.device_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for device_code: {e}"));
                self
            }
            pub fn grant_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.grant_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for grant_type: {e}"));
                self
            }
            pub fn pkce_verifier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.pkce_verifier = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pkce_verifier: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<DeviceTokenExchangeRequest> for super::DeviceTokenExchangeRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DeviceTokenExchangeRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    client_id: value.client_id?,
                    device_code: value.device_code?,
                    grant_type: value.grant_type?,
                    pkce_verifier: value.pkce_verifier?,
                })
            }
        }

        impl ::std::convert::From<super::DeviceTokenExchangeRequest> for DeviceTokenExchangeRequest {
            fn from(value: super::DeviceTokenExchangeRequest) -> Self {
                Self {
                    client_id: Ok(value.client_id),
                    device_code: Ok(value.device_code),
                    grant_type: Ok(value.grant_type),
                    pkce_verifier: Ok(value.pkce_verifier),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Error {
            error_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
            request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for Error {
            fn default() -> Self {
                Self {
                    error_code: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                    request_id: Err("no value supplied for request_id".to_string()),
                }
            }
        }

        impl Error {
            pub fn error_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.error_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error_code: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
            pub fn request_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.request_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for request_id: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Error> for super::Error {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Error,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    error_code: value.error_code?,
                    message: value.message?,
                    request_id: value.request_id?,
                })
            }
        }

        impl ::std::convert::From<super::Error> for Error {
            fn from(value: super::Error) -> Self {
                Self {
                    error_code: Ok(value.error_code),
                    message: Ok(value.message),
                    request_id: Ok(value.request_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetUserResponseForApiPermissions {
            info: ::std::result::Result<super::ApiUserForApiPermissions, ::std::string::String>,
            providers: ::std::result::Result<
                ::std::vec::Vec<super::ApiUserProvider>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetUserResponseForApiPermissions {
            fn default() -> Self {
                Self {
                    info: Err("no value supplied for info".to_string()),
                    providers: Err("no value supplied for providers".to_string()),
                }
            }
        }

        impl GetUserResponseForApiPermissions {
            pub fn info<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ApiUserForApiPermissions>,
                T::Error: ::std::fmt::Display,
            {
                self.info = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for info: {e}"));
                self
            }
            pub fn providers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::ApiUserProvider>>,
                T::Error: ::std::fmt::Display,
            {
                self.providers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for providers: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetUserResponseForApiPermissions>
            for super::GetUserResponseForApiPermissions
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetUserResponseForApiPermissions,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    info: value.info?,
                    providers: value.providers?,
                })
            }
        }

        impl ::std::convert::From<super::GetUserResponseForApiPermissions>
            for GetUserResponseForApiPermissions
        {
            fn from(value: super::GetUserResponseForApiPermissions) -> Self {
                Self {
                    info: Ok(value.info),
                    providers: Ok(value.providers),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InitialApiKeyResponseForApiPermissions {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForApiKeyId, ::std::string::String>,
            key: ::std::result::Result<super::SecretString, ::std::string::String>,
            permissions: ::std::result::Result<
                ::std::option::Option<super::PermissionsForApiPermissions>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for InitialApiKeyResponseForApiPermissions {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    key: Err("no value supplied for key".to_string()),
                    permissions: Ok(Default::default()),
                }
            }
        }

        impl InitialApiKeyResponseForApiPermissions {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForApiKeyId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SecretString>,
                T::Error: ::std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {e}"));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::PermissionsForApiPermissions>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<InitialApiKeyResponseForApiPermissions>
            for super::InitialApiKeyResponseForApiPermissions
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InitialApiKeyResponseForApiPermissions,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    key: value.key?,
                    permissions: value.permissions?,
                })
            }
        }

        impl ::std::convert::From<super::InitialApiKeyResponseForApiPermissions>
            for InitialApiKeyResponseForApiPermissions
        {
            fn from(value: super::InitialApiKeyResponseForApiPermissions) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    key: Ok(value.key),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InitialMagicLinkSecretResponse {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForMagicLinkSecretId, ::std::string::String>,
            key: ::std::result::Result<super::SecretString, ::std::string::String>,
        }

        impl ::std::default::Default for InitialMagicLinkSecretResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    key: Err("no value supplied for key".to_string()),
                }
            }
        }

        impl InitialMagicLinkSecretResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkSecretId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SecretString>,
                T::Error: ::std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<InitialMagicLinkSecretResponse>
            for super::InitialMagicLinkSecretResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InitialMagicLinkSecretResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    key: value.key?,
                })
            }
        }

        impl ::std::convert::From<super::InitialMagicLinkSecretResponse>
            for InitialMagicLinkSecretResponse
        {
            fn from(value: super::InitialMagicLinkSecretResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    key: Ok(value.key),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InitialOAuthClientSecretResponse {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForOAuthSecretId, ::std::string::String>,
            key: ::std::result::Result<super::SecretString, ::std::string::String>,
        }

        impl ::std::default::Default for InitialOAuthClientSecretResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    key: Err("no value supplied for key".to_string()),
                }
            }
        }

        impl InitialOAuthClientSecretResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthSecretId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SecretString>,
                T::Error: ::std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<InitialOAuthClientSecretResponse>
            for super::InitialOAuthClientSecretResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InitialOAuthClientSecretResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    key: value.key?,
                })
            }
        }

        impl ::std::convert::From<super::InitialOAuthClientSecretResponse>
            for InitialOAuthClientSecretResponse
        {
            fn from(value: super::InitialOAuthClientSecretResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    key: Ok(value.key),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Jwk {
            e: ::std::result::Result<::std::string::String, ::std::string::String>,
            kid: ::std::result::Result<::std::string::String, ::std::string::String>,
            kty: ::std::result::Result<::std::string::String, ::std::string::String>,
            n: ::std::result::Result<::std::string::String, ::std::string::String>,
            use_: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for Jwk {
            fn default() -> Self {
                Self {
                    e: Err("no value supplied for e".to_string()),
                    kid: Err("no value supplied for kid".to_string()),
                    kty: Err("no value supplied for kty".to_string()),
                    n: Err("no value supplied for n".to_string()),
                    use_: Err("no value supplied for use_".to_string()),
                }
            }
        }

        impl Jwk {
            pub fn e<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.e = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for e: {e}"));
                self
            }
            pub fn kid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.kid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for kid: {e}"));
                self
            }
            pub fn kty<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.kty = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for kty: {e}"));
                self
            }
            pub fn n<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.n = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for n: {e}"));
                self
            }
            pub fn use_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.use_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for use_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Jwk> for super::Jwk {
            type Error = super::error::ConversionError;
            fn try_from(value: Jwk) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    e: value.e?,
                    kid: value.kid?,
                    kty: value.kty?,
                    n: value.n?,
                    use_: value.use_?,
                })
            }
        }

        impl ::std::convert::From<super::Jwk> for Jwk {
            fn from(value: super::Jwk) -> Self {
                Self {
                    e: Ok(value.e),
                    kid: Ok(value.kid),
                    kty: Ok(value.kty),
                    n: Ok(value.n),
                    use_: Ok(value.use_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Jwks {
            keys: ::std::result::Result<::std::vec::Vec<super::Jwk>, ::std::string::String>,
        }

        impl ::std::default::Default for Jwks {
            fn default() -> Self {
                Self {
                    keys: Err("no value supplied for keys".to_string()),
                }
            }
        }

        impl Jwks {
            pub fn keys<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Jwk>>,
                T::Error: ::std::fmt::Display,
            {
                self.keys = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for keys: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Jwks> for super::Jwks {
            type Error = super::error::ConversionError;
            fn try_from(value: Jwks) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self { keys: value.keys? })
            }
        }

        impl ::std::convert::From<super::Jwks> for Jwks {
            fn from(value: super::Jwks) -> Self {
                Self {
                    keys: Ok(value.keys),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLink {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForMagicLinkId, ::std::string::String>,
            redirect_uris: ::std::result::Result<
                ::std::vec::Vec<super::MagicLinkRedirectUri>,
                ::std::string::String,
            >,
            secrets: ::std::result::Result<
                ::std::vec::Vec<super::MagicLinkSecret>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for MagicLink {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    redirect_uris: Err("no value supplied for redirect_uris".to_string()),
                    secrets: Err("no value supplied for secrets".to_string()),
                }
            }
        }

        impl MagicLink {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn redirect_uris<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MagicLinkRedirectUri>>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uris = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for redirect_uris: {e}"));
                self
            }
            pub fn secrets<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MagicLinkSecret>>,
                T::Error: ::std::fmt::Display,
            {
                self.secrets = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secrets: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLink> for super::MagicLink {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLink,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    redirect_uris: value.redirect_uris?,
                    secrets: value.secrets?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLink> for MagicLink {
            fn from(value: super::MagicLink) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    redirect_uris: Ok(value.redirect_uris),
                    secrets: Ok(value.secrets),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkExchangeRequest {
            attempt_id:
                ::std::result::Result<super::TypedUuidForMagicLinkAttemptId, ::std::string::String>,
            recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
            secret: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkExchangeRequest {
            fn default() -> Self {
                Self {
                    attempt_id: Err("no value supplied for attempt_id".to_string()),
                    recipient: Err("no value supplied for recipient".to_string()),
                    secret: Err("no value supplied for secret".to_string()),
                }
            }
        }

        impl MagicLinkExchangeRequest {
            pub fn attempt_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkAttemptId>,
                T::Error: ::std::fmt::Display,
            {
                self.attempt_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for attempt_id: {e}"));
                self
            }
            pub fn recipient<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.recipient = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for recipient: {e}"));
                self
            }
            pub fn secret<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.secret = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secret: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkExchangeRequest> for super::MagicLinkExchangeRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkExchangeRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    attempt_id: value.attempt_id?,
                    recipient: value.recipient?,
                    secret: value.secret?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkExchangeRequest> for MagicLinkExchangeRequest {
            fn from(value: super::MagicLinkExchangeRequest) -> Self {
                Self {
                    attempt_id: Ok(value.attempt_id),
                    recipient: Ok(value.recipient),
                    secret: Ok(value.secret),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkExchangeResponse {
            access_token: ::std::result::Result<::std::string::String, ::std::string::String>,
            expires_in: ::std::result::Result<i64, ::std::string::String>,
            token_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkExchangeResponse {
            fn default() -> Self {
                Self {
                    access_token: Err("no value supplied for access_token".to_string()),
                    expires_in: Err("no value supplied for expires_in".to_string()),
                    token_type: Err("no value supplied for token_type".to_string()),
                }
            }
        }

        impl MagicLinkExchangeResponse {
            pub fn access_token<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.access_token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for access_token: {e}"));
                self
            }
            pub fn expires_in<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i64>,
                T::Error: ::std::fmt::Display,
            {
                self.expires_in = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_in: {e}"));
                self
            }
            pub fn token_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.token_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token_type: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkExchangeResponse> for super::MagicLinkExchangeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkExchangeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    access_token: value.access_token?,
                    expires_in: value.expires_in?,
                    token_type: value.token_type?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkExchangeResponse> for MagicLinkExchangeResponse {
            fn from(value: super::MagicLinkExchangeResponse) -> Self {
                Self {
                    access_token: Ok(value.access_token),
                    expires_in: Ok(value.expires_in),
                    token_type: Ok(value.token_type),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkRedirectUri {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                super::TypedUuidForMagicLinkRedirectUriId,
                ::std::string::String,
            >,
            magic_link_client_id:
                ::std::result::Result<super::TypedUuidForMagicLinkId, ::std::string::String>,
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkRedirectUri {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    magic_link_client_id: Err(
                        "no value supplied for magic_link_client_id".to_string()
                    ),
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl MagicLinkRedirectUri {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkRedirectUriId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn magic_link_client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkId>,
                T::Error: ::std::fmt::Display,
            {
                self.magic_link_client_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for magic_link_client_id: {e}")
                });
                self
            }
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for redirect_uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkRedirectUri> for super::MagicLinkRedirectUri {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkRedirectUri,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    magic_link_client_id: value.magic_link_client_id?,
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkRedirectUri> for MagicLinkRedirectUri {
            fn from(value: super::MagicLinkRedirectUri) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    magic_link_client_id: Ok(value.magic_link_client_id),
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkSecret {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForMagicLinkSecretId, ::std::string::String>,
            magic_link_client_id:
                ::std::result::Result<super::TypedUuidForMagicLinkId, ::std::string::String>,
            secret_signature: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkSecret {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    magic_link_client_id: Err(
                        "no value supplied for magic_link_client_id".to_string()
                    ),
                    secret_signature: Err("no value supplied for secret_signature".to_string()),
                }
            }
        }

        impl MagicLinkSecret {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkSecretId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn magic_link_client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkId>,
                T::Error: ::std::fmt::Display,
            {
                self.magic_link_client_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for magic_link_client_id: {e}")
                });
                self
            }
            pub fn secret_signature<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.secret_signature = value.try_into().map_err(|e| {
                    format!("error converting supplied value for secret_signature: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkSecret> for super::MagicLinkSecret {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkSecret,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    magic_link_client_id: value.magic_link_client_id?,
                    secret_signature: value.secret_signature?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkSecret> for MagicLinkSecret {
            fn from(value: super::MagicLinkSecret) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    magic_link_client_id: Ok(value.magic_link_client_id),
                    secret_signature: Ok(value.secret_signature),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkSendRequest {
            expires_in: ::std::result::Result<i64, ::std::string::String>,
            medium: ::std::result::Result<super::MagicLinkMedium, ::std::string::String>,
            recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
            scope: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            secret: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkSendRequest {
            fn default() -> Self {
                Self {
                    expires_in: Err("no value supplied for expires_in".to_string()),
                    medium: Err("no value supplied for medium".to_string()),
                    recipient: Err("no value supplied for recipient".to_string()),
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                    scope: Ok(Default::default()),
                    secret: Err("no value supplied for secret".to_string()),
                }
            }
        }

        impl MagicLinkSendRequest {
            pub fn expires_in<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i64>,
                T::Error: ::std::fmt::Display,
            {
                self.expires_in = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_in: {e}"));
                self
            }
            pub fn medium<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MagicLinkMedium>,
                T::Error: ::std::fmt::Display,
            {
                self.medium = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for medium: {e}"));
                self
            }
            pub fn recipient<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.recipient = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for recipient: {e}"));
                self
            }
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for redirect_uri: {e}"));
                self
            }
            pub fn scope<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.scope = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for scope: {e}"));
                self
            }
            pub fn secret<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.secret = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secret: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkSendRequest> for super::MagicLinkSendRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkSendRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    expires_in: value.expires_in?,
                    medium: value.medium?,
                    recipient: value.recipient?,
                    redirect_uri: value.redirect_uri?,
                    scope: value.scope?,
                    secret: value.secret?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkSendRequest> for MagicLinkSendRequest {
            fn from(value: super::MagicLinkSendRequest) -> Self {
                Self {
                    expires_in: Ok(value.expires_in),
                    medium: Ok(value.medium),
                    recipient: Ok(value.recipient),
                    redirect_uri: Ok(value.redirect_uri),
                    scope: Ok(value.scope),
                    secret: Ok(value.secret),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkSendResponse {
            attempt_id:
                ::std::result::Result<super::TypedUuidForMagicLinkAttemptId, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkSendResponse {
            fn default() -> Self {
                Self {
                    attempt_id: Err("no value supplied for attempt_id".to_string()),
                }
            }
        }

        impl MagicLinkSendResponse {
            pub fn attempt_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkAttemptId>,
                T::Error: ::std::fmt::Display,
            {
                self.attempt_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for attempt_id: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkSendResponse> for super::MagicLinkSendResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkSendResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    attempt_id: value.attempt_id?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkSendResponse> for MagicLinkSendResponse {
            fn from(value: super::MagicLinkSendResponse) -> Self {
                Self {
                    attempt_id: Ok(value.attempt_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Mapper {
            activations: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            depleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForMapperId, ::std::string::String>,
            max_activations:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            rule: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Mapper {
            fn default() -> Self {
                Self {
                    activations: Ok(Default::default()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    depleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    max_activations: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    rule: Err("no value supplied for rule".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                }
            }
        }

        impl Mapper {
            pub fn activations<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.activations = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for activations: {e}"));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn depleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.depleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for depleted_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMapperId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn max_activations<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.max_activations = value.try_into().map_err(|e| {
                    format!("error converting supplied value for max_activations: {e}")
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
            pub fn rule<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.rule = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rule: {e}"));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Mapper> for super::Mapper {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Mapper,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    activations: value.activations?,
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    depleted_at: value.depleted_at?,
                    id: value.id?,
                    max_activations: value.max_activations?,
                    name: value.name?,
                    rule: value.rule?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl ::std::convert::From<super::Mapper> for Mapper {
            fn from(value: super::Mapper) -> Self {
                Self {
                    activations: Ok(value.activations),
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    depleted_at: Ok(value.depleted_at),
                    id: Ok(value.id),
                    max_activations: Ok(value.max_activations),
                    name: Ok(value.name),
                    rule: Ok(value.rule),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthAuthzCodeExchangeBody {
            client_id: ::std::result::Result<
                ::std::option::Option<super::TypedUuidForOAuthClientId>,
                ::std::string::String,
            >,
            client_secret: ::std::result::Result<
                ::std::option::Option<super::SecretString>,
                ::std::string::String,
            >,
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            grant_type: ::std::result::Result<::std::string::String, ::std::string::String>,
            pkce_verifier: ::std::result::Result<::std::string::String, ::std::string::String>,
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthAuthzCodeExchangeBody {
            fn default() -> Self {
                Self {
                    client_id: Ok(Default::default()),
                    client_secret: Ok(Default::default()),
                    code: Err("no value supplied for code".to_string()),
                    grant_type: Err("no value supplied for grant_type".to_string()),
                    pkce_verifier: Err("no value supplied for pkce_verifier".to_string()),
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl OAuthAuthzCodeExchangeBody {
            pub fn client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TypedUuidForOAuthClientId>>,
                T::Error: ::std::fmt::Display,
            {
                self.client_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for client_id: {e}"));
                self
            }
            pub fn client_secret<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::SecretString>>,
                T::Error: ::std::fmt::Display,
            {
                self.client_secret = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for client_secret: {e}"));
                self
            }
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {e}"));
                self
            }
            pub fn grant_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.grant_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for grant_type: {e}"));
                self
            }
            pub fn pkce_verifier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.pkce_verifier = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pkce_verifier: {e}"));
                self
            }
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for redirect_uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthAuthzCodeExchangeBody> for super::OAuthAuthzCodeExchangeBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthAuthzCodeExchangeBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    client_id: value.client_id?,
                    client_secret: value.client_secret?,
                    code: value.code?,
                    grant_type: value.grant_type?,
                    pkce_verifier: value.pkce_verifier?,
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthAuthzCodeExchangeBody> for OAuthAuthzCodeExchangeBody {
            fn from(value: super::OAuthAuthzCodeExchangeBody) -> Self {
                Self {
                    client_id: Ok(value.client_id),
                    client_secret: Ok(value.client_secret),
                    code: Ok(value.code),
                    grant_type: Ok(value.grant_type),
                    pkce_verifier: Ok(value.pkce_verifier),
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthAuthzCodeExchangeResponse {
            access_token: ::std::result::Result<::std::string::String, ::std::string::String>,
            expires_in: ::std::result::Result<i64, ::std::string::String>,
            idp_token: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            scope: ::std::result::Result<::std::string::String, ::std::string::String>,
            token_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthAuthzCodeExchangeResponse {
            fn default() -> Self {
                Self {
                    access_token: Err("no value supplied for access_token".to_string()),
                    expires_in: Err("no value supplied for expires_in".to_string()),
                    idp_token: Ok(Default::default()),
                    scope: Err("no value supplied for scope".to_string()),
                    token_type: Err("no value supplied for token_type".to_string()),
                }
            }
        }

        impl OAuthAuthzCodeExchangeResponse {
            pub fn access_token<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.access_token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for access_token: {e}"));
                self
            }
            pub fn expires_in<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i64>,
                T::Error: ::std::fmt::Display,
            {
                self.expires_in = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_in: {e}"));
                self
            }
            pub fn idp_token<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.idp_token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for idp_token: {e}"));
                self
            }
            pub fn scope<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.scope = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for scope: {e}"));
                self
            }
            pub fn token_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.token_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token_type: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthAuthzCodeExchangeResponse>
            for super::OAuthAuthzCodeExchangeResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthAuthzCodeExchangeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    access_token: value.access_token?,
                    expires_in: value.expires_in?,
                    idp_token: value.idp_token?,
                    scope: value.scope?,
                    token_type: value.token_type?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthAuthzCodeExchangeResponse>
            for OAuthAuthzCodeExchangeResponse
        {
            fn from(value: super::OAuthAuthzCodeExchangeResponse) -> Self {
                Self {
                    access_token: Ok(value.access_token),
                    expires_in: Ok(value.expires_in),
                    idp_token: Ok(value.idp_token),
                    scope: Ok(value.scope),
                    token_type: Ok(value.token_type),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthClient {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForOAuthClientId, ::std::string::String>,
            redirect_uris: ::std::result::Result<
                ::std::vec::Vec<super::OAuthClientRedirectUri>,
                ::std::string::String,
            >,
            secrets: ::std::result::Result<
                ::std::vec::Vec<super::OAuthClientSecret>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for OAuthClient {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    redirect_uris: Err("no value supplied for redirect_uris".to_string()),
                    secrets: Err("no value supplied for secrets".to_string()),
                }
            }
        }

        impl OAuthClient {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthClientId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn redirect_uris<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OAuthClientRedirectUri>>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uris = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for redirect_uris: {e}"));
                self
            }
            pub fn secrets<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OAuthClientSecret>>,
                T::Error: ::std::fmt::Display,
            {
                self.secrets = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secrets: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthClient> for super::OAuthClient {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthClient,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    redirect_uris: value.redirect_uris?,
                    secrets: value.secrets?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthClient> for OAuthClient {
            fn from(value: super::OAuthClient) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    redirect_uris: Ok(value.redirect_uris),
                    secrets: Ok(value.secrets),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthClientRedirectUri {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForOAuthRedirectUriId, ::std::string::String>,
            oauth_client_id:
                ::std::result::Result<super::TypedUuidForOAuthClientId, ::std::string::String>,
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthClientRedirectUri {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    oauth_client_id: Err("no value supplied for oauth_client_id".to_string()),
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl OAuthClientRedirectUri {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthRedirectUriId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn oauth_client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthClientId>,
                T::Error: ::std::fmt::Display,
            {
                self.oauth_client_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for oauth_client_id: {e}")
                });
                self
            }
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for redirect_uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthClientRedirectUri> for super::OAuthClientRedirectUri {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthClientRedirectUri,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    oauth_client_id: value.oauth_client_id?,
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthClientRedirectUri> for OAuthClientRedirectUri {
            fn from(value: super::OAuthClientRedirectUri) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    oauth_client_id: Ok(value.oauth_client_id),
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthClientSecret {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForOAuthSecretId, ::std::string::String>,
            oauth_client_id:
                ::std::result::Result<super::TypedUuidForOAuthClientId, ::std::string::String>,
            secret_signature: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthClientSecret {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    oauth_client_id: Err("no value supplied for oauth_client_id".to_string()),
                    secret_signature: Err("no value supplied for secret_signature".to_string()),
                }
            }
        }

        impl OAuthClientSecret {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {e}"));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthSecretId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn oauth_client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthClientId>,
                T::Error: ::std::fmt::Display,
            {
                self.oauth_client_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for oauth_client_id: {e}")
                });
                self
            }
            pub fn secret_signature<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.secret_signature = value.try_into().map_err(|e| {
                    format!("error converting supplied value for secret_signature: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthClientSecret> for super::OAuthClientSecret {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthClientSecret,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    oauth_client_id: value.oauth_client_id?,
                    secret_signature: value.secret_signature?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthClientSecret> for OAuthClientSecret {
            fn from(value: super::OAuthClientSecret) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    oauth_client_id: Ok(value.oauth_client_id),
                    secret_signature: Ok(value.secret_signature),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthProviderAuthorizationCodeInfo {
            auth_url_endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
            redirect_endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
            token_endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
            token_endpoint_content_type:
                ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthProviderAuthorizationCodeInfo {
            fn default() -> Self {
                Self {
                    auth_url_endpoint: Err("no value supplied for auth_url_endpoint".to_string()),
                    redirect_endpoint: Err("no value supplied for redirect_endpoint".to_string()),
                    token_endpoint: Err("no value supplied for token_endpoint".to_string()),
                    token_endpoint_content_type: Err("no value supplied for \
                                                      token_endpoint_content_type"
                        .to_string()),
                }
            }
        }

        impl OAuthProviderAuthorizationCodeInfo {
            pub fn auth_url_endpoint<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.auth_url_endpoint = value.try_into().map_err(|e| {
                    format!("error converting supplied value for auth_url_endpoint: {e}")
                });
                self
            }
            pub fn redirect_endpoint<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_endpoint = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_endpoint: {e}")
                });
                self
            }
            pub fn token_endpoint<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.token_endpoint = value.try_into().map_err(|e| {
                    format!("error converting supplied value for token_endpoint: {e}")
                });
                self
            }
            pub fn token_endpoint_content_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.token_endpoint_content_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for token_endpoint_content_type: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthProviderAuthorizationCodeInfo>
            for super::OAuthProviderAuthorizationCodeInfo
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthProviderAuthorizationCodeInfo,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    auth_url_endpoint: value.auth_url_endpoint?,
                    redirect_endpoint: value.redirect_endpoint?,
                    token_endpoint: value.token_endpoint?,
                    token_endpoint_content_type: value.token_endpoint_content_type?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthProviderAuthorizationCodeInfo>
            for OAuthProviderAuthorizationCodeInfo
        {
            fn from(value: super::OAuthProviderAuthorizationCodeInfo) -> Self {
                Self {
                    auth_url_endpoint: Ok(value.auth_url_endpoint),
                    redirect_endpoint: Ok(value.redirect_endpoint),
                    token_endpoint: Ok(value.token_endpoint),
                    token_endpoint_content_type: Ok(value.token_endpoint_content_type),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthProviderAuthorizationCodePkceInfo {
            client_id:
                ::std::result::Result<super::TypedUuidForOAuthClientId, ::std::string::String>,
            proxy_port: ::std::result::Result<u16, ::std::string::String>,
            redirect_endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
            web: ::std::result::Result<
                super::OAuthProviderAuthorizationCodeInfo,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for OAuthProviderAuthorizationCodePkceInfo {
            fn default() -> Self {
                Self {
                    client_id: Err("no value supplied for client_id".to_string()),
                    proxy_port: Err("no value supplied for proxy_port".to_string()),
                    redirect_endpoint: Err("no value supplied for redirect_endpoint".to_string()),
                    web: Err("no value supplied for web".to_string()),
                }
            }
        }

        impl OAuthProviderAuthorizationCodePkceInfo {
            pub fn client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthClientId>,
                T::Error: ::std::fmt::Display,
            {
                self.client_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for client_id: {e}"));
                self
            }
            pub fn proxy_port<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u16>,
                T::Error: ::std::fmt::Display,
            {
                self.proxy_port = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for proxy_port: {e}"));
                self
            }
            pub fn redirect_endpoint<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_endpoint = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_endpoint: {e}")
                });
                self
            }
            pub fn web<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::OAuthProviderAuthorizationCodeInfo>,
                T::Error: ::std::fmt::Display,
            {
                self.web = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthProviderAuthorizationCodePkceInfo>
            for super::OAuthProviderAuthorizationCodePkceInfo
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthProviderAuthorizationCodePkceInfo,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    client_id: value.client_id?,
                    proxy_port: value.proxy_port?,
                    redirect_endpoint: value.redirect_endpoint?,
                    web: value.web?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthProviderAuthorizationCodePkceInfo>
            for OAuthProviderAuthorizationCodePkceInfo
        {
            fn from(value: super::OAuthProviderAuthorizationCodePkceInfo) -> Self {
                Self {
                    client_id: Ok(value.client_id),
                    proxy_port: Ok(value.proxy_port),
                    redirect_endpoint: Ok(value.redirect_endpoint),
                    web: Ok(value.web),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthProviderDeviceInfo {
            auth_url_endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
            client_id:
                ::std::result::Result<super::TypedUuidForOAuthClientId, ::std::string::String>,
            token_endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthProviderDeviceInfo {
            fn default() -> Self {
                Self {
                    auth_url_endpoint: Err("no value supplied for auth_url_endpoint".to_string()),
                    client_id: Err("no value supplied for client_id".to_string()),
                    token_endpoint: Err("no value supplied for token_endpoint".to_string()),
                }
            }
        }

        impl OAuthProviderDeviceInfo {
            pub fn auth_url_endpoint<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.auth_url_endpoint = value.try_into().map_err(|e| {
                    format!("error converting supplied value for auth_url_endpoint: {e}")
                });
                self
            }
            pub fn client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthClientId>,
                T::Error: ::std::fmt::Display,
            {
                self.client_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for client_id: {e}"));
                self
            }
            pub fn token_endpoint<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.token_endpoint = value.try_into().map_err(|e| {
                    format!("error converting supplied value for token_endpoint: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthProviderDeviceInfo> for super::OAuthProviderDeviceInfo {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthProviderDeviceInfo,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    auth_url_endpoint: value.auth_url_endpoint?,
                    client_id: value.client_id?,
                    token_endpoint: value.token_endpoint?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthProviderDeviceInfo> for OAuthProviderDeviceInfo {
            fn from(value: super::OAuthProviderDeviceInfo) -> Self {
                Self {
                    auth_url_endpoint: Ok(value.auth_url_endpoint),
                    client_id: Ok(value.client_id),
                    token_endpoint: Ok(value.token_endpoint),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OpenIdConfiguration {
            jwks_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OpenIdConfiguration {
            fn default() -> Self {
                Self {
                    jwks_uri: Err("no value supplied for jwks_uri".to_string()),
                }
            }
        }

        impl OpenIdConfiguration {
            pub fn jwks_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.jwks_uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for jwks_uri: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OpenIdConfiguration> for super::OpenIdConfiguration {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OpenIdConfiguration,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    jwks_uri: value.jwks_uri?,
                })
            }
        }

        impl ::std::convert::From<super::OpenIdConfiguration> for OpenIdConfiguration {
            fn from(value: super::OpenIdConfiguration) -> Self {
                Self {
                    jwks_uri: Ok(value.jwks_uri),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
/// Client for Customer Support API
///
/// Programmatic access to the Oxide customer support API.
///
/// Version: 0.1.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "0.1.0"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}

impl Client {
    /// Sends a `GET` request to `/.well-known/jwks.json`
    ///
    /// ```ignore
    /// let response = client.jwks_json()
    ///    .send()
    ///    .await;
    /// ```
    pub fn jwks_json(&self) -> builder::JwksJson<'_> {
        builder::JwksJson::new(self)
    }

    /// Sends a `GET` request to `/.well-known/openid-configuration`
    ///
    /// ```ignore
    /// let response = client.openid_configuration()
    ///    .send()
    ///    .await;
    /// ```
    pub fn openid_configuration(&self) -> builder::OpenidConfiguration<'_> {
        builder::OpenidConfiguration::new(self)
    }

    /// List details for users
    ///
    /// Sends a `GET` request to `/api-user`
    ///
    /// ```ignore
    /// let response = client.list_api_users()
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_api_users(&self) -> builder::ListApiUsers<'_> {
        builder::ListApiUsers::new(self)
    }

    /// Create a new user
    ///
    /// Sends a `POST` request to `/api-user`
    ///
    /// ```ignore
    /// let response = client.create_api_user()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_api_user(&self) -> builder::CreateApiUser<'_> {
        builder::CreateApiUser::new(self)
    }

    /// View details for a user
    ///
    /// Sends a `GET` request to `/api-user/{user_id}`
    ///
    /// ```ignore
    /// let response = client.get_api_user()
    ///    .user_id(user_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_api_user(&self) -> builder::GetApiUser<'_> {
        builder::GetApiUser::new(self)
    }

    /// Update the permissions assigned to a given user
    ///
    /// Sends a `POST` request to `/api-user/{user_id}`
    ///
    /// ```ignore
    /// let response = client.update_api_user()
    ///    .user_id(user_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_api_user(&self) -> builder::UpdateApiUser<'_> {
        builder::UpdateApiUser::new(self)
    }

    /// Set the contact email for a user
    ///
    /// Sends a `PUT` request to `/api-user/{user_id}/contact/email`
    ///
    /// ```ignore
    /// let response = client.set_api_user_contact_email()
    ///    .user_id(user_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn set_api_user_contact_email(&self) -> builder::SetApiUserContactEmail<'_> {
        builder::SetApiUserContactEmail::new(self)
    }

    /// Add a user to a group
    ///
    /// Sends a `POST` request to `/api-user/{user_id}/group`
    ///
    /// ```ignore
    /// let response = client.add_api_user_to_group()
    ///    .user_id(user_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn add_api_user_to_group(&self) -> builder::AddApiUserToGroup<'_> {
        builder::AddApiUserToGroup::new(self)
    }

    /// Remove a user from a group
    ///
    /// Sends a `DELETE` request to `/api-user/{user_id}/group/{group_id}`
    ///
    /// ```ignore
    /// let response = client.remove_api_user_from_group()
    ///    .user_id(user_id)
    ///    .group_id(group_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn remove_api_user_from_group(&self) -> builder::RemoveApiUserFromGroup<'_> {
        builder::RemoveApiUserFromGroup::new(self)
    }

    /// Link an existing login provider to this user
    ///
    /// Sends a `POST` request to `/api-user/{user_id}/link`
    ///
    /// ```ignore
    /// let response = client.link_provider()
    ///    .user_id(user_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn link_provider(&self) -> builder::LinkProvider<'_> {
        builder::LinkProvider::new(self)
    }

    /// List api keys for a user
    ///
    /// Sends a `GET` request to `/api-user/{user_id}/token`
    ///
    /// ```ignore
    /// let response = client.list_api_user_tokens()
    ///    .user_id(user_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_api_user_tokens(&self) -> builder::ListApiUserTokens<'_> {
        builder::ListApiUserTokens::new(self)
    }

    /// Create a new api key for a user
    ///
    /// Sends a `POST` request to `/api-user/{user_id}/token`
    ///
    /// ```ignore
    /// let response = client.create_api_user_token()
    ///    .user_id(user_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_api_user_token(&self) -> builder::CreateApiUserToken<'_> {
        builder::CreateApiUserToken::new(self)
    }

    /// View details of an api key for a user
    ///
    /// Sends a `GET` request to `/api-user/{user_id}/token/{api_key_id}`
    ///
    /// ```ignore
    /// let response = client.get_api_user_token()
    ///    .user_id(user_id)
    ///    .api_key_id(api_key_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_api_user_token(&self) -> builder::GetApiUserToken<'_> {
        builder::GetApiUserToken::new(self)
    }

    /// Revoke an api key for a user
    ///
    /// Sends a `DELETE` request to `/api-user/{user_id}/token/{api_key_id}`
    ///
    /// ```ignore
    /// let response = client.delete_api_user_token()
    ///    .user_id(user_id)
    ///    .api_key_id(api_key_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_api_user_token(&self) -> builder::DeleteApiUserToken<'_> {
        builder::DeleteApiUserToken::new(self)
    }

    /// Create a new link token for linking this provider to a different api
    /// user
    ///
    /// Sends a `POST` request to `/api-user-provider/{provider_id}/link-token`
    ///
    /// ```ignore
    /// let response = client.create_link_token()
    ///    .provider_id(provider_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_link_token(&self) -> builder::CreateLinkToken<'_> {
        builder::CreateLinkToken::new(self)
    }

    /// List all groups
    ///
    /// Sends a `GET` request to `/group`
    ///
    /// ```ignore
    /// let response = client.get_groups()
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_groups(&self) -> builder::GetGroups<'_> {
        builder::GetGroups::new(self)
    }

    /// Create a group
    ///
    /// Sends a `POST` request to `/group`
    ///
    /// ```ignore
    /// let response = client.create_group()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_group(&self) -> builder::CreateGroup<'_> {
        builder::CreateGroup::new(self)
    }

    /// Update a group
    ///
    /// Sends a `PUT` request to `/group/{group_id}`
    ///
    /// ```ignore
    /// let response = client.update_group()
    ///    .group_id(group_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_group(&self) -> builder::UpdateGroup<'_> {
        builder::UpdateGroup::new(self)
    }

    /// Delete a group
    ///
    /// Sends a `DELETE` request to `/group/{group_id}`
    ///
    /// ```ignore
    /// let response = client.delete_group()
    ///    .group_id(group_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_group(&self) -> builder::DeleteGroup<'_> {
        builder::DeleteGroup::new(self)
    }

    /// Get members of a group
    ///
    /// Sends a `GET` request to `/group-membership/{group_id}`
    ///
    /// ```ignore
    /// let response = client.get_group_members()
    ///    .group_id(group_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_group_members(&self) -> builder::GetGroupMembers<'_> {
        builder::GetGroupMembers::new(self)
    }

    /// Exchange a magic link access code for an access token
    ///
    /// Sends a `POST` request to `/login/magic/{channel}/exchange`
    ///
    /// ```ignore
    /// let response = client.magic_link_exchange()
    ///    .channel(channel)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn magic_link_exchange(&self) -> builder::MagicLinkExchange<'_> {
        builder::MagicLinkExchange::new(self)
    }

    /// Send a new magic link authentication link
    ///
    /// Sends a `POST` request to `/login/magic/{channel}/send`
    ///
    /// ```ignore
    /// let response = client.magic_link_send()
    ///    .channel(channel)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn magic_link_send(&self) -> builder::MagicLinkSend<'_> {
        builder::MagicLinkSend::new(self)
    }

    /// Generate the remote provider login url and redirect the user
    ///
    /// Sends a `GET` request to `/login/oauth/{provider}/code/authorize`
    ///
    /// Arguments:
    /// - `provider`
    /// - `client_id`
    /// - `code_challenge`: PKCE code challenge (RFC 7636). Required for all
    ///   authorization code flows.
    /// - `code_challenge_method`: PKCE code challenge method. Must be "S256".
    /// - `redirect_uri`
    /// - `response_type`
    /// - `scope`
    /// - `state`
    /// ```ignore
    /// let response = client.authz_code_redirect()
    ///    .provider(provider)
    ///    .client_id(client_id)
    ///    .code_challenge(code_challenge)
    ///    .code_challenge_method(code_challenge_method)
    ///    .redirect_uri(redirect_uri)
    ///    .response_type(response_type)
    ///    .scope(scope)
    ///    .state(state)
    ///    .send()
    ///    .await;
    /// ```
    pub fn authz_code_redirect(&self) -> builder::AuthzCodeRedirect<'_> {
        builder::AuthzCodeRedirect::new(self)
    }

    /// Handle return calls from a remote OAuth provider
    ///
    /// Sends a `GET` request to `/login/oauth/{provider}/code/callback`
    ///
    /// ```ignore
    /// let response = client.authz_code_callback()
    ///    .provider(provider)
    ///    .code(code)
    ///    .error(error)
    ///    .state(state)
    ///    .send()
    ///    .await;
    /// ```
    pub fn authz_code_callback(&self) -> builder::AuthzCodeCallback<'_> {
        builder::AuthzCodeCallback::new(self)
    }

    /// Exchange an authorization code for an access token
    ///
    /// Sends a `POST` request to `/login/oauth/{provider}/code/token`
    ///
    /// ```ignore
    /// let response = client.authz_code_exchange()
    ///    .provider(provider)
    ///    .request_idp_token(request_idp_token)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn authz_code_exchange(&self) -> builder::AuthzCodeExchange<'_> {
        builder::AuthzCodeExchange::new(self)
    }

    /// Retrieve the metadata about an OAuth provider for device authorization
    /// flow
    ///
    /// Sends a `GET` request to `/login/oauth/{provider}/device`
    ///
    /// ```ignore
    /// let response = client.get_device_provider()
    ///    .provider(provider)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_device_provider(&self) -> builder::GetDeviceProvider<'_> {
        builder::GetDeviceProvider::new(self)
    }

    /// Initiate a device authorization flow by proxying the request to the
    ///
    /// upstream OAuth provider. Creates a login attempt and returns the
    /// upstream device authorization response.
    ///
    /// Sends a `POST` request to `/login/oauth/{provider}/device`
    ///
    /// ```ignore
    /// let response = client.device_authz()
    ///    .provider(provider)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn device_authz(&self) -> builder::DeviceAuthz<'_> {
        builder::DeviceAuthz::new(self)
    }

    /// Exchange an OAuth device code for an access token. The client polls
    ///
    /// this endpoint until the user completes authorization.
    ///
    /// Sends a `POST` request to `/login/oauth/{provider}/device/exchange`
    ///
    /// ```ignore
    /// let response = client.exchange_device_token()
    ///    .provider(provider)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn exchange_device_token(&self) -> builder::ExchangeDeviceToken<'_> {
        builder::ExchangeDeviceToken::new(self)
    }

    /// Retrieve the metadata about an OAuth provider for public PKCE
    /// authorization code flow
    ///
    /// Sends a `GET` request to `/login/oauth/{provider}/public-pkce`
    ///
    /// ```ignore
    /// let response = client.get_web_pkce_provider()
    ///    .provider(provider)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_web_pkce_provider(&self) -> builder::GetWebPkceProvider<'_> {
        builder::GetWebPkceProvider::new(self)
    }

    /// List Magic Link clients
    ///
    /// Sends a `GET` request to `/magic/client`
    ///
    /// ```ignore
    /// let response = client.list_magic_links()
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_magic_links(&self) -> builder::ListMagicLinks<'_> {
        builder::ListMagicLinks::new(self)
    }

    /// Create a new Magic Link Client
    ///
    /// Sends a `POST` request to `/magic/client`
    ///
    /// ```ignore
    /// let response = client.create_magic_link()
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_magic_link(&self) -> builder::CreateMagicLink<'_> {
        builder::CreateMagicLink::new(self)
    }

    /// Get a Magic Link Client
    ///
    /// Sends a `GET` request to `/magic/client/{client_id}`
    ///
    /// ```ignore
    /// let response = client.get_magic_link()
    ///    .client_id(client_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_magic_link(&self) -> builder::GetMagicLink<'_> {
        builder::GetMagicLink::new(self)
    }

    /// Add a Magic Link client redirect uri
    ///
    /// Sends a `POST` request to `/magic/client/{client_id}/redirect_uri`
    ///
    /// ```ignore
    /// let response = client.create_magic_link_redirect_uri()
    ///    .client_id(client_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_magic_link_redirect_uri(&self) -> builder::CreateMagicLinkRedirectUri<'_> {
        builder::CreateMagicLinkRedirectUri::new(self)
    }

    /// Delete a Magic Link client redirect uri
    ///
    /// Sends a `DELETE` request to
    /// `/magic/client/{client_id}/redirect_uri/{redirect_uri_id}`
    ///
    /// ```ignore
    /// let response = client.delete_magic_link_redirect_uri()
    ///    .client_id(client_id)
    ///    .redirect_uri_id(redirect_uri_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_magic_link_redirect_uri(&self) -> builder::DeleteMagicLinkRedirectUri<'_> {
        builder::DeleteMagicLinkRedirectUri::new(self)
    }

    /// Add a Magic Link client secret
    ///
    /// Sends a `POST` request to `/magic/client/{client_id}/secret`
    ///
    /// ```ignore
    /// let response = client.create_magic_link_secret()
    ///    .client_id(client_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_magic_link_secret(&self) -> builder::CreateMagicLinkSecret<'_> {
        builder::CreateMagicLinkSecret::new(self)
    }

    /// Delete a Magic Link client secret
    ///
    /// Sends a `DELETE` request to
    /// `/magic/client/{client_id}/secret/{secret_id}`
    ///
    /// ```ignore
    /// let response = client.delete_magic_link_secret()
    ///    .client_id(client_id)
    ///    .secret_id(secret_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_magic_link_secret(&self) -> builder::DeleteMagicLinkSecret<'_> {
        builder::DeleteMagicLinkSecret::new(self)
    }

    /// List all mappers
    ///
    /// Sends a `GET` request to `/mapper`
    ///
    /// Arguments:
    /// - `include_depleted`: Include depleted mappers in the returned results
    /// ```ignore
    /// let response = client.get_mappers()
    ///    .include_depleted(include_depleted)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_mappers(&self) -> builder::GetMappers<'_> {
        builder::GetMappers::new(self)
    }

    /// Create a mapper
    ///
    /// Sends a `POST` request to `/mapper`
    ///
    /// ```ignore
    /// let response = client.create_mapper()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_mapper(&self) -> builder::CreateMapper<'_> {
        builder::CreateMapper::new(self)
    }

    /// Delete a mapper
    ///
    /// Sends a `DELETE` request to `/mapper/{mapper_id}`
    ///
    /// ```ignore
    /// let response = client.delete_mapper()
    ///    .mapper_id(mapper_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_mapper(&self) -> builder::DeleteMapper<'_> {
        builder::DeleteMapper::new(self)
    }

    /// List OAuth clients
    ///
    /// Sends a `GET` request to `/oauth/client`
    ///
    /// ```ignore
    /// let response = client.list_oauth_clients()
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_oauth_clients(&self) -> builder::ListOauthClients<'_> {
        builder::ListOauthClients::new(self)
    }

    /// Create a new OAuth Client
    ///
    /// Sends a `POST` request to `/oauth/client`
    ///
    /// ```ignore
    /// let response = client.create_oauth_client()
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_oauth_client(&self) -> builder::CreateOauthClient<'_> {
        builder::CreateOauthClient::new(self)
    }

    /// Get an new OAuth Client
    ///
    /// Sends a `GET` request to `/oauth/client/{client_id}`
    ///
    /// ```ignore
    /// let response = client.get_oauth_client()
    ///    .client_id(client_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_oauth_client(&self) -> builder::GetOauthClient<'_> {
        builder::GetOauthClient::new(self)
    }

    /// Add an OAuth client redirect uri
    ///
    /// Sends a `POST` request to `/oauth/client/{client_id}/redirect_uri`
    ///
    /// ```ignore
    /// let response = client.create_oauth_client_redirect_uri()
    ///    .client_id(client_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_oauth_client_redirect_uri(&self) -> builder::CreateOauthClientRedirectUri<'_> {
        builder::CreateOauthClientRedirectUri::new(self)
    }

    /// Delete an OAuth client redirect uri
    ///
    /// Sends a `DELETE` request to
    /// `/oauth/client/{client_id}/redirect_uri/{redirect_uri_id}`
    ///
    /// ```ignore
    /// let response = client.delete_oauth_client_redirect_uri()
    ///    .client_id(client_id)
    ///    .redirect_uri_id(redirect_uri_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_oauth_client_redirect_uri(&self) -> builder::DeleteOauthClientRedirectUri<'_> {
        builder::DeleteOauthClientRedirectUri::new(self)
    }

    /// Add an OAuth client secret
    ///
    /// Sends a `POST` request to `/oauth/client/{client_id}/secret`
    ///
    /// ```ignore
    /// let response = client.create_oauth_client_secret()
    ///    .client_id(client_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_oauth_client_secret(&self) -> builder::CreateOauthClientSecret<'_> {
        builder::CreateOauthClientSecret::new(self)
    }

    /// Delete an OAuth client secret
    ///
    /// Sends a `DELETE` request to
    /// `/oauth/client/{client_id}/secret/{secret_id}`
    ///
    /// ```ignore
    /// let response = client.delete_oauth_client_secret()
    ///    .client_id(client_id)
    ///    .secret_id(secret_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_oauth_client_secret(&self) -> builder::DeleteOauthClientSecret<'_> {
        builder::DeleteOauthClientSecret::new(self)
    }

    /// View details for the calling user
    ///
    /// Sends a `GET` request to `/self`
    ///
    /// ```ignore
    /// let response = client.get_self()
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_self(&self) -> builder::GetSelf<'_> {
        builder::GetSelf::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        ByteStream, ClientHooks, ClientInfo, Error, OperationInfo, RequestBuilderExt,
        ResponseValue, encode_path,
    };
    /// Builder for [`Client::jwks_json`]
    ///
    /// [`Client::jwks_json`]: super::Client::jwks_json
    #[derive(Debug, Clone)]
    pub struct JwksJson<'a> {
        client: &'a super::Client,
    }

    impl<'a> JwksJson<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `GET` request to `/.well-known/jwks.json`
        pub async fn send(self) -> Result<ResponseValue<types::Jwks>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/.well-known/jwks.json", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "jwks_json",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::openid_configuration`]
    ///
    /// [`Client::openid_configuration`]: super::Client::openid_configuration
    #[derive(Debug, Clone)]
    pub struct OpenidConfiguration<'a> {
        client: &'a super::Client,
    }

    impl<'a> OpenidConfiguration<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `GET` request to `/.well-known/openid-configuration`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OpenIdConfiguration>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/.well-known/openid-configuration", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "openid_configuration",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::list_api_users`]
    ///
    /// [`Client::list_api_users`]: super::Client::list_api_users
    #[derive(Debug, Clone)]
    pub struct ListApiUsers<'a> {
        client: &'a super::Client,
    }

    impl<'a> ListApiUsers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `GET` request to `/api-user`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<::std::vec::Vec<types::GetUserResponseForApiPermissions>>,
            Error<types::Error>,
        > {
            let Self { client } = self;
            let url = format!("{}/api-user", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "list_api_users",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_api_user`]
    ///
    /// [`Client::create_api_user`]: super::Client::create_api_user
    #[derive(Debug, Clone)]
    pub struct CreateApiUser<'a> {
        client: &'a super::Client,
        body: Result<types::builder::ApiUserUpdateParamsForApiPermissions, String>,
    }

    impl<'a> CreateApiUser<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserUpdateParamsForApiPermissions>,
            <V as std::convert::TryInto<types::ApiUserUpdateParamsForApiPermissions>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiUserUpdateParamsForApiPermissions` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::ApiUserUpdateParamsForApiPermissions,
                )
                    -> types::builder::ApiUserUpdateParamsForApiPermissions,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetUserResponseForApiPermissions>, Error<types::Error>>
        {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| {
                    types::ApiUserUpdateParamsForApiPermissions::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api-user", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_api_user",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_api_user`]
    ///
    /// [`Client::get_api_user`]: super::Client::get_api_user
    #[derive(Debug, Clone)]
    pub struct GetApiUser<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
    }

    impl<'a> GetApiUser<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        /// Sends a `GET` request to `/api-user/{user_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetUserResponseForApiPermissions>, Error<types::Error>>
        {
            let Self { client, user_id } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_api_user",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::update_api_user`]
    ///
    /// [`Client::update_api_user`]: super::Client::update_api_user
    #[derive(Debug, Clone)]
    pub struct UpdateApiUser<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
        body: Result<types::builder::ApiUserUpdateParamsForApiPermissions, String>,
    }

    impl<'a> UpdateApiUser<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserUpdateParamsForApiPermissions>,
            <V as std::convert::TryInto<types::ApiUserUpdateParamsForApiPermissions>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiUserUpdateParamsForApiPermissions` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::ApiUserUpdateParamsForApiPermissions,
                )
                    -> types::builder::ApiUserUpdateParamsForApiPermissions,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{user_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetUserResponseForApiPermissions>, Error<types::Error>>
        {
            let Self {
                client,
                user_id,
                body,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::ApiUserUpdateParamsForApiPermissions::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "update_api_user",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::set_api_user_contact_email`]
    ///
    /// [`Client::set_api_user_contact_email`]: super::Client::set_api_user_contact_email
    #[derive(Debug, Clone)]
    pub struct SetApiUserContactEmail<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
        body: Result<types::builder::ApiUserEmailUpdateParams, String>,
    }

    impl<'a> SetApiUserContactEmail<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserEmailUpdateParams>,
            <V as std::convert::TryInto<types::ApiUserEmailUpdateParams>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiUserEmailUpdateParams` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::ApiUserEmailUpdateParams,
                ) -> types::builder::ApiUserEmailUpdateParams,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `PUT` request to `/api-user/{user_id}/contact/email`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserContactEmail>, Error<types::Error>> {
            let Self {
                client,
                user_id,
                body,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::ApiUserEmailUpdateParams::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/contact/email",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "set_api_user_contact_email",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::add_api_user_to_group`]
    ///
    /// [`Client::add_api_user_to_group`]: super::Client::add_api_user_to_group
    #[derive(Debug, Clone)]
    pub struct AddApiUserToGroup<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
        body: Result<types::builder::AddGroupBody, String>,
    }

    impl<'a> AddApiUserToGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddGroupBody>,
            <V as std::convert::TryInto<types::AddGroupBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `AddGroupBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::AddGroupBody) -> types::builder::AddGroupBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{user_id}/group`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetUserResponseForApiPermissions>, Error<types::Error>>
        {
            let Self {
                client,
                user_id,
                body,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::AddGroupBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/group",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "add_api_user_to_group",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::remove_api_user_from_group`]
    ///
    /// [`Client::remove_api_user_from_group`]: super::Client::remove_api_user_from_group
    #[derive(Debug, Clone)]
    pub struct RemoveApiUserFromGroup<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
        group_id: Result<types::TypedUuidForAccessGroupId, String>,
    }

    impl<'a> RemoveApiUserFromGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                group_id: Err("group_id was not initialized".to_string()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn group_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForAccessGroupId>,
        {
            self.group_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForAccessGroupId` for group_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to `/api-user/{user_id}/group/{group_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetUserResponseForApiPermissions>, Error<types::Error>>
        {
            let Self {
                client,
                user_id,
                group_id,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let group_id = group_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/group/{}",
                client.baseurl,
                encode_path(&user_id.to_string()),
                encode_path(&group_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "remove_api_user_from_group",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::link_provider`]
    ///
    /// [`Client::link_provider`]: super::Client::link_provider
    #[derive(Debug, Clone)]
    pub struct LinkProvider<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
        body: Result<types::builder::ApiUserProviderLinkPayload, String>,
    }

    impl<'a> LinkProvider<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserProviderLinkPayload>,
            <V as std::convert::TryInto<types::ApiUserProviderLinkPayload>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiUserProviderLinkPayload` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::ApiUserProviderLinkPayload,
                ) -> types::builder::ApiUserProviderLinkPayload,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{user_id}/link`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self {
                client,
                user_id,
                body,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::ApiUserProviderLinkPayload::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/link",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "link_provider",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::list_api_user_tokens`]
    ///
    /// [`Client::list_api_user_tokens`]: super::Client::list_api_user_tokens
    #[derive(Debug, Clone)]
    pub struct ListApiUserTokens<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
    }

    impl<'a> ListApiUserTokens<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        /// Sends a `GET` request to `/api-user/{user_id}/token`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<::std::vec::Vec<types::ApiKeyResponseForApiPermissions>>,
            Error<types::Error>,
        > {
            let Self { client, user_id } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "list_api_user_tokens",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_api_user_token`]
    ///
    /// [`Client::create_api_user_token`]: super::Client::create_api_user_token
    #[derive(Debug, Clone)]
    pub struct CreateApiUserToken<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
        body: Result<types::builder::ApiKeyCreateParamsForApiPermissions, String>,
    }

    impl<'a> CreateApiUserToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiKeyCreateParamsForApiPermissions>,
            <V as std::convert::TryInto<types::ApiKeyCreateParamsForApiPermissions>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiKeyCreateParamsForApiPermissions` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::ApiKeyCreateParamsForApiPermissions,
                )
                    -> types::builder::ApiKeyCreateParamsForApiPermissions,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{user_id}/token`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InitialApiKeyResponseForApiPermissions>, Error<types::Error>>
        {
            let Self {
                client,
                user_id,
                body,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::ApiKeyCreateParamsForApiPermissions::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_api_user_token",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_api_user_token`]
    ///
    /// [`Client::get_api_user_token`]: super::Client::get_api_user_token
    #[derive(Debug, Clone)]
    pub struct GetApiUserToken<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
        api_key_id: Result<types::TypedUuidForApiKeyId, String>,
    }

    impl<'a> GetApiUserToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                api_key_id: Err("api_key_id was not initialized".to_string()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn api_key_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForApiKeyId>,
        {
            self.api_key_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForApiKeyId` for api_key_id failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/api-user/{user_id}/token/{api_key_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiKeyResponseForApiPermissions>, Error<types::Error>>
        {
            let Self {
                client,
                user_id,
                api_key_id,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let api_key_id = api_key_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token/{}",
                client.baseurl,
                encode_path(&user_id.to_string()),
                encode_path(&api_key_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_api_user_token",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_api_user_token`]
    ///
    /// [`Client::delete_api_user_token`]: super::Client::delete_api_user_token
    #[derive(Debug, Clone)]
    pub struct DeleteApiUserToken<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
        api_key_id: Result<types::TypedUuidForApiKeyId, String>,
    }

    impl<'a> DeleteApiUserToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                api_key_id: Err("api_key_id was not initialized".to_string()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn api_key_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForApiKeyId>,
        {
            self.api_key_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForApiKeyId` for api_key_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to `/api-user/{user_id}/token/{api_key_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiKeyResponseForApiPermissions>, Error<types::Error>>
        {
            let Self {
                client,
                user_id,
                api_key_id,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let api_key_id = api_key_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token/{}",
                client.baseurl,
                encode_path(&user_id.to_string()),
                encode_path(&api_key_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_api_user_token",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_link_token`]
    ///
    /// [`Client::create_link_token`]: super::Client::create_link_token
    #[derive(Debug, Clone)]
    pub struct CreateLinkToken<'a> {
        client: &'a super::Client,
        provider_id: Result<types::TypedUuidForUserProviderId, String>,
        body: Result<types::builder::ApiUserLinkRequestPayload, String>,
    }

    impl<'a> CreateLinkToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                provider_id: Err("provider_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn provider_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserProviderId>,
        {
            self.provider_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForUserProviderId` for provider_id failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserLinkRequestPayload>,
            <V as std::convert::TryInto<types::ApiUserLinkRequestPayload>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiUserLinkRequestPayload` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::ApiUserLinkRequestPayload,
                ) -> types::builder::ApiUserLinkRequestPayload,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to
        /// `/api-user-provider/{provider_id}/link-token`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserLinkRequestResponse>, Error<types::Error>> {
            let Self {
                client,
                provider_id,
                body,
            } = self;
            let provider_id = provider_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::ApiUserLinkRequestPayload::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user-provider/{}/link-token",
                client.baseurl,
                encode_path(&provider_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_link_token",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_groups`]
    ///
    /// [`Client::get_groups`]: super::Client::get_groups
    #[derive(Debug, Clone)]
    pub struct GetGroups<'a> {
        client: &'a super::Client,
    }

    impl<'a> GetGroups<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `GET` request to `/group`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<::std::vec::Vec<types::AccessGroupForApiPermissions>>,
            Error<types::Error>,
        > {
            let Self { client } = self;
            let url = format!("{}/group", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_groups",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_group`]
    ///
    /// [`Client::create_group`]: super::Client::create_group
    #[derive(Debug, Clone)]
    pub struct CreateGroup<'a> {
        client: &'a super::Client,
        body: Result<types::builder::AccessGroupUpdateParamsForApiPermissions, String>,
    }

    impl<'a> CreateGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AccessGroupUpdateParamsForApiPermissions>,
            <V as std::convert::TryInto<types::AccessGroupUpdateParamsForApiPermissions>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `AccessGroupUpdateParamsForApiPermissions` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::AccessGroupUpdateParamsForApiPermissions,
                )
                    -> types::builder::AccessGroupUpdateParamsForApiPermissions,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/group`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::AccessGroupForApiPermissions>, Error<types::Error>>
        {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| {
                    types::AccessGroupUpdateParamsForApiPermissions::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/group", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_group",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::update_group`]
    ///
    /// [`Client::update_group`]: super::Client::update_group
    #[derive(Debug, Clone)]
    pub struct UpdateGroup<'a> {
        client: &'a super::Client,
        group_id: Result<types::TypedUuidForAccessGroupId, String>,
        body: Result<types::builder::AccessGroupUpdateParamsForApiPermissions, String>,
    }

    impl<'a> UpdateGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                group_id: Err("group_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn group_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForAccessGroupId>,
        {
            self.group_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForAccessGroupId` for group_id failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AccessGroupUpdateParamsForApiPermissions>,
            <V as std::convert::TryInto<types::AccessGroupUpdateParamsForApiPermissions>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `AccessGroupUpdateParamsForApiPermissions` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::AccessGroupUpdateParamsForApiPermissions,
                )
                    -> types::builder::AccessGroupUpdateParamsForApiPermissions,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `PUT` request to `/group/{group_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::AccessGroupForApiPermissions>, Error<types::Error>>
        {
            let Self {
                client,
                group_id,
                body,
            } = self;
            let group_id = group_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::AccessGroupUpdateParamsForApiPermissions::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/group/{}",
                client.baseurl,
                encode_path(&group_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "update_group",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_group`]
    ///
    /// [`Client::delete_group`]: super::Client::delete_group
    #[derive(Debug, Clone)]
    pub struct DeleteGroup<'a> {
        client: &'a super::Client,
        group_id: Result<types::TypedUuidForAccessGroupId, String>,
    }

    impl<'a> DeleteGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                group_id: Err("group_id was not initialized".to_string()),
            }
        }

        pub fn group_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForAccessGroupId>,
        {
            self.group_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForAccessGroupId` for group_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to `/group/{group_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::AccessGroupForApiPermissions>, Error<types::Error>>
        {
            let Self { client, group_id } = self;
            let group_id = group_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/group/{}",
                client.baseurl,
                encode_path(&group_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_group",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_group_members`]
    ///
    /// [`Client::get_group_members`]: super::Client::get_group_members
    #[derive(Debug, Clone)]
    pub struct GetGroupMembers<'a> {
        client: &'a super::Client,
        group_id: Result<types::TypedUuidForAccessGroupId, String>,
    }

    impl<'a> GetGroupMembers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                group_id: Err("group_id was not initialized".to_string()),
            }
        }

        pub fn group_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForAccessGroupId>,
        {
            self.group_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForAccessGroupId` for group_id failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/group-membership/{group_id}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<::std::vec::Vec<types::GetUserResponseForApiPermissions>>,
            Error<types::Error>,
        > {
            let Self { client, group_id } = self;
            let group_id = group_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/group-membership/{}",
                client.baseurl,
                encode_path(&group_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_group_members",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::magic_link_exchange`]
    ///
    /// [`Client::magic_link_exchange`]: super::Client::magic_link_exchange
    #[derive(Debug, Clone)]
    pub struct MagicLinkExchange<'a> {
        client: &'a super::Client,
        channel: Result<::std::string::String, String>,
        body: Result<types::builder::MagicLinkExchangeRequest, String>,
    }

    impl<'a> MagicLinkExchange<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                channel: Err("channel was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn channel<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.channel = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for channel failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::MagicLinkExchangeRequest>,
            <V as std::convert::TryInto<types::MagicLinkExchangeRequest>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `MagicLinkExchangeRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::MagicLinkExchangeRequest,
                ) -> types::builder::MagicLinkExchangeRequest,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/magic/{channel}/exchange`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::MagicLinkExchangeResponse>, Error<types::Error>> {
            let Self {
                client,
                channel,
                body,
            } = self;
            let channel = channel.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::MagicLinkExchangeRequest::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/magic/{}/exchange",
                client.baseurl,
                encode_path(&channel.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "magic_link_exchange",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::magic_link_send`]
    ///
    /// [`Client::magic_link_send`]: super::Client::magic_link_send
    #[derive(Debug, Clone)]
    pub struct MagicLinkSend<'a> {
        client: &'a super::Client,
        channel: Result<::std::string::String, String>,
        body: Result<types::builder::MagicLinkSendRequest, String>,
    }

    impl<'a> MagicLinkSend<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                channel: Err("channel was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn channel<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.channel = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for channel failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::MagicLinkSendRequest>,
            <V as std::convert::TryInto<types::MagicLinkSendRequest>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `MagicLinkSendRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::MagicLinkSendRequest,
                ) -> types::builder::MagicLinkSendRequest,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/magic/{channel}/send`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::MagicLinkSendResponse>, Error<types::Error>> {
            let Self {
                client,
                channel,
                body,
            } = self;
            let channel = channel.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::MagicLinkSendRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/magic/{}/send",
                client.baseurl,
                encode_path(&channel.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "magic_link_send",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::authz_code_redirect`]
    ///
    /// [`Client::authz_code_redirect`]: super::Client::authz_code_redirect
    #[derive(Debug, Clone)]
    pub struct AuthzCodeRedirect<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
        code_challenge: Result<::std::string::String, String>,
        code_challenge_method: Result<::std::string::String, String>,
        redirect_uri: Result<::std::string::String, String>,
        response_type: Result<::std::string::String, String>,
        scope: Result<Option<::std::string::String>, String>,
        state: Result<::std::string::String, String>,
    }

    impl<'a> AuthzCodeRedirect<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                provider: Err("provider was not initialized".to_string()),
                client_id: Err("client_id was not initialized".to_string()),
                code_challenge: Err("code_challenge was not initialized".to_string()),
                code_challenge_method: Err("code_challenge_method was not initialized".to_string()),
                redirect_uri: Err("redirect_uri was not initialized".to_string()),
                response_type: Err("response_type was not initialized".to_string()),
                scope: Ok(None),
                state: Err("state was not initialized".to_string()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
            self
        }

        pub fn code_challenge<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.code_challenge = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for code_challenge failed".to_string()
            });
            self
        }

        pub fn code_challenge_method<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.code_challenge_method = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for code_challenge_method failed"
                    .to_string()
            });
            self
        }

        pub fn redirect_uri<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.redirect_uri = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for redirect_uri failed".to_string()
            });
            self
        }

        pub fn response_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.response_type = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for response_type failed".to_string()
            });
            self
        }

        pub fn scope<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.scope = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for scope failed".to_string()
            });
            self
        }

        pub fn state<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.state = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for state failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/login/oauth/{provider}/code/authorize`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
            let Self {
                client,
                provider,
                client_id,
                code_challenge,
                code_challenge_method,
                redirect_uri,
                response_type,
                scope,
                state,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let code_challenge = code_challenge.map_err(Error::InvalidRequest)?;
            let code_challenge_method = code_challenge_method.map_err(Error::InvalidRequest)?;
            let redirect_uri = redirect_uri.map_err(Error::InvalidRequest)?;
            let response_type = response_type.map_err(Error::InvalidRequest)?;
            let scope = scope.map_err(Error::InvalidRequest)?;
            let state = state.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/code/authorize",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .query(&progenitor_client::QueryParam::new("client_id", &client_id))
                .query(&progenitor_client::QueryParam::new(
                    "code_challenge",
                    &code_challenge,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "code_challenge_method",
                    &code_challenge_method,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "redirect_uri",
                    &redirect_uri,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "response_type",
                    &response_type,
                ))
                .query(&progenitor_client::QueryParam::new("scope", &scope))
                .query(&progenitor_client::QueryParam::new("state", &state))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "authz_code_redirect",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::authz_code_callback`]
    ///
    /// [`Client::authz_code_callback`]: super::Client::authz_code_callback
    #[derive(Debug, Clone)]
    pub struct AuthzCodeCallback<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
        code: Result<Option<::std::string::String>, String>,
        error: Result<Option<::std::string::String>, String>,
        state: Result<Option<::std::string::String>, String>,
    }

    impl<'a> AuthzCodeCallback<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                provider: Err("provider was not initialized".to_string()),
                code: Ok(None),
                error: Ok(None),
                state: Ok(None),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        pub fn code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.code = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for code failed".to_string()
            });
            self
        }

        pub fn error<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.error = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for error failed".to_string()
            });
            self
        }

        pub fn state<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.state = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for state failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/login/oauth/{provider}/code/callback`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
            let Self {
                client,
                provider,
                code,
                error,
                state,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let code = code.map_err(Error::InvalidRequest)?;
            let error = error.map_err(Error::InvalidRequest)?;
            let state = state.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/code/callback",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .query(&progenitor_client::QueryParam::new("code", &code))
                .query(&progenitor_client::QueryParam::new("error", &error))
                .query(&progenitor_client::QueryParam::new("state", &state))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "authz_code_callback",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::authz_code_exchange`]
    ///
    /// [`Client::authz_code_exchange`]: super::Client::authz_code_exchange
    #[derive(Debug, Clone)]
    pub struct AuthzCodeExchange<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
        request_idp_token: Result<Option<bool>, String>,
        body: Result<types::builder::OAuthAuthzCodeExchangeBody, String>,
    }

    impl<'a> AuthzCodeExchange<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                provider: Err("provider was not initialized".to_string()),
                request_idp_token: Ok(None),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        pub fn request_idp_token<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.request_idp_token = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for request_idp_token failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthAuthzCodeExchangeBody>,
            <V as std::convert::TryInto<types::OAuthAuthzCodeExchangeBody>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `OAuthAuthzCodeExchangeBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::OAuthAuthzCodeExchangeBody,
                ) -> types::builder::OAuthAuthzCodeExchangeBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/oauth/{provider}/code/token`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthAuthzCodeExchangeResponse>, Error<types::Error>>
        {
            let Self {
                client,
                provider,
                request_idp_token,
                body,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let request_idp_token = request_idp_token.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::OAuthAuthzCodeExchangeBody::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/code/token",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .query(&progenitor_client::QueryParam::new(
                    "request_idp_token",
                    &request_idp_token,
                ))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "authz_code_exchange",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_device_provider`]
    ///
    /// [`Client::get_device_provider`]: super::Client::get_device_provider
    #[derive(Debug, Clone)]
    pub struct GetDeviceProvider<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
    }

    impl<'a> GetDeviceProvider<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                provider: Err("provider was not initialized".to_string()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        /// Sends a `GET` request to `/login/oauth/{provider}/device`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthProviderDeviceInfo>, Error<types::Error>> {
            let Self { client, provider } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/device",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_device_provider",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::device_authz`]
    ///
    /// [`Client::device_authz`]: super::Client::device_authz
    #[derive(Debug, Clone)]
    pub struct DeviceAuthz<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
        body: Result<types::builder::DeviceAuthorizationRequest, String>,
    }

    impl<'a> DeviceAuthz<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                provider: Err("provider was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeviceAuthorizationRequest>,
            <V as std::convert::TryInto<types::DeviceAuthorizationRequest>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `DeviceAuthorizationRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::DeviceAuthorizationRequest,
                ) -> types::builder::DeviceAuthorizationRequest,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/oauth/{provider}/device`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<ByteStream>> {
            let Self {
                client,
                provider,
                body,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::DeviceAuthorizationRequest::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/device",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "device_authz",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::ErrorResponse(ResponseValue::stream(response))),
            }
        }
    }

    /// Builder for [`Client::exchange_device_token`]
    ///
    /// [`Client::exchange_device_token`]: super::Client::exchange_device_token
    #[derive(Debug, Clone)]
    pub struct ExchangeDeviceToken<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
        body: Result<types::builder::DeviceTokenExchangeRequest, String>,
    }

    impl<'a> ExchangeDeviceToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                provider: Err("provider was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeviceTokenExchangeRequest>,
            <V as std::convert::TryInto<types::DeviceTokenExchangeRequest>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `DeviceTokenExchangeRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::DeviceTokenExchangeRequest,
                ) -> types::builder::DeviceTokenExchangeRequest,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/oauth/{provider}/device/exchange`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<ByteStream>> {
            let Self {
                client,
                provider,
                body,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::DeviceTokenExchangeRequest::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/device/exchange",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "exchange_device_token",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::ErrorResponse(ResponseValue::stream(response))),
            }
        }
    }

    /// Builder for [`Client::get_web_pkce_provider`]
    ///
    /// [`Client::get_web_pkce_provider`]: super::Client::get_web_pkce_provider
    #[derive(Debug, Clone)]
    pub struct GetWebPkceProvider<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
    }

    impl<'a> GetWebPkceProvider<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                provider: Err("provider was not initialized".to_string()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        /// Sends a `GET` request to `/login/oauth/{provider}/public-pkce`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthProviderAuthorizationCodePkceInfo>, Error<types::Error>>
        {
            let Self { client, provider } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/public-pkce",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_web_pkce_provider",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::list_magic_links`]
    ///
    /// [`Client::list_magic_links`]: super::Client::list_magic_links
    #[derive(Debug, Clone)]
    pub struct ListMagicLinks<'a> {
        client: &'a super::Client,
    }

    impl<'a> ListMagicLinks<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `GET` request to `/magic/client`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::MagicLink>>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/magic/client", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "list_magic_links",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_magic_link`]
    ///
    /// [`Client::create_magic_link`]: super::Client::create_magic_link
    #[derive(Debug, Clone)]
    pub struct CreateMagicLink<'a> {
        client: &'a super::Client,
    }

    impl<'a> CreateMagicLink<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `POST` request to `/magic/client`
        pub async fn send(self) -> Result<ResponseValue<types::MagicLink>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/magic/client", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_magic_link",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_magic_link`]
    ///
    /// [`Client::get_magic_link`]: super::Client::get_magic_link
    #[derive(Debug, Clone)]
    pub struct GetMagicLink<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForMagicLinkId, String>,
    }

    impl<'a> GetMagicLink<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkId` for client_id failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/magic/client/{client_id}`
        pub async fn send(self) -> Result<ResponseValue<types::MagicLink>, Error<types::Error>> {
            let Self { client, client_id } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/magic/client/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_magic_link",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_magic_link_redirect_uri`]
    ///
    /// [`Client::create_magic_link_redirect_uri`]: super::Client::create_magic_link_redirect_uri
    #[derive(Debug, Clone)]
    pub struct CreateMagicLinkRedirectUri<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForMagicLinkId, String>,
        body: Result<types::builder::AddMagicLinkRedirectBody, String>,
    }

    impl<'a> CreateMagicLinkRedirectUri<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkId` for client_id failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddMagicLinkRedirectBody>,
            <V as std::convert::TryInto<types::AddMagicLinkRedirectBody>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `AddMagicLinkRedirectBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::AddMagicLinkRedirectBody,
                ) -> types::builder::AddMagicLinkRedirectBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/magic/client/{client_id}/redirect_uri`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::MagicLinkRedirectUri>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                body,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::AddMagicLinkRedirectBody::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/magic/client/{}/redirect_uri",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_magic_link_redirect_uri",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_magic_link_redirect_uri`]
    ///
    /// [`Client::delete_magic_link_redirect_uri`]: super::Client::delete_magic_link_redirect_uri
    #[derive(Debug, Clone)]
    pub struct DeleteMagicLinkRedirectUri<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForMagicLinkId, String>,
        redirect_uri_id: Result<types::TypedUuidForMagicLinkRedirectUriId, String>,
    }

    impl<'a> DeleteMagicLinkRedirectUri<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                redirect_uri_id: Err("redirect_uri_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkId` for client_id failed".to_string()
            });
            self
        }

        pub fn redirect_uri_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkRedirectUriId>,
        {
            self.redirect_uri_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkRedirectUriId` for redirect_uri_id failed"
                    .to_string()
            });
            self
        }

        /// Sends a `DELETE` request to
        /// `/magic/client/{client_id}/redirect_uri/{redirect_uri_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::MagicLinkRedirectUri>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                redirect_uri_id,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let redirect_uri_id = redirect_uri_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/magic/client/{}/redirect_uri/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
                encode_path(&redirect_uri_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_magic_link_redirect_uri",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_magic_link_secret`]
    ///
    /// [`Client::create_magic_link_secret`]: super::Client::create_magic_link_secret
    #[derive(Debug, Clone)]
    pub struct CreateMagicLinkSecret<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForMagicLinkId, String>,
    }

    impl<'a> CreateMagicLinkSecret<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkId` for client_id failed".to_string()
            });
            self
        }

        /// Sends a `POST` request to `/magic/client/{client_id}/secret`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InitialMagicLinkSecretResponse>, Error<types::Error>>
        {
            let Self { client, client_id } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/magic/client/{}/secret",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_magic_link_secret",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_magic_link_secret`]
    ///
    /// [`Client::delete_magic_link_secret`]: super::Client::delete_magic_link_secret
    #[derive(Debug, Clone)]
    pub struct DeleteMagicLinkSecret<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForMagicLinkId, String>,
        secret_id: Result<types::TypedUuidForMagicLinkSecretId, String>,
    }

    impl<'a> DeleteMagicLinkSecret<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                secret_id: Err("secret_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkId` for client_id failed".to_string()
            });
            self
        }

        pub fn secret_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkSecretId>,
        {
            self.secret_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkSecretId` for secret_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to
        /// `/magic/client/{client_id}/secret/{secret_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::MagicLinkSecret>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                secret_id,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let secret_id = secret_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/magic/client/{}/secret/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
                encode_path(&secret_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_magic_link_secret",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_mappers`]
    ///
    /// [`Client::get_mappers`]: super::Client::get_mappers
    #[derive(Debug, Clone)]
    pub struct GetMappers<'a> {
        client: &'a super::Client,
        include_depleted: Result<Option<bool>, String>,
    }

    impl<'a> GetMappers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                include_depleted: Ok(None),
            }
        }

        pub fn include_depleted<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.include_depleted = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for include_depleted failed".to_string());
            self
        }

        /// Sends a `GET` request to `/mapper`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::Mapper>>, Error<types::Error>> {
            let Self {
                client,
                include_depleted,
            } = self;
            let include_depleted = include_depleted.map_err(Error::InvalidRequest)?;
            let url = format!("{}/mapper", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new(
                    "include_depleted",
                    &include_depleted,
                ))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_mappers",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_mapper`]
    ///
    /// [`Client::create_mapper`]: super::Client::create_mapper
    #[derive(Debug, Clone)]
    pub struct CreateMapper<'a> {
        client: &'a super::Client,
        body: Result<types::builder::CreateMapper, String>,
    }

    impl<'a> CreateMapper<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateMapper>,
            <V as std::convert::TryInto<types::CreateMapper>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `CreateMapper` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::CreateMapper) -> types::builder::CreateMapper,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/mapper`
        pub async fn send(self) -> Result<ResponseValue<types::Mapper>, Error<types::Error>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::CreateMapper::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/mapper", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_mapper",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_mapper`]
    ///
    /// [`Client::delete_mapper`]: super::Client::delete_mapper
    #[derive(Debug, Clone)]
    pub struct DeleteMapper<'a> {
        client: &'a super::Client,
        mapper_id: Result<types::TypedUuidForMapperId, String>,
    }

    impl<'a> DeleteMapper<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                mapper_id: Err("mapper_id was not initialized".to_string()),
            }
        }

        pub fn mapper_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMapperId>,
        {
            self.mapper_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMapperId` for mapper_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to `/mapper/{mapper_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Mapper>, Error<types::Error>> {
            let Self { client, mapper_id } = self;
            let mapper_id = mapper_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/mapper/{}",
                client.baseurl,
                encode_path(&mapper_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_mapper",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::list_oauth_clients`]
    ///
    /// [`Client::list_oauth_clients`]: super::Client::list_oauth_clients
    #[derive(Debug, Clone)]
    pub struct ListOauthClients<'a> {
        client: &'a super::Client,
    }

    impl<'a> ListOauthClients<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `GET` request to `/oauth/client`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::OAuthClient>>, Error<types::Error>>
        {
            let Self { client } = self;
            let url = format!("{}/oauth/client", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "list_oauth_clients",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_oauth_client`]
    ///
    /// [`Client::create_oauth_client`]: super::Client::create_oauth_client
    #[derive(Debug, Clone)]
    pub struct CreateOauthClient<'a> {
        client: &'a super::Client,
    }

    impl<'a> CreateOauthClient<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `POST` request to `/oauth/client`
        pub async fn send(self) -> Result<ResponseValue<types::OAuthClient>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/oauth/client", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_oauth_client",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_oauth_client`]
    ///
    /// [`Client::get_oauth_client`]: super::Client::get_oauth_client
    #[derive(Debug, Clone)]
    pub struct GetOauthClient<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
    }

    impl<'a> GetOauthClient<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/oauth/client/{client_id}`
        pub async fn send(self) -> Result<ResponseValue<types::OAuthClient>, Error<types::Error>> {
            let Self { client, client_id } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_oauth_client",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_oauth_client_redirect_uri`]
    ///
    /// [`Client::create_oauth_client_redirect_uri`]: super::Client::create_oauth_client_redirect_uri
    #[derive(Debug, Clone)]
    pub struct CreateOauthClientRedirectUri<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
        body: Result<types::builder::AddOAuthClientRedirectBody, String>,
    }

    impl<'a> CreateOauthClientRedirectUri<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddOAuthClientRedirectBody>,
            <V as std::convert::TryInto<types::AddOAuthClientRedirectBody>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `AddOAuthClientRedirectBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                    types::builder::AddOAuthClientRedirectBody,
                ) -> types::builder::AddOAuthClientRedirectBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/oauth/client/{client_id}/redirect_uri`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthClientRedirectUri>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                body,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::AddOAuthClientRedirectBody::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}/redirect_uri",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_oauth_client_redirect_uri",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_oauth_client_redirect_uri`]
    ///
    /// [`Client::delete_oauth_client_redirect_uri`]: super::Client::delete_oauth_client_redirect_uri
    #[derive(Debug, Clone)]
    pub struct DeleteOauthClientRedirectUri<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
        redirect_uri_id: Result<types::TypedUuidForOAuthRedirectUriId, String>,
    }

    impl<'a> DeleteOauthClientRedirectUri<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                redirect_uri_id: Err("redirect_uri_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
            self
        }

        pub fn redirect_uri_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthRedirectUriId>,
        {
            self.redirect_uri_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthRedirectUriId` for redirect_uri_id failed"
                    .to_string()
            });
            self
        }

        /// Sends a `DELETE` request to
        /// `/oauth/client/{client_id}/redirect_uri/{redirect_uri_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthClientRedirectUri>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                redirect_uri_id,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let redirect_uri_id = redirect_uri_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}/redirect_uri/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
                encode_path(&redirect_uri_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_oauth_client_redirect_uri",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_oauth_client_secret`]
    ///
    /// [`Client::create_oauth_client_secret`]: super::Client::create_oauth_client_secret
    #[derive(Debug, Clone)]
    pub struct CreateOauthClientSecret<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
    }

    impl<'a> CreateOauthClientSecret<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
            self
        }

        /// Sends a `POST` request to `/oauth/client/{client_id}/secret`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InitialOAuthClientSecretResponse>, Error<types::Error>>
        {
            let Self { client, client_id } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}/secret",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_oauth_client_secret",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_oauth_client_secret`]
    ///
    /// [`Client::delete_oauth_client_secret`]: super::Client::delete_oauth_client_secret
    #[derive(Debug, Clone)]
    pub struct DeleteOauthClientSecret<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
        secret_id: Result<types::TypedUuidForOAuthSecretId, String>,
    }

    impl<'a> DeleteOauthClientSecret<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                secret_id: Err("secret_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
            self
        }

        pub fn secret_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthSecretId>,
        {
            self.secret_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthSecretId` for secret_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to
        /// `/oauth/client/{client_id}/secret/{secret_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthClientSecret>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                secret_id,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let secret_id = secret_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}/secret/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
                encode_path(&secret_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_oauth_client_secret",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_self`]
    ///
    /// [`Client::get_self`]: super::Client::get_self
    #[derive(Debug, Clone)]
    pub struct GetSelf<'a> {
        client: &'a super::Client,
    }

    impl<'a> GetSelf<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `GET` request to `/self`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetUserResponseForApiPermissions>, Error<types::Error>>
        {
            let Self { client } = self;
            let url = format!("{}/self", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_self",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client and
/// extension traits.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
