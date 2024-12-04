// @generated
impl serde::Serialize for AttributeContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.origin.is_some() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        if self.destination.is_some() {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        if self.response.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if self.api.is_some() {
            len += 1;
        }
        if !self.extensions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.context.AttributeContext", len)?;
        if let Some(v) = self.origin.as_ref() {
            struct_ser.serialize_field("origin", v)?;
        }
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if let Some(v) = self.destination.as_ref() {
            struct_ser.serialize_field("destination", v)?;
        }
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if let Some(v) = self.response.as_ref() {
            struct_ser.serialize_field("response", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if let Some(v) = self.api.as_ref() {
            struct_ser.serialize_field("api", v)?;
        }
        if !self.extensions.is_empty() {
            struct_ser.serialize_field("extensions", &self.extensions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributeContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "origin",
            "source",
            "destination",
            "request",
            "response",
            "resource",
            "api",
            "extensions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Origin,
            Source,
            Destination,
            Request,
            Response,
            Resource,
            Api,
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
                            "origin" => Ok(GeneratedField::Origin),
                            "source" => Ok(GeneratedField::Source),
                            "destination" => Ok(GeneratedField::Destination),
                            "request" => Ok(GeneratedField::Request),
                            "response" => Ok(GeneratedField::Response),
                            "resource" => Ok(GeneratedField::Resource),
                            "api" => Ok(GeneratedField::Api),
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
            type Value = AttributeContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.context.AttributeContext")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributeContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut origin__ = None;
                let mut source__ = None;
                let mut destination__ = None;
                let mut request__ = None;
                let mut response__ = None;
                let mut resource__ = None;
                let mut api__ = None;
                let mut extensions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Origin => {
                            if origin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            origin__ = map_.next_value()?;
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = map_.next_value()?;
                        }
                        GeneratedField::Destination => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            destination__ = map_.next_value()?;
                        }
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map_.next_value()?;
                        }
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = map_.next_value()?;
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::Api => {
                            if api__.is_some() {
                                return Err(serde::de::Error::duplicate_field("api"));
                            }
                            api__ = map_.next_value()?;
                        }
                        GeneratedField::Extensions => {
                            if extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            extensions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AttributeContext {
                    origin: origin__,
                    source: source__,
                    destination: destination__,
                    request: request__,
                    response: response__,
                    resource: resource__,
                    api: api__,
                    extensions: extensions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.context.AttributeContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_context::Api {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service.is_empty() {
            len += 1;
        }
        if !self.operation.is_empty() {
            len += 1;
        }
        if !self.protocol.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.context.AttributeContext.Api", len)?;
        if !self.service.is_empty() {
            struct_ser.serialize_field("service", &self.service)?;
        }
        if !self.operation.is_empty() {
            struct_ser.serialize_field("operation", &self.operation)?;
        }
        if !self.protocol.is_empty() {
            struct_ser.serialize_field("protocol", &self.protocol)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_context::Api {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service",
            "operation",
            "protocol",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Service,
            Operation,
            Protocol,
            Version,
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
                            "service" => Ok(GeneratedField::Service),
                            "operation" => Ok(GeneratedField::Operation),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_context::Api;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.context.AttributeContext.Api")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_context::Api, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service__ = None;
                let mut operation__ = None;
                let mut protocol__ = None;
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(attribute_context::Api {
                    service: service__.unwrap_or_default(),
                    operation: operation__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.context.AttributeContext.Api", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_context::Auth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal.is_empty() {
            len += 1;
        }
        if !self.audiences.is_empty() {
            len += 1;
        }
        if !self.presenter.is_empty() {
            len += 1;
        }
        if self.claims.is_some() {
            len += 1;
        }
        if !self.access_levels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.context.AttributeContext.Auth", len)?;
        if !self.principal.is_empty() {
            struct_ser.serialize_field("principal", &self.principal)?;
        }
        if !self.audiences.is_empty() {
            struct_ser.serialize_field("audiences", &self.audiences)?;
        }
        if !self.presenter.is_empty() {
            struct_ser.serialize_field("presenter", &self.presenter)?;
        }
        if let Some(v) = self.claims.as_ref() {
            struct_ser.serialize_field("claims", v)?;
        }
        if !self.access_levels.is_empty() {
            struct_ser.serialize_field("accessLevels", &self.access_levels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_context::Auth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal",
            "audiences",
            "presenter",
            "claims",
            "access_levels",
            "accessLevels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Principal,
            Audiences,
            Presenter,
            Claims,
            AccessLevels,
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
                            "principal" => Ok(GeneratedField::Principal),
                            "audiences" => Ok(GeneratedField::Audiences),
                            "presenter" => Ok(GeneratedField::Presenter),
                            "claims" => Ok(GeneratedField::Claims),
                            "accessLevels" | "access_levels" => Ok(GeneratedField::AccessLevels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_context::Auth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.context.AttributeContext.Auth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_context::Auth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal__ = None;
                let mut audiences__ = None;
                let mut presenter__ = None;
                let mut claims__ = None;
                let mut access_levels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Principal => {
                            if principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Audiences => {
                            if audiences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audiences"));
                            }
                            audiences__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Presenter => {
                            if presenter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presenter"));
                            }
                            presenter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Claims => {
                            if claims__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claims"));
                            }
                            claims__ = map_.next_value()?;
                        }
                        GeneratedField::AccessLevels => {
                            if access_levels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLevels"));
                            }
                            access_levels__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(attribute_context::Auth {
                    principal: principal__.unwrap_or_default(),
                    audiences: audiences__.unwrap_or_default(),
                    presenter: presenter__.unwrap_or_default(),
                    claims: claims__,
                    access_levels: access_levels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.context.AttributeContext.Auth", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_context::Peer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ip.is_empty() {
            len += 1;
        }
        if self.port != 0 {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if !self.principal.is_empty() {
            len += 1;
        }
        if !self.region_code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.context.AttributeContext.Peer", len)?;
        if !self.ip.is_empty() {
            struct_ser.serialize_field("ip", &self.ip)?;
        }
        if self.port != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("port", ToString::to_string(&self.port).as_str())?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if !self.principal.is_empty() {
            struct_ser.serialize_field("principal", &self.principal)?;
        }
        if !self.region_code.is_empty() {
            struct_ser.serialize_field("regionCode", &self.region_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_context::Peer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ip",
            "port",
            "labels",
            "principal",
            "region_code",
            "regionCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ip,
            Port,
            Labels,
            Principal,
            RegionCode,
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
                            "ip" => Ok(GeneratedField::Ip),
                            "port" => Ok(GeneratedField::Port),
                            "labels" => Ok(GeneratedField::Labels),
                            "principal" => Ok(GeneratedField::Principal),
                            "regionCode" | "region_code" => Ok(GeneratedField::RegionCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_context::Peer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.context.AttributeContext.Peer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_context::Peer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ip__ = None;
                let mut port__ = None;
                let mut labels__ = None;
                let mut principal__ = None;
                let mut region_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ip => {
                            if ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Principal => {
                            if principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RegionCode => {
                            if region_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regionCode"));
                            }
                            region_code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(attribute_context::Peer {
                    ip: ip__.unwrap_or_default(),
                    port: port__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                    principal: principal__.unwrap_or_default(),
                    region_code: region_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.context.AttributeContext.Peer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_context::Request {
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
        if !self.method.is_empty() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.host.is_empty() {
            len += 1;
        }
        if !self.scheme.is_empty() {
            len += 1;
        }
        if !self.query.is_empty() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        if !self.protocol.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        if self.auth.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.context.AttributeContext.Request", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.host.is_empty() {
            struct_ser.serialize_field("host", &self.host)?;
        }
        if !self.scheme.is_empty() {
            struct_ser.serialize_field("scheme", &self.scheme)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if self.size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("size", ToString::to_string(&self.size).as_str())?;
        }
        if !self.protocol.is_empty() {
            struct_ser.serialize_field("protocol", &self.protocol)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if let Some(v) = self.auth.as_ref() {
            struct_ser.serialize_field("auth", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_context::Request {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "method",
            "headers",
            "path",
            "host",
            "scheme",
            "query",
            "time",
            "size",
            "protocol",
            "reason",
            "auth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Method,
            Headers,
            Path,
            Host,
            Scheme,
            Query,
            Time,
            Size,
            Protocol,
            Reason,
            Auth,
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
                            "method" => Ok(GeneratedField::Method),
                            "headers" => Ok(GeneratedField::Headers),
                            "path" => Ok(GeneratedField::Path),
                            "host" => Ok(GeneratedField::Host),
                            "scheme" => Ok(GeneratedField::Scheme),
                            "query" => Ok(GeneratedField::Query),
                            "time" => Ok(GeneratedField::Time),
                            "size" => Ok(GeneratedField::Size),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "reason" => Ok(GeneratedField::Reason),
                            "auth" => Ok(GeneratedField::Auth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_context::Request;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.context.AttributeContext.Request")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_context::Request, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut method__ = None;
                let mut headers__ = None;
                let mut path__ = None;
                let mut host__ = None;
                let mut scheme__ = None;
                let mut query__ = None;
                let mut time__ = None;
                let mut size__ = None;
                let mut protocol__ = None;
                let mut reason__ = None;
                let mut auth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheme"));
                            }
                            scheme__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Auth => {
                            if auth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auth"));
                            }
                            auth__ = map_.next_value()?;
                        }
                    }
                }
                Ok(attribute_context::Request {
                    id: id__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    host: host__.unwrap_or_default(),
                    scheme: scheme__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                    time: time__,
                    size: size__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    auth: auth__,
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.context.AttributeContext.Request", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_context::Resource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if !self.uid.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.create_time.is_some() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        if self.delete_time.is_some() {
            len += 1;
        }
        if !self.etag.is_empty() {
            len += 1;
        }
        if !self.location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.context.AttributeContext.Resource", len)?;
        if !self.service.is_empty() {
            struct_ser.serialize_field("service", &self.service)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        if let Some(v) = self.delete_time.as_ref() {
            struct_ser.serialize_field("deleteTime", v)?;
        }
        if !self.etag.is_empty() {
            struct_ser.serialize_field("etag", &self.etag)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_context::Resource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service",
            "name",
            "type",
            "labels",
            "uid",
            "annotations",
            "display_name",
            "displayName",
            "create_time",
            "createTime",
            "update_time",
            "updateTime",
            "delete_time",
            "deleteTime",
            "etag",
            "location",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Service,
            Name,
            Type,
            Labels,
            Uid,
            Annotations,
            DisplayName,
            CreateTime,
            UpdateTime,
            DeleteTime,
            Etag,
            Location,
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
                            "service" => Ok(GeneratedField::Service),
                            "name" => Ok(GeneratedField::Name),
                            "type" => Ok(GeneratedField::Type),
                            "labels" => Ok(GeneratedField::Labels),
                            "uid" => Ok(GeneratedField::Uid),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
                            "deleteTime" | "delete_time" => Ok(GeneratedField::DeleteTime),
                            "etag" => Ok(GeneratedField::Etag),
                            "location" => Ok(GeneratedField::Location),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_context::Resource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.context.AttributeContext.Resource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_context::Resource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service__ = None;
                let mut name__ = None;
                let mut r#type__ = None;
                let mut labels__ = None;
                let mut uid__ = None;
                let mut annotations__ = None;
                let mut display_name__ = None;
                let mut create_time__ = None;
                let mut update_time__ = None;
                let mut delete_time__ = None;
                let mut etag__ = None;
                let mut location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = Some(map_.next_value()?);
                        }
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
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreateTime => {
                            if create_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createTime"));
                            }
                            create_time__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateTime => {
                            if update_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateTime"));
                            }
                            update_time__ = map_.next_value()?;
                        }
                        GeneratedField::DeleteTime => {
                            if delete_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteTime"));
                            }
                            delete_time__ = map_.next_value()?;
                        }
                        GeneratedField::Etag => {
                            if etag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("etag"));
                            }
                            etag__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(attribute_context::Resource {
                    service: service__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                    uid: uid__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    create_time: create_time__,
                    update_time: update_time__,
                    delete_time: delete_time__,
                    etag: etag__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.context.AttributeContext.Resource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_context::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.backend_latency.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.context.AttributeContext.Response", len)?;
        if self.code != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("code", ToString::to_string(&self.code).as_str())?;
        }
        if self.size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("size", ToString::to_string(&self.size).as_str())?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if let Some(v) = self.backend_latency.as_ref() {
            struct_ser.serialize_field("backendLatency", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_context::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "size",
            "headers",
            "time",
            "backend_latency",
            "backendLatency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Size,
            Headers,
            Time,
            BackendLatency,
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
                            "code" => Ok(GeneratedField::Code),
                            "size" => Ok(GeneratedField::Size),
                            "headers" => Ok(GeneratedField::Headers),
                            "time" => Ok(GeneratedField::Time),
                            "backendLatency" | "backend_latency" => Ok(GeneratedField::BackendLatency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_context::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.context.AttributeContext.Response")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_context::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut size__ = None;
                let mut headers__ = None;
                let mut time__ = None;
                let mut backend_latency__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::BackendLatency => {
                            if backend_latency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backendLatency"));
                            }
                            backend_latency__ = map_.next_value()?;
                        }
                    }
                }
                Ok(attribute_context::Response {
                    code: code__.unwrap_or_default(),
                    size: size__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    time: time__,
                    backend_latency: backend_latency__,
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.context.AttributeContext.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuditContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.audit_log.is_empty() {
            len += 1;
        }
        if self.scrubbed_request.is_some() {
            len += 1;
        }
        if self.scrubbed_response.is_some() {
            len += 1;
        }
        if self.scrubbed_response_item_count != 0 {
            len += 1;
        }
        if !self.target_resource.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.context.AuditContext", len)?;
        if !self.audit_log.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("auditLog", pbjson::private::base64::encode(&self.audit_log).as_str())?;
        }
        if let Some(v) = self.scrubbed_request.as_ref() {
            struct_ser.serialize_field("scrubbedRequest", v)?;
        }
        if let Some(v) = self.scrubbed_response.as_ref() {
            struct_ser.serialize_field("scrubbedResponse", v)?;
        }
        if self.scrubbed_response_item_count != 0 {
            struct_ser.serialize_field("scrubbedResponseItemCount", &self.scrubbed_response_item_count)?;
        }
        if !self.target_resource.is_empty() {
            struct_ser.serialize_field("targetResource", &self.target_resource)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuditContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "audit_log",
            "auditLog",
            "scrubbed_request",
            "scrubbedRequest",
            "scrubbed_response",
            "scrubbedResponse",
            "scrubbed_response_item_count",
            "scrubbedResponseItemCount",
            "target_resource",
            "targetResource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuditLog,
            ScrubbedRequest,
            ScrubbedResponse,
            ScrubbedResponseItemCount,
            TargetResource,
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
                            "auditLog" | "audit_log" => Ok(GeneratedField::AuditLog),
                            "scrubbedRequest" | "scrubbed_request" => Ok(GeneratedField::ScrubbedRequest),
                            "scrubbedResponse" | "scrubbed_response" => Ok(GeneratedField::ScrubbedResponse),
                            "scrubbedResponseItemCount" | "scrubbed_response_item_count" => Ok(GeneratedField::ScrubbedResponseItemCount),
                            "targetResource" | "target_resource" => Ok(GeneratedField::TargetResource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuditContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.context.AuditContext")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuditContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut audit_log__ = None;
                let mut scrubbed_request__ = None;
                let mut scrubbed_response__ = None;
                let mut scrubbed_response_item_count__ = None;
                let mut target_resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuditLog => {
                            if audit_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auditLog"));
                            }
                            audit_log__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ScrubbedRequest => {
                            if scrubbed_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scrubbedRequest"));
                            }
                            scrubbed_request__ = map_.next_value()?;
                        }
                        GeneratedField::ScrubbedResponse => {
                            if scrubbed_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scrubbedResponse"));
                            }
                            scrubbed_response__ = map_.next_value()?;
                        }
                        GeneratedField::ScrubbedResponseItemCount => {
                            if scrubbed_response_item_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scrubbedResponseItemCount"));
                            }
                            scrubbed_response_item_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TargetResource => {
                            if target_resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetResource"));
                            }
                            target_resource__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuditContext {
                    audit_log: audit_log__.unwrap_or_default(),
                    scrubbed_request: scrubbed_request__,
                    scrubbed_response: scrubbed_response__,
                    scrubbed_response_item_count: scrubbed_response_item_count__.unwrap_or_default(),
                    target_resource: target_resource__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.context.AuditContext", FIELDS, GeneratedVisitor)
    }
}
