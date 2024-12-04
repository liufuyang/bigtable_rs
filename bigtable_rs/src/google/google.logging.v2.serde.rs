// @generated
impl serde::Serialize for BigQueryDataset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dataset_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.BigQueryDataset", len)?;
        if !self.dataset_id.is_empty() {
            struct_ser.serialize_field("datasetId", &self.dataset_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryDataset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dataset_id",
            "datasetId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DatasetId,
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
                            "datasetId" | "dataset_id" => Ok(GeneratedField::DatasetId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryDataset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.BigQueryDataset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryDataset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dataset_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DatasetId => {
                            if dataset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("datasetId"));
                            }
                            dataset_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryDataset {
                    dataset_id: dataset_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.BigQueryDataset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.use_partitioned_tables {
            len += 1;
        }
        if self.uses_timestamp_column_partitioning {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.BigQueryOptions", len)?;
        if self.use_partitioned_tables {
            struct_ser.serialize_field("usePartitionedTables", &self.use_partitioned_tables)?;
        }
        if self.uses_timestamp_column_partitioning {
            struct_ser.serialize_field("usesTimestampColumnPartitioning", &self.uses_timestamp_column_partitioning)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "use_partitioned_tables",
            "usePartitionedTables",
            "uses_timestamp_column_partitioning",
            "usesTimestampColumnPartitioning",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UsePartitionedTables,
            UsesTimestampColumnPartitioning,
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
                            "usePartitionedTables" | "use_partitioned_tables" => Ok(GeneratedField::UsePartitionedTables),
                            "usesTimestampColumnPartitioning" | "uses_timestamp_column_partitioning" => Ok(GeneratedField::UsesTimestampColumnPartitioning),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.BigQueryOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut use_partitioned_tables__ = None;
                let mut uses_timestamp_column_partitioning__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UsePartitionedTables => {
                            if use_partitioned_tables__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usePartitionedTables"));
                            }
                            use_partitioned_tables__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UsesTimestampColumnPartitioning => {
                            if uses_timestamp_column_partitioning__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usesTimestampColumnPartitioning"));
                            }
                            uses_timestamp_column_partitioning__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryOptions {
                    use_partitioned_tables: use_partitioned_tables__.unwrap_or_default(),
                    uses_timestamp_column_partitioning: uses_timestamp_column_partitioning__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.BigQueryOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BucketMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.BucketMetadata", len)?;
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if self.state != 0 {
            let v = OperationState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.request.as_ref() {
            match v {
                bucket_metadata::Request::CreateBucketRequest(v) => {
                    struct_ser.serialize_field("createBucketRequest", v)?;
                }
                bucket_metadata::Request::UpdateBucketRequest(v) => {
                    struct_ser.serialize_field("updateBucketRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BucketMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "state",
            "create_bucket_request",
            "createBucketRequest",
            "update_bucket_request",
            "updateBucketRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartTime,
            EndTime,
            State,
            CreateBucketRequest,
            UpdateBucketRequest,
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
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "state" => Ok(GeneratedField::State),
                            "createBucketRequest" | "create_bucket_request" => Ok(GeneratedField::CreateBucketRequest),
                            "updateBucketRequest" | "update_bucket_request" => Ok(GeneratedField::UpdateBucketRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BucketMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.BucketMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BucketMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut state__ = None;
                let mut request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<OperationState>()? as i32);
                        }
                        GeneratedField::CreateBucketRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createBucketRequest"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(bucket_metadata::Request::CreateBucketRequest)
;
                        }
                        GeneratedField::UpdateBucketRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateBucketRequest"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(bucket_metadata::Request::UpdateBucketRequest)
;
                        }
                    }
                }
                Ok(BucketMetadata {
                    start_time: start_time__,
                    end_time: end_time__,
                    state: state__.unwrap_or_default(),
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.BucketMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CmekSettings {
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
        if !self.kms_key_name.is_empty() {
            len += 1;
        }
        if !self.kms_key_version_name.is_empty() {
            len += 1;
        }
        if !self.service_account_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.CmekSettings", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.kms_key_name.is_empty() {
            struct_ser.serialize_field("kmsKeyName", &self.kms_key_name)?;
        }
        if !self.kms_key_version_name.is_empty() {
            struct_ser.serialize_field("kmsKeyVersionName", &self.kms_key_version_name)?;
        }
        if !self.service_account_id.is_empty() {
            struct_ser.serialize_field("serviceAccountId", &self.service_account_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CmekSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "kms_key_name",
            "kmsKeyName",
            "kms_key_version_name",
            "kmsKeyVersionName",
            "service_account_id",
            "serviceAccountId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            KmsKeyName,
            KmsKeyVersionName,
            ServiceAccountId,
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
                            "kmsKeyName" | "kms_key_name" => Ok(GeneratedField::KmsKeyName),
                            "kmsKeyVersionName" | "kms_key_version_name" => Ok(GeneratedField::KmsKeyVersionName),
                            "serviceAccountId" | "service_account_id" => Ok(GeneratedField::ServiceAccountId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CmekSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.CmekSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CmekSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut kms_key_name__ = None;
                let mut kms_key_version_name__ = None;
                let mut service_account_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KmsKeyName => {
                            if kms_key_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kmsKeyName"));
                            }
                            kms_key_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KmsKeyVersionName => {
                            if kms_key_version_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kmsKeyVersionName"));
                            }
                            kms_key_version_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceAccountId => {
                            if service_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceAccountId"));
                            }
                            service_account_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CmekSettings {
                    name: name__.unwrap_or_default(),
                    kms_key_name: kms_key_name__.unwrap_or_default(),
                    kms_key_version_name: kms_key_version_name__.unwrap_or_default(),
                    service_account_id: service_account_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.CmekSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CopyLogEntriesMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.cancellation_requested {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        if self.progress != 0 {
            len += 1;
        }
        if !self.writer_identity.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.CopyLogEntriesMetadata", len)?;
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if self.state != 0 {
            let v = OperationState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if self.cancellation_requested {
            struct_ser.serialize_field("cancellationRequested", &self.cancellation_requested)?;
        }
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if self.progress != 0 {
            struct_ser.serialize_field("progress", &self.progress)?;
        }
        if !self.writer_identity.is_empty() {
            struct_ser.serialize_field("writerIdentity", &self.writer_identity)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CopyLogEntriesMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "state",
            "cancellation_requested",
            "cancellationRequested",
            "request",
            "progress",
            "writer_identity",
            "writerIdentity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartTime,
            EndTime,
            State,
            CancellationRequested,
            Request,
            Progress,
            WriterIdentity,
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
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "state" => Ok(GeneratedField::State),
                            "cancellationRequested" | "cancellation_requested" => Ok(GeneratedField::CancellationRequested),
                            "request" => Ok(GeneratedField::Request),
                            "progress" => Ok(GeneratedField::Progress),
                            "writerIdentity" | "writer_identity" => Ok(GeneratedField::WriterIdentity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CopyLogEntriesMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.CopyLogEntriesMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CopyLogEntriesMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut state__ = None;
                let mut cancellation_requested__ = None;
                let mut request__ = None;
                let mut progress__ = None;
                let mut writer_identity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<OperationState>()? as i32);
                        }
                        GeneratedField::CancellationRequested => {
                            if cancellation_requested__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancellationRequested"));
                            }
                            cancellation_requested__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map_.next_value()?;
                        }
                        GeneratedField::Progress => {
                            if progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("progress"));
                            }
                            progress__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::WriterIdentity => {
                            if writer_identity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("writerIdentity"));
                            }
                            writer_identity__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CopyLogEntriesMetadata {
                    start_time: start_time__,
                    end_time: end_time__,
                    state: state__.unwrap_or_default(),
                    cancellation_requested: cancellation_requested__.unwrap_or_default(),
                    request: request__,
                    progress: progress__.unwrap_or_default(),
                    writer_identity: writer_identity__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.CopyLogEntriesMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CopyLogEntriesRequest {
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
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.destination.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.CopyLogEntriesRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.destination.is_empty() {
            struct_ser.serialize_field("destination", &self.destination)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CopyLogEntriesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "filter",
            "destination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Filter,
            Destination,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "destination" => Ok(GeneratedField::Destination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CopyLogEntriesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.CopyLogEntriesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CopyLogEntriesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut filter__ = None;
                let mut destination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Destination => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            destination__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CopyLogEntriesRequest {
                    name: name__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    destination: destination__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.CopyLogEntriesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CopyLogEntriesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.log_entries_copied_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.CopyLogEntriesResponse", len)?;
        if self.log_entries_copied_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("logEntriesCopiedCount", ToString::to_string(&self.log_entries_copied_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CopyLogEntriesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_entries_copied_count",
            "logEntriesCopiedCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogEntriesCopiedCount,
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
                            "logEntriesCopiedCount" | "log_entries_copied_count" => Ok(GeneratedField::LogEntriesCopiedCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CopyLogEntriesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.CopyLogEntriesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CopyLogEntriesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_entries_copied_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogEntriesCopiedCount => {
                            if log_entries_copied_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logEntriesCopiedCount"));
                            }
                            log_entries_copied_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CopyLogEntriesResponse {
                    log_entries_copied_count: log_entries_copied_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.CopyLogEntriesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateBucketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.bucket_id.is_empty() {
            len += 1;
        }
        if self.bucket.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.CreateBucketRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.bucket_id.is_empty() {
            struct_ser.serialize_field("bucketId", &self.bucket_id)?;
        }
        if let Some(v) = self.bucket.as_ref() {
            struct_ser.serialize_field("bucket", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "bucket_id",
            "bucketId",
            "bucket",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            BucketId,
            Bucket,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "bucketId" | "bucket_id" => Ok(GeneratedField::BucketId),
                            "bucket" => Ok(GeneratedField::Bucket),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.CreateBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut bucket_id__ = None;
                let mut bucket__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BucketId => {
                            if bucket_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketId"));
                            }
                            bucket_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bucket => {
                            if bucket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucket"));
                            }
                            bucket__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateBucketRequest {
                    parent: parent__.unwrap_or_default(),
                    bucket_id: bucket_id__.unwrap_or_default(),
                    bucket: bucket__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.CreateBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateExclusionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.exclusion.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.CreateExclusionRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if let Some(v) = self.exclusion.as_ref() {
            struct_ser.serialize_field("exclusion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateExclusionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "exclusion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            Exclusion,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "exclusion" => Ok(GeneratedField::Exclusion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateExclusionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.CreateExclusionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateExclusionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut exclusion__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exclusion => {
                            if exclusion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exclusion"));
                            }
                            exclusion__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateExclusionRequest {
                    parent: parent__.unwrap_or_default(),
                    exclusion: exclusion__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.CreateExclusionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateLinkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.link.is_some() {
            len += 1;
        }
        if !self.link_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.CreateLinkRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if let Some(v) = self.link.as_ref() {
            struct_ser.serialize_field("link", v)?;
        }
        if !self.link_id.is_empty() {
            struct_ser.serialize_field("linkId", &self.link_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateLinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "link",
            "link_id",
            "linkId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            Link,
            LinkId,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "link" => Ok(GeneratedField::Link),
                            "linkId" | "link_id" => Ok(GeneratedField::LinkId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateLinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.CreateLinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateLinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut link__ = None;
                let mut link_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Link => {
                            if link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            link__ = map_.next_value()?;
                        }
                        GeneratedField::LinkId => {
                            if link_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkId"));
                            }
                            link_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateLinkRequest {
                    parent: parent__.unwrap_or_default(),
                    link: link__,
                    link_id: link_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.CreateLinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateLogMetricRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.metric.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.CreateLogMetricRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if let Some(v) = self.metric.as_ref() {
            struct_ser.serialize_field("metric", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateLogMetricRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "metric",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            Metric,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "metric" => Ok(GeneratedField::Metric),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateLogMetricRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.CreateLogMetricRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateLogMetricRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut metric__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metric => {
                            if metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }
                            metric__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateLogMetricRequest {
                    parent: parent__.unwrap_or_default(),
                    metric: metric__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.CreateLogMetricRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateSinkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.sink.is_some() {
            len += 1;
        }
        if self.unique_writer_identity {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.CreateSinkRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if let Some(v) = self.sink.as_ref() {
            struct_ser.serialize_field("sink", v)?;
        }
        if self.unique_writer_identity {
            struct_ser.serialize_field("uniqueWriterIdentity", &self.unique_writer_identity)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "sink",
            "unique_writer_identity",
            "uniqueWriterIdentity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            Sink,
            UniqueWriterIdentity,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "sink" => Ok(GeneratedField::Sink),
                            "uniqueWriterIdentity" | "unique_writer_identity" => Ok(GeneratedField::UniqueWriterIdentity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateSinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.CreateSinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateSinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut sink__ = None;
                let mut unique_writer_identity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sink => {
                            if sink__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sink"));
                            }
                            sink__ = map_.next_value()?;
                        }
                        GeneratedField::UniqueWriterIdentity => {
                            if unique_writer_identity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uniqueWriterIdentity"));
                            }
                            unique_writer_identity__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateSinkRequest {
                    parent: parent__.unwrap_or_default(),
                    sink: sink__,
                    unique_writer_identity: unique_writer_identity__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.CreateSinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateViewRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.view_id.is_empty() {
            len += 1;
        }
        if self.view.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.CreateViewRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.view_id.is_empty() {
            struct_ser.serialize_field("viewId", &self.view_id)?;
        }
        if let Some(v) = self.view.as_ref() {
            struct_ser.serialize_field("view", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "view_id",
            "viewId",
            "view",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            ViewId,
            View,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "viewId" | "view_id" => Ok(GeneratedField::ViewId),
                            "view" => Ok(GeneratedField::View),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.CreateViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut view_id__ = None;
                let mut view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ViewId => {
                            if view_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewId"));
                            }
                            view_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateViewRequest {
                    parent: parent__.unwrap_or_default(),
                    view_id: view_id__.unwrap_or_default(),
                    view: view__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.CreateViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteBucketRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.DeleteBucketRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.DeleteBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteBucketRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.DeleteBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteExclusionRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.DeleteExclusionRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteExclusionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteExclusionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.DeleteExclusionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteExclusionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteExclusionRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.DeleteExclusionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteLinkRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.DeleteLinkRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteLinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteLinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.DeleteLinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteLinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteLinkRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.DeleteLinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteLogMetricRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metric_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.DeleteLogMetricRequest", len)?;
        if !self.metric_name.is_empty() {
            struct_ser.serialize_field("metricName", &self.metric_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteLogMetricRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metric_name",
            "metricName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetricName,
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
                            "metricName" | "metric_name" => Ok(GeneratedField::MetricName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteLogMetricRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.DeleteLogMetricRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteLogMetricRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metric_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MetricName => {
                            if metric_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricName"));
                            }
                            metric_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteLogMetricRequest {
                    metric_name: metric_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.DeleteLogMetricRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteLogRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.DeleteLogRequest", len)?;
        if !self.log_name.is_empty() {
            struct_ser.serialize_field("logName", &self.log_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteLogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_name",
            "logName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogName,
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
                            "logName" | "log_name" => Ok(GeneratedField::LogName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteLogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.DeleteLogRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteLogRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogName => {
                            if log_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logName"));
                            }
                            log_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteLogRequest {
                    log_name: log_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.DeleteLogRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSinkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sink_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.DeleteSinkRequest", len)?;
        if !self.sink_name.is_empty() {
            struct_ser.serialize_field("sinkName", &self.sink_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sink_name",
            "sinkName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SinkName,
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
                            "sinkName" | "sink_name" => Ok(GeneratedField::SinkName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteSinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.DeleteSinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteSinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sink_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SinkName => {
                            if sink_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sinkName"));
                            }
                            sink_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteSinkRequest {
                    sink_name: sink_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.DeleteSinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteViewRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.DeleteViewRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.DeleteViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteViewRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.DeleteViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBucketRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.GetBucketRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.GetBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetBucketRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.GetBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCmekSettingsRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.GetCmekSettingsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCmekSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCmekSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.GetCmekSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCmekSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetCmekSettingsRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.GetCmekSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetExclusionRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.GetExclusionRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetExclusionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetExclusionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.GetExclusionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetExclusionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetExclusionRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.GetExclusionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLinkRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.GetLinkRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.GetLinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetLinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetLinkRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.GetLinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLogMetricRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metric_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.GetLogMetricRequest", len)?;
        if !self.metric_name.is_empty() {
            struct_ser.serialize_field("metricName", &self.metric_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLogMetricRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metric_name",
            "metricName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetricName,
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
                            "metricName" | "metric_name" => Ok(GeneratedField::MetricName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLogMetricRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.GetLogMetricRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetLogMetricRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metric_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MetricName => {
                            if metric_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricName"));
                            }
                            metric_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetLogMetricRequest {
                    metric_name: metric_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.GetLogMetricRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSettingsRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.GetSettingsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.GetSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetSettingsRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.GetSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSinkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sink_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.GetSinkRequest", len)?;
        if !self.sink_name.is_empty() {
            struct_ser.serialize_field("sinkName", &self.sink_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sink_name",
            "sinkName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SinkName,
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
                            "sinkName" | "sink_name" => Ok(GeneratedField::SinkName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.GetSinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sink_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SinkName => {
                            if sink_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sinkName"));
                            }
                            sink_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetSinkRequest {
                    sink_name: sink_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.GetSinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetViewRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.GetViewRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.GetViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetViewRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.GetViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IndexConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field_path.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.create_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.IndexConfig", len)?;
        if !self.field_path.is_empty() {
            struct_ser.serialize_field("fieldPath", &self.field_path)?;
        }
        if self.r#type != 0 {
            let v = IndexType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IndexConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_path",
            "fieldPath",
            "type",
            "create_time",
            "createTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldPath,
            Type,
            CreateTime,
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
                            "fieldPath" | "field_path" => Ok(GeneratedField::FieldPath),
                            "type" => Ok(GeneratedField::Type),
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IndexConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.IndexConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IndexConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_path__ = None;
                let mut r#type__ = None;
                let mut create_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldPath => {
                            if field_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldPath"));
                            }
                            field_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<IndexType>()? as i32);
                        }
                        GeneratedField::CreateTime => {
                            if create_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createTime"));
                            }
                            create_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(IndexConfig {
                    field_path: field_path__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    create_time: create_time__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.IndexConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IndexType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "INDEX_TYPE_UNSPECIFIED",
            Self::String => "INDEX_TYPE_STRING",
            Self::Integer => "INDEX_TYPE_INTEGER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IndexType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INDEX_TYPE_UNSPECIFIED",
            "INDEX_TYPE_STRING",
            "INDEX_TYPE_INTEGER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IndexType;

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
                    "INDEX_TYPE_UNSPECIFIED" => Ok(IndexType::Unspecified),
                    "INDEX_TYPE_STRING" => Ok(IndexType::String),
                    "INDEX_TYPE_INTEGER" => Ok(IndexType::Integer),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LifecycleState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "LIFECYCLE_STATE_UNSPECIFIED",
            Self::Active => "ACTIVE",
            Self::DeleteRequested => "DELETE_REQUESTED",
            Self::Updating => "UPDATING",
            Self::Creating => "CREATING",
            Self::Failed => "FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LIFECYCLE_STATE_UNSPECIFIED",
            "ACTIVE",
            "DELETE_REQUESTED",
            "UPDATING",
            "CREATING",
            "FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LifecycleState;

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
                    "LIFECYCLE_STATE_UNSPECIFIED" => Ok(LifecycleState::Unspecified),
                    "ACTIVE" => Ok(LifecycleState::Active),
                    "DELETE_REQUESTED" => Ok(LifecycleState::DeleteRequested),
                    "UPDATING" => Ok(LifecycleState::Updating),
                    "CREATING" => Ok(LifecycleState::Creating),
                    "FAILED" => Ok(LifecycleState::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Link {
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
        if self.create_time.is_some() {
            len += 1;
        }
        if self.lifecycle_state != 0 {
            len += 1;
        }
        if self.bigquery_dataset.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.Link", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if self.lifecycle_state != 0 {
            let v = LifecycleState::try_from(self.lifecycle_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.lifecycle_state)))?;
            struct_ser.serialize_field("lifecycleState", &v)?;
        }
        if let Some(v) = self.bigquery_dataset.as_ref() {
            struct_ser.serialize_field("bigqueryDataset", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Link {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "create_time",
            "createTime",
            "lifecycle_state",
            "lifecycleState",
            "bigquery_dataset",
            "bigqueryDataset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            CreateTime,
            LifecycleState,
            BigqueryDataset,
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
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "lifecycleState" | "lifecycle_state" => Ok(GeneratedField::LifecycleState),
                            "bigqueryDataset" | "bigquery_dataset" => Ok(GeneratedField::BigqueryDataset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Link;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.Link")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Link, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut create_time__ = None;
                let mut lifecycle_state__ = None;
                let mut bigquery_dataset__ = None;
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
                        GeneratedField::CreateTime => {
                            if create_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createTime"));
                            }
                            create_time__ = map_.next_value()?;
                        }
                        GeneratedField::LifecycleState => {
                            if lifecycle_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycleState"));
                            }
                            lifecycle_state__ = Some(map_.next_value::<LifecycleState>()? as i32);
                        }
                        GeneratedField::BigqueryDataset => {
                            if bigquery_dataset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bigqueryDataset"));
                            }
                            bigquery_dataset__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Link {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    create_time: create_time__,
                    lifecycle_state: lifecycle_state__.unwrap_or_default(),
                    bigquery_dataset: bigquery_dataset__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.Link", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LinkMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LinkMetadata", len)?;
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if self.state != 0 {
            let v = OperationState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.request.as_ref() {
            match v {
                link_metadata::Request::CreateLinkRequest(v) => {
                    struct_ser.serialize_field("createLinkRequest", v)?;
                }
                link_metadata::Request::DeleteLinkRequest(v) => {
                    struct_ser.serialize_field("deleteLinkRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinkMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "state",
            "create_link_request",
            "createLinkRequest",
            "delete_link_request",
            "deleteLinkRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartTime,
            EndTime,
            State,
            CreateLinkRequest,
            DeleteLinkRequest,
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
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "state" => Ok(GeneratedField::State),
                            "createLinkRequest" | "create_link_request" => Ok(GeneratedField::CreateLinkRequest),
                            "deleteLinkRequest" | "delete_link_request" => Ok(GeneratedField::DeleteLinkRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LinkMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LinkMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LinkMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut state__ = None;
                let mut request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<OperationState>()? as i32);
                        }
                        GeneratedField::CreateLinkRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createLinkRequest"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(link_metadata::Request::CreateLinkRequest)
;
                        }
                        GeneratedField::DeleteLinkRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteLinkRequest"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(link_metadata::Request::DeleteLinkRequest)
;
                        }
                    }
                }
                Ok(LinkMetadata {
                    start_time: start_time__,
                    end_time: end_time__,
                    state: state__.unwrap_or_default(),
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LinkMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListBucketsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListBucketsRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListBucketsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_token",
            "pageToken",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageToken,
            PageSize,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListBucketsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListBucketsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListBucketsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_token__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListBucketsRequest {
                    parent: parent__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListBucketsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListBucketsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.buckets.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListBucketsResponse", len)?;
        if !self.buckets.is_empty() {
            struct_ser.serialize_field("buckets", &self.buckets)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListBucketsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "buckets",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Buckets,
            NextPageToken,
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
                            "buckets" => Ok(GeneratedField::Buckets),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListBucketsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListBucketsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListBucketsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut buckets__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Buckets => {
                            if buckets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            buckets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListBucketsResponse {
                    buckets: buckets__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListBucketsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExclusionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListExclusionsRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExclusionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_token",
            "pageToken",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageToken,
            PageSize,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExclusionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListExclusionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExclusionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_token__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListExclusionsRequest {
                    parent: parent__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListExclusionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExclusionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.exclusions.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListExclusionsResponse", len)?;
        if !self.exclusions.is_empty() {
            struct_ser.serialize_field("exclusions", &self.exclusions)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExclusionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exclusions",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Exclusions,
            NextPageToken,
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
                            "exclusions" => Ok(GeneratedField::Exclusions),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExclusionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListExclusionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExclusionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut exclusions__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Exclusions => {
                            if exclusions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exclusions"));
                            }
                            exclusions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListExclusionsResponse {
                    exclusions: exclusions__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListExclusionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListLinksRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListLinksRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListLinksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_token",
            "pageToken",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageToken,
            PageSize,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListLinksRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListLinksRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListLinksRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_token__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListLinksRequest {
                    parent: parent__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListLinksRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListLinksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.links.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListLinksResponse", len)?;
        if !self.links.is_empty() {
            struct_ser.serialize_field("links", &self.links)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListLinksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "links",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Links,
            NextPageToken,
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
                            "links" => Ok(GeneratedField::Links),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListLinksResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListLinksResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListLinksResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut links__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Links => {
                            if links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("links"));
                            }
                            links__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListLinksResponse {
                    links: links__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListLinksResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListLogEntriesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_names.is_empty() {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListLogEntriesRequest", len)?;
        if !self.resource_names.is_empty() {
            struct_ser.serialize_field("resourceNames", &self.resource_names)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListLogEntriesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_names",
            "resourceNames",
            "filter",
            "order_by",
            "orderBy",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceNames,
            Filter,
            OrderBy,
            PageSize,
            PageToken,
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
                            "resourceNames" | "resource_names" => Ok(GeneratedField::ResourceNames),
                            "filter" => Ok(GeneratedField::Filter),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListLogEntriesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListLogEntriesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListLogEntriesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_names__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceNames => {
                            if resource_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNames"));
                            }
                            resource_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListLogEntriesRequest {
                    resource_names: resource_names__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListLogEntriesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListLogEntriesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListLogEntriesResponse", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListLogEntriesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entries",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
            NextPageToken,
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
                            "entries" => Ok(GeneratedField::Entries),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListLogEntriesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListLogEntriesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListLogEntriesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListLogEntriesResponse {
                    entries: entries__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListLogEntriesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListLogMetricsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListLogMetricsRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListLogMetricsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_token",
            "pageToken",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageToken,
            PageSize,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListLogMetricsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListLogMetricsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListLogMetricsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_token__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListLogMetricsRequest {
                    parent: parent__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListLogMetricsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListLogMetricsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metrics.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListLogMetricsResponse", len)?;
        if !self.metrics.is_empty() {
            struct_ser.serialize_field("metrics", &self.metrics)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListLogMetricsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metrics",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metrics,
            NextPageToken,
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
                            "metrics" => Ok(GeneratedField::Metrics),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListLogMetricsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListLogMetricsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListLogMetricsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metrics__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListLogMetricsResponse {
                    metrics: metrics__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListLogMetricsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListLogsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.resource_names.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListLogsRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.resource_names.is_empty() {
            struct_ser.serialize_field("resourceNames", &self.resource_names)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListLogsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "resource_names",
            "resourceNames",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            ResourceNames,
            PageSize,
            PageToken,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "resourceNames" | "resource_names" => Ok(GeneratedField::ResourceNames),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListLogsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListLogsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListLogsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut resource_names__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceNames => {
                            if resource_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNames"));
                            }
                            resource_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListLogsRequest {
                    parent: parent__.unwrap_or_default(),
                    resource_names: resource_names__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListLogsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListLogsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_names.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListLogsResponse", len)?;
        if !self.log_names.is_empty() {
            struct_ser.serialize_field("logNames", &self.log_names)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListLogsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_names",
            "logNames",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogNames,
            NextPageToken,
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
                            "logNames" | "log_names" => Ok(GeneratedField::LogNames),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListLogsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListLogsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListLogsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_names__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogNames => {
                            if log_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logNames"));
                            }
                            log_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListLogsResponse {
                    log_names: log_names__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListLogsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMonitoredResourceDescriptorsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListMonitoredResourceDescriptorsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMonitoredResourceDescriptorsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
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
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListMonitoredResourceDescriptorsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListMonitoredResourceDescriptorsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMonitoredResourceDescriptorsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListMonitoredResourceDescriptorsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListMonitoredResourceDescriptorsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMonitoredResourceDescriptorsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_descriptors.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListMonitoredResourceDescriptorsResponse", len)?;
        if !self.resource_descriptors.is_empty() {
            struct_ser.serialize_field("resourceDescriptors", &self.resource_descriptors)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMonitoredResourceDescriptorsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_descriptors",
            "resourceDescriptors",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceDescriptors,
            NextPageToken,
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
                            "resourceDescriptors" | "resource_descriptors" => Ok(GeneratedField::ResourceDescriptors),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListMonitoredResourceDescriptorsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListMonitoredResourceDescriptorsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMonitoredResourceDescriptorsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_descriptors__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceDescriptors => {
                            if resource_descriptors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceDescriptors"));
                            }
                            resource_descriptors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListMonitoredResourceDescriptorsResponse {
                    resource_descriptors: resource_descriptors__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListMonitoredResourceDescriptorsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSinksRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListSinksRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSinksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_token",
            "pageToken",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageToken,
            PageSize,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListSinksRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListSinksRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSinksRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_token__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListSinksRequest {
                    parent: parent__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListSinksRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSinksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sinks.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListSinksResponse", len)?;
        if !self.sinks.is_empty() {
            struct_ser.serialize_field("sinks", &self.sinks)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSinksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sinks",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sinks,
            NextPageToken,
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
                            "sinks" => Ok(GeneratedField::Sinks),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListSinksResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListSinksResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSinksResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sinks__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sinks => {
                            if sinks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sinks"));
                            }
                            sinks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListSinksResponse {
                    sinks: sinks__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListSinksResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListViewsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListViewsRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListViewsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_token",
            "pageToken",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageToken,
            PageSize,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListViewsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListViewsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListViewsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_token__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListViewsRequest {
                    parent: parent__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListViewsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListViewsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.views.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.ListViewsResponse", len)?;
        if !self.views.is_empty() {
            struct_ser.serialize_field("views", &self.views)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListViewsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "views",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Views,
            NextPageToken,
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
                            "views" => Ok(GeneratedField::Views),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListViewsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.ListViewsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListViewsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut views__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Views => {
                            if views__.is_some() {
                                return Err(serde::de::Error::duplicate_field("views"));
                            }
                            views__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListViewsResponse {
                    views: views__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.ListViewsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocationMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.log_analytics_enabled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LocationMetadata", len)?;
        if self.log_analytics_enabled {
            struct_ser.serialize_field("logAnalyticsEnabled", &self.log_analytics_enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocationMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_analytics_enabled",
            "logAnalyticsEnabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogAnalyticsEnabled,
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
                            "logAnalyticsEnabled" | "log_analytics_enabled" => Ok(GeneratedField::LogAnalyticsEnabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocationMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LocationMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocationMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_analytics_enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogAnalyticsEnabled => {
                            if log_analytics_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logAnalyticsEnabled"));
                            }
                            log_analytics_enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LocationMetadata {
                    log_analytics_enabled: log_analytics_enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LocationMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogBucket {
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
        if self.create_time.is_some() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        if self.retention_days != 0 {
            len += 1;
        }
        if self.locked {
            len += 1;
        }
        if self.lifecycle_state != 0 {
            len += 1;
        }
        if self.analytics_enabled {
            len += 1;
        }
        if !self.restricted_fields.is_empty() {
            len += 1;
        }
        if !self.index_configs.is_empty() {
            len += 1;
        }
        if self.cmek_settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LogBucket", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        if self.retention_days != 0 {
            struct_ser.serialize_field("retentionDays", &self.retention_days)?;
        }
        if self.locked {
            struct_ser.serialize_field("locked", &self.locked)?;
        }
        if self.lifecycle_state != 0 {
            let v = LifecycleState::try_from(self.lifecycle_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.lifecycle_state)))?;
            struct_ser.serialize_field("lifecycleState", &v)?;
        }
        if self.analytics_enabled {
            struct_ser.serialize_field("analyticsEnabled", &self.analytics_enabled)?;
        }
        if !self.restricted_fields.is_empty() {
            struct_ser.serialize_field("restrictedFields", &self.restricted_fields)?;
        }
        if !self.index_configs.is_empty() {
            struct_ser.serialize_field("indexConfigs", &self.index_configs)?;
        }
        if let Some(v) = self.cmek_settings.as_ref() {
            struct_ser.serialize_field("cmekSettings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogBucket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "create_time",
            "createTime",
            "update_time",
            "updateTime",
            "retention_days",
            "retentionDays",
            "locked",
            "lifecycle_state",
            "lifecycleState",
            "analytics_enabled",
            "analyticsEnabled",
            "restricted_fields",
            "restrictedFields",
            "index_configs",
            "indexConfigs",
            "cmek_settings",
            "cmekSettings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            CreateTime,
            UpdateTime,
            RetentionDays,
            Locked,
            LifecycleState,
            AnalyticsEnabled,
            RestrictedFields,
            IndexConfigs,
            CmekSettings,
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
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
                            "retentionDays" | "retention_days" => Ok(GeneratedField::RetentionDays),
                            "locked" => Ok(GeneratedField::Locked),
                            "lifecycleState" | "lifecycle_state" => Ok(GeneratedField::LifecycleState),
                            "analyticsEnabled" | "analytics_enabled" => Ok(GeneratedField::AnalyticsEnabled),
                            "restrictedFields" | "restricted_fields" => Ok(GeneratedField::RestrictedFields),
                            "indexConfigs" | "index_configs" => Ok(GeneratedField::IndexConfigs),
                            "cmekSettings" | "cmek_settings" => Ok(GeneratedField::CmekSettings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogBucket;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LogBucket")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogBucket, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut create_time__ = None;
                let mut update_time__ = None;
                let mut retention_days__ = None;
                let mut locked__ = None;
                let mut lifecycle_state__ = None;
                let mut analytics_enabled__ = None;
                let mut restricted_fields__ = None;
                let mut index_configs__ = None;
                let mut cmek_settings__ = None;
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
                        GeneratedField::RetentionDays => {
                            if retention_days__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retentionDays"));
                            }
                            retention_days__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Locked => {
                            if locked__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locked"));
                            }
                            locked__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LifecycleState => {
                            if lifecycle_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycleState"));
                            }
                            lifecycle_state__ = Some(map_.next_value::<LifecycleState>()? as i32);
                        }
                        GeneratedField::AnalyticsEnabled => {
                            if analytics_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyticsEnabled"));
                            }
                            analytics_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RestrictedFields => {
                            if restricted_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restrictedFields"));
                            }
                            restricted_fields__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IndexConfigs => {
                            if index_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexConfigs"));
                            }
                            index_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CmekSettings => {
                            if cmek_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cmekSettings"));
                            }
                            cmek_settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LogBucket {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    create_time: create_time__,
                    update_time: update_time__,
                    retention_days: retention_days__.unwrap_or_default(),
                    locked: locked__.unwrap_or_default(),
                    lifecycle_state: lifecycle_state__.unwrap_or_default(),
                    analytics_enabled: analytics_enabled__.unwrap_or_default(),
                    restricted_fields: restricted_fields__.unwrap_or_default(),
                    index_configs: index_configs__.unwrap_or_default(),
                    cmek_settings: cmek_settings__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LogBucket", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_name.is_empty() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.receive_timestamp.is_some() {
            len += 1;
        }
        if self.severity != 0 {
            len += 1;
        }
        if !self.insert_id.is_empty() {
            len += 1;
        }
        if self.http_request.is_some() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if self.operation.is_some() {
            len += 1;
        }
        if !self.trace.is_empty() {
            len += 1;
        }
        if !self.span_id.is_empty() {
            len += 1;
        }
        if self.trace_sampled {
            len += 1;
        }
        if self.source_location.is_some() {
            len += 1;
        }
        if self.split.is_some() {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LogEntry", len)?;
        if !self.log_name.is_empty() {
            struct_ser.serialize_field("logName", &self.log_name)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.receive_timestamp.as_ref() {
            struct_ser.serialize_field("receiveTimestamp", v)?;
        }
        if self.severity != 0 {
            let v = super::r#type::LogSeverity::try_from(self.severity)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.severity)))?;
            struct_ser.serialize_field("severity", &v)?;
        }
        if !self.insert_id.is_empty() {
            struct_ser.serialize_field("insertId", &self.insert_id)?;
        }
        if let Some(v) = self.http_request.as_ref() {
            struct_ser.serialize_field("httpRequest", v)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if let Some(v) = self.operation.as_ref() {
            struct_ser.serialize_field("operation", v)?;
        }
        if !self.trace.is_empty() {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        if !self.span_id.is_empty() {
            struct_ser.serialize_field("spanId", &self.span_id)?;
        }
        if self.trace_sampled {
            struct_ser.serialize_field("traceSampled", &self.trace_sampled)?;
        }
        if let Some(v) = self.source_location.as_ref() {
            struct_ser.serialize_field("sourceLocation", v)?;
        }
        if let Some(v) = self.split.as_ref() {
            struct_ser.serialize_field("split", v)?;
        }
        if let Some(v) = self.payload.as_ref() {
            match v {
                log_entry::Payload::ProtoPayload(v) => {
                    struct_ser.serialize_field("protoPayload", v)?;
                }
                log_entry::Payload::TextPayload(v) => {
                    struct_ser.serialize_field("textPayload", v)?;
                }
                log_entry::Payload::JsonPayload(v) => {
                    struct_ser.serialize_field("jsonPayload", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_name",
            "logName",
            "resource",
            "timestamp",
            "receive_timestamp",
            "receiveTimestamp",
            "severity",
            "insert_id",
            "insertId",
            "http_request",
            "httpRequest",
            "labels",
            "operation",
            "trace",
            "span_id",
            "spanId",
            "trace_sampled",
            "traceSampled",
            "source_location",
            "sourceLocation",
            "split",
            "proto_payload",
            "protoPayload",
            "text_payload",
            "textPayload",
            "json_payload",
            "jsonPayload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogName,
            Resource,
            Timestamp,
            ReceiveTimestamp,
            Severity,
            InsertId,
            HttpRequest,
            Labels,
            Operation,
            Trace,
            SpanId,
            TraceSampled,
            SourceLocation,
            Split,
            ProtoPayload,
            TextPayload,
            JsonPayload,
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
                            "logName" | "log_name" => Ok(GeneratedField::LogName),
                            "resource" => Ok(GeneratedField::Resource),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "receiveTimestamp" | "receive_timestamp" => Ok(GeneratedField::ReceiveTimestamp),
                            "severity" => Ok(GeneratedField::Severity),
                            "insertId" | "insert_id" => Ok(GeneratedField::InsertId),
                            "httpRequest" | "http_request" => Ok(GeneratedField::HttpRequest),
                            "labels" => Ok(GeneratedField::Labels),
                            "operation" => Ok(GeneratedField::Operation),
                            "trace" => Ok(GeneratedField::Trace),
                            "spanId" | "span_id" => Ok(GeneratedField::SpanId),
                            "traceSampled" | "trace_sampled" => Ok(GeneratedField::TraceSampled),
                            "sourceLocation" | "source_location" => Ok(GeneratedField::SourceLocation),
                            "split" => Ok(GeneratedField::Split),
                            "protoPayload" | "proto_payload" => Ok(GeneratedField::ProtoPayload),
                            "textPayload" | "text_payload" => Ok(GeneratedField::TextPayload),
                            "jsonPayload" | "json_payload" => Ok(GeneratedField::JsonPayload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LogEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_name__ = None;
                let mut resource__ = None;
                let mut timestamp__ = None;
                let mut receive_timestamp__ = None;
                let mut severity__ = None;
                let mut insert_id__ = None;
                let mut http_request__ = None;
                let mut labels__ = None;
                let mut operation__ = None;
                let mut trace__ = None;
                let mut span_id__ = None;
                let mut trace_sampled__ = None;
                let mut source_location__ = None;
                let mut split__ = None;
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogName => {
                            if log_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logName"));
                            }
                            log_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::ReceiveTimestamp => {
                            if receive_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiveTimestamp"));
                            }
                            receive_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::Severity => {
                            if severity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("severity"));
                            }
                            severity__ = Some(map_.next_value::<super::r#type::LogSeverity>()? as i32);
                        }
                        GeneratedField::InsertId => {
                            if insert_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("insertId"));
                            }
                            insert_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HttpRequest => {
                            if http_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpRequest"));
                            }
                            http_request__ = map_.next_value()?;
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = map_.next_value()?;
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SpanId => {
                            if span_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spanId"));
                            }
                            span_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TraceSampled => {
                            if trace_sampled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceSampled"));
                            }
                            trace_sampled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceLocation => {
                            if source_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceLocation"));
                            }
                            source_location__ = map_.next_value()?;
                        }
                        GeneratedField::Split => {
                            if split__.is_some() {
                                return Err(serde::de::Error::duplicate_field("split"));
                            }
                            split__ = map_.next_value()?;
                        }
                        GeneratedField::ProtoPayload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoPayload"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(log_entry::Payload::ProtoPayload)
;
                        }
                        GeneratedField::TextPayload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("textPayload"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(log_entry::Payload::TextPayload);
                        }
                        GeneratedField::JsonPayload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonPayload"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(log_entry::Payload::JsonPayload)
;
                        }
                    }
                }
                Ok(LogEntry {
                    log_name: log_name__.unwrap_or_default(),
                    resource: resource__,
                    timestamp: timestamp__,
                    receive_timestamp: receive_timestamp__,
                    severity: severity__.unwrap_or_default(),
                    insert_id: insert_id__.unwrap_or_default(),
                    http_request: http_request__,
                    labels: labels__.unwrap_or_default(),
                    operation: operation__,
                    trace: trace__.unwrap_or_default(),
                    span_id: span_id__.unwrap_or_default(),
                    trace_sampled: trace_sampled__.unwrap_or_default(),
                    source_location: source_location__,
                    split: split__,
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LogEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogEntryOperation {
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
        if !self.producer.is_empty() {
            len += 1;
        }
        if self.first {
            len += 1;
        }
        if self.last {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LogEntryOperation", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.producer.is_empty() {
            struct_ser.serialize_field("producer", &self.producer)?;
        }
        if self.first {
            struct_ser.serialize_field("first", &self.first)?;
        }
        if self.last {
            struct_ser.serialize_field("last", &self.last)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogEntryOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "producer",
            "first",
            "last",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Producer,
            First,
            Last,
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
                            "producer" => Ok(GeneratedField::Producer),
                            "first" => Ok(GeneratedField::First),
                            "last" => Ok(GeneratedField::Last),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogEntryOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LogEntryOperation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogEntryOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut producer__ = None;
                let mut first__ = None;
                let mut last__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Producer => {
                            if producer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("producer"));
                            }
                            producer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::First => {
                            if first__.is_some() {
                                return Err(serde::de::Error::duplicate_field("first"));
                            }
                            first__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Last => {
                            if last__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last"));
                            }
                            last__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LogEntryOperation {
                    id: id__.unwrap_or_default(),
                    producer: producer__.unwrap_or_default(),
                    first: first__.unwrap_or_default(),
                    last: last__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LogEntryOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogEntrySourceLocation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file.is_empty() {
            len += 1;
        }
        if self.line != 0 {
            len += 1;
        }
        if !self.function.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LogEntrySourceLocation", len)?;
        if !self.file.is_empty() {
            struct_ser.serialize_field("file", &self.file)?;
        }
        if self.line != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("line", ToString::to_string(&self.line).as_str())?;
        }
        if !self.function.is_empty() {
            struct_ser.serialize_field("function", &self.function)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogEntrySourceLocation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file",
            "line",
            "function",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            File,
            Line,
            Function,
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
                            "file" => Ok(GeneratedField::File),
                            "line" => Ok(GeneratedField::Line),
                            "function" => Ok(GeneratedField::Function),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogEntrySourceLocation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LogEntrySourceLocation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogEntrySourceLocation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file__ = None;
                let mut line__ = None;
                let mut function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::File => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Line => {
                            if line__.is_some() {
                                return Err(serde::de::Error::duplicate_field("line"));
                            }
                            line__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Function => {
                            if function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            function__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LogEntrySourceLocation {
                    file: file__.unwrap_or_default(),
                    line: line__.unwrap_or_default(),
                    function: function__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LogEntrySourceLocation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogExclusion {
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
        if !self.filter.is_empty() {
            len += 1;
        }
        if self.disabled {
            len += 1;
        }
        if self.create_time.is_some() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LogExclusion", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogExclusion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "filter",
            "disabled",
            "create_time",
            "createTime",
            "update_time",
            "updateTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Filter,
            Disabled,
            CreateTime,
            UpdateTime,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "disabled" => Ok(GeneratedField::Disabled),
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogExclusion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LogExclusion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogExclusion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut filter__ = None;
                let mut disabled__ = None;
                let mut create_time__ = None;
                let mut update_time__ = None;
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
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(LogExclusion {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    disabled: disabled__.unwrap_or_default(),
                    create_time: create_time__,
                    update_time: update_time__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LogExclusion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogMetric {
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
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.bucket_name.is_empty() {
            len += 1;
        }
        if self.disabled {
            len += 1;
        }
        if self.metric_descriptor.is_some() {
            len += 1;
        }
        if !self.value_extractor.is_empty() {
            len += 1;
        }
        if !self.label_extractors.is_empty() {
            len += 1;
        }
        if self.bucket_options.is_some() {
            len += 1;
        }
        if self.create_time.is_some() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LogMetric", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.bucket_name.is_empty() {
            struct_ser.serialize_field("bucketName", &self.bucket_name)?;
        }
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        if let Some(v) = self.metric_descriptor.as_ref() {
            struct_ser.serialize_field("metricDescriptor", v)?;
        }
        if !self.value_extractor.is_empty() {
            struct_ser.serialize_field("valueExtractor", &self.value_extractor)?;
        }
        if !self.label_extractors.is_empty() {
            struct_ser.serialize_field("labelExtractors", &self.label_extractors)?;
        }
        if let Some(v) = self.bucket_options.as_ref() {
            struct_ser.serialize_field("bucketOptions", v)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        if self.version != 0 {
            let v = log_metric::ApiVersion::try_from(self.version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.version)))?;
            struct_ser.serialize_field("version", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogMetric {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "filter",
            "bucket_name",
            "bucketName",
            "disabled",
            "metric_descriptor",
            "metricDescriptor",
            "value_extractor",
            "valueExtractor",
            "label_extractors",
            "labelExtractors",
            "bucket_options",
            "bucketOptions",
            "create_time",
            "createTime",
            "update_time",
            "updateTime",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Filter,
            BucketName,
            Disabled,
            MetricDescriptor,
            ValueExtractor,
            LabelExtractors,
            BucketOptions,
            CreateTime,
            UpdateTime,
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
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "filter" => Ok(GeneratedField::Filter),
                            "bucketName" | "bucket_name" => Ok(GeneratedField::BucketName),
                            "disabled" => Ok(GeneratedField::Disabled),
                            "metricDescriptor" | "metric_descriptor" => Ok(GeneratedField::MetricDescriptor),
                            "valueExtractor" | "value_extractor" => Ok(GeneratedField::ValueExtractor),
                            "labelExtractors" | "label_extractors" => Ok(GeneratedField::LabelExtractors),
                            "bucketOptions" | "bucket_options" => Ok(GeneratedField::BucketOptions),
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
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
            type Value = LogMetric;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LogMetric")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogMetric, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut filter__ = None;
                let mut bucket_name__ = None;
                let mut disabled__ = None;
                let mut metric_descriptor__ = None;
                let mut value_extractor__ = None;
                let mut label_extractors__ = None;
                let mut bucket_options__ = None;
                let mut create_time__ = None;
                let mut update_time__ = None;
                let mut version__ = None;
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
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BucketName => {
                            if bucket_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketName"));
                            }
                            bucket_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetricDescriptor => {
                            if metric_descriptor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricDescriptor"));
                            }
                            metric_descriptor__ = map_.next_value()?;
                        }
                        GeneratedField::ValueExtractor => {
                            if value_extractor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueExtractor"));
                            }
                            value_extractor__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LabelExtractors => {
                            if label_extractors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labelExtractors"));
                            }
                            label_extractors__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::BucketOptions => {
                            if bucket_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketOptions"));
                            }
                            bucket_options__ = map_.next_value()?;
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
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value::<log_metric::ApiVersion>()? as i32);
                        }
                    }
                }
                Ok(LogMetric {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    bucket_name: bucket_name__.unwrap_or_default(),
                    disabled: disabled__.unwrap_or_default(),
                    metric_descriptor: metric_descriptor__,
                    value_extractor: value_extractor__.unwrap_or_default(),
                    label_extractors: label_extractors__.unwrap_or_default(),
                    bucket_options: bucket_options__,
                    create_time: create_time__,
                    update_time: update_time__,
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LogMetric", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for log_metric::ApiVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::V2 => "V2",
            Self::V1 => "V1",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for log_metric::ApiVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "V2",
            "V1",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = log_metric::ApiVersion;

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
                    "V2" => Ok(log_metric::ApiVersion::V2),
                    "V1" => Ok(log_metric::ApiVersion::V1),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LogSink {
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
        if !self.destination.is_empty() {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.disabled {
            len += 1;
        }
        if !self.exclusions.is_empty() {
            len += 1;
        }
        if self.output_version_format != 0 {
            len += 1;
        }
        if !self.writer_identity.is_empty() {
            len += 1;
        }
        if self.include_children {
            len += 1;
        }
        if self.create_time.is_some() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LogSink", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.destination.is_empty() {
            struct_ser.serialize_field("destination", &self.destination)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        if !self.exclusions.is_empty() {
            struct_ser.serialize_field("exclusions", &self.exclusions)?;
        }
        if self.output_version_format != 0 {
            let v = log_sink::VersionFormat::try_from(self.output_version_format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.output_version_format)))?;
            struct_ser.serialize_field("outputVersionFormat", &v)?;
        }
        if !self.writer_identity.is_empty() {
            struct_ser.serialize_field("writerIdentity", &self.writer_identity)?;
        }
        if self.include_children {
            struct_ser.serialize_field("includeChildren", &self.include_children)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        if let Some(v) = self.options.as_ref() {
            match v {
                log_sink::Options::BigqueryOptions(v) => {
                    struct_ser.serialize_field("bigqueryOptions", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "destination",
            "filter",
            "description",
            "disabled",
            "exclusions",
            "output_version_format",
            "outputVersionFormat",
            "writer_identity",
            "writerIdentity",
            "include_children",
            "includeChildren",
            "create_time",
            "createTime",
            "update_time",
            "updateTime",
            "bigquery_options",
            "bigqueryOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Destination,
            Filter,
            Description,
            Disabled,
            Exclusions,
            OutputVersionFormat,
            WriterIdentity,
            IncludeChildren,
            CreateTime,
            UpdateTime,
            BigqueryOptions,
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
                            "destination" => Ok(GeneratedField::Destination),
                            "filter" => Ok(GeneratedField::Filter),
                            "description" => Ok(GeneratedField::Description),
                            "disabled" => Ok(GeneratedField::Disabled),
                            "exclusions" => Ok(GeneratedField::Exclusions),
                            "outputVersionFormat" | "output_version_format" => Ok(GeneratedField::OutputVersionFormat),
                            "writerIdentity" | "writer_identity" => Ok(GeneratedField::WriterIdentity),
                            "includeChildren" | "include_children" => Ok(GeneratedField::IncludeChildren),
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
                            "bigqueryOptions" | "bigquery_options" => Ok(GeneratedField::BigqueryOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LogSink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut destination__ = None;
                let mut filter__ = None;
                let mut description__ = None;
                let mut disabled__ = None;
                let mut exclusions__ = None;
                let mut output_version_format__ = None;
                let mut writer_identity__ = None;
                let mut include_children__ = None;
                let mut create_time__ = None;
                let mut update_time__ = None;
                let mut options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Destination => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            destination__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exclusions => {
                            if exclusions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exclusions"));
                            }
                            exclusions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputVersionFormat => {
                            if output_version_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputVersionFormat"));
                            }
                            output_version_format__ = Some(map_.next_value::<log_sink::VersionFormat>()? as i32);
                        }
                        GeneratedField::WriterIdentity => {
                            if writer_identity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("writerIdentity"));
                            }
                            writer_identity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeChildren => {
                            if include_children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeChildren"));
                            }
                            include_children__ = Some(map_.next_value()?);
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
                        GeneratedField::BigqueryOptions => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bigqueryOptions"));
                            }
                            options__ = map_.next_value::<::std::option::Option<_>>()?.map(log_sink::Options::BigqueryOptions)
;
                        }
                    }
                }
                Ok(LogSink {
                    name: name__.unwrap_or_default(),
                    destination: destination__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    disabled: disabled__.unwrap_or_default(),
                    exclusions: exclusions__.unwrap_or_default(),
                    output_version_format: output_version_format__.unwrap_or_default(),
                    writer_identity: writer_identity__.unwrap_or_default(),
                    include_children: include_children__.unwrap_or_default(),
                    create_time: create_time__,
                    update_time: update_time__,
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LogSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for log_sink::VersionFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VERSION_FORMAT_UNSPECIFIED",
            Self::V2 => "V2",
            Self::V1 => "V1",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for log_sink::VersionFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VERSION_FORMAT_UNSPECIFIED",
            "V2",
            "V1",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = log_sink::VersionFormat;

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
                    "VERSION_FORMAT_UNSPECIFIED" => Ok(log_sink::VersionFormat::Unspecified),
                    "V2" => Ok(log_sink::VersionFormat::V2),
                    "V1" => Ok(log_sink::VersionFormat::V1),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LogSplit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uid.is_empty() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if self.total_splits != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LogSplit", len)?;
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        if self.total_splits != 0 {
            struct_ser.serialize_field("totalSplits", &self.total_splits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogSplit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uid",
            "index",
            "total_splits",
            "totalSplits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uid,
            Index,
            TotalSplits,
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
                            "uid" => Ok(GeneratedField::Uid),
                            "index" => Ok(GeneratedField::Index),
                            "totalSplits" | "total_splits" => Ok(GeneratedField::TotalSplits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogSplit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LogSplit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogSplit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uid__ = None;
                let mut index__ = None;
                let mut total_splits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalSplits => {
                            if total_splits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSplits"));
                            }
                            total_splits__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LogSplit {
                    uid: uid__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    total_splits: total_splits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LogSplit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogView {
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
        if self.create_time.is_some() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.LogView", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "create_time",
            "createTime",
            "update_time",
            "updateTime",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            CreateTime,
            UpdateTime,
            Filter,
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
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
                            "filter" => Ok(GeneratedField::Filter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.LogView")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogView, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut create_time__ = None;
                let mut update_time__ = None;
                let mut filter__ = None;
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
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LogView {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    create_time: create_time__,
                    update_time: update_time__,
                    filter: filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.LogView", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OperationState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OPERATION_STATE_UNSPECIFIED",
            Self::Scheduled => "OPERATION_STATE_SCHEDULED",
            Self::WaitingForPermissions => "OPERATION_STATE_WAITING_FOR_PERMISSIONS",
            Self::Running => "OPERATION_STATE_RUNNING",
            Self::Succeeded => "OPERATION_STATE_SUCCEEDED",
            Self::Failed => "OPERATION_STATE_FAILED",
            Self::Cancelled => "OPERATION_STATE_CANCELLED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OperationState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OPERATION_STATE_UNSPECIFIED",
            "OPERATION_STATE_SCHEDULED",
            "OPERATION_STATE_WAITING_FOR_PERMISSIONS",
            "OPERATION_STATE_RUNNING",
            "OPERATION_STATE_SUCCEEDED",
            "OPERATION_STATE_FAILED",
            "OPERATION_STATE_CANCELLED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OperationState;

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
                    "OPERATION_STATE_UNSPECIFIED" => Ok(OperationState::Unspecified),
                    "OPERATION_STATE_SCHEDULED" => Ok(OperationState::Scheduled),
                    "OPERATION_STATE_WAITING_FOR_PERMISSIONS" => Ok(OperationState::WaitingForPermissions),
                    "OPERATION_STATE_RUNNING" => Ok(OperationState::Running),
                    "OPERATION_STATE_SUCCEEDED" => Ok(OperationState::Succeeded),
                    "OPERATION_STATE_FAILED" => Ok(OperationState::Failed),
                    "OPERATION_STATE_CANCELLED" => Ok(OperationState::Cancelled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Settings {
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
        if !self.kms_key_name.is_empty() {
            len += 1;
        }
        if !self.kms_service_account_id.is_empty() {
            len += 1;
        }
        if !self.storage_location.is_empty() {
            len += 1;
        }
        if self.disable_default_sink {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.Settings", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.kms_key_name.is_empty() {
            struct_ser.serialize_field("kmsKeyName", &self.kms_key_name)?;
        }
        if !self.kms_service_account_id.is_empty() {
            struct_ser.serialize_field("kmsServiceAccountId", &self.kms_service_account_id)?;
        }
        if !self.storage_location.is_empty() {
            struct_ser.serialize_field("storageLocation", &self.storage_location)?;
        }
        if self.disable_default_sink {
            struct_ser.serialize_field("disableDefaultSink", &self.disable_default_sink)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Settings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "kms_key_name",
            "kmsKeyName",
            "kms_service_account_id",
            "kmsServiceAccountId",
            "storage_location",
            "storageLocation",
            "disable_default_sink",
            "disableDefaultSink",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            KmsKeyName,
            KmsServiceAccountId,
            StorageLocation,
            DisableDefaultSink,
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
                            "kmsKeyName" | "kms_key_name" => Ok(GeneratedField::KmsKeyName),
                            "kmsServiceAccountId" | "kms_service_account_id" => Ok(GeneratedField::KmsServiceAccountId),
                            "storageLocation" | "storage_location" => Ok(GeneratedField::StorageLocation),
                            "disableDefaultSink" | "disable_default_sink" => Ok(GeneratedField::DisableDefaultSink),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Settings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.Settings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Settings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut kms_key_name__ = None;
                let mut kms_service_account_id__ = None;
                let mut storage_location__ = None;
                let mut disable_default_sink__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KmsKeyName => {
                            if kms_key_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kmsKeyName"));
                            }
                            kms_key_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KmsServiceAccountId => {
                            if kms_service_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kmsServiceAccountId"));
                            }
                            kms_service_account_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorageLocation => {
                            if storage_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageLocation"));
                            }
                            storage_location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableDefaultSink => {
                            if disable_default_sink__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableDefaultSink"));
                            }
                            disable_default_sink__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Settings {
                    name: name__.unwrap_or_default(),
                    kms_key_name: kms_key_name__.unwrap_or_default(),
                    kms_service_account_id: kms_service_account_id__.unwrap_or_default(),
                    storage_location: storage_location__.unwrap_or_default(),
                    disable_default_sink: disable_default_sink__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.Settings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TailLogEntriesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_names.is_empty() {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        if self.buffer_window.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.TailLogEntriesRequest", len)?;
        if !self.resource_names.is_empty() {
            struct_ser.serialize_field("resourceNames", &self.resource_names)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if let Some(v) = self.buffer_window.as_ref() {
            struct_ser.serialize_field("bufferWindow", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TailLogEntriesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_names",
            "resourceNames",
            "filter",
            "buffer_window",
            "bufferWindow",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceNames,
            Filter,
            BufferWindow,
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
                            "resourceNames" | "resource_names" => Ok(GeneratedField::ResourceNames),
                            "filter" => Ok(GeneratedField::Filter),
                            "bufferWindow" | "buffer_window" => Ok(GeneratedField::BufferWindow),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TailLogEntriesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.TailLogEntriesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TailLogEntriesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_names__ = None;
                let mut filter__ = None;
                let mut buffer_window__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceNames => {
                            if resource_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNames"));
                            }
                            resource_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BufferWindow => {
                            if buffer_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufferWindow"));
                            }
                            buffer_window__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TailLogEntriesRequest {
                    resource_names: resource_names__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    buffer_window: buffer_window__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.TailLogEntriesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TailLogEntriesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        if !self.suppression_info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.TailLogEntriesResponse", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if !self.suppression_info.is_empty() {
            struct_ser.serialize_field("suppressionInfo", &self.suppression_info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TailLogEntriesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entries",
            "suppression_info",
            "suppressionInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
            SuppressionInfo,
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
                            "entries" => Ok(GeneratedField::Entries),
                            "suppressionInfo" | "suppression_info" => Ok(GeneratedField::SuppressionInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TailLogEntriesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.TailLogEntriesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TailLogEntriesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                let mut suppression_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SuppressionInfo => {
                            if suppression_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suppressionInfo"));
                            }
                            suppression_info__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TailLogEntriesResponse {
                    entries: entries__.unwrap_or_default(),
                    suppression_info: suppression_info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.TailLogEntriesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tail_log_entries_response::SuppressionInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason != 0 {
            len += 1;
        }
        if self.suppressed_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.TailLogEntriesResponse.SuppressionInfo", len)?;
        if self.reason != 0 {
            let v = tail_log_entries_response::suppression_info::Reason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        if self.suppressed_count != 0 {
            struct_ser.serialize_field("suppressedCount", &self.suppressed_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tail_log_entries_response::SuppressionInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
            "suppressed_count",
            "suppressedCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
            SuppressedCount,
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
                            "reason" => Ok(GeneratedField::Reason),
                            "suppressedCount" | "suppressed_count" => Ok(GeneratedField::SuppressedCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tail_log_entries_response::SuppressionInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.TailLogEntriesResponse.SuppressionInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<tail_log_entries_response::SuppressionInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                let mut suppressed_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<tail_log_entries_response::suppression_info::Reason>()? as i32);
                        }
                        GeneratedField::SuppressedCount => {
                            if suppressed_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suppressedCount"));
                            }
                            suppressed_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(tail_log_entries_response::SuppressionInfo {
                    reason: reason__.unwrap_or_default(),
                    suppressed_count: suppressed_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.TailLogEntriesResponse.SuppressionInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tail_log_entries_response::suppression_info::Reason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REASON_UNSPECIFIED",
            Self::RateLimit => "RATE_LIMIT",
            Self::NotConsumed => "NOT_CONSUMED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for tail_log_entries_response::suppression_info::Reason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REASON_UNSPECIFIED",
            "RATE_LIMIT",
            "NOT_CONSUMED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tail_log_entries_response::suppression_info::Reason;

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
                    "REASON_UNSPECIFIED" => Ok(tail_log_entries_response::suppression_info::Reason::Unspecified),
                    "RATE_LIMIT" => Ok(tail_log_entries_response::suppression_info::Reason::RateLimit),
                    "NOT_CONSUMED" => Ok(tail_log_entries_response::suppression_info::Reason::NotConsumed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UndeleteBucketRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.UndeleteBucketRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UndeleteBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UndeleteBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.UndeleteBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UndeleteBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UndeleteBucketRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.UndeleteBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateBucketRequest {
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
        if self.bucket.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.UpdateBucketRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.bucket.as_ref() {
            struct_ser.serialize_field("bucket", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "bucket",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Bucket,
            UpdateMask,
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
                            "bucket" => Ok(GeneratedField::Bucket),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.UpdateBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut bucket__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bucket => {
                            if bucket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucket"));
                            }
                            bucket__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateBucketRequest {
                    name: name__.unwrap_or_default(),
                    bucket: bucket__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.UpdateBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCmekSettingsRequest {
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
        if self.cmek_settings.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.UpdateCmekSettingsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.cmek_settings.as_ref() {
            struct_ser.serialize_field("cmekSettings", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCmekSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "cmek_settings",
            "cmekSettings",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CmekSettings,
            UpdateMask,
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
                            "cmekSettings" | "cmek_settings" => Ok(GeneratedField::CmekSettings),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateCmekSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.UpdateCmekSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCmekSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut cmek_settings__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CmekSettings => {
                            if cmek_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cmekSettings"));
                            }
                            cmek_settings__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateCmekSettingsRequest {
                    name: name__.unwrap_or_default(),
                    cmek_settings: cmek_settings__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.UpdateCmekSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateExclusionRequest {
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
        if self.exclusion.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.UpdateExclusionRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.exclusion.as_ref() {
            struct_ser.serialize_field("exclusion", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateExclusionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "exclusion",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Exclusion,
            UpdateMask,
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
                            "exclusion" => Ok(GeneratedField::Exclusion),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateExclusionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.UpdateExclusionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateExclusionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut exclusion__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exclusion => {
                            if exclusion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exclusion"));
                            }
                            exclusion__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateExclusionRequest {
                    name: name__.unwrap_or_default(),
                    exclusion: exclusion__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.UpdateExclusionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateLogMetricRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metric_name.is_empty() {
            len += 1;
        }
        if self.metric.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.UpdateLogMetricRequest", len)?;
        if !self.metric_name.is_empty() {
            struct_ser.serialize_field("metricName", &self.metric_name)?;
        }
        if let Some(v) = self.metric.as_ref() {
            struct_ser.serialize_field("metric", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateLogMetricRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metric_name",
            "metricName",
            "metric",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetricName,
            Metric,
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
                            "metricName" | "metric_name" => Ok(GeneratedField::MetricName),
                            "metric" => Ok(GeneratedField::Metric),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateLogMetricRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.UpdateLogMetricRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateLogMetricRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metric_name__ = None;
                let mut metric__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MetricName => {
                            if metric_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricName"));
                            }
                            metric_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metric => {
                            if metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }
                            metric__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateLogMetricRequest {
                    metric_name: metric_name__.unwrap_or_default(),
                    metric: metric__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.UpdateLogMetricRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSettingsRequest {
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
        if self.settings.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.UpdateSettingsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "settings",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Settings,
            UpdateMask,
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
                            "settings" => Ok(GeneratedField::Settings),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.UpdateSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut settings__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateSettingsRequest {
                    name: name__.unwrap_or_default(),
                    settings: settings__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.UpdateSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSinkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sink_name.is_empty() {
            len += 1;
        }
        if self.sink.is_some() {
            len += 1;
        }
        if self.unique_writer_identity {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.UpdateSinkRequest", len)?;
        if !self.sink_name.is_empty() {
            struct_ser.serialize_field("sinkName", &self.sink_name)?;
        }
        if let Some(v) = self.sink.as_ref() {
            struct_ser.serialize_field("sink", v)?;
        }
        if self.unique_writer_identity {
            struct_ser.serialize_field("uniqueWriterIdentity", &self.unique_writer_identity)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sink_name",
            "sinkName",
            "sink",
            "unique_writer_identity",
            "uniqueWriterIdentity",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SinkName,
            Sink,
            UniqueWriterIdentity,
            UpdateMask,
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
                            "sinkName" | "sink_name" => Ok(GeneratedField::SinkName),
                            "sink" => Ok(GeneratedField::Sink),
                            "uniqueWriterIdentity" | "unique_writer_identity" => Ok(GeneratedField::UniqueWriterIdentity),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateSinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.UpdateSinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateSinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sink_name__ = None;
                let mut sink__ = None;
                let mut unique_writer_identity__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SinkName => {
                            if sink_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sinkName"));
                            }
                            sink_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sink => {
                            if sink__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sink"));
                            }
                            sink__ = map_.next_value()?;
                        }
                        GeneratedField::UniqueWriterIdentity => {
                            if unique_writer_identity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uniqueWriterIdentity"));
                            }
                            unique_writer_identity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateSinkRequest {
                    sink_name: sink_name__.unwrap_or_default(),
                    sink: sink__,
                    unique_writer_identity: unique_writer_identity__.unwrap_or_default(),
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.UpdateSinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateViewRequest {
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
        if self.view.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.UpdateViewRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.view.as_ref() {
            struct_ser.serialize_field("view", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "view",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            View,
            UpdateMask,
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
                            "view" => Ok(GeneratedField::View),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.UpdateViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut view__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateViewRequest {
                    name: name__.unwrap_or_default(),
                    view: view__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.UpdateViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteLogEntriesPartialErrors {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_entry_errors.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.WriteLogEntriesPartialErrors", len)?;
        if !self.log_entry_errors.is_empty() {
            struct_ser.serialize_field("logEntryErrors", &self.log_entry_errors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteLogEntriesPartialErrors {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_entry_errors",
            "logEntryErrors",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogEntryErrors,
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
                            "logEntryErrors" | "log_entry_errors" => Ok(GeneratedField::LogEntryErrors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteLogEntriesPartialErrors;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.WriteLogEntriesPartialErrors")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteLogEntriesPartialErrors, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_entry_errors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogEntryErrors => {
                            if log_entry_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logEntryErrors"));
                            }
                            log_entry_errors__ = Some(
                                map_.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<i32>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                    }
                }
                Ok(WriteLogEntriesPartialErrors {
                    log_entry_errors: log_entry_errors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.WriteLogEntriesPartialErrors", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteLogEntriesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_name.is_empty() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if !self.entries.is_empty() {
            len += 1;
        }
        if self.partial_success {
            len += 1;
        }
        if self.dry_run {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.logging.v2.WriteLogEntriesRequest", len)?;
        if !self.log_name.is_empty() {
            struct_ser.serialize_field("logName", &self.log_name)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if self.partial_success {
            struct_ser.serialize_field("partialSuccess", &self.partial_success)?;
        }
        if self.dry_run {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteLogEntriesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_name",
            "logName",
            "resource",
            "labels",
            "entries",
            "partial_success",
            "partialSuccess",
            "dry_run",
            "dryRun",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogName,
            Resource,
            Labels,
            Entries,
            PartialSuccess,
            DryRun,
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
                            "logName" | "log_name" => Ok(GeneratedField::LogName),
                            "resource" => Ok(GeneratedField::Resource),
                            "labels" => Ok(GeneratedField::Labels),
                            "entries" => Ok(GeneratedField::Entries),
                            "partialSuccess" | "partial_success" => Ok(GeneratedField::PartialSuccess),
                            "dryRun" | "dry_run" => Ok(GeneratedField::DryRun),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteLogEntriesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.WriteLogEntriesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteLogEntriesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_name__ = None;
                let mut resource__ = None;
                let mut labels__ = None;
                let mut entries__ = None;
                let mut partial_success__ = None;
                let mut dry_run__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogName => {
                            if log_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logName"));
                            }
                            log_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PartialSuccess => {
                            if partial_success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partialSuccess"));
                            }
                            partial_success__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WriteLogEntriesRequest {
                    log_name: log_name__.unwrap_or_default(),
                    resource: resource__,
                    labels: labels__.unwrap_or_default(),
                    entries: entries__.unwrap_or_default(),
                    partial_success: partial_success__.unwrap_or_default(),
                    dry_run: dry_run__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.WriteLogEntriesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteLogEntriesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.logging.v2.WriteLogEntriesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteLogEntriesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteLogEntriesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.logging.v2.WriteLogEntriesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteLogEntriesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WriteLogEntriesResponse {
                })
            }
        }
        deserializer.deserialize_struct("google.logging.v2.WriteLogEntriesResponse", FIELDS, GeneratedVisitor)
    }
}
