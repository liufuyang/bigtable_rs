// @generated
impl serde::Serialize for HttpRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_method.is_empty() {
            len += 1;
        }
        if !self.request_url.is_empty() {
            len += 1;
        }
        if self.request_size != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.response_size != 0 {
            len += 1;
        }
        if !self.user_agent.is_empty() {
            len += 1;
        }
        if !self.remote_ip.is_empty() {
            len += 1;
        }
        if !self.server_ip.is_empty() {
            len += 1;
        }
        if !self.referer.is_empty() {
            len += 1;
        }
        if self.latency.is_some() {
            len += 1;
        }
        if self.cache_lookup {
            len += 1;
        }
        if self.cache_hit {
            len += 1;
        }
        if self.cache_validated_with_origin_server {
            len += 1;
        }
        if self.cache_fill_bytes != 0 {
            len += 1;
        }
        if !self.protocol.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.r#type.HttpRequest", len)?;
        if !self.request_method.is_empty() {
            struct_ser.serialize_field("requestMethod", &self.request_method)?;
        }
        if !self.request_url.is_empty() {
            struct_ser.serialize_field("requestUrl", &self.request_url)?;
        }
        if self.request_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("requestSize", ToString::to_string(&self.request_size).as_str())?;
        }
        if self.status != 0 {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if self.response_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("responseSize", ToString::to_string(&self.response_size).as_str())?;
        }
        if !self.user_agent.is_empty() {
            struct_ser.serialize_field("userAgent", &self.user_agent)?;
        }
        if !self.remote_ip.is_empty() {
            struct_ser.serialize_field("remoteIp", &self.remote_ip)?;
        }
        if !self.server_ip.is_empty() {
            struct_ser.serialize_field("serverIp", &self.server_ip)?;
        }
        if !self.referer.is_empty() {
            struct_ser.serialize_field("referer", &self.referer)?;
        }
        if let Some(v) = self.latency.as_ref() {
            struct_ser.serialize_field("latency", v)?;
        }
        if self.cache_lookup {
            struct_ser.serialize_field("cacheLookup", &self.cache_lookup)?;
        }
        if self.cache_hit {
            struct_ser.serialize_field("cacheHit", &self.cache_hit)?;
        }
        if self.cache_validated_with_origin_server {
            struct_ser.serialize_field("cacheValidatedWithOriginServer", &self.cache_validated_with_origin_server)?;
        }
        if self.cache_fill_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("cacheFillBytes", ToString::to_string(&self.cache_fill_bytes).as_str())?;
        }
        if !self.protocol.is_empty() {
            struct_ser.serialize_field("protocol", &self.protocol)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_method",
            "requestMethod",
            "request_url",
            "requestUrl",
            "request_size",
            "requestSize",
            "status",
            "response_size",
            "responseSize",
            "user_agent",
            "userAgent",
            "remote_ip",
            "remoteIp",
            "server_ip",
            "serverIp",
            "referer",
            "latency",
            "cache_lookup",
            "cacheLookup",
            "cache_hit",
            "cacheHit",
            "cache_validated_with_origin_server",
            "cacheValidatedWithOriginServer",
            "cache_fill_bytes",
            "cacheFillBytes",
            "protocol",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestMethod,
            RequestUrl,
            RequestSize,
            Status,
            ResponseSize,
            UserAgent,
            RemoteIp,
            ServerIp,
            Referer,
            Latency,
            CacheLookup,
            CacheHit,
            CacheValidatedWithOriginServer,
            CacheFillBytes,
            Protocol,
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
                            "requestMethod" | "request_method" => Ok(GeneratedField::RequestMethod),
                            "requestUrl" | "request_url" => Ok(GeneratedField::RequestUrl),
                            "requestSize" | "request_size" => Ok(GeneratedField::RequestSize),
                            "status" => Ok(GeneratedField::Status),
                            "responseSize" | "response_size" => Ok(GeneratedField::ResponseSize),
                            "userAgent" | "user_agent" => Ok(GeneratedField::UserAgent),
                            "remoteIp" | "remote_ip" => Ok(GeneratedField::RemoteIp),
                            "serverIp" | "server_ip" => Ok(GeneratedField::ServerIp),
                            "referer" => Ok(GeneratedField::Referer),
                            "latency" => Ok(GeneratedField::Latency),
                            "cacheLookup" | "cache_lookup" => Ok(GeneratedField::CacheLookup),
                            "cacheHit" | "cache_hit" => Ok(GeneratedField::CacheHit),
                            "cacheValidatedWithOriginServer" | "cache_validated_with_origin_server" => Ok(GeneratedField::CacheValidatedWithOriginServer),
                            "cacheFillBytes" | "cache_fill_bytes" => Ok(GeneratedField::CacheFillBytes),
                            "protocol" => Ok(GeneratedField::Protocol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.r#type.HttpRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_method__ = None;
                let mut request_url__ = None;
                let mut request_size__ = None;
                let mut status__ = None;
                let mut response_size__ = None;
                let mut user_agent__ = None;
                let mut remote_ip__ = None;
                let mut server_ip__ = None;
                let mut referer__ = None;
                let mut latency__ = None;
                let mut cache_lookup__ = None;
                let mut cache_hit__ = None;
                let mut cache_validated_with_origin_server__ = None;
                let mut cache_fill_bytes__ = None;
                let mut protocol__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestMethod => {
                            if request_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMethod"));
                            }
                            request_method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestUrl => {
                            if request_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestUrl"));
                            }
                            request_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestSize => {
                            if request_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestSize"));
                            }
                            request_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResponseSize => {
                            if response_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseSize"));
                            }
                            response_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UserAgent => {
                            if user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgent"));
                            }
                            user_agent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoteIp => {
                            if remote_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteIp"));
                            }
                            remote_ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServerIp => {
                            if server_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverIp"));
                            }
                            server_ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Referer => {
                            if referer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referer"));
                            }
                            referer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Latency => {
                            if latency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latency"));
                            }
                            latency__ = map_.next_value()?;
                        }
                        GeneratedField::CacheLookup => {
                            if cache_lookup__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheLookup"));
                            }
                            cache_lookup__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CacheHit => {
                            if cache_hit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheHit"));
                            }
                            cache_hit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CacheValidatedWithOriginServer => {
                            if cache_validated_with_origin_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheValidatedWithOriginServer"));
                            }
                            cache_validated_with_origin_server__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CacheFillBytes => {
                            if cache_fill_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheFillBytes"));
                            }
                            cache_fill_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HttpRequest {
                    request_method: request_method__.unwrap_or_default(),
                    request_url: request_url__.unwrap_or_default(),
                    request_size: request_size__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    response_size: response_size__.unwrap_or_default(),
                    user_agent: user_agent__.unwrap_or_default(),
                    remote_ip: remote_ip__.unwrap_or_default(),
                    server_ip: server_ip__.unwrap_or_default(),
                    referer: referer__.unwrap_or_default(),
                    latency: latency__,
                    cache_lookup: cache_lookup__.unwrap_or_default(),
                    cache_hit: cache_hit__.unwrap_or_default(),
                    cache_validated_with_origin_server: cache_validated_with_origin_server__.unwrap_or_default(),
                    cache_fill_bytes: cache_fill_bytes__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.r#type.HttpRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogSeverity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Notice => "NOTICE",
            Self::Warning => "WARNING",
            Self::Error => "ERROR",
            Self::Critical => "CRITICAL",
            Self::Alert => "ALERT",
            Self::Emergency => "EMERGENCY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for LogSeverity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "DEBUG",
            "INFO",
            "NOTICE",
            "WARNING",
            "ERROR",
            "CRITICAL",
            "ALERT",
            "EMERGENCY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogSeverity;

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
                    "DEFAULT" => Ok(LogSeverity::Default),
                    "DEBUG" => Ok(LogSeverity::Debug),
                    "INFO" => Ok(LogSeverity::Info),
                    "NOTICE" => Ok(LogSeverity::Notice),
                    "WARNING" => Ok(LogSeverity::Warning),
                    "ERROR" => Ok(LogSeverity::Error),
                    "CRITICAL" => Ok(LogSeverity::Critical),
                    "ALERT" => Ok(LogSeverity::Alert),
                    "EMERGENCY" => Ok(LogSeverity::Emergency),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
