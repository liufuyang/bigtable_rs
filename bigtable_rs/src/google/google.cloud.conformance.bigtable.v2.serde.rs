// @generated
impl serde::Serialize for ReadRowsTest {
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
        if !self.chunks.is_empty() {
            len += 1;
        }
        if !self.results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.cloud.conformance.bigtable.v2.ReadRowsTest", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadRowsTest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "chunks",
            "results",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Chunks,
            Results,
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
                            "chunks" => Ok(GeneratedField::Chunks),
                            "results" => Ok(GeneratedField::Results),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadRowsTest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.cloud.conformance.bigtable.v2.ReadRowsTest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadRowsTest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut chunks__ = None;
                let mut results__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Chunks => {
                            if chunks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadRowsTest {
                    description: description__.unwrap_or_default(),
                    chunks: chunks__.unwrap_or_default(),
                    results: results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.cloud.conformance.bigtable.v2.ReadRowsTest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_rows_test::Result {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.row_key.is_empty() {
            len += 1;
        }
        if !self.family_name.is_empty() {
            len += 1;
        }
        if !self.qualifier.is_empty() {
            len += 1;
        }
        if self.timestamp_micros != 0 {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if self.error {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.cloud.conformance.bigtable.v2.ReadRowsTest.Result", len)?;
        if !self.row_key.is_empty() {
            struct_ser.serialize_field("rowKey", &self.row_key)?;
        }
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        if !self.qualifier.is_empty() {
            struct_ser.serialize_field("qualifier", &self.qualifier)?;
        }
        if self.timestamp_micros != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("timestampMicros", ToString::to_string(&self.timestamp_micros).as_str())?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if self.error {
            struct_ser.serialize_field("error", &self.error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_rows_test::Result {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "row_key",
            "rowKey",
            "family_name",
            "familyName",
            "qualifier",
            "timestamp_micros",
            "timestampMicros",
            "value",
            "label",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RowKey,
            FamilyName,
            Qualifier,
            TimestampMicros,
            Value,
            Label,
            Error,
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
                            "rowKey" | "row_key" => Ok(GeneratedField::RowKey),
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
                            "qualifier" => Ok(GeneratedField::Qualifier),
                            "timestampMicros" | "timestamp_micros" => Ok(GeneratedField::TimestampMicros),
                            "value" => Ok(GeneratedField::Value),
                            "label" => Ok(GeneratedField::Label),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_rows_test::Result;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.cloud.conformance.bigtable.v2.ReadRowsTest.Result")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<read_rows_test::Result, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut row_key__ = None;
                let mut family_name__ = None;
                let mut qualifier__ = None;
                let mut timestamp_micros__ = None;
                let mut value__ = None;
                let mut label__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RowKey => {
                            if row_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowKey"));
                            }
                            row_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Qualifier => {
                            if qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("qualifier"));
                            }
                            qualifier__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TimestampMicros => {
                            if timestamp_micros__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampMicros"));
                            }
                            timestamp_micros__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(read_rows_test::Result {
                    row_key: row_key__.unwrap_or_default(),
                    family_name: family_name__.unwrap_or_default(),
                    qualifier: qualifier__.unwrap_or_default(),
                    timestamp_micros: timestamp_micros__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    error: error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.cloud.conformance.bigtable.v2.ReadRowsTest.Result", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestFile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.read_rows_tests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.cloud.conformance.bigtable.v2.TestFile", len)?;
        if !self.read_rows_tests.is_empty() {
            struct_ser.serialize_field("readRowsTests", &self.read_rows_tests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestFile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "read_rows_tests",
            "readRowsTests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadRowsTests,
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
                            "readRowsTests" | "read_rows_tests" => Ok(GeneratedField::ReadRowsTests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestFile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.cloud.conformance.bigtable.v2.TestFile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestFile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut read_rows_tests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReadRowsTests => {
                            if read_rows_tests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readRowsTests"));
                            }
                            read_rows_tests__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestFile {
                    read_rows_tests: read_rows_tests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.cloud.conformance.bigtable.v2.TestFile", FIELDS, GeneratedVisitor)
    }
}
