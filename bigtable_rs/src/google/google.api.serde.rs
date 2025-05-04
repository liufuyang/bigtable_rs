// @generated
impl serde::Serialize for Advice {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Advice", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Advice {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Advice;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Advice")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Advice, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Advice {
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Advice", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.jwks_uri.is_empty() {
            len += 1;
        }
        if !self.audiences.is_empty() {
            len += 1;
        }
        if !self.authorization_url.is_empty() {
            len += 1;
        }
        if !self.jwt_locations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.AuthProvider", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.jwks_uri.is_empty() {
            struct_ser.serialize_field("jwksUri", &self.jwks_uri)?;
        }
        if !self.audiences.is_empty() {
            struct_ser.serialize_field("audiences", &self.audiences)?;
        }
        if !self.authorization_url.is_empty() {
            struct_ser.serialize_field("authorizationUrl", &self.authorization_url)?;
        }
        if !self.jwt_locations.is_empty() {
            struct_ser.serialize_field("jwtLocations", &self.jwt_locations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "issuer",
            "jwks_uri",
            "jwksUri",
            "audiences",
            "authorization_url",
            "authorizationUrl",
            "jwt_locations",
            "jwtLocations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Issuer,
            JwksUri,
            Audiences,
            AuthorizationUrl,
            JwtLocations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "issuer" => Ok(GeneratedField::Issuer),
                            "jwksUri" | "jwks_uri" => Ok(GeneratedField::JwksUri),
                            "audiences" => Ok(GeneratedField::Audiences),
                            "authorizationUrl" | "authorization_url" => Ok(GeneratedField::AuthorizationUrl),
                            "jwtLocations" | "jwt_locations" => Ok(GeneratedField::JwtLocations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.AuthProvider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut issuer__ = None;
                let mut jwks_uri__ = None;
                let mut audiences__ = None;
                let mut authorization_url__ = None;
                let mut jwt_locations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JwksUri => {
                            if jwks_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwksUri"));
                            }
                            jwks_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Audiences => {
                            if audiences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audiences"));
                            }
                            audiences__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizationUrl => {
                            if authorization_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationUrl"));
                            }
                            authorization_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JwtLocations => {
                            if jwt_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwtLocations"));
                            }
                            jwt_locations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthProvider {
                    id: id__.unwrap_or_default(),
                    issuer: issuer__.unwrap_or_default(),
                    jwks_uri: jwks_uri__.unwrap_or_default(),
                    audiences: audiences__.unwrap_or_default(),
                    authorization_url: authorization_url__.unwrap_or_default(),
                    jwt_locations: jwt_locations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.AuthProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthRequirement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider_id.is_empty() {
            len += 1;
        }
        if !self.audiences.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.AuthRequirement", len)?;
        if !self.provider_id.is_empty() {
            struct_ser.serialize_field("providerId", &self.provider_id)?;
        }
        if !self.audiences.is_empty() {
            struct_ser.serialize_field("audiences", &self.audiences)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthRequirement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_id",
            "providerId",
            "audiences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderId,
            Audiences,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "providerId" | "provider_id" => Ok(GeneratedField::ProviderId),
                            "audiences" => Ok(GeneratedField::Audiences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthRequirement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.AuthRequirement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthRequirement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_id__ = None;
                let mut audiences__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderId => {
                            if provider_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerId"));
                            }
                            provider_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Audiences => {
                            if audiences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audiences"));
                            }
                            audiences__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthRequirement {
                    provider_id: provider_id__.unwrap_or_default(),
                    audiences: audiences__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.AuthRequirement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Authentication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        if !self.providers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Authentication", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if !self.providers.is_empty() {
            struct_ser.serialize_field("providers", &self.providers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Authentication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
            "providers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
            Providers,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rules" => Ok(GeneratedField::Rules),
                            "providers" => Ok(GeneratedField::Providers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Authentication;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Authentication")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Authentication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                let mut providers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Providers => {
                            if providers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providers"));
                            }
                            providers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Authentication {
                    rules: rules__.unwrap_or_default(),
                    providers: providers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Authentication", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticationRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if self.oauth.is_some() {
            len += 1;
        }
        if self.allow_without_credential {
            len += 1;
        }
        if !self.requirements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.AuthenticationRule", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if let Some(v) = self.oauth.as_ref() {
            struct_ser.serialize_field("oauth", v)?;
        }
        if self.allow_without_credential {
            struct_ser.serialize_field("allowWithoutCredential", &self.allow_without_credential)?;
        }
        if !self.requirements.is_empty() {
            struct_ser.serialize_field("requirements", &self.requirements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "oauth",
            "allow_without_credential",
            "allowWithoutCredential",
            "requirements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            Oauth,
            AllowWithoutCredential,
            Requirements,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "oauth" => Ok(GeneratedField::Oauth),
                            "allowWithoutCredential" | "allow_without_credential" => Ok(GeneratedField::AllowWithoutCredential),
                            "requirements" => Ok(GeneratedField::Requirements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.AuthenticationRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthenticationRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut oauth__ = None;
                let mut allow_without_credential__ = None;
                let mut requirements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Oauth => {
                            if oauth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oauth"));
                            }
                            oauth__ = map_.next_value()?;
                        }
                        GeneratedField::AllowWithoutCredential => {
                            if allow_without_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowWithoutCredential"));
                            }
                            allow_without_credential__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Requirements => {
                            if requirements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirements"));
                            }
                            requirements__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthenticationRule {
                    selector: selector__.unwrap_or_default(),
                    oauth: oauth__,
                    allow_without_credential: allow_without_credential__.unwrap_or_default(),
                    requirements: requirements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.AuthenticationRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Backend {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Backend", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Backend {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Backend;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Backend")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Backend, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Backend {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Backend", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BackendRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if self.deadline != 0. {
            len += 1;
        }
        if self.min_deadline != 0. {
            len += 1;
        }
        if self.operation_deadline != 0. {
            len += 1;
        }
        if self.path_translation != 0 {
            len += 1;
        }
        if !self.protocol.is_empty() {
            len += 1;
        }
        if !self.overrides_by_request_protocol.is_empty() {
            len += 1;
        }
        if self.authentication.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.BackendRule", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.deadline != 0. {
            struct_ser.serialize_field("deadline", &self.deadline)?;
        }
        if self.min_deadline != 0. {
            struct_ser.serialize_field("minDeadline", &self.min_deadline)?;
        }
        if self.operation_deadline != 0. {
            struct_ser.serialize_field("operationDeadline", &self.operation_deadline)?;
        }
        if self.path_translation != 0 {
            let v = backend_rule::PathTranslation::try_from(self.path_translation)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.path_translation)))?;
            struct_ser.serialize_field("pathTranslation", &v)?;
        }
        if !self.protocol.is_empty() {
            struct_ser.serialize_field("protocol", &self.protocol)?;
        }
        if !self.overrides_by_request_protocol.is_empty() {
            struct_ser.serialize_field("overridesByRequestProtocol", &self.overrides_by_request_protocol)?;
        }
        if let Some(v) = self.authentication.as_ref() {
            match v {
                backend_rule::Authentication::JwtAudience(v) => {
                    struct_ser.serialize_field("jwtAudience", v)?;
                }
                backend_rule::Authentication::DisableAuth(v) => {
                    struct_ser.serialize_field("disableAuth", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BackendRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "address",
            "deadline",
            "min_deadline",
            "minDeadline",
            "operation_deadline",
            "operationDeadline",
            "path_translation",
            "pathTranslation",
            "protocol",
            "overrides_by_request_protocol",
            "overridesByRequestProtocol",
            "jwt_audience",
            "jwtAudience",
            "disable_auth",
            "disableAuth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            Address,
            Deadline,
            MinDeadline,
            OperationDeadline,
            PathTranslation,
            Protocol,
            OverridesByRequestProtocol,
            JwtAudience,
            DisableAuth,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "address" => Ok(GeneratedField::Address),
                            "deadline" => Ok(GeneratedField::Deadline),
                            "minDeadline" | "min_deadline" => Ok(GeneratedField::MinDeadline),
                            "operationDeadline" | "operation_deadline" => Ok(GeneratedField::OperationDeadline),
                            "pathTranslation" | "path_translation" => Ok(GeneratedField::PathTranslation),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "overridesByRequestProtocol" | "overrides_by_request_protocol" => Ok(GeneratedField::OverridesByRequestProtocol),
                            "jwtAudience" | "jwt_audience" => Ok(GeneratedField::JwtAudience),
                            "disableAuth" | "disable_auth" => Ok(GeneratedField::DisableAuth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BackendRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.BackendRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BackendRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut address__ = None;
                let mut deadline__ = None;
                let mut min_deadline__ = None;
                let mut operation_deadline__ = None;
                let mut path_translation__ = None;
                let mut protocol__ = None;
                let mut overrides_by_request_protocol__ = None;
                let mut authentication__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Deadline => {
                            if deadline__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deadline"));
                            }
                            deadline__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinDeadline => {
                            if min_deadline__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minDeadline"));
                            }
                            min_deadline__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OperationDeadline => {
                            if operation_deadline__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationDeadline"));
                            }
                            operation_deadline__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PathTranslation => {
                            if path_translation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathTranslation"));
                            }
                            path_translation__ = Some(map_.next_value::<backend_rule::PathTranslation>()? as i32);
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OverridesByRequestProtocol => {
                            if overrides_by_request_protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overridesByRequestProtocol"));
                            }
                            overrides_by_request_protocol__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::JwtAudience => {
                            if authentication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwtAudience"));
                            }
                            authentication__ = map_.next_value::<::std::option::Option<_>>()?.map(backend_rule::Authentication::JwtAudience);
                        }
                        GeneratedField::DisableAuth => {
                            if authentication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableAuth"));
                            }
                            authentication__ = map_.next_value::<::std::option::Option<_>>()?.map(backend_rule::Authentication::DisableAuth);
                        }
                    }
                }
                Ok(BackendRule {
                    selector: selector__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    deadline: deadline__.unwrap_or_default(),
                    min_deadline: min_deadline__.unwrap_or_default(),
                    operation_deadline: operation_deadline__.unwrap_or_default(),
                    path_translation: path_translation__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                    overrides_by_request_protocol: overrides_by_request_protocol__.unwrap_or_default(),
                    authentication: authentication__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.BackendRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for backend_rule::PathTranslation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PATH_TRANSLATION_UNSPECIFIED",
            Self::ConstantAddress => "CONSTANT_ADDRESS",
            Self::AppendPathToAddress => "APPEND_PATH_TO_ADDRESS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for backend_rule::PathTranslation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PATH_TRANSLATION_UNSPECIFIED",
            "CONSTANT_ADDRESS",
            "APPEND_PATH_TO_ADDRESS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = backend_rule::PathTranslation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PATH_TRANSLATION_UNSPECIFIED" => Ok(backend_rule::PathTranslation::Unspecified),
                    "CONSTANT_ADDRESS" => Ok(backend_rule::PathTranslation::ConstantAddress),
                    "APPEND_PATH_TO_ADDRESS" => Ok(backend_rule::PathTranslation::AppendPathToAddress),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Billing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.consumer_destinations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Billing", len)?;
        if !self.consumer_destinations.is_empty() {
            struct_ser.serialize_field("consumerDestinations", &self.consumer_destinations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Billing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consumer_destinations",
            "consumerDestinations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConsumerDestinations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "consumerDestinations" | "consumer_destinations" => Ok(GeneratedField::ConsumerDestinations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Billing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Billing")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Billing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consumer_destinations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConsumerDestinations => {
                            if consumer_destinations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumerDestinations"));
                            }
                            consumer_destinations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Billing {
                    consumer_destinations: consumer_destinations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Billing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for billing::BillingDestination {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.monitored_resource.is_empty() {
            len += 1;
        }
        if !self.metrics.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Billing.BillingDestination", len)?;
        if !self.monitored_resource.is_empty() {
            struct_ser.serialize_field("monitoredResource", &self.monitored_resource)?;
        }
        if !self.metrics.is_empty() {
            struct_ser.serialize_field("metrics", &self.metrics)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for billing::BillingDestination {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "monitored_resource",
            "monitoredResource",
            "metrics",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MonitoredResource,
            Metrics,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "monitoredResource" | "monitored_resource" => Ok(GeneratedField::MonitoredResource),
                            "metrics" => Ok(GeneratedField::Metrics),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = billing::BillingDestination;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Billing.BillingDestination")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<billing::BillingDestination, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut monitored_resource__ = None;
                let mut metrics__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MonitoredResource => {
                            if monitored_resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitoredResource"));
                            }
                            monitored_resource__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(billing::BillingDestination {
                    monitored_resource: monitored_resource__.unwrap_or_default(),
                    metrics: metrics__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Billing.BillingDestination", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChangeType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CHANGE_TYPE_UNSPECIFIED",
            Self::Added => "ADDED",
            Self::Removed => "REMOVED",
            Self::Modified => "MODIFIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ChangeType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CHANGE_TYPE_UNSPECIFIED",
            "ADDED",
            "REMOVED",
            "MODIFIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChangeType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CHANGE_TYPE_UNSPECIFIED" => Ok(ChangeType::Unspecified),
                    "ADDED" => Ok(ChangeType::Added),
                    "REMOVED" => Ok(ChangeType::Removed),
                    "MODIFIED" => Ok(ChangeType::Modified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ClientLibraryDestination {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CLIENT_LIBRARY_DESTINATION_UNSPECIFIED",
            Self::Github => "GITHUB",
            Self::PackageManager => "PACKAGE_MANAGER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ClientLibraryDestination {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CLIENT_LIBRARY_DESTINATION_UNSPECIFIED",
            "GITHUB",
            "PACKAGE_MANAGER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientLibraryDestination;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CLIENT_LIBRARY_DESTINATION_UNSPECIFIED" => Ok(ClientLibraryDestination::Unspecified),
                    "GITHUB" => Ok(ClientLibraryDestination::Github),
                    "PACKAGE_MANAGER" => Ok(ClientLibraryDestination::PackageManager),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ClientLibraryOrganization {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CLIENT_LIBRARY_ORGANIZATION_UNSPECIFIED",
            Self::Cloud => "CLOUD",
            Self::Ads => "ADS",
            Self::Photos => "PHOTOS",
            Self::StreetView => "STREET_VIEW",
            Self::Shopping => "SHOPPING",
            Self::Geo => "GEO",
            Self::GenerativeAi => "GENERATIVE_AI",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ClientLibraryOrganization {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CLIENT_LIBRARY_ORGANIZATION_UNSPECIFIED",
            "CLOUD",
            "ADS",
            "PHOTOS",
            "STREET_VIEW",
            "SHOPPING",
            "GEO",
            "GENERATIVE_AI",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientLibraryOrganization;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CLIENT_LIBRARY_ORGANIZATION_UNSPECIFIED" => Ok(ClientLibraryOrganization::Unspecified),
                    "CLOUD" => Ok(ClientLibraryOrganization::Cloud),
                    "ADS" => Ok(ClientLibraryOrganization::Ads),
                    "PHOTOS" => Ok(ClientLibraryOrganization::Photos),
                    "STREET_VIEW" => Ok(ClientLibraryOrganization::StreetView),
                    "SHOPPING" => Ok(ClientLibraryOrganization::Shopping),
                    "GEO" => Ok(ClientLibraryOrganization::Geo),
                    "GENERATIVE_AI" => Ok(ClientLibraryOrganization::GenerativeAi),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ClientLibrarySettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if self.launch_stage != 0 {
            len += 1;
        }
        if self.rest_numeric_enums {
            len += 1;
        }
        if self.java_settings.is_some() {
            len += 1;
        }
        if self.cpp_settings.is_some() {
            len += 1;
        }
        if self.php_settings.is_some() {
            len += 1;
        }
        if self.python_settings.is_some() {
            len += 1;
        }
        if self.node_settings.is_some() {
            len += 1;
        }
        if self.dotnet_settings.is_some() {
            len += 1;
        }
        if self.ruby_settings.is_some() {
            len += 1;
        }
        if self.go_settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.ClientLibrarySettings", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.launch_stage != 0 {
            let v = LaunchStage::try_from(self.launch_stage)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.launch_stage)))?;
            struct_ser.serialize_field("launchStage", &v)?;
        }
        if self.rest_numeric_enums {
            struct_ser.serialize_field("restNumericEnums", &self.rest_numeric_enums)?;
        }
        if let Some(v) = self.java_settings.as_ref() {
            struct_ser.serialize_field("javaSettings", v)?;
        }
        if let Some(v) = self.cpp_settings.as_ref() {
            struct_ser.serialize_field("cppSettings", v)?;
        }
        if let Some(v) = self.php_settings.as_ref() {
            struct_ser.serialize_field("phpSettings", v)?;
        }
        if let Some(v) = self.python_settings.as_ref() {
            struct_ser.serialize_field("pythonSettings", v)?;
        }
        if let Some(v) = self.node_settings.as_ref() {
            struct_ser.serialize_field("nodeSettings", v)?;
        }
        if let Some(v) = self.dotnet_settings.as_ref() {
            struct_ser.serialize_field("dotnetSettings", v)?;
        }
        if let Some(v) = self.ruby_settings.as_ref() {
            struct_ser.serialize_field("rubySettings", v)?;
        }
        if let Some(v) = self.go_settings.as_ref() {
            struct_ser.serialize_field("goSettings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientLibrarySettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "launch_stage",
            "launchStage",
            "rest_numeric_enums",
            "restNumericEnums",
            "java_settings",
            "javaSettings",
            "cpp_settings",
            "cppSettings",
            "php_settings",
            "phpSettings",
            "python_settings",
            "pythonSettings",
            "node_settings",
            "nodeSettings",
            "dotnet_settings",
            "dotnetSettings",
            "ruby_settings",
            "rubySettings",
            "go_settings",
            "goSettings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            LaunchStage,
            RestNumericEnums,
            JavaSettings,
            CppSettings,
            PhpSettings,
            PythonSettings,
            NodeSettings,
            DotnetSettings,
            RubySettings,
            GoSettings,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "launchStage" | "launch_stage" => Ok(GeneratedField::LaunchStage),
                            "restNumericEnums" | "rest_numeric_enums" => Ok(GeneratedField::RestNumericEnums),
                            "javaSettings" | "java_settings" => Ok(GeneratedField::JavaSettings),
                            "cppSettings" | "cpp_settings" => Ok(GeneratedField::CppSettings),
                            "phpSettings" | "php_settings" => Ok(GeneratedField::PhpSettings),
                            "pythonSettings" | "python_settings" => Ok(GeneratedField::PythonSettings),
                            "nodeSettings" | "node_settings" => Ok(GeneratedField::NodeSettings),
                            "dotnetSettings" | "dotnet_settings" => Ok(GeneratedField::DotnetSettings),
                            "rubySettings" | "ruby_settings" => Ok(GeneratedField::RubySettings),
                            "goSettings" | "go_settings" => Ok(GeneratedField::GoSettings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientLibrarySettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.ClientLibrarySettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClientLibrarySettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut launch_stage__ = None;
                let mut rest_numeric_enums__ = None;
                let mut java_settings__ = None;
                let mut cpp_settings__ = None;
                let mut php_settings__ = None;
                let mut python_settings__ = None;
                let mut node_settings__ = None;
                let mut dotnet_settings__ = None;
                let mut ruby_settings__ = None;
                let mut go_settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LaunchStage => {
                            if launch_stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("launchStage"));
                            }
                            launch_stage__ = Some(map_.next_value::<LaunchStage>()? as i32);
                        }
                        GeneratedField::RestNumericEnums => {
                            if rest_numeric_enums__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restNumericEnums"));
                            }
                            rest_numeric_enums__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JavaSettings => {
                            if java_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("javaSettings"));
                            }
                            java_settings__ = map_.next_value()?;
                        }
                        GeneratedField::CppSettings => {
                            if cpp_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cppSettings"));
                            }
                            cpp_settings__ = map_.next_value()?;
                        }
                        GeneratedField::PhpSettings => {
                            if php_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phpSettings"));
                            }
                            php_settings__ = map_.next_value()?;
                        }
                        GeneratedField::PythonSettings => {
                            if python_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pythonSettings"));
                            }
                            python_settings__ = map_.next_value()?;
                        }
                        GeneratedField::NodeSettings => {
                            if node_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeSettings"));
                            }
                            node_settings__ = map_.next_value()?;
                        }
                        GeneratedField::DotnetSettings => {
                            if dotnet_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dotnetSettings"));
                            }
                            dotnet_settings__ = map_.next_value()?;
                        }
                        GeneratedField::RubySettings => {
                            if ruby_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rubySettings"));
                            }
                            ruby_settings__ = map_.next_value()?;
                        }
                        GeneratedField::GoSettings => {
                            if go_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("goSettings"));
                            }
                            go_settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClientLibrarySettings {
                    version: version__.unwrap_or_default(),
                    launch_stage: launch_stage__.unwrap_or_default(),
                    rest_numeric_enums: rest_numeric_enums__.unwrap_or_default(),
                    java_settings: java_settings__,
                    cpp_settings: cpp_settings__,
                    php_settings: php_settings__,
                    python_settings: python_settings__,
                    node_settings: node_settings__,
                    dotnet_settings: dotnet_settings__,
                    ruby_settings: ruby_settings__,
                    go_settings: go_settings__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.ClientLibrarySettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommonLanguageSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reference_docs_uri.is_empty() {
            len += 1;
        }
        if !self.destinations.is_empty() {
            len += 1;
        }
        if self.selective_gapic_generation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.CommonLanguageSettings", len)?;
        if !self.reference_docs_uri.is_empty() {
            struct_ser.serialize_field("referenceDocsUri", &self.reference_docs_uri)?;
        }
        if !self.destinations.is_empty() {
            let v = self.destinations.iter().cloned().map(|v| {
                ClientLibraryDestination::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("destinations", &v)?;
        }
        if let Some(v) = self.selective_gapic_generation.as_ref() {
            struct_ser.serialize_field("selectiveGapicGeneration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommonLanguageSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reference_docs_uri",
            "referenceDocsUri",
            "destinations",
            "selective_gapic_generation",
            "selectiveGapicGeneration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReferenceDocsUri,
            Destinations,
            SelectiveGapicGeneration,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "referenceDocsUri" | "reference_docs_uri" => Ok(GeneratedField::ReferenceDocsUri),
                            "destinations" => Ok(GeneratedField::Destinations),
                            "selectiveGapicGeneration" | "selective_gapic_generation" => Ok(GeneratedField::SelectiveGapicGeneration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommonLanguageSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.CommonLanguageSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CommonLanguageSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reference_docs_uri__ = None;
                let mut destinations__ = None;
                let mut selective_gapic_generation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReferenceDocsUri => {
                            if reference_docs_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceDocsUri"));
                            }
                            reference_docs_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Destinations => {
                            if destinations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinations"));
                            }
                            destinations__ = Some(map_.next_value::<Vec<ClientLibraryDestination>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::SelectiveGapicGeneration => {
                            if selective_gapic_generation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selectiveGapicGeneration"));
                            }
                            selective_gapic_generation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CommonLanguageSettings {
                    reference_docs_uri: reference_docs_uri__.unwrap_or_default(),
                    destinations: destinations__.unwrap_or_default(),
                    selective_gapic_generation: selective_gapic_generation__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.CommonLanguageSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigChange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.element.is_empty() {
            len += 1;
        }
        if !self.old_value.is_empty() {
            len += 1;
        }
        if !self.new_value.is_empty() {
            len += 1;
        }
        if self.change_type != 0 {
            len += 1;
        }
        if !self.advices.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.ConfigChange", len)?;
        if !self.element.is_empty() {
            struct_ser.serialize_field("element", &self.element)?;
        }
        if !self.old_value.is_empty() {
            struct_ser.serialize_field("oldValue", &self.old_value)?;
        }
        if !self.new_value.is_empty() {
            struct_ser.serialize_field("newValue", &self.new_value)?;
        }
        if self.change_type != 0 {
            let v = ChangeType::try_from(self.change_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.change_type)))?;
            struct_ser.serialize_field("changeType", &v)?;
        }
        if !self.advices.is_empty() {
            struct_ser.serialize_field("advices", &self.advices)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigChange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "element",
            "old_value",
            "oldValue",
            "new_value",
            "newValue",
            "change_type",
            "changeType",
            "advices",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Element,
            OldValue,
            NewValue,
            ChangeType,
            Advices,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "element" => Ok(GeneratedField::Element),
                            "oldValue" | "old_value" => Ok(GeneratedField::OldValue),
                            "newValue" | "new_value" => Ok(GeneratedField::NewValue),
                            "changeType" | "change_type" => Ok(GeneratedField::ChangeType),
                            "advices" => Ok(GeneratedField::Advices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigChange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.ConfigChange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConfigChange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut element__ = None;
                let mut old_value__ = None;
                let mut new_value__ = None;
                let mut change_type__ = None;
                let mut advices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Element => {
                            if element__.is_some() {
                                return Err(serde::de::Error::duplicate_field("element"));
                            }
                            element__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OldValue => {
                            if old_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oldValue"));
                            }
                            old_value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewValue => {
                            if new_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newValue"));
                            }
                            new_value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChangeType => {
                            if change_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeType"));
                            }
                            change_type__ = Some(map_.next_value::<ChangeType>()? as i32);
                        }
                        GeneratedField::Advices => {
                            if advices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("advices"));
                            }
                            advices__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ConfigChange {
                    element: element__.unwrap_or_default(),
                    old_value: old_value__.unwrap_or_default(),
                    new_value: new_value__.unwrap_or_default(),
                    change_type: change_type__.unwrap_or_default(),
                    advices: advices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.ConfigChange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Context {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Context", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Context {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Context;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Context")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Context, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Context {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Context", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContextRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.requested.is_empty() {
            len += 1;
        }
        if !self.provided.is_empty() {
            len += 1;
        }
        if !self.allowed_request_extensions.is_empty() {
            len += 1;
        }
        if !self.allowed_response_extensions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.ContextRule", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.requested.is_empty() {
            struct_ser.serialize_field("requested", &self.requested)?;
        }
        if !self.provided.is_empty() {
            struct_ser.serialize_field("provided", &self.provided)?;
        }
        if !self.allowed_request_extensions.is_empty() {
            struct_ser.serialize_field("allowedRequestExtensions", &self.allowed_request_extensions)?;
        }
        if !self.allowed_response_extensions.is_empty() {
            struct_ser.serialize_field("allowedResponseExtensions", &self.allowed_response_extensions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContextRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "requested",
            "provided",
            "allowed_request_extensions",
            "allowedRequestExtensions",
            "allowed_response_extensions",
            "allowedResponseExtensions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            Requested,
            Provided,
            AllowedRequestExtensions,
            AllowedResponseExtensions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "requested" => Ok(GeneratedField::Requested),
                            "provided" => Ok(GeneratedField::Provided),
                            "allowedRequestExtensions" | "allowed_request_extensions" => Ok(GeneratedField::AllowedRequestExtensions),
                            "allowedResponseExtensions" | "allowed_response_extensions" => Ok(GeneratedField::AllowedResponseExtensions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContextRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.ContextRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ContextRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut requested__ = None;
                let mut provided__ = None;
                let mut allowed_request_extensions__ = None;
                let mut allowed_response_extensions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Requested => {
                            if requested__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requested"));
                            }
                            requested__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Provided => {
                            if provided__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provided"));
                            }
                            provided__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedRequestExtensions => {
                            if allowed_request_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedRequestExtensions"));
                            }
                            allowed_request_extensions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedResponseExtensions => {
                            if allowed_response_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedResponseExtensions"));
                            }
                            allowed_response_extensions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ContextRule {
                    selector: selector__.unwrap_or_default(),
                    requested: requested__.unwrap_or_default(),
                    provided: provided__.unwrap_or_default(),
                    allowed_request_extensions: allowed_request_extensions__.unwrap_or_default(),
                    allowed_response_extensions: allowed_response_extensions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.ContextRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Control {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.environment.is_empty() {
            len += 1;
        }
        if !self.method_policies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Control", len)?;
        if !self.environment.is_empty() {
            struct_ser.serialize_field("environment", &self.environment)?;
        }
        if !self.method_policies.is_empty() {
            struct_ser.serialize_field("methodPolicies", &self.method_policies)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Control {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "environment",
            "method_policies",
            "methodPolicies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Environment,
            MethodPolicies,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "environment" => Ok(GeneratedField::Environment),
                            "methodPolicies" | "method_policies" => Ok(GeneratedField::MethodPolicies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Control;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Control")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Control, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut environment__ = None;
                let mut method_policies__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Environment => {
                            if environment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("environment"));
                            }
                            environment__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MethodPolicies => {
                            if method_policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methodPolicies"));
                            }
                            method_policies__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Control {
                    environment: environment__.unwrap_or_default(),
                    method_policies: method_policies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Control", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CppSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.CppSettings", len)?;
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CppSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Common,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "common" => Ok(GeneratedField::Common),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CppSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.CppSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CppSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CppSettings {
                    common: common__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.CppSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CustomHttpPattern {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.CustomHttpPattern", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CustomHttpPattern {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Path,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kind" => Ok(GeneratedField::Kind),
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CustomHttpPattern;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.CustomHttpPattern")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CustomHttpPattern, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CustomHttpPattern {
                    kind: kind__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.CustomHttpPattern", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Distribution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.count != 0 {
            len += 1;
        }
        if self.mean != 0. {
            len += 1;
        }
        if self.sum_of_squared_deviation != 0. {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.bucket_options.is_some() {
            len += 1;
        }
        if !self.bucket_counts.is_empty() {
            len += 1;
        }
        if !self.exemplars.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Distribution", len)?;
        if self.count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        if self.mean != 0. {
            struct_ser.serialize_field("mean", &self.mean)?;
        }
        if self.sum_of_squared_deviation != 0. {
            struct_ser.serialize_field("sumOfSquaredDeviation", &self.sum_of_squared_deviation)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.bucket_options.as_ref() {
            struct_ser.serialize_field("bucketOptions", v)?;
        }
        if !self.bucket_counts.is_empty() {
            struct_ser.serialize_field("bucketCounts", &self.bucket_counts.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.exemplars.is_empty() {
            struct_ser.serialize_field("exemplars", &self.exemplars)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Distribution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "count",
            "mean",
            "sum_of_squared_deviation",
            "sumOfSquaredDeviation",
            "range",
            "bucket_options",
            "bucketOptions",
            "bucket_counts",
            "bucketCounts",
            "exemplars",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Count,
            Mean,
            SumOfSquaredDeviation,
            Range,
            BucketOptions,
            BucketCounts,
            Exemplars,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "count" => Ok(GeneratedField::Count),
                            "mean" => Ok(GeneratedField::Mean),
                            "sumOfSquaredDeviation" | "sum_of_squared_deviation" => Ok(GeneratedField::SumOfSquaredDeviation),
                            "range" => Ok(GeneratedField::Range),
                            "bucketOptions" | "bucket_options" => Ok(GeneratedField::BucketOptions),
                            "bucketCounts" | "bucket_counts" => Ok(GeneratedField::BucketCounts),
                            "exemplars" => Ok(GeneratedField::Exemplars),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Distribution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Distribution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Distribution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut count__ = None;
                let mut mean__ = None;
                let mut sum_of_squared_deviation__ = None;
                let mut range__ = None;
                let mut bucket_options__ = None;
                let mut bucket_counts__ = None;
                let mut exemplars__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mean => {
                            if mean__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mean"));
                            }
                            mean__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SumOfSquaredDeviation => {
                            if sum_of_squared_deviation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sumOfSquaredDeviation"));
                            }
                            sum_of_squared_deviation__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Range => {
                            if range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range__ = map_.next_value()?;
                        }
                        GeneratedField::BucketOptions => {
                            if bucket_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketOptions"));
                            }
                            bucket_options__ = map_.next_value()?;
                        }
                        GeneratedField::BucketCounts => {
                            if bucket_counts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketCounts"));
                            }
                            bucket_counts__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Exemplars => {
                            if exemplars__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exemplars"));
                            }
                            exemplars__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Distribution {
                    count: count__.unwrap_or_default(),
                    mean: mean__.unwrap_or_default(),
                    sum_of_squared_deviation: sum_of_squared_deviation__.unwrap_or_default(),
                    range: range__,
                    bucket_options: bucket_options__,
                    bucket_counts: bucket_counts__.unwrap_or_default(),
                    exemplars: exemplars__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Distribution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for distribution::BucketOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Distribution.BucketOptions", len)?;
        if let Some(v) = self.options.as_ref() {
            match v {
                distribution::bucket_options::Options::LinearBuckets(v) => {
                    struct_ser.serialize_field("linearBuckets", v)?;
                }
                distribution::bucket_options::Options::ExponentialBuckets(v) => {
                    struct_ser.serialize_field("exponentialBuckets", v)?;
                }
                distribution::bucket_options::Options::ExplicitBuckets(v) => {
                    struct_ser.serialize_field("explicitBuckets", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for distribution::BucketOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "linear_buckets",
            "linearBuckets",
            "exponential_buckets",
            "exponentialBuckets",
            "explicit_buckets",
            "explicitBuckets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LinearBuckets,
            ExponentialBuckets,
            ExplicitBuckets,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "linearBuckets" | "linear_buckets" => Ok(GeneratedField::LinearBuckets),
                            "exponentialBuckets" | "exponential_buckets" => Ok(GeneratedField::ExponentialBuckets),
                            "explicitBuckets" | "explicit_buckets" => Ok(GeneratedField::ExplicitBuckets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = distribution::BucketOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Distribution.BucketOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<distribution::BucketOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LinearBuckets => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linearBuckets"));
                            }
                            options__ = map_.next_value::<::std::option::Option<_>>()?.map(distribution::bucket_options::Options::LinearBuckets)
;
                        }
                        GeneratedField::ExponentialBuckets => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponentialBuckets"));
                            }
                            options__ = map_.next_value::<::std::option::Option<_>>()?.map(distribution::bucket_options::Options::ExponentialBuckets)
;
                        }
                        GeneratedField::ExplicitBuckets => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("explicitBuckets"));
                            }
                            options__ = map_.next_value::<::std::option::Option<_>>()?.map(distribution::bucket_options::Options::ExplicitBuckets)
;
                        }
                    }
                }
                Ok(distribution::BucketOptions {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.Distribution.BucketOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for distribution::bucket_options::Explicit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bounds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Distribution.BucketOptions.Explicit", len)?;
        if !self.bounds.is_empty() {
            struct_ser.serialize_field("bounds", &self.bounds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for distribution::bucket_options::Explicit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bounds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bounds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bounds" => Ok(GeneratedField::Bounds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = distribution::bucket_options::Explicit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Distribution.BucketOptions.Explicit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<distribution::bucket_options::Explicit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bounds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bounds => {
                            if bounds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bounds"));
                            }
                            bounds__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(distribution::bucket_options::Explicit {
                    bounds: bounds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Distribution.BucketOptions.Explicit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for distribution::bucket_options::Exponential {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.num_finite_buckets != 0 {
            len += 1;
        }
        if self.growth_factor != 0. {
            len += 1;
        }
        if self.scale != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Distribution.BucketOptions.Exponential", len)?;
        if self.num_finite_buckets != 0 {
            struct_ser.serialize_field("numFiniteBuckets", &self.num_finite_buckets)?;
        }
        if self.growth_factor != 0. {
            struct_ser.serialize_field("growthFactor", &self.growth_factor)?;
        }
        if self.scale != 0. {
            struct_ser.serialize_field("scale", &self.scale)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for distribution::bucket_options::Exponential {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "num_finite_buckets",
            "numFiniteBuckets",
            "growth_factor",
            "growthFactor",
            "scale",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NumFiniteBuckets,
            GrowthFactor,
            Scale,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "numFiniteBuckets" | "num_finite_buckets" => Ok(GeneratedField::NumFiniteBuckets),
                            "growthFactor" | "growth_factor" => Ok(GeneratedField::GrowthFactor),
                            "scale" => Ok(GeneratedField::Scale),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = distribution::bucket_options::Exponential;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Distribution.BucketOptions.Exponential")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<distribution::bucket_options::Exponential, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut num_finite_buckets__ = None;
                let mut growth_factor__ = None;
                let mut scale__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NumFiniteBuckets => {
                            if num_finite_buckets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numFiniteBuckets"));
                            }
                            num_finite_buckets__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GrowthFactor => {
                            if growth_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("growthFactor"));
                            }
                            growth_factor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Scale => {
                            if scale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scale"));
                            }
                            scale__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(distribution::bucket_options::Exponential {
                    num_finite_buckets: num_finite_buckets__.unwrap_or_default(),
                    growth_factor: growth_factor__.unwrap_or_default(),
                    scale: scale__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Distribution.BucketOptions.Exponential", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for distribution::bucket_options::Linear {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.num_finite_buckets != 0 {
            len += 1;
        }
        if self.width != 0. {
            len += 1;
        }
        if self.offset != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Distribution.BucketOptions.Linear", len)?;
        if self.num_finite_buckets != 0 {
            struct_ser.serialize_field("numFiniteBuckets", &self.num_finite_buckets)?;
        }
        if self.width != 0. {
            struct_ser.serialize_field("width", &self.width)?;
        }
        if self.offset != 0. {
            struct_ser.serialize_field("offset", &self.offset)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for distribution::bucket_options::Linear {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "num_finite_buckets",
            "numFiniteBuckets",
            "width",
            "offset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NumFiniteBuckets,
            Width,
            Offset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "numFiniteBuckets" | "num_finite_buckets" => Ok(GeneratedField::NumFiniteBuckets),
                            "width" => Ok(GeneratedField::Width),
                            "offset" => Ok(GeneratedField::Offset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = distribution::bucket_options::Linear;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Distribution.BucketOptions.Linear")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<distribution::bucket_options::Linear, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut num_finite_buckets__ = None;
                let mut width__ = None;
                let mut offset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NumFiniteBuckets => {
                            if num_finite_buckets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numFiniteBuckets"));
                            }
                            num_finite_buckets__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Width => {
                            if width__.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            width__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(distribution::bucket_options::Linear {
                    num_finite_buckets: num_finite_buckets__.unwrap_or_default(),
                    width: width__.unwrap_or_default(),
                    offset: offset__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Distribution.BucketOptions.Linear", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for distribution::Exemplar {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0. {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.attachments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Distribution.Exemplar", len)?;
        if self.value != 0. {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.attachments.is_empty() {
            struct_ser.serialize_field("attachments", &self.attachments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for distribution::Exemplar {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "timestamp",
            "attachments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Timestamp,
            Attachments,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "attachments" => Ok(GeneratedField::Attachments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = distribution::Exemplar;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Distribution.Exemplar")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<distribution::Exemplar, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut timestamp__ = None;
                let mut attachments__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::Attachments => {
                            if attachments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachments"));
                            }
                            attachments__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(distribution::Exemplar {
                    value: value__.unwrap_or_default(),
                    timestamp: timestamp__,
                    attachments: attachments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Distribution.Exemplar", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for distribution::Range {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min != 0. {
            len += 1;
        }
        if self.max != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Distribution.Range", len)?;
        if self.min != 0. {
            struct_ser.serialize_field("min", &self.min)?;
        }
        if self.max != 0. {
            struct_ser.serialize_field("max", &self.max)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for distribution::Range {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min",
            "max",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Min,
            Max,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "min" => Ok(GeneratedField::Min),
                            "max" => Ok(GeneratedField::Max),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = distribution::Range;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Distribution.Range")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<distribution::Range, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min__ = None;
                let mut max__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Min => {
                            if min__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            min__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Max => {
                            if max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            max__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(distribution::Range {
                    min: min__.unwrap_or_default(),
                    max: max__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Distribution.Range", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Documentation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.summary.is_empty() {
            len += 1;
        }
        if !self.pages.is_empty() {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        if !self.documentation_root_url.is_empty() {
            len += 1;
        }
        if !self.service_root_url.is_empty() {
            len += 1;
        }
        if !self.overview.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Documentation", len)?;
        if !self.summary.is_empty() {
            struct_ser.serialize_field("summary", &self.summary)?;
        }
        if !self.pages.is_empty() {
            struct_ser.serialize_field("pages", &self.pages)?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if !self.documentation_root_url.is_empty() {
            struct_ser.serialize_field("documentationRootUrl", &self.documentation_root_url)?;
        }
        if !self.service_root_url.is_empty() {
            struct_ser.serialize_field("serviceRootUrl", &self.service_root_url)?;
        }
        if !self.overview.is_empty() {
            struct_ser.serialize_field("overview", &self.overview)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Documentation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "summary",
            "pages",
            "rules",
            "documentation_root_url",
            "documentationRootUrl",
            "service_root_url",
            "serviceRootUrl",
            "overview",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Summary,
            Pages,
            Rules,
            DocumentationRootUrl,
            ServiceRootUrl,
            Overview,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "summary" => Ok(GeneratedField::Summary),
                            "pages" => Ok(GeneratedField::Pages),
                            "rules" => Ok(GeneratedField::Rules),
                            "documentationRootUrl" | "documentation_root_url" => Ok(GeneratedField::DocumentationRootUrl),
                            "serviceRootUrl" | "service_root_url" => Ok(GeneratedField::ServiceRootUrl),
                            "overview" => Ok(GeneratedField::Overview),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Documentation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Documentation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Documentation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut summary__ = None;
                let mut pages__ = None;
                let mut rules__ = None;
                let mut documentation_root_url__ = None;
                let mut service_root_url__ = None;
                let mut overview__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Summary => {
                            if summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summary"));
                            }
                            summary__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pages => {
                            if pages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pages"));
                            }
                            pages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DocumentationRootUrl => {
                            if documentation_root_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentationRootUrl"));
                            }
                            documentation_root_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceRootUrl => {
                            if service_root_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceRootUrl"));
                            }
                            service_root_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Overview => {
                            if overview__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overview"));
                            }
                            overview__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Documentation {
                    summary: summary__.unwrap_or_default(),
                    pages: pages__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                    documentation_root_url: documentation_root_url__.unwrap_or_default(),
                    service_root_url: service_root_url__.unwrap_or_default(),
                    overview: overview__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Documentation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DocumentationRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.deprecation_description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.DocumentationRule", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.deprecation_description.is_empty() {
            struct_ser.serialize_field("deprecationDescription", &self.deprecation_description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DocumentationRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "description",
            "deprecation_description",
            "deprecationDescription",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            Description,
            DeprecationDescription,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "description" => Ok(GeneratedField::Description),
                            "deprecationDescription" | "deprecation_description" => Ok(GeneratedField::DeprecationDescription),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DocumentationRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.DocumentationRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DocumentationRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut description__ = None;
                let mut deprecation_description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeprecationDescription => {
                            if deprecation_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecationDescription"));
                            }
                            deprecation_description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DocumentationRule {
                    selector: selector__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    deprecation_description: deprecation_description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.DocumentationRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DotnetSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common.is_some() {
            len += 1;
        }
        if !self.renamed_services.is_empty() {
            len += 1;
        }
        if !self.renamed_resources.is_empty() {
            len += 1;
        }
        if !self.ignored_resources.is_empty() {
            len += 1;
        }
        if !self.forced_namespace_aliases.is_empty() {
            len += 1;
        }
        if !self.handwritten_signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.DotnetSettings", len)?;
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        if !self.renamed_services.is_empty() {
            struct_ser.serialize_field("renamedServices", &self.renamed_services)?;
        }
        if !self.renamed_resources.is_empty() {
            struct_ser.serialize_field("renamedResources", &self.renamed_resources)?;
        }
        if !self.ignored_resources.is_empty() {
            struct_ser.serialize_field("ignoredResources", &self.ignored_resources)?;
        }
        if !self.forced_namespace_aliases.is_empty() {
            struct_ser.serialize_field("forcedNamespaceAliases", &self.forced_namespace_aliases)?;
        }
        if !self.handwritten_signatures.is_empty() {
            struct_ser.serialize_field("handwrittenSignatures", &self.handwritten_signatures)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DotnetSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common",
            "renamed_services",
            "renamedServices",
            "renamed_resources",
            "renamedResources",
            "ignored_resources",
            "ignoredResources",
            "forced_namespace_aliases",
            "forcedNamespaceAliases",
            "handwritten_signatures",
            "handwrittenSignatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Common,
            RenamedServices,
            RenamedResources,
            IgnoredResources,
            ForcedNamespaceAliases,
            HandwrittenSignatures,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "common" => Ok(GeneratedField::Common),
                            "renamedServices" | "renamed_services" => Ok(GeneratedField::RenamedServices),
                            "renamedResources" | "renamed_resources" => Ok(GeneratedField::RenamedResources),
                            "ignoredResources" | "ignored_resources" => Ok(GeneratedField::IgnoredResources),
                            "forcedNamespaceAliases" | "forced_namespace_aliases" => Ok(GeneratedField::ForcedNamespaceAliases),
                            "handwrittenSignatures" | "handwritten_signatures" => Ok(GeneratedField::HandwrittenSignatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DotnetSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.DotnetSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DotnetSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common__ = None;
                let mut renamed_services__ = None;
                let mut renamed_resources__ = None;
                let mut ignored_resources__ = None;
                let mut forced_namespace_aliases__ = None;
                let mut handwritten_signatures__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                        GeneratedField::RenamedServices => {
                            if renamed_services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("renamedServices"));
                            }
                            renamed_services__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::RenamedResources => {
                            if renamed_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("renamedResources"));
                            }
                            renamed_resources__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::IgnoredResources => {
                            if ignored_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoredResources"));
                            }
                            ignored_resources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ForcedNamespaceAliases => {
                            if forced_namespace_aliases__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forcedNamespaceAliases"));
                            }
                            forced_namespace_aliases__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HandwrittenSignatures => {
                            if handwritten_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("handwrittenSignatures"));
                            }
                            handwritten_signatures__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DotnetSettings {
                    common: common__,
                    renamed_services: renamed_services__.unwrap_or_default(),
                    renamed_resources: renamed_resources__.unwrap_or_default(),
                    ignored_resources: ignored_resources__.unwrap_or_default(),
                    forced_namespace_aliases: forced_namespace_aliases__.unwrap_or_default(),
                    handwritten_signatures: handwritten_signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.DotnetSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Endpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.aliases.is_empty() {
            len += 1;
        }
        if !self.target.is_empty() {
            len += 1;
        }
        if self.allow_cors {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Endpoint", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.aliases.is_empty() {
            struct_ser.serialize_field("aliases", &self.aliases)?;
        }
        if !self.target.is_empty() {
            struct_ser.serialize_field("target", &self.target)?;
        }
        if self.allow_cors {
            struct_ser.serialize_field("allowCors", &self.allow_cors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Endpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "aliases",
            "target",
            "allow_cors",
            "allowCors",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Aliases,
            Target,
            AllowCors,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "aliases" => Ok(GeneratedField::Aliases),
                            "target" => Ok(GeneratedField::Target),
                            "allowCors" | "allow_cors" => Ok(GeneratedField::AllowCors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Endpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Endpoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Endpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut aliases__ = None;
                let mut target__ = None;
                let mut allow_cors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Aliases => {
                            if aliases__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            aliases__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowCors => {
                            if allow_cors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowCors"));
                            }
                            allow_cors__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Endpoint {
                    name: name__.unwrap_or_default(),
                    aliases: aliases__.unwrap_or_default(),
                    target: target__.unwrap_or_default(),
                    allow_cors: allow_cors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Endpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ERROR_REASON_UNSPECIFIED",
            Self::ServiceDisabled => "SERVICE_DISABLED",
            Self::BillingDisabled => "BILLING_DISABLED",
            Self::ApiKeyInvalid => "API_KEY_INVALID",
            Self::ApiKeyServiceBlocked => "API_KEY_SERVICE_BLOCKED",
            Self::ApiKeyHttpReferrerBlocked => "API_KEY_HTTP_REFERRER_BLOCKED",
            Self::ApiKeyIpAddressBlocked => "API_KEY_IP_ADDRESS_BLOCKED",
            Self::ApiKeyAndroidAppBlocked => "API_KEY_ANDROID_APP_BLOCKED",
            Self::ApiKeyIosAppBlocked => "API_KEY_IOS_APP_BLOCKED",
            Self::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            Self::ResourceQuotaExceeded => "RESOURCE_QUOTA_EXCEEDED",
            Self::LocationTaxPolicyViolated => "LOCATION_TAX_POLICY_VIOLATED",
            Self::UserProjectDenied => "USER_PROJECT_DENIED",
            Self::ConsumerSuspended => "CONSUMER_SUSPENDED",
            Self::ConsumerInvalid => "CONSUMER_INVALID",
            Self::SecurityPolicyViolated => "SECURITY_POLICY_VIOLATED",
            Self::AccessTokenExpired => "ACCESS_TOKEN_EXPIRED",
            Self::AccessTokenScopeInsufficient => "ACCESS_TOKEN_SCOPE_INSUFFICIENT",
            Self::AccountStateInvalid => "ACCOUNT_STATE_INVALID",
            Self::AccessTokenTypeUnsupported => "ACCESS_TOKEN_TYPE_UNSUPPORTED",
            Self::CredentialsMissing => "CREDENTIALS_MISSING",
            Self::ResourceProjectInvalid => "RESOURCE_PROJECT_INVALID",
            Self::SessionCookieInvalid => "SESSION_COOKIE_INVALID",
            Self::UserBlockedByAdmin => "USER_BLOCKED_BY_ADMIN",
            Self::ResourceUsageRestrictionViolated => "RESOURCE_USAGE_RESTRICTION_VIOLATED",
            Self::SystemParameterUnsupported => "SYSTEM_PARAMETER_UNSUPPORTED",
            Self::OrgRestrictionViolation => "ORG_RESTRICTION_VIOLATION",
            Self::OrgRestrictionHeaderInvalid => "ORG_RESTRICTION_HEADER_INVALID",
            Self::ServiceNotVisible => "SERVICE_NOT_VISIBLE",
            Self::GcpSuspended => "GCP_SUSPENDED",
            Self::LocationPolicyViolated => "LOCATION_POLICY_VIOLATED",
            Self::MissingOrigin => "MISSING_ORIGIN",
            Self::OverloadedCredentials => "OVERLOADED_CREDENTIALS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ERROR_REASON_UNSPECIFIED",
            "SERVICE_DISABLED",
            "BILLING_DISABLED",
            "API_KEY_INVALID",
            "API_KEY_SERVICE_BLOCKED",
            "API_KEY_HTTP_REFERRER_BLOCKED",
            "API_KEY_IP_ADDRESS_BLOCKED",
            "API_KEY_ANDROID_APP_BLOCKED",
            "API_KEY_IOS_APP_BLOCKED",
            "RATE_LIMIT_EXCEEDED",
            "RESOURCE_QUOTA_EXCEEDED",
            "LOCATION_TAX_POLICY_VIOLATED",
            "USER_PROJECT_DENIED",
            "CONSUMER_SUSPENDED",
            "CONSUMER_INVALID",
            "SECURITY_POLICY_VIOLATED",
            "ACCESS_TOKEN_EXPIRED",
            "ACCESS_TOKEN_SCOPE_INSUFFICIENT",
            "ACCOUNT_STATE_INVALID",
            "ACCESS_TOKEN_TYPE_UNSUPPORTED",
            "CREDENTIALS_MISSING",
            "RESOURCE_PROJECT_INVALID",
            "SESSION_COOKIE_INVALID",
            "USER_BLOCKED_BY_ADMIN",
            "RESOURCE_USAGE_RESTRICTION_VIOLATED",
            "SYSTEM_PARAMETER_UNSUPPORTED",
            "ORG_RESTRICTION_VIOLATION",
            "ORG_RESTRICTION_HEADER_INVALID",
            "SERVICE_NOT_VISIBLE",
            "GCP_SUSPENDED",
            "LOCATION_POLICY_VIOLATED",
            "MISSING_ORIGIN",
            "OVERLOADED_CREDENTIALS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorReason;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ERROR_REASON_UNSPECIFIED" => Ok(ErrorReason::Unspecified),
                    "SERVICE_DISABLED" => Ok(ErrorReason::ServiceDisabled),
                    "BILLING_DISABLED" => Ok(ErrorReason::BillingDisabled),
                    "API_KEY_INVALID" => Ok(ErrorReason::ApiKeyInvalid),
                    "API_KEY_SERVICE_BLOCKED" => Ok(ErrorReason::ApiKeyServiceBlocked),
                    "API_KEY_HTTP_REFERRER_BLOCKED" => Ok(ErrorReason::ApiKeyHttpReferrerBlocked),
                    "API_KEY_IP_ADDRESS_BLOCKED" => Ok(ErrorReason::ApiKeyIpAddressBlocked),
                    "API_KEY_ANDROID_APP_BLOCKED" => Ok(ErrorReason::ApiKeyAndroidAppBlocked),
                    "API_KEY_IOS_APP_BLOCKED" => Ok(ErrorReason::ApiKeyIosAppBlocked),
                    "RATE_LIMIT_EXCEEDED" => Ok(ErrorReason::RateLimitExceeded),
                    "RESOURCE_QUOTA_EXCEEDED" => Ok(ErrorReason::ResourceQuotaExceeded),
                    "LOCATION_TAX_POLICY_VIOLATED" => Ok(ErrorReason::LocationTaxPolicyViolated),
                    "USER_PROJECT_DENIED" => Ok(ErrorReason::UserProjectDenied),
                    "CONSUMER_SUSPENDED" => Ok(ErrorReason::ConsumerSuspended),
                    "CONSUMER_INVALID" => Ok(ErrorReason::ConsumerInvalid),
                    "SECURITY_POLICY_VIOLATED" => Ok(ErrorReason::SecurityPolicyViolated),
                    "ACCESS_TOKEN_EXPIRED" => Ok(ErrorReason::AccessTokenExpired),
                    "ACCESS_TOKEN_SCOPE_INSUFFICIENT" => Ok(ErrorReason::AccessTokenScopeInsufficient),
                    "ACCOUNT_STATE_INVALID" => Ok(ErrorReason::AccountStateInvalid),
                    "ACCESS_TOKEN_TYPE_UNSUPPORTED" => Ok(ErrorReason::AccessTokenTypeUnsupported),
                    "CREDENTIALS_MISSING" => Ok(ErrorReason::CredentialsMissing),
                    "RESOURCE_PROJECT_INVALID" => Ok(ErrorReason::ResourceProjectInvalid),
                    "SESSION_COOKIE_INVALID" => Ok(ErrorReason::SessionCookieInvalid),
                    "USER_BLOCKED_BY_ADMIN" => Ok(ErrorReason::UserBlockedByAdmin),
                    "RESOURCE_USAGE_RESTRICTION_VIOLATED" => Ok(ErrorReason::ResourceUsageRestrictionViolated),
                    "SYSTEM_PARAMETER_UNSUPPORTED" => Ok(ErrorReason::SystemParameterUnsupported),
                    "ORG_RESTRICTION_VIOLATION" => Ok(ErrorReason::OrgRestrictionViolation),
                    "ORG_RESTRICTION_HEADER_INVALID" => Ok(ErrorReason::OrgRestrictionHeaderInvalid),
                    "SERVICE_NOT_VISIBLE" => Ok(ErrorReason::ServiceNotVisible),
                    "GCP_SUSPENDED" => Ok(ErrorReason::GcpSuspended),
                    "LOCATION_POLICY_VIOLATED" => Ok(ErrorReason::LocationPolicyViolated),
                    "MISSING_ORIGIN" => Ok(ErrorReason::MissingOrigin),
                    "OVERLOADED_CREDENTIALS" => Ok(ErrorReason::OverloadedCredentials),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FieldBehavior {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FIELD_BEHAVIOR_UNSPECIFIED",
            Self::Optional => "OPTIONAL",
            Self::Required => "REQUIRED",
            Self::OutputOnly => "OUTPUT_ONLY",
            Self::InputOnly => "INPUT_ONLY",
            Self::Immutable => "IMMUTABLE",
            Self::UnorderedList => "UNORDERED_LIST",
            Self::NonEmptyDefault => "NON_EMPTY_DEFAULT",
            Self::Identifier => "IDENTIFIER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FieldBehavior {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FIELD_BEHAVIOR_UNSPECIFIED",
            "OPTIONAL",
            "REQUIRED",
            "OUTPUT_ONLY",
            "INPUT_ONLY",
            "IMMUTABLE",
            "UNORDERED_LIST",
            "NON_EMPTY_DEFAULT",
            "IDENTIFIER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldBehavior;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FIELD_BEHAVIOR_UNSPECIFIED" => Ok(FieldBehavior::Unspecified),
                    "OPTIONAL" => Ok(FieldBehavior::Optional),
                    "REQUIRED" => Ok(FieldBehavior::Required),
                    "OUTPUT_ONLY" => Ok(FieldBehavior::OutputOnly),
                    "INPUT_ONLY" => Ok(FieldBehavior::InputOnly),
                    "IMMUTABLE" => Ok(FieldBehavior::Immutable),
                    "UNORDERED_LIST" => Ok(FieldBehavior::UnorderedList),
                    "NON_EMPTY_DEFAULT" => Ok(FieldBehavior::NonEmptyDefault),
                    "IDENTIFIER" => Ok(FieldBehavior::Identifier),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FieldInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.format != 0 {
            len += 1;
        }
        if !self.referenced_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.FieldInfo", len)?;
        if self.format != 0 {
            let v = field_info::Format::try_from(self.format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        if !self.referenced_types.is_empty() {
            struct_ser.serialize_field("referencedTypes", &self.referenced_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "format",
            "referenced_types",
            "referencedTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Format,
            ReferencedTypes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "format" => Ok(GeneratedField::Format),
                            "referencedTypes" | "referenced_types" => Ok(GeneratedField::ReferencedTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.FieldInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut format__ = None;
                let mut referenced_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value::<field_info::Format>()? as i32);
                        }
                        GeneratedField::ReferencedTypes => {
                            if referenced_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referencedTypes"));
                            }
                            referenced_types__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FieldInfo {
                    format: format__.unwrap_or_default(),
                    referenced_types: referenced_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.FieldInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for field_info::Format {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FORMAT_UNSPECIFIED",
            Self::Uuid4 => "UUID4",
            Self::Ipv4 => "IPV4",
            Self::Ipv6 => "IPV6",
            Self::Ipv4OrIpv6 => "IPV4_OR_IPV6",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for field_info::Format {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FORMAT_UNSPECIFIED",
            "UUID4",
            "IPV4",
            "IPV6",
            "IPV4_OR_IPV6",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = field_info::Format;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FORMAT_UNSPECIFIED" => Ok(field_info::Format::Unspecified),
                    "UUID4" => Ok(field_info::Format::Uuid4),
                    "IPV4" => Ok(field_info::Format::Ipv4),
                    "IPV6" => Ok(field_info::Format::Ipv6),
                    "IPV4_OR_IPV6" => Ok(field_info::Format::Ipv4OrIpv6),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FieldPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.resource_permission.is_empty() {
            len += 1;
        }
        if !self.resource_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.FieldPolicy", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.resource_permission.is_empty() {
            struct_ser.serialize_field("resourcePermission", &self.resource_permission)?;
        }
        if !self.resource_type.is_empty() {
            struct_ser.serialize_field("resourceType", &self.resource_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "resource_permission",
            "resourcePermission",
            "resource_type",
            "resourceType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            ResourcePermission,
            ResourceType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "resourcePermission" | "resource_permission" => Ok(GeneratedField::ResourcePermission),
                            "resourceType" | "resource_type" => Ok(GeneratedField::ResourceType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.FieldPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut resource_permission__ = None;
                let mut resource_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourcePermission => {
                            if resource_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourcePermission"));
                            }
                            resource_permission__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceType"));
                            }
                            resource_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FieldPolicy {
                    selector: selector__.unwrap_or_default(),
                    resource_permission: resource_permission__.unwrap_or_default(),
                    resource_type: resource_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.FieldPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GoSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.GoSettings", len)?;
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GoSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Common,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "common" => Ok(GeneratedField::Common),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GoSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.GoSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GoSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GoSettings {
                    common: common__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.GoSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Http {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        if self.fully_decode_reserved_expansion {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Http", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if self.fully_decode_reserved_expansion {
            struct_ser.serialize_field("fullyDecodeReservedExpansion", &self.fully_decode_reserved_expansion)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Http {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
            "fully_decode_reserved_expansion",
            "fullyDecodeReservedExpansion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
            FullyDecodeReservedExpansion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rules" => Ok(GeneratedField::Rules),
                            "fullyDecodeReservedExpansion" | "fully_decode_reserved_expansion" => Ok(GeneratedField::FullyDecodeReservedExpansion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Http;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Http")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Http, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                let mut fully_decode_reserved_expansion__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FullyDecodeReservedExpansion => {
                            if fully_decode_reserved_expansion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullyDecodeReservedExpansion"));
                            }
                            fully_decode_reserved_expansion__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Http {
                    rules: rules__.unwrap_or_default(),
                    fully_decode_reserved_expansion: fully_decode_reserved_expansion__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Http", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpBody {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.content_type.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.extensions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.HttpBody", len)?;
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if !self.extensions.is_empty() {
            struct_ser.serialize_field("extensions", &self.extensions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpBody {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "content_type",
            "contentType",
            "data",
            "extensions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContentType,
            Data,
            Extensions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
                            "data" => Ok(GeneratedField::Data),
                            "extensions" => Ok(GeneratedField::Extensions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpBody;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.HttpBody")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpBody, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut content_type__ = None;
                let mut data__ = None;
                let mut extensions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Extensions => {
                            if extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            extensions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HttpBody {
                    content_type: content_type__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    extensions: extensions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.HttpBody", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.body.is_empty() {
            len += 1;
        }
        if !self.response_body.is_empty() {
            len += 1;
        }
        if !self.additional_bindings.is_empty() {
            len += 1;
        }
        if self.pattern.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.HttpRule", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.body.is_empty() {
            struct_ser.serialize_field("body", &self.body)?;
        }
        if !self.response_body.is_empty() {
            struct_ser.serialize_field("responseBody", &self.response_body)?;
        }
        if !self.additional_bindings.is_empty() {
            struct_ser.serialize_field("additionalBindings", &self.additional_bindings)?;
        }
        if let Some(v) = self.pattern.as_ref() {
            match v {
                http_rule::Pattern::Get(v) => {
                    struct_ser.serialize_field("get", v)?;
                }
                http_rule::Pattern::Put(v) => {
                    struct_ser.serialize_field("put", v)?;
                }
                http_rule::Pattern::Post(v) => {
                    struct_ser.serialize_field("post", v)?;
                }
                http_rule::Pattern::Delete(v) => {
                    struct_ser.serialize_field("delete", v)?;
                }
                http_rule::Pattern::Patch(v) => {
                    struct_ser.serialize_field("patch", v)?;
                }
                http_rule::Pattern::Custom(v) => {
                    struct_ser.serialize_field("custom", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "body",
            "response_body",
            "responseBody",
            "additional_bindings",
            "additionalBindings",
            "get",
            "put",
            "post",
            "delete",
            "patch",
            "custom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            Body,
            ResponseBody,
            AdditionalBindings,
            Get,
            Put,
            Post,
            Delete,
            Patch,
            Custom,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "body" => Ok(GeneratedField::Body),
                            "responseBody" | "response_body" => Ok(GeneratedField::ResponseBody),
                            "additionalBindings" | "additional_bindings" => Ok(GeneratedField::AdditionalBindings),
                            "get" => Ok(GeneratedField::Get),
                            "put" => Ok(GeneratedField::Put),
                            "post" => Ok(GeneratedField::Post),
                            "delete" => Ok(GeneratedField::Delete),
                            "patch" => Ok(GeneratedField::Patch),
                            "custom" => Ok(GeneratedField::Custom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.HttpRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut body__ = None;
                let mut response_body__ = None;
                let mut additional_bindings__ = None;
                let mut pattern__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseBody => {
                            if response_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseBody"));
                            }
                            response_body__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdditionalBindings => {
                            if additional_bindings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalBindings"));
                            }
                            additional_bindings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Get => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("get"));
                            }
                            pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(http_rule::Pattern::Get);
                        }
                        GeneratedField::Put => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("put"));
                            }
                            pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(http_rule::Pattern::Put);
                        }
                        GeneratedField::Post => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("post"));
                            }
                            pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(http_rule::Pattern::Post);
                        }
                        GeneratedField::Delete => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delete"));
                            }
                            pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(http_rule::Pattern::Delete);
                        }
                        GeneratedField::Patch => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("patch"));
                            }
                            pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(http_rule::Pattern::Patch);
                        }
                        GeneratedField::Custom => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("custom"));
                            }
                            pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(http_rule::Pattern::Custom)
;
                        }
                    }
                }
                Ok(HttpRule {
                    selector: selector__.unwrap_or_default(),
                    body: body__.unwrap_or_default(),
                    response_body: response_body__.unwrap_or_default(),
                    additional_bindings: additional_bindings__.unwrap_or_default(),
                    pattern: pattern__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.HttpRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JavaSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.library_package.is_empty() {
            len += 1;
        }
        if !self.service_class_names.is_empty() {
            len += 1;
        }
        if self.common.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.JavaSettings", len)?;
        if !self.library_package.is_empty() {
            struct_ser.serialize_field("libraryPackage", &self.library_package)?;
        }
        if !self.service_class_names.is_empty() {
            struct_ser.serialize_field("serviceClassNames", &self.service_class_names)?;
        }
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JavaSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "library_package",
            "libraryPackage",
            "service_class_names",
            "serviceClassNames",
            "common",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LibraryPackage,
            ServiceClassNames,
            Common,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "libraryPackage" | "library_package" => Ok(GeneratedField::LibraryPackage),
                            "serviceClassNames" | "service_class_names" => Ok(GeneratedField::ServiceClassNames),
                            "common" => Ok(GeneratedField::Common),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JavaSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.JavaSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JavaSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut library_package__ = None;
                let mut service_class_names__ = None;
                let mut common__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LibraryPackage => {
                            if library_package__.is_some() {
                                return Err(serde::de::Error::duplicate_field("libraryPackage"));
                            }
                            library_package__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceClassNames => {
                            if service_class_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceClassNames"));
                            }
                            service_class_names__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                    }
                }
                Ok(JavaSettings {
                    library_package: library_package__.unwrap_or_default(),
                    service_class_names: service_class_names__.unwrap_or_default(),
                    common: common__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.JavaSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwtLocation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value_prefix.is_empty() {
            len += 1;
        }
        if self.r#in.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.JwtLocation", len)?;
        if !self.value_prefix.is_empty() {
            struct_ser.serialize_field("valuePrefix", &self.value_prefix)?;
        }
        if let Some(v) = self.r#in.as_ref() {
            match v {
                jwt_location::In::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                jwt_location::In::Query(v) => {
                    struct_ser.serialize_field("query", v)?;
                }
                jwt_location::In::Cookie(v) => {
                    struct_ser.serialize_field("cookie", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwtLocation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_prefix",
            "valuePrefix",
            "header",
            "query",
            "cookie",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValuePrefix,
            Header,
            Query,
            Cookie,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valuePrefix" | "value_prefix" => Ok(GeneratedField::ValuePrefix),
                            "header" => Ok(GeneratedField::Header),
                            "query" => Ok(GeneratedField::Query),
                            "cookie" => Ok(GeneratedField::Cookie),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwtLocation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.JwtLocation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JwtLocation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_prefix__ = None;
                let mut r#in__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValuePrefix => {
                            if value_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePrefix"));
                            }
                            value_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Header => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            r#in__ = map_.next_value::<::std::option::Option<_>>()?.map(jwt_location::In::Header);
                        }
                        GeneratedField::Query => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            r#in__ = map_.next_value::<::std::option::Option<_>>()?.map(jwt_location::In::Query);
                        }
                        GeneratedField::Cookie => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cookie"));
                            }
                            r#in__ = map_.next_value::<::std::option::Option<_>>()?.map(jwt_location::In::Cookie);
                        }
                    }
                }
                Ok(JwtLocation {
                    value_prefix: value_prefix__.unwrap_or_default(),
                    r#in: r#in__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.JwtLocation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LabelDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.value_type != 0 {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.LabelDescriptor", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if self.value_type != 0 {
            let v = label_descriptor::ValueType::try_from(self.value_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.value_type)))?;
            struct_ser.serialize_field("valueType", &v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LabelDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value_type",
            "valueType",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            ValueType,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "valueType" | "value_type" => Ok(GeneratedField::ValueType),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LabelDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.LabelDescriptor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LabelDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value_type__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueType => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueType"));
                            }
                            value_type__ = Some(map_.next_value::<label_descriptor::ValueType>()? as i32);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LabelDescriptor {
                    key: key__.unwrap_or_default(),
                    value_type: value_type__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.LabelDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for label_descriptor::ValueType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::String => "STRING",
            Self::Bool => "BOOL",
            Self::Int64 => "INT64",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for label_descriptor::ValueType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STRING",
            "BOOL",
            "INT64",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = label_descriptor::ValueType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STRING" => Ok(label_descriptor::ValueType::String),
                    "BOOL" => Ok(label_descriptor::ValueType::Bool),
                    "INT64" => Ok(label_descriptor::ValueType::Int64),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LaunchStage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "LAUNCH_STAGE_UNSPECIFIED",
            Self::Unimplemented => "UNIMPLEMENTED",
            Self::Prelaunch => "PRELAUNCH",
            Self::EarlyAccess => "EARLY_ACCESS",
            Self::Alpha => "ALPHA",
            Self::Beta => "BETA",
            Self::Ga => "GA",
            Self::Deprecated => "DEPRECATED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for LaunchStage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LAUNCH_STAGE_UNSPECIFIED",
            "UNIMPLEMENTED",
            "PRELAUNCH",
            "EARLY_ACCESS",
            "ALPHA",
            "BETA",
            "GA",
            "DEPRECATED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LaunchStage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "LAUNCH_STAGE_UNSPECIFIED" => Ok(LaunchStage::Unspecified),
                    "UNIMPLEMENTED" => Ok(LaunchStage::Unimplemented),
                    "PRELAUNCH" => Ok(LaunchStage::Prelaunch),
                    "EARLY_ACCESS" => Ok(LaunchStage::EarlyAccess),
                    "ALPHA" => Ok(LaunchStage::Alpha),
                    "BETA" => Ok(LaunchStage::Beta),
                    "GA" => Ok(LaunchStage::Ga),
                    "DEPRECATED" => Ok(LaunchStage::Deprecated),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LogDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.LogDescriptor", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "labels",
            "description",
            "display_name",
            "displayName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Labels,
            Description,
            DisplayName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "labels" => Ok(GeneratedField::Labels),
                            "description" => Ok(GeneratedField::Description),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.LogDescriptor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut labels__ = None;
                let mut description__ = None;
                let mut display_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LogDescriptor {
                    name: name__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.LogDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Logging {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.producer_destinations.is_empty() {
            len += 1;
        }
        if !self.consumer_destinations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Logging", len)?;
        if !self.producer_destinations.is_empty() {
            struct_ser.serialize_field("producerDestinations", &self.producer_destinations)?;
        }
        if !self.consumer_destinations.is_empty() {
            struct_ser.serialize_field("consumerDestinations", &self.consumer_destinations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Logging {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "producer_destinations",
            "producerDestinations",
            "consumer_destinations",
            "consumerDestinations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProducerDestinations,
            ConsumerDestinations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "producerDestinations" | "producer_destinations" => Ok(GeneratedField::ProducerDestinations),
                            "consumerDestinations" | "consumer_destinations" => Ok(GeneratedField::ConsumerDestinations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Logging;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Logging")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Logging, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut producer_destinations__ = None;
                let mut consumer_destinations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProducerDestinations => {
                            if producer_destinations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("producerDestinations"));
                            }
                            producer_destinations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsumerDestinations => {
                            if consumer_destinations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumerDestinations"));
                            }
                            consumer_destinations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Logging {
                    producer_destinations: producer_destinations__.unwrap_or_default(),
                    consumer_destinations: consumer_destinations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Logging", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for logging::LoggingDestination {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.monitored_resource.is_empty() {
            len += 1;
        }
        if !self.logs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Logging.LoggingDestination", len)?;
        if !self.monitored_resource.is_empty() {
            struct_ser.serialize_field("monitoredResource", &self.monitored_resource)?;
        }
        if !self.logs.is_empty() {
            struct_ser.serialize_field("logs", &self.logs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for logging::LoggingDestination {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "monitored_resource",
            "monitoredResource",
            "logs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MonitoredResource,
            Logs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "monitoredResource" | "monitored_resource" => Ok(GeneratedField::MonitoredResource),
                            "logs" => Ok(GeneratedField::Logs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = logging::LoggingDestination;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Logging.LoggingDestination")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<logging::LoggingDestination, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut monitored_resource__ = None;
                let mut logs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MonitoredResource => {
                            if monitored_resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitoredResource"));
                            }
                            monitored_resource__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Logs => {
                            if logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logs"));
                            }
                            logs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(logging::LoggingDestination {
                    monitored_resource: monitored_resource__.unwrap_or_default(),
                    logs: logs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Logging.LoggingDestination", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MethodPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.request_policies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.MethodPolicy", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.request_policies.is_empty() {
            struct_ser.serialize_field("requestPolicies", &self.request_policies)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MethodPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "request_policies",
            "requestPolicies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            RequestPolicies,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "requestPolicies" | "request_policies" => Ok(GeneratedField::RequestPolicies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MethodPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.MethodPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MethodPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut request_policies__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestPolicies => {
                            if request_policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestPolicies"));
                            }
                            request_policies__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MethodPolicy {
                    selector: selector__.unwrap_or_default(),
                    request_policies: request_policies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.MethodPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MethodSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if self.long_running.is_some() {
            len += 1;
        }
        if !self.auto_populated_fields.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.MethodSettings", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if let Some(v) = self.long_running.as_ref() {
            struct_ser.serialize_field("longRunning", v)?;
        }
        if !self.auto_populated_fields.is_empty() {
            struct_ser.serialize_field("autoPopulatedFields", &self.auto_populated_fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MethodSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "long_running",
            "longRunning",
            "auto_populated_fields",
            "autoPopulatedFields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            LongRunning,
            AutoPopulatedFields,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "longRunning" | "long_running" => Ok(GeneratedField::LongRunning),
                            "autoPopulatedFields" | "auto_populated_fields" => Ok(GeneratedField::AutoPopulatedFields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MethodSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.MethodSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MethodSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut long_running__ = None;
                let mut auto_populated_fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LongRunning => {
                            if long_running__.is_some() {
                                return Err(serde::de::Error::duplicate_field("longRunning"));
                            }
                            long_running__ = map_.next_value()?;
                        }
                        GeneratedField::AutoPopulatedFields => {
                            if auto_populated_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoPopulatedFields"));
                            }
                            auto_populated_fields__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MethodSettings {
                    selector: selector__.unwrap_or_default(),
                    long_running: long_running__,
                    auto_populated_fields: auto_populated_fields__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.MethodSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for method_settings::LongRunning {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.initial_poll_delay.is_some() {
            len += 1;
        }
        if self.poll_delay_multiplier != 0. {
            len += 1;
        }
        if self.max_poll_delay.is_some() {
            len += 1;
        }
        if self.total_poll_timeout.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.MethodSettings.LongRunning", len)?;
        if let Some(v) = self.initial_poll_delay.as_ref() {
            struct_ser.serialize_field("initialPollDelay", v)?;
        }
        if self.poll_delay_multiplier != 0. {
            struct_ser.serialize_field("pollDelayMultiplier", &self.poll_delay_multiplier)?;
        }
        if let Some(v) = self.max_poll_delay.as_ref() {
            struct_ser.serialize_field("maxPollDelay", v)?;
        }
        if let Some(v) = self.total_poll_timeout.as_ref() {
            struct_ser.serialize_field("totalPollTimeout", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for method_settings::LongRunning {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "initial_poll_delay",
            "initialPollDelay",
            "poll_delay_multiplier",
            "pollDelayMultiplier",
            "max_poll_delay",
            "maxPollDelay",
            "total_poll_timeout",
            "totalPollTimeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InitialPollDelay,
            PollDelayMultiplier,
            MaxPollDelay,
            TotalPollTimeout,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "initialPollDelay" | "initial_poll_delay" => Ok(GeneratedField::InitialPollDelay),
                            "pollDelayMultiplier" | "poll_delay_multiplier" => Ok(GeneratedField::PollDelayMultiplier),
                            "maxPollDelay" | "max_poll_delay" => Ok(GeneratedField::MaxPollDelay),
                            "totalPollTimeout" | "total_poll_timeout" => Ok(GeneratedField::TotalPollTimeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = method_settings::LongRunning;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.MethodSettings.LongRunning")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<method_settings::LongRunning, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut initial_poll_delay__ = None;
                let mut poll_delay_multiplier__ = None;
                let mut max_poll_delay__ = None;
                let mut total_poll_timeout__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InitialPollDelay => {
                            if initial_poll_delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialPollDelay"));
                            }
                            initial_poll_delay__ = map_.next_value()?;
                        }
                        GeneratedField::PollDelayMultiplier => {
                            if poll_delay_multiplier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pollDelayMultiplier"));
                            }
                            poll_delay_multiplier__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxPollDelay => {
                            if max_poll_delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPollDelay"));
                            }
                            max_poll_delay__ = map_.next_value()?;
                        }
                        GeneratedField::TotalPollTimeout => {
                            if total_poll_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalPollTimeout"));
                            }
                            total_poll_timeout__ = map_.next_value()?;
                        }
                    }
                }
                Ok(method_settings::LongRunning {
                    initial_poll_delay: initial_poll_delay__,
                    poll_delay_multiplier: poll_delay_multiplier__.unwrap_or_default(),
                    max_poll_delay: max_poll_delay__,
                    total_poll_timeout: total_poll_timeout__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.MethodSettings.LongRunning", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Metric {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Metric", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Metric {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "labels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Labels,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "labels" => Ok(GeneratedField::Labels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Metric;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Metric")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Metric, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut labels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(Metric {
                    r#type: r#type__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Metric", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetricDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if self.metric_kind != 0 {
            len += 1;
        }
        if self.value_type != 0 {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.launch_stage != 0 {
            len += 1;
        }
        if !self.monitored_resource_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.MetricDescriptor", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if self.metric_kind != 0 {
            let v = metric_descriptor::MetricKind::try_from(self.metric_kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.metric_kind)))?;
            struct_ser.serialize_field("metricKind", &v)?;
        }
        if self.value_type != 0 {
            let v = metric_descriptor::ValueType::try_from(self.value_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.value_type)))?;
            struct_ser.serialize_field("valueType", &v)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.launch_stage != 0 {
            let v = LaunchStage::try_from(self.launch_stage)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.launch_stage)))?;
            struct_ser.serialize_field("launchStage", &v)?;
        }
        if !self.monitored_resource_types.is_empty() {
            struct_ser.serialize_field("monitoredResourceTypes", &self.monitored_resource_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetricDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type",
            "labels",
            "metric_kind",
            "metricKind",
            "value_type",
            "valueType",
            "unit",
            "description",
            "display_name",
            "displayName",
            "metadata",
            "launch_stage",
            "launchStage",
            "monitored_resource_types",
            "monitoredResourceTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Type,
            Labels,
            MetricKind,
            ValueType,
            Unit,
            Description,
            DisplayName,
            Metadata,
            LaunchStage,
            MonitoredResourceTypes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "type" => Ok(GeneratedField::Type),
                            "labels" => Ok(GeneratedField::Labels),
                            "metricKind" | "metric_kind" => Ok(GeneratedField::MetricKind),
                            "valueType" | "value_type" => Ok(GeneratedField::ValueType),
                            "unit" => Ok(GeneratedField::Unit),
                            "description" => Ok(GeneratedField::Description),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "launchStage" | "launch_stage" => Ok(GeneratedField::LaunchStage),
                            "monitoredResourceTypes" | "monitored_resource_types" => Ok(GeneratedField::MonitoredResourceTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetricDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.MetricDescriptor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetricDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                let mut labels__ = None;
                let mut metric_kind__ = None;
                let mut value_type__ = None;
                let mut unit__ = None;
                let mut description__ = None;
                let mut display_name__ = None;
                let mut metadata__ = None;
                let mut launch_stage__ = None;
                let mut monitored_resource_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetricKind => {
                            if metric_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricKind"));
                            }
                            metric_kind__ = Some(map_.next_value::<metric_descriptor::MetricKind>()? as i32);
                        }
                        GeneratedField::ValueType => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueType"));
                            }
                            value_type__ = Some(map_.next_value::<metric_descriptor::ValueType>()? as i32);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::LaunchStage => {
                            if launch_stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("launchStage"));
                            }
                            launch_stage__ = Some(map_.next_value::<LaunchStage>()? as i32);
                        }
                        GeneratedField::MonitoredResourceTypes => {
                            if monitored_resource_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitoredResourceTypes"));
                            }
                            monitored_resource_types__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MetricDescriptor {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                    metric_kind: metric_kind__.unwrap_or_default(),
                    value_type: value_type__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    metadata: metadata__,
                    launch_stage: launch_stage__.unwrap_or_default(),
                    monitored_resource_types: monitored_resource_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.MetricDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for metric_descriptor::MetricDescriptorMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.launch_stage != 0 {
            len += 1;
        }
        if self.sample_period.is_some() {
            len += 1;
        }
        if self.ingest_delay.is_some() {
            len += 1;
        }
        if !self.time_series_resource_hierarchy_level.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.MetricDescriptor.MetricDescriptorMetadata", len)?;
        if self.launch_stage != 0 {
            let v = LaunchStage::try_from(self.launch_stage)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.launch_stage)))?;
            struct_ser.serialize_field("launchStage", &v)?;
        }
        if let Some(v) = self.sample_period.as_ref() {
            struct_ser.serialize_field("samplePeriod", v)?;
        }
        if let Some(v) = self.ingest_delay.as_ref() {
            struct_ser.serialize_field("ingestDelay", v)?;
        }
        if !self.time_series_resource_hierarchy_level.is_empty() {
            let v = self.time_series_resource_hierarchy_level.iter().cloned().map(|v| {
                metric_descriptor::metric_descriptor_metadata::TimeSeriesResourceHierarchyLevel::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("timeSeriesResourceHierarchyLevel", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for metric_descriptor::MetricDescriptorMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "launch_stage",
            "launchStage",
            "sample_period",
            "samplePeriod",
            "ingest_delay",
            "ingestDelay",
            "time_series_resource_hierarchy_level",
            "timeSeriesResourceHierarchyLevel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LaunchStage,
            SamplePeriod,
            IngestDelay,
            TimeSeriesResourceHierarchyLevel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "launchStage" | "launch_stage" => Ok(GeneratedField::LaunchStage),
                            "samplePeriod" | "sample_period" => Ok(GeneratedField::SamplePeriod),
                            "ingestDelay" | "ingest_delay" => Ok(GeneratedField::IngestDelay),
                            "timeSeriesResourceHierarchyLevel" | "time_series_resource_hierarchy_level" => Ok(GeneratedField::TimeSeriesResourceHierarchyLevel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = metric_descriptor::MetricDescriptorMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.MetricDescriptor.MetricDescriptorMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<metric_descriptor::MetricDescriptorMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut launch_stage__ = None;
                let mut sample_period__ = None;
                let mut ingest_delay__ = None;
                let mut time_series_resource_hierarchy_level__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LaunchStage => {
                            if launch_stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("launchStage"));
                            }
                            launch_stage__ = Some(map_.next_value::<LaunchStage>()? as i32);
                        }
                        GeneratedField::SamplePeriod => {
                            if sample_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplePeriod"));
                            }
                            sample_period__ = map_.next_value()?;
                        }
                        GeneratedField::IngestDelay => {
                            if ingest_delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingestDelay"));
                            }
                            ingest_delay__ = map_.next_value()?;
                        }
                        GeneratedField::TimeSeriesResourceHierarchyLevel => {
                            if time_series_resource_hierarchy_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeSeriesResourceHierarchyLevel"));
                            }
                            time_series_resource_hierarchy_level__ = Some(map_.next_value::<Vec<metric_descriptor::metric_descriptor_metadata::TimeSeriesResourceHierarchyLevel>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(metric_descriptor::MetricDescriptorMetadata {
                    launch_stage: launch_stage__.unwrap_or_default(),
                    sample_period: sample_period__,
                    ingest_delay: ingest_delay__,
                    time_series_resource_hierarchy_level: time_series_resource_hierarchy_level__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.MetricDescriptor.MetricDescriptorMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for metric_descriptor::metric_descriptor_metadata::TimeSeriesResourceHierarchyLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TIME_SERIES_RESOURCE_HIERARCHY_LEVEL_UNSPECIFIED",
            Self::Project => "PROJECT",
            Self::Organization => "ORGANIZATION",
            Self::Folder => "FOLDER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for metric_descriptor::metric_descriptor_metadata::TimeSeriesResourceHierarchyLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TIME_SERIES_RESOURCE_HIERARCHY_LEVEL_UNSPECIFIED",
            "PROJECT",
            "ORGANIZATION",
            "FOLDER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = metric_descriptor::metric_descriptor_metadata::TimeSeriesResourceHierarchyLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TIME_SERIES_RESOURCE_HIERARCHY_LEVEL_UNSPECIFIED" => Ok(metric_descriptor::metric_descriptor_metadata::TimeSeriesResourceHierarchyLevel::Unspecified),
                    "PROJECT" => Ok(metric_descriptor::metric_descriptor_metadata::TimeSeriesResourceHierarchyLevel::Project),
                    "ORGANIZATION" => Ok(metric_descriptor::metric_descriptor_metadata::TimeSeriesResourceHierarchyLevel::Organization),
                    "FOLDER" => Ok(metric_descriptor::metric_descriptor_metadata::TimeSeriesResourceHierarchyLevel::Folder),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for metric_descriptor::MetricKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "METRIC_KIND_UNSPECIFIED",
            Self::Gauge => "GAUGE",
            Self::Delta => "DELTA",
            Self::Cumulative => "CUMULATIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for metric_descriptor::MetricKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "METRIC_KIND_UNSPECIFIED",
            "GAUGE",
            "DELTA",
            "CUMULATIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = metric_descriptor::MetricKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "METRIC_KIND_UNSPECIFIED" => Ok(metric_descriptor::MetricKind::Unspecified),
                    "GAUGE" => Ok(metric_descriptor::MetricKind::Gauge),
                    "DELTA" => Ok(metric_descriptor::MetricKind::Delta),
                    "CUMULATIVE" => Ok(metric_descriptor::MetricKind::Cumulative),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for metric_descriptor::ValueType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VALUE_TYPE_UNSPECIFIED",
            Self::Bool => "BOOL",
            Self::Int64 => "INT64",
            Self::Double => "DOUBLE",
            Self::String => "STRING",
            Self::Distribution => "DISTRIBUTION",
            Self::Money => "MONEY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for metric_descriptor::ValueType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VALUE_TYPE_UNSPECIFIED",
            "BOOL",
            "INT64",
            "DOUBLE",
            "STRING",
            "DISTRIBUTION",
            "MONEY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = metric_descriptor::ValueType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "VALUE_TYPE_UNSPECIFIED" => Ok(metric_descriptor::ValueType::Unspecified),
                    "BOOL" => Ok(metric_descriptor::ValueType::Bool),
                    "INT64" => Ok(metric_descriptor::ValueType::Int64),
                    "DOUBLE" => Ok(metric_descriptor::ValueType::Double),
                    "STRING" => Ok(metric_descriptor::ValueType::String),
                    "DISTRIBUTION" => Ok(metric_descriptor::ValueType::Distribution),
                    "MONEY" => Ok(metric_descriptor::ValueType::Money),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MetricRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.metric_costs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.MetricRule", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.metric_costs.is_empty() {
            let v: std::collections::HashMap<_, _> = self.metric_costs.iter()
                .map(|(k, v)| (k, v.to_string())).collect();
            struct_ser.serialize_field("metricCosts", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetricRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "metric_costs",
            "metricCosts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            MetricCosts,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "metricCosts" | "metric_costs" => Ok(GeneratedField::MetricCosts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetricRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.MetricRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetricRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut metric_costs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetricCosts => {
                            if metric_costs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricCosts"));
                            }
                            metric_costs__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<i64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                    }
                }
                Ok(MetricRule {
                    selector: selector__.unwrap_or_default(),
                    metric_costs: metric_costs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.MetricRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MonitoredResource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.MonitoredResource", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MonitoredResource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "labels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Labels,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "labels" => Ok(GeneratedField::Labels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MonitoredResource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.MonitoredResource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MonitoredResource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut labels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(MonitoredResource {
                    r#type: r#type__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.MonitoredResource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MonitoredResourceDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if self.launch_stage != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.MonitoredResourceDescriptor", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if self.launch_stage != 0 {
            let v = LaunchStage::try_from(self.launch_stage)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.launch_stage)))?;
            struct_ser.serialize_field("launchStage", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MonitoredResourceDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type",
            "display_name",
            "displayName",
            "description",
            "labels",
            "launch_stage",
            "launchStage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Type,
            DisplayName,
            Description,
            Labels,
            LaunchStage,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "type" => Ok(GeneratedField::Type),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
                            "labels" => Ok(GeneratedField::Labels),
                            "launchStage" | "launch_stage" => Ok(GeneratedField::LaunchStage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MonitoredResourceDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.MonitoredResourceDescriptor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MonitoredResourceDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut labels__ = None;
                let mut launch_stage__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LaunchStage => {
                            if launch_stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("launchStage"));
                            }
                            launch_stage__ = Some(map_.next_value::<LaunchStage>()? as i32);
                        }
                    }
                }
                Ok(MonitoredResourceDescriptor {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                    launch_stage: launch_stage__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.MonitoredResourceDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MonitoredResourceMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.system_labels.is_some() {
            len += 1;
        }
        if !self.user_labels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.MonitoredResourceMetadata", len)?;
        if let Some(v) = self.system_labels.as_ref() {
            struct_ser.serialize_field("systemLabels", v)?;
        }
        if !self.user_labels.is_empty() {
            struct_ser.serialize_field("userLabels", &self.user_labels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MonitoredResourceMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "system_labels",
            "systemLabels",
            "user_labels",
            "userLabels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SystemLabels,
            UserLabels,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "systemLabels" | "system_labels" => Ok(GeneratedField::SystemLabels),
                            "userLabels" | "user_labels" => Ok(GeneratedField::UserLabels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MonitoredResourceMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.MonitoredResourceMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MonitoredResourceMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut system_labels__ = None;
                let mut user_labels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SystemLabels => {
                            if system_labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemLabels"));
                            }
                            system_labels__ = map_.next_value()?;
                        }
                        GeneratedField::UserLabels => {
                            if user_labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userLabels"));
                            }
                            user_labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(MonitoredResourceMetadata {
                    system_labels: system_labels__,
                    user_labels: user_labels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.MonitoredResourceMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Monitoring {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.producer_destinations.is_empty() {
            len += 1;
        }
        if !self.consumer_destinations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Monitoring", len)?;
        if !self.producer_destinations.is_empty() {
            struct_ser.serialize_field("producerDestinations", &self.producer_destinations)?;
        }
        if !self.consumer_destinations.is_empty() {
            struct_ser.serialize_field("consumerDestinations", &self.consumer_destinations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Monitoring {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "producer_destinations",
            "producerDestinations",
            "consumer_destinations",
            "consumerDestinations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProducerDestinations,
            ConsumerDestinations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "producerDestinations" | "producer_destinations" => Ok(GeneratedField::ProducerDestinations),
                            "consumerDestinations" | "consumer_destinations" => Ok(GeneratedField::ConsumerDestinations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Monitoring;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Monitoring")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Monitoring, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut producer_destinations__ = None;
                let mut consumer_destinations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProducerDestinations => {
                            if producer_destinations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("producerDestinations"));
                            }
                            producer_destinations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsumerDestinations => {
                            if consumer_destinations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumerDestinations"));
                            }
                            consumer_destinations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Monitoring {
                    producer_destinations: producer_destinations__.unwrap_or_default(),
                    consumer_destinations: consumer_destinations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Monitoring", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for monitoring::MonitoringDestination {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.monitored_resource.is_empty() {
            len += 1;
        }
        if !self.metrics.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Monitoring.MonitoringDestination", len)?;
        if !self.monitored_resource.is_empty() {
            struct_ser.serialize_field("monitoredResource", &self.monitored_resource)?;
        }
        if !self.metrics.is_empty() {
            struct_ser.serialize_field("metrics", &self.metrics)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for monitoring::MonitoringDestination {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "monitored_resource",
            "monitoredResource",
            "metrics",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MonitoredResource,
            Metrics,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "monitoredResource" | "monitored_resource" => Ok(GeneratedField::MonitoredResource),
                            "metrics" => Ok(GeneratedField::Metrics),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = monitoring::MonitoringDestination;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Monitoring.MonitoringDestination")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<monitoring::MonitoringDestination, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut monitored_resource__ = None;
                let mut metrics__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MonitoredResource => {
                            if monitored_resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitoredResource"));
                            }
                            monitored_resource__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(monitoring::MonitoringDestination {
                    monitored_resource: monitored_resource__.unwrap_or_default(),
                    metrics: metrics__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Monitoring.MonitoringDestination", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.NodeSettings", len)?;
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Common,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "common" => Ok(GeneratedField::Common),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.NodeSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NodeSettings {
                    common: common__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.NodeSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OAuthRequirements {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.canonical_scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.OAuthRequirements", len)?;
        if !self.canonical_scopes.is_empty() {
            struct_ser.serialize_field("canonicalScopes", &self.canonical_scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OAuthRequirements {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "canonical_scopes",
            "canonicalScopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CanonicalScopes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "canonicalScopes" | "canonical_scopes" => Ok(GeneratedField::CanonicalScopes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OAuthRequirements;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.OAuthRequirements")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OAuthRequirements, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut canonical_scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CanonicalScopes => {
                            if canonical_scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canonicalScopes"));
                            }
                            canonical_scopes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OAuthRequirements {
                    canonical_scopes: canonical_scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.OAuthRequirements", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Page {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if !self.subpages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Page", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if !self.subpages.is_empty() {
            struct_ser.serialize_field("subpages", &self.subpages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Page {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "content",
            "subpages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Content,
            Subpages,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "content" => Ok(GeneratedField::Content),
                            "subpages" => Ok(GeneratedField::Subpages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Page;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Page")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Page, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut content__ = None;
                let mut subpages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Subpages => {
                            if subpages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subpages"));
                            }
                            subpages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Page {
                    name: name__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                    subpages: subpages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Page", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PhpSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.PhpSettings", len)?;
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PhpSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Common,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "common" => Ok(GeneratedField::Common),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PhpSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.PhpSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PhpSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PhpSettings {
                    common: common__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.PhpSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.properties.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.ProjectProperties", len)?;
        if !self.properties.is_empty() {
            struct_ser.serialize_field("properties", &self.properties)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "properties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Properties,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "properties" => Ok(GeneratedField::Properties),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.ProjectProperties")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut properties__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProjectProperties {
                    properties: properties__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.ProjectProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Property {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Property", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.r#type != 0 {
            let v = property::PropertyType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Property {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Type,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "type" => Ok(GeneratedField::Type),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Property;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Property")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Property, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<property::PropertyType>()? as i32);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Property {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Property", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for property::PropertyType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Int64 => "INT64",
            Self::Bool => "BOOL",
            Self::String => "STRING",
            Self::Double => "DOUBLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for property::PropertyType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "INT64",
            "BOOL",
            "STRING",
            "DOUBLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = property::PropertyType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(property::PropertyType::Unspecified),
                    "INT64" => Ok(property::PropertyType::Int64),
                    "BOOL" => Ok(property::PropertyType::Bool),
                    "STRING" => Ok(property::PropertyType::String),
                    "DOUBLE" => Ok(property::PropertyType::Double),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Publishing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.method_settings.is_empty() {
            len += 1;
        }
        if !self.new_issue_uri.is_empty() {
            len += 1;
        }
        if !self.documentation_uri.is_empty() {
            len += 1;
        }
        if !self.api_short_name.is_empty() {
            len += 1;
        }
        if !self.github_label.is_empty() {
            len += 1;
        }
        if !self.codeowner_github_teams.is_empty() {
            len += 1;
        }
        if !self.doc_tag_prefix.is_empty() {
            len += 1;
        }
        if self.organization != 0 {
            len += 1;
        }
        if !self.library_settings.is_empty() {
            len += 1;
        }
        if !self.proto_reference_documentation_uri.is_empty() {
            len += 1;
        }
        if !self.rest_reference_documentation_uri.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Publishing", len)?;
        if !self.method_settings.is_empty() {
            struct_ser.serialize_field("methodSettings", &self.method_settings)?;
        }
        if !self.new_issue_uri.is_empty() {
            struct_ser.serialize_field("newIssueUri", &self.new_issue_uri)?;
        }
        if !self.documentation_uri.is_empty() {
            struct_ser.serialize_field("documentationUri", &self.documentation_uri)?;
        }
        if !self.api_short_name.is_empty() {
            struct_ser.serialize_field("apiShortName", &self.api_short_name)?;
        }
        if !self.github_label.is_empty() {
            struct_ser.serialize_field("githubLabel", &self.github_label)?;
        }
        if !self.codeowner_github_teams.is_empty() {
            struct_ser.serialize_field("codeownerGithubTeams", &self.codeowner_github_teams)?;
        }
        if !self.doc_tag_prefix.is_empty() {
            struct_ser.serialize_field("docTagPrefix", &self.doc_tag_prefix)?;
        }
        if self.organization != 0 {
            let v = ClientLibraryOrganization::try_from(self.organization)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.organization)))?;
            struct_ser.serialize_field("organization", &v)?;
        }
        if !self.library_settings.is_empty() {
            struct_ser.serialize_field("librarySettings", &self.library_settings)?;
        }
        if !self.proto_reference_documentation_uri.is_empty() {
            struct_ser.serialize_field("protoReferenceDocumentationUri", &self.proto_reference_documentation_uri)?;
        }
        if !self.rest_reference_documentation_uri.is_empty() {
            struct_ser.serialize_field("restReferenceDocumentationUri", &self.rest_reference_documentation_uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Publishing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "method_settings",
            "methodSettings",
            "new_issue_uri",
            "newIssueUri",
            "documentation_uri",
            "documentationUri",
            "api_short_name",
            "apiShortName",
            "github_label",
            "githubLabel",
            "codeowner_github_teams",
            "codeownerGithubTeams",
            "doc_tag_prefix",
            "docTagPrefix",
            "organization",
            "library_settings",
            "librarySettings",
            "proto_reference_documentation_uri",
            "protoReferenceDocumentationUri",
            "rest_reference_documentation_uri",
            "restReferenceDocumentationUri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MethodSettings,
            NewIssueUri,
            DocumentationUri,
            ApiShortName,
            GithubLabel,
            CodeownerGithubTeams,
            DocTagPrefix,
            Organization,
            LibrarySettings,
            ProtoReferenceDocumentationUri,
            RestReferenceDocumentationUri,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "methodSettings" | "method_settings" => Ok(GeneratedField::MethodSettings),
                            "newIssueUri" | "new_issue_uri" => Ok(GeneratedField::NewIssueUri),
                            "documentationUri" | "documentation_uri" => Ok(GeneratedField::DocumentationUri),
                            "apiShortName" | "api_short_name" => Ok(GeneratedField::ApiShortName),
                            "githubLabel" | "github_label" => Ok(GeneratedField::GithubLabel),
                            "codeownerGithubTeams" | "codeowner_github_teams" => Ok(GeneratedField::CodeownerGithubTeams),
                            "docTagPrefix" | "doc_tag_prefix" => Ok(GeneratedField::DocTagPrefix),
                            "organization" => Ok(GeneratedField::Organization),
                            "librarySettings" | "library_settings" => Ok(GeneratedField::LibrarySettings),
                            "protoReferenceDocumentationUri" | "proto_reference_documentation_uri" => Ok(GeneratedField::ProtoReferenceDocumentationUri),
                            "restReferenceDocumentationUri" | "rest_reference_documentation_uri" => Ok(GeneratedField::RestReferenceDocumentationUri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Publishing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Publishing")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Publishing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut method_settings__ = None;
                let mut new_issue_uri__ = None;
                let mut documentation_uri__ = None;
                let mut api_short_name__ = None;
                let mut github_label__ = None;
                let mut codeowner_github_teams__ = None;
                let mut doc_tag_prefix__ = None;
                let mut organization__ = None;
                let mut library_settings__ = None;
                let mut proto_reference_documentation_uri__ = None;
                let mut rest_reference_documentation_uri__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MethodSettings => {
                            if method_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methodSettings"));
                            }
                            method_settings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewIssueUri => {
                            if new_issue_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newIssueUri"));
                            }
                            new_issue_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DocumentationUri => {
                            if documentation_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentationUri"));
                            }
                            documentation_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApiShortName => {
                            if api_short_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiShortName"));
                            }
                            api_short_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GithubLabel => {
                            if github_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("githubLabel"));
                            }
                            github_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeownerGithubTeams => {
                            if codeowner_github_teams__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeownerGithubTeams"));
                            }
                            codeowner_github_teams__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DocTagPrefix => {
                            if doc_tag_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("docTagPrefix"));
                            }
                            doc_tag_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Organization => {
                            if organization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            organization__ = Some(map_.next_value::<ClientLibraryOrganization>()? as i32);
                        }
                        GeneratedField::LibrarySettings => {
                            if library_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("librarySettings"));
                            }
                            library_settings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProtoReferenceDocumentationUri => {
                            if proto_reference_documentation_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoReferenceDocumentationUri"));
                            }
                            proto_reference_documentation_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RestReferenceDocumentationUri => {
                            if rest_reference_documentation_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restReferenceDocumentationUri"));
                            }
                            rest_reference_documentation_uri__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Publishing {
                    method_settings: method_settings__.unwrap_or_default(),
                    new_issue_uri: new_issue_uri__.unwrap_or_default(),
                    documentation_uri: documentation_uri__.unwrap_or_default(),
                    api_short_name: api_short_name__.unwrap_or_default(),
                    github_label: github_label__.unwrap_or_default(),
                    codeowner_github_teams: codeowner_github_teams__.unwrap_or_default(),
                    doc_tag_prefix: doc_tag_prefix__.unwrap_or_default(),
                    organization: organization__.unwrap_or_default(),
                    library_settings: library_settings__.unwrap_or_default(),
                    proto_reference_documentation_uri: proto_reference_documentation_uri__.unwrap_or_default(),
                    rest_reference_documentation_uri: rest_reference_documentation_uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Publishing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PythonSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common.is_some() {
            len += 1;
        }
        if self.experimental_features.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.PythonSettings", len)?;
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        if let Some(v) = self.experimental_features.as_ref() {
            struct_ser.serialize_field("experimentalFeatures", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PythonSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common",
            "experimental_features",
            "experimentalFeatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Common,
            ExperimentalFeatures,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "common" => Ok(GeneratedField::Common),
                            "experimentalFeatures" | "experimental_features" => Ok(GeneratedField::ExperimentalFeatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PythonSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.PythonSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PythonSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common__ = None;
                let mut experimental_features__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                        GeneratedField::ExperimentalFeatures => {
                            if experimental_features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentalFeatures"));
                            }
                            experimental_features__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PythonSettings {
                    common: common__,
                    experimental_features: experimental_features__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.PythonSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for python_settings::ExperimentalFeatures {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rest_async_io_enabled {
            len += 1;
        }
        if self.protobuf_pythonic_types_enabled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.PythonSettings.ExperimentalFeatures", len)?;
        if self.rest_async_io_enabled {
            struct_ser.serialize_field("restAsyncIoEnabled", &self.rest_async_io_enabled)?;
        }
        if self.protobuf_pythonic_types_enabled {
            struct_ser.serialize_field("protobufPythonicTypesEnabled", &self.protobuf_pythonic_types_enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for python_settings::ExperimentalFeatures {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rest_async_io_enabled",
            "restAsyncIoEnabled",
            "protobuf_pythonic_types_enabled",
            "protobufPythonicTypesEnabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RestAsyncIoEnabled,
            ProtobufPythonicTypesEnabled,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "restAsyncIoEnabled" | "rest_async_io_enabled" => Ok(GeneratedField::RestAsyncIoEnabled),
                            "protobufPythonicTypesEnabled" | "protobuf_pythonic_types_enabled" => Ok(GeneratedField::ProtobufPythonicTypesEnabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = python_settings::ExperimentalFeatures;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.PythonSettings.ExperimentalFeatures")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<python_settings::ExperimentalFeatures, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rest_async_io_enabled__ = None;
                let mut protobuf_pythonic_types_enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RestAsyncIoEnabled => {
                            if rest_async_io_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restAsyncIoEnabled"));
                            }
                            rest_async_io_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProtobufPythonicTypesEnabled => {
                            if protobuf_pythonic_types_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protobufPythonicTypesEnabled"));
                            }
                            protobuf_pythonic_types_enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(python_settings::ExperimentalFeatures {
                    rest_async_io_enabled: rest_async_io_enabled__.unwrap_or_default(),
                    protobuf_pythonic_types_enabled: protobuf_pythonic_types_enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.PythonSettings.ExperimentalFeatures", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Quota {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.limits.is_empty() {
            len += 1;
        }
        if !self.metric_rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Quota", len)?;
        if !self.limits.is_empty() {
            struct_ser.serialize_field("limits", &self.limits)?;
        }
        if !self.metric_rules.is_empty() {
            struct_ser.serialize_field("metricRules", &self.metric_rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Quota {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limits",
            "metric_rules",
            "metricRules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limits,
            MetricRules,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "limits" => Ok(GeneratedField::Limits),
                            "metricRules" | "metric_rules" => Ok(GeneratedField::MetricRules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Quota;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Quota")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Quota, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limits__ = None;
                let mut metric_rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Limits => {
                            if limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limits"));
                            }
                            limits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetricRules => {
                            if metric_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricRules"));
                            }
                            metric_rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Quota {
                    limits: limits__.unwrap_or_default(),
                    metric_rules: metric_rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Quota", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.default_limit != 0 {
            len += 1;
        }
        if self.max_limit != 0 {
            len += 1;
        }
        if self.free_tier != 0 {
            len += 1;
        }
        if !self.duration.is_empty() {
            len += 1;
        }
        if !self.metric.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.QuotaLimit", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.default_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("defaultLimit", ToString::to_string(&self.default_limit).as_str())?;
        }
        if self.max_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxLimit", ToString::to_string(&self.max_limit).as_str())?;
        }
        if self.free_tier != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("freeTier", ToString::to_string(&self.free_tier).as_str())?;
        }
        if !self.duration.is_empty() {
            struct_ser.serialize_field("duration", &self.duration)?;
        }
        if !self.metric.is_empty() {
            struct_ser.serialize_field("metric", &self.metric)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if !self.values.is_empty() {
            let v: std::collections::HashMap<_, _> = self.values.iter()
                .map(|(k, v)| (k, v.to_string())).collect();
            struct_ser.serialize_field("values", &v)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "default_limit",
            "defaultLimit",
            "max_limit",
            "maxLimit",
            "free_tier",
            "freeTier",
            "duration",
            "metric",
            "unit",
            "values",
            "display_name",
            "displayName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            DefaultLimit,
            MaxLimit,
            FreeTier,
            Duration,
            Metric,
            Unit,
            Values,
            DisplayName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "defaultLimit" | "default_limit" => Ok(GeneratedField::DefaultLimit),
                            "maxLimit" | "max_limit" => Ok(GeneratedField::MaxLimit),
                            "freeTier" | "free_tier" => Ok(GeneratedField::FreeTier),
                            "duration" => Ok(GeneratedField::Duration),
                            "metric" => Ok(GeneratedField::Metric),
                            "unit" => Ok(GeneratedField::Unit),
                            "values" => Ok(GeneratedField::Values),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.QuotaLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut default_limit__ = None;
                let mut max_limit__ = None;
                let mut free_tier__ = None;
                let mut duration__ = None;
                let mut metric__ = None;
                let mut unit__ = None;
                let mut values__ = None;
                let mut display_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultLimit => {
                            if default_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultLimit"));
                            }
                            default_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxLimit => {
                            if max_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLimit"));
                            }
                            max_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FreeTier => {
                            if free_tier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("freeTier"));
                            }
                            free_tier__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metric => {
                            if metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }
                            metric__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<i64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuotaLimit {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    default_limit: default_limit__.unwrap_or_default(),
                    max_limit: max_limit__.unwrap_or_default(),
                    free_tier: free_tier__.unwrap_or_default(),
                    duration: duration__.unwrap_or_default(),
                    metric: metric__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.QuotaLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.pattern.is_empty() {
            len += 1;
        }
        if !self.name_field.is_empty() {
            len += 1;
        }
        if self.history != 0 {
            len += 1;
        }
        if !self.plural.is_empty() {
            len += 1;
        }
        if !self.singular.is_empty() {
            len += 1;
        }
        if !self.style.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.ResourceDescriptor", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.pattern.is_empty() {
            struct_ser.serialize_field("pattern", &self.pattern)?;
        }
        if !self.name_field.is_empty() {
            struct_ser.serialize_field("nameField", &self.name_field)?;
        }
        if self.history != 0 {
            let v = resource_descriptor::History::try_from(self.history)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.history)))?;
            struct_ser.serialize_field("history", &v)?;
        }
        if !self.plural.is_empty() {
            struct_ser.serialize_field("plural", &self.plural)?;
        }
        if !self.singular.is_empty() {
            struct_ser.serialize_field("singular", &self.singular)?;
        }
        if !self.style.is_empty() {
            let v = self.style.iter().cloned().map(|v| {
                resource_descriptor::Style::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("style", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "pattern",
            "name_field",
            "nameField",
            "history",
            "plural",
            "singular",
            "style",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Pattern,
            NameField,
            History,
            Plural,
            Singular,
            Style,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "pattern" => Ok(GeneratedField::Pattern),
                            "nameField" | "name_field" => Ok(GeneratedField::NameField),
                            "history" => Ok(GeneratedField::History),
                            "plural" => Ok(GeneratedField::Plural),
                            "singular" => Ok(GeneratedField::Singular),
                            "style" => Ok(GeneratedField::Style),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.ResourceDescriptor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut pattern__ = None;
                let mut name_field__ = None;
                let mut history__ = None;
                let mut plural__ = None;
                let mut singular__ = None;
                let mut style__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pattern => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            pattern__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NameField => {
                            if name_field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nameField"));
                            }
                            name_field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::History => {
                            if history__.is_some() {
                                return Err(serde::de::Error::duplicate_field("history"));
                            }
                            history__ = Some(map_.next_value::<resource_descriptor::History>()? as i32);
                        }
                        GeneratedField::Plural => {
                            if plural__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plural"));
                            }
                            plural__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Singular => {
                            if singular__.is_some() {
                                return Err(serde::de::Error::duplicate_field("singular"));
                            }
                            singular__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Style => {
                            if style__.is_some() {
                                return Err(serde::de::Error::duplicate_field("style"));
                            }
                            style__ = Some(map_.next_value::<Vec<resource_descriptor::Style>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(ResourceDescriptor {
                    r#type: r#type__.unwrap_or_default(),
                    pattern: pattern__.unwrap_or_default(),
                    name_field: name_field__.unwrap_or_default(),
                    history: history__.unwrap_or_default(),
                    plural: plural__.unwrap_or_default(),
                    singular: singular__.unwrap_or_default(),
                    style: style__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.ResourceDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for resource_descriptor::History {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "HISTORY_UNSPECIFIED",
            Self::OriginallySinglePattern => "ORIGINALLY_SINGLE_PATTERN",
            Self::FutureMultiPattern => "FUTURE_MULTI_PATTERN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for resource_descriptor::History {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HISTORY_UNSPECIFIED",
            "ORIGINALLY_SINGLE_PATTERN",
            "FUTURE_MULTI_PATTERN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = resource_descriptor::History;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "HISTORY_UNSPECIFIED" => Ok(resource_descriptor::History::Unspecified),
                    "ORIGINALLY_SINGLE_PATTERN" => Ok(resource_descriptor::History::OriginallySinglePattern),
                    "FUTURE_MULTI_PATTERN" => Ok(resource_descriptor::History::FutureMultiPattern),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for resource_descriptor::Style {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STYLE_UNSPECIFIED",
            Self::DeclarativeFriendly => "DECLARATIVE_FRIENDLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for resource_descriptor::Style {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STYLE_UNSPECIFIED",
            "DECLARATIVE_FRIENDLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = resource_descriptor::Style;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STYLE_UNSPECIFIED" => Ok(resource_descriptor::Style::Unspecified),
                    "DECLARATIVE_FRIENDLY" => Ok(resource_descriptor::Style::DeclarativeFriendly),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.child_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.ResourceReference", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.child_type.is_empty() {
            struct_ser.serialize_field("childType", &self.child_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "child_type",
            "childType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            ChildType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "childType" | "child_type" => Ok(GeneratedField::ChildType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.ResourceReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut child_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChildType => {
                            if child_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("childType"));
                            }
                            child_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourceReference {
                    r#type: r#type__.unwrap_or_default(),
                    child_type: child_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.ResourceReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutingParameter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field.is_empty() {
            len += 1;
        }
        if !self.path_template.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.RoutingParameter", len)?;
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if !self.path_template.is_empty() {
            struct_ser.serialize_field("pathTemplate", &self.path_template)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutingParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "path_template",
            "pathTemplate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            PathTemplate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "field" => Ok(GeneratedField::Field),
                            "pathTemplate" | "path_template" => Ok(GeneratedField::PathTemplate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutingParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.RoutingParameter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoutingParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut path_template__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PathTemplate => {
                            if path_template__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathTemplate"));
                            }
                            path_template__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RoutingParameter {
                    field: field__.unwrap_or_default(),
                    path_template: path_template__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.RoutingParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutingRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.routing_parameters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.RoutingRule", len)?;
        if !self.routing_parameters.is_empty() {
            struct_ser.serialize_field("routingParameters", &self.routing_parameters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutingRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "routing_parameters",
            "routingParameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoutingParameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "routingParameters" | "routing_parameters" => Ok(GeneratedField::RoutingParameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutingRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.RoutingRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoutingRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut routing_parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoutingParameters => {
                            if routing_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routingParameters"));
                            }
                            routing_parameters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RoutingRule {
                    routing_parameters: routing_parameters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.RoutingRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RubySettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.RubySettings", len)?;
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RubySettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Common,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "common" => Ok(GeneratedField::Common),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RubySettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.RubySettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RubySettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RubySettings {
                    common: common__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.RubySettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SelectiveGapicGeneration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.methods.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.SelectiveGapicGeneration", len)?;
        if !self.methods.is_empty() {
            struct_ser.serialize_field("methods", &self.methods)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SelectiveGapicGeneration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "methods",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Methods,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "methods" => Ok(GeneratedField::Methods),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SelectiveGapicGeneration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.SelectiveGapicGeneration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SelectiveGapicGeneration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut methods__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Methods => {
                            if methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methods"));
                            }
                            methods__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SelectiveGapicGeneration {
                    methods: methods__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.SelectiveGapicGeneration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Service {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.producer_project_id.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.apis.is_empty() {
            len += 1;
        }
        if !self.types.is_empty() {
            len += 1;
        }
        if !self.enums.is_empty() {
            len += 1;
        }
        if self.documentation.is_some() {
            len += 1;
        }
        if self.backend.is_some() {
            len += 1;
        }
        if self.http.is_some() {
            len += 1;
        }
        if self.quota.is_some() {
            len += 1;
        }
        if self.authentication.is_some() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        if self.usage.is_some() {
            len += 1;
        }
        if !self.endpoints.is_empty() {
            len += 1;
        }
        if self.control.is_some() {
            len += 1;
        }
        if !self.logs.is_empty() {
            len += 1;
        }
        if !self.metrics.is_empty() {
            len += 1;
        }
        if !self.monitored_resources.is_empty() {
            len += 1;
        }
        if self.billing.is_some() {
            len += 1;
        }
        if self.logging.is_some() {
            len += 1;
        }
        if self.monitoring.is_some() {
            len += 1;
        }
        if self.system_parameters.is_some() {
            len += 1;
        }
        if self.source_info.is_some() {
            len += 1;
        }
        if self.publishing.is_some() {
            len += 1;
        }
        if self.config_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Service", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.producer_project_id.is_empty() {
            struct_ser.serialize_field("producerProjectId", &self.producer_project_id)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.apis.is_empty() {
            struct_ser.serialize_field("apis", &self.apis)?;
        }
        if !self.types.is_empty() {
            struct_ser.serialize_field("types", &self.types)?;
        }
        if !self.enums.is_empty() {
            struct_ser.serialize_field("enums", &self.enums)?;
        }
        if let Some(v) = self.documentation.as_ref() {
            struct_ser.serialize_field("documentation", v)?;
        }
        if let Some(v) = self.backend.as_ref() {
            struct_ser.serialize_field("backend", v)?;
        }
        if let Some(v) = self.http.as_ref() {
            struct_ser.serialize_field("http", v)?;
        }
        if let Some(v) = self.quota.as_ref() {
            struct_ser.serialize_field("quota", v)?;
        }
        if let Some(v) = self.authentication.as_ref() {
            struct_ser.serialize_field("authentication", v)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if let Some(v) = self.usage.as_ref() {
            struct_ser.serialize_field("usage", v)?;
        }
        if !self.endpoints.is_empty() {
            struct_ser.serialize_field("endpoints", &self.endpoints)?;
        }
        if let Some(v) = self.control.as_ref() {
            struct_ser.serialize_field("control", v)?;
        }
        if !self.logs.is_empty() {
            struct_ser.serialize_field("logs", &self.logs)?;
        }
        if !self.metrics.is_empty() {
            struct_ser.serialize_field("metrics", &self.metrics)?;
        }
        if !self.monitored_resources.is_empty() {
            struct_ser.serialize_field("monitoredResources", &self.monitored_resources)?;
        }
        if let Some(v) = self.billing.as_ref() {
            struct_ser.serialize_field("billing", v)?;
        }
        if let Some(v) = self.logging.as_ref() {
            struct_ser.serialize_field("logging", v)?;
        }
        if let Some(v) = self.monitoring.as_ref() {
            struct_ser.serialize_field("monitoring", v)?;
        }
        if let Some(v) = self.system_parameters.as_ref() {
            struct_ser.serialize_field("systemParameters", v)?;
        }
        if let Some(v) = self.source_info.as_ref() {
            struct_ser.serialize_field("sourceInfo", v)?;
        }
        if let Some(v) = self.publishing.as_ref() {
            struct_ser.serialize_field("publishing", v)?;
        }
        if let Some(v) = self.config_version.as_ref() {
            struct_ser.serialize_field("configVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Service {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "title",
            "producer_project_id",
            "producerProjectId",
            "id",
            "apis",
            "types",
            "enums",
            "documentation",
            "backend",
            "http",
            "quota",
            "authentication",
            "context",
            "usage",
            "endpoints",
            "control",
            "logs",
            "metrics",
            "monitored_resources",
            "monitoredResources",
            "billing",
            "logging",
            "monitoring",
            "system_parameters",
            "systemParameters",
            "source_info",
            "sourceInfo",
            "publishing",
            "config_version",
            "configVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Title,
            ProducerProjectId,
            Id,
            Apis,
            Types,
            Enums,
            Documentation,
            Backend,
            Http,
            Quota,
            Authentication,
            Context,
            Usage,
            Endpoints,
            Control,
            Logs,
            Metrics,
            MonitoredResources,
            Billing,
            Logging,
            Monitoring,
            SystemParameters,
            SourceInfo,
            Publishing,
            ConfigVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "title" => Ok(GeneratedField::Title),
                            "producerProjectId" | "producer_project_id" => Ok(GeneratedField::ProducerProjectId),
                            "id" => Ok(GeneratedField::Id),
                            "apis" => Ok(GeneratedField::Apis),
                            "types" => Ok(GeneratedField::Types),
                            "enums" => Ok(GeneratedField::Enums),
                            "documentation" => Ok(GeneratedField::Documentation),
                            "backend" => Ok(GeneratedField::Backend),
                            "http" => Ok(GeneratedField::Http),
                            "quota" => Ok(GeneratedField::Quota),
                            "authentication" => Ok(GeneratedField::Authentication),
                            "context" => Ok(GeneratedField::Context),
                            "usage" => Ok(GeneratedField::Usage),
                            "endpoints" => Ok(GeneratedField::Endpoints),
                            "control" => Ok(GeneratedField::Control),
                            "logs" => Ok(GeneratedField::Logs),
                            "metrics" => Ok(GeneratedField::Metrics),
                            "monitoredResources" | "monitored_resources" => Ok(GeneratedField::MonitoredResources),
                            "billing" => Ok(GeneratedField::Billing),
                            "logging" => Ok(GeneratedField::Logging),
                            "monitoring" => Ok(GeneratedField::Monitoring),
                            "systemParameters" | "system_parameters" => Ok(GeneratedField::SystemParameters),
                            "sourceInfo" | "source_info" => Ok(GeneratedField::SourceInfo),
                            "publishing" => Ok(GeneratedField::Publishing),
                            "configVersion" | "config_version" => Ok(GeneratedField::ConfigVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Service;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Service")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Service, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut title__ = None;
                let mut producer_project_id__ = None;
                let mut id__ = None;
                let mut apis__ = None;
                let mut types__ = None;
                let mut enums__ = None;
                let mut documentation__ = None;
                let mut backend__ = None;
                let mut http__ = None;
                let mut quota__ = None;
                let mut authentication__ = None;
                let mut context__ = None;
                let mut usage__ = None;
                let mut endpoints__ = None;
                let mut control__ = None;
                let mut logs__ = None;
                let mut metrics__ = None;
                let mut monitored_resources__ = None;
                let mut billing__ = None;
                let mut logging__ = None;
                let mut monitoring__ = None;
                let mut system_parameters__ = None;
                let mut source_info__ = None;
                let mut publishing__ = None;
                let mut config_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProducerProjectId => {
                            if producer_project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("producerProjectId"));
                            }
                            producer_project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Apis => {
                            if apis__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apis"));
                            }
                            apis__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Types => {
                            if types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("types"));
                            }
                            types__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Enums => {
                            if enums__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enums"));
                            }
                            enums__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Documentation => {
                            if documentation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            documentation__ = map_.next_value()?;
                        }
                        GeneratedField::Backend => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backend"));
                            }
                            backend__ = map_.next_value()?;
                        }
                        GeneratedField::Http => {
                            if http__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            http__ = map_.next_value()?;
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = map_.next_value()?;
                        }
                        GeneratedField::Authentication => {
                            if authentication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authentication"));
                            }
                            authentication__ = map_.next_value()?;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map_.next_value()?;
                        }
                        GeneratedField::Usage => {
                            if usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage"));
                            }
                            usage__ = map_.next_value()?;
                        }
                        GeneratedField::Endpoints => {
                            if endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoints"));
                            }
                            endpoints__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Control => {
                            if control__.is_some() {
                                return Err(serde::de::Error::duplicate_field("control"));
                            }
                            control__ = map_.next_value()?;
                        }
                        GeneratedField::Logs => {
                            if logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logs"));
                            }
                            logs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MonitoredResources => {
                            if monitored_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitoredResources"));
                            }
                            monitored_resources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Billing => {
                            if billing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("billing"));
                            }
                            billing__ = map_.next_value()?;
                        }
                        GeneratedField::Logging => {
                            if logging__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logging"));
                            }
                            logging__ = map_.next_value()?;
                        }
                        GeneratedField::Monitoring => {
                            if monitoring__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitoring"));
                            }
                            monitoring__ = map_.next_value()?;
                        }
                        GeneratedField::SystemParameters => {
                            if system_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemParameters"));
                            }
                            system_parameters__ = map_.next_value()?;
                        }
                        GeneratedField::SourceInfo => {
                            if source_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceInfo"));
                            }
                            source_info__ = map_.next_value()?;
                        }
                        GeneratedField::Publishing => {
                            if publishing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publishing"));
                            }
                            publishing__ = map_.next_value()?;
                        }
                        GeneratedField::ConfigVersion => {
                            if config_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configVersion"));
                            }
                            config_version__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Service {
                    name: name__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    producer_project_id: producer_project_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    apis: apis__.unwrap_or_default(),
                    types: types__.unwrap_or_default(),
                    enums: enums__.unwrap_or_default(),
                    documentation: documentation__,
                    backend: backend__,
                    http: http__,
                    quota: quota__,
                    authentication: authentication__,
                    context: context__,
                    usage: usage__,
                    endpoints: endpoints__.unwrap_or_default(),
                    control: control__,
                    logs: logs__.unwrap_or_default(),
                    metrics: metrics__.unwrap_or_default(),
                    monitored_resources: monitored_resources__.unwrap_or_default(),
                    billing: billing__,
                    logging: logging__,
                    monitoring: monitoring__,
                    system_parameters: system_parameters__,
                    source_info: source_info__,
                    publishing: publishing__,
                    config_version: config_version__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.Service", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.source_files.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.SourceInfo", len)?;
        if !self.source_files.is_empty() {
            struct_ser.serialize_field("sourceFiles", &self.source_files)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_files",
            "sourceFiles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceFiles,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sourceFiles" | "source_files" => Ok(GeneratedField::SourceFiles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.SourceInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SourceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_files__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceFiles => {
                            if source_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceFiles"));
                            }
                            source_files__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SourceInfo {
                    source_files: source_files__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.SourceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SystemParameter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.http_header.is_empty() {
            len += 1;
        }
        if !self.url_query_parameter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.SystemParameter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.http_header.is_empty() {
            struct_ser.serialize_field("httpHeader", &self.http_header)?;
        }
        if !self.url_query_parameter.is_empty() {
            struct_ser.serialize_field("urlQueryParameter", &self.url_query_parameter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SystemParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "http_header",
            "httpHeader",
            "url_query_parameter",
            "urlQueryParameter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            HttpHeader,
            UrlQueryParameter,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "httpHeader" | "http_header" => Ok(GeneratedField::HttpHeader),
                            "urlQueryParameter" | "url_query_parameter" => Ok(GeneratedField::UrlQueryParameter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SystemParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.SystemParameter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SystemParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut http_header__ = None;
                let mut url_query_parameter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HttpHeader => {
                            if http_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpHeader"));
                            }
                            http_header__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UrlQueryParameter => {
                            if url_query_parameter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urlQueryParameter"));
                            }
                            url_query_parameter__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SystemParameter {
                    name: name__.unwrap_or_default(),
                    http_header: http_header__.unwrap_or_default(),
                    url_query_parameter: url_query_parameter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.SystemParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SystemParameterRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.SystemParameterRule", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SystemParameterRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SystemParameterRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.SystemParameterRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SystemParameterRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SystemParameterRule {
                    selector: selector__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.SystemParameterRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SystemParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.SystemParameters", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SystemParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SystemParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.SystemParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SystemParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SystemParameters {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.SystemParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TypeReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.type_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.TypeReference", len)?;
        if !self.type_name.is_empty() {
            struct_ser.serialize_field("typeName", &self.type_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypeReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type_name",
            "typeName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypeName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "typeName" | "type_name" => Ok(GeneratedField::TypeName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypeReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.TypeReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TypeReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut type_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TypeName => {
                            if type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeName"));
                            }
                            type_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TypeReference {
                    type_name: type_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.TypeReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Usage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requirements.is_empty() {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        if !self.producer_notification_channel.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Usage", len)?;
        if !self.requirements.is_empty() {
            struct_ser.serialize_field("requirements", &self.requirements)?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if !self.producer_notification_channel.is_empty() {
            struct_ser.serialize_field("producerNotificationChannel", &self.producer_notification_channel)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Usage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requirements",
            "rules",
            "producer_notification_channel",
            "producerNotificationChannel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requirements,
            Rules,
            ProducerNotificationChannel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requirements" => Ok(GeneratedField::Requirements),
                            "rules" => Ok(GeneratedField::Rules),
                            "producerNotificationChannel" | "producer_notification_channel" => Ok(GeneratedField::ProducerNotificationChannel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Usage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Usage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Usage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requirements__ = None;
                let mut rules__ = None;
                let mut producer_notification_channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requirements => {
                            if requirements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirements"));
                            }
                            requirements__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProducerNotificationChannel => {
                            if producer_notification_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("producerNotificationChannel"));
                            }
                            producer_notification_channel__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Usage {
                    requirements: requirements__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                    producer_notification_channel: producer_notification_channel__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Usage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UsageRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if self.allow_unregistered_calls {
            len += 1;
        }
        if self.skip_service_control {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.UsageRule", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if self.allow_unregistered_calls {
            struct_ser.serialize_field("allowUnregisteredCalls", &self.allow_unregistered_calls)?;
        }
        if self.skip_service_control {
            struct_ser.serialize_field("skipServiceControl", &self.skip_service_control)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UsageRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "allow_unregistered_calls",
            "allowUnregisteredCalls",
            "skip_service_control",
            "skipServiceControl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            AllowUnregisteredCalls,
            SkipServiceControl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "allowUnregisteredCalls" | "allow_unregistered_calls" => Ok(GeneratedField::AllowUnregisteredCalls),
                            "skipServiceControl" | "skip_service_control" => Ok(GeneratedField::SkipServiceControl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UsageRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.UsageRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UsageRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut allow_unregistered_calls__ = None;
                let mut skip_service_control__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowUnregisteredCalls => {
                            if allow_unregistered_calls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowUnregisteredCalls"));
                            }
                            allow_unregistered_calls__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SkipServiceControl => {
                            if skip_service_control__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipServiceControl"));
                            }
                            skip_service_control__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UsageRule {
                    selector: selector__.unwrap_or_default(),
                    allow_unregistered_calls: allow_unregistered_calls__.unwrap_or_default(),
                    skip_service_control: skip_service_control__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.UsageRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Visibility {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.Visibility", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Visibility {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Visibility;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.Visibility")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Visibility, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Visibility {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.Visibility", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VisibilityRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.restriction.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.VisibilityRule", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.restriction.is_empty() {
            struct_ser.serialize_field("restriction", &self.restriction)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VisibilityRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "restriction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            Restriction,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "restriction" => Ok(GeneratedField::Restriction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VisibilityRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.VisibilityRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VisibilityRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut restriction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Restriction => {
                            if restriction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restriction"));
                            }
                            restriction__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VisibilityRule {
                    selector: selector__.unwrap_or_default(),
                    restriction: restriction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.VisibilityRule", FIELDS, GeneratedVisitor)
    }
}
