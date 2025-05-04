// @generated
impl serde::Serialize for ArrayValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ArrayValue", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArrayValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
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
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArrayValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ArrayValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArrayValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArrayValue {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ArrayValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Cell {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp_micros != 0 {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Cell", len)?;
        if self.timestamp_micros != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("timestampMicros", ToString::to_string(&self.timestamp_micros).as_str())?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Cell {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp_micros",
            "timestampMicros",
            "value",
            "labels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TimestampMicros,
            Value,
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
                            "timestampMicros" | "timestamp_micros" => Ok(GeneratedField::TimestampMicros),
                            "value" => Ok(GeneratedField::Value),
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
            type Value = Cell;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Cell")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Cell, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp_micros__ = None;
                let mut value__ = None;
                let mut labels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Cell {
                    timestamp_micros: timestamp_micros__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Cell", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckAndMutateRowRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.authorized_view_name.is_empty() {
            len += 1;
        }
        if !self.app_profile_id.is_empty() {
            len += 1;
        }
        if !self.row_key.is_empty() {
            len += 1;
        }
        if self.predicate_filter.is_some() {
            len += 1;
        }
        if !self.true_mutations.is_empty() {
            len += 1;
        }
        if !self.false_mutations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.CheckAndMutateRowRequest", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.authorized_view_name.is_empty() {
            struct_ser.serialize_field("authorizedViewName", &self.authorized_view_name)?;
        }
        if !self.app_profile_id.is_empty() {
            struct_ser.serialize_field("appProfileId", &self.app_profile_id)?;
        }
        if !self.row_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rowKey", pbjson::private::base64::encode(&self.row_key).as_str())?;
        }
        if let Some(v) = self.predicate_filter.as_ref() {
            struct_ser.serialize_field("predicateFilter", v)?;
        }
        if !self.true_mutations.is_empty() {
            struct_ser.serialize_field("trueMutations", &self.true_mutations)?;
        }
        if !self.false_mutations.is_empty() {
            struct_ser.serialize_field("falseMutations", &self.false_mutations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckAndMutateRowRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_name",
            "tableName",
            "authorized_view_name",
            "authorizedViewName",
            "app_profile_id",
            "appProfileId",
            "row_key",
            "rowKey",
            "predicate_filter",
            "predicateFilter",
            "true_mutations",
            "trueMutations",
            "false_mutations",
            "falseMutations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            AuthorizedViewName,
            AppProfileId,
            RowKey,
            PredicateFilter,
            TrueMutations,
            FalseMutations,
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
                            "tableName" | "table_name" => Ok(GeneratedField::TableName),
                            "authorizedViewName" | "authorized_view_name" => Ok(GeneratedField::AuthorizedViewName),
                            "appProfileId" | "app_profile_id" => Ok(GeneratedField::AppProfileId),
                            "rowKey" | "row_key" => Ok(GeneratedField::RowKey),
                            "predicateFilter" | "predicate_filter" => Ok(GeneratedField::PredicateFilter),
                            "trueMutations" | "true_mutations" => Ok(GeneratedField::TrueMutations),
                            "falseMutations" | "false_mutations" => Ok(GeneratedField::FalseMutations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckAndMutateRowRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.CheckAndMutateRowRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckAndMutateRowRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name__ = None;
                let mut authorized_view_name__ = None;
                let mut app_profile_id__ = None;
                let mut row_key__ = None;
                let mut predicate_filter__ = None;
                let mut true_mutations__ = None;
                let mut false_mutations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizedViewName => {
                            if authorized_view_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizedViewName"));
                            }
                            authorized_view_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppProfileId => {
                            if app_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appProfileId"));
                            }
                            app_profile_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RowKey => {
                            if row_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowKey"));
                            }
                            row_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PredicateFilter => {
                            if predicate_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicateFilter"));
                            }
                            predicate_filter__ = map_.next_value()?;
                        }
                        GeneratedField::TrueMutations => {
                            if true_mutations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trueMutations"));
                            }
                            true_mutations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FalseMutations => {
                            if false_mutations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("falseMutations"));
                            }
                            false_mutations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckAndMutateRowRequest {
                    table_name: table_name__.unwrap_or_default(),
                    authorized_view_name: authorized_view_name__.unwrap_or_default(),
                    app_profile_id: app_profile_id__.unwrap_or_default(),
                    row_key: row_key__.unwrap_or_default(),
                    predicate_filter: predicate_filter__,
                    true_mutations: true_mutations__.unwrap_or_default(),
                    false_mutations: false_mutations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.CheckAndMutateRowRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckAndMutateRowResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.predicate_matched {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.CheckAndMutateRowResponse", len)?;
        if self.predicate_matched {
            struct_ser.serialize_field("predicateMatched", &self.predicate_matched)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckAndMutateRowResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "predicate_matched",
            "predicateMatched",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PredicateMatched,
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
                            "predicateMatched" | "predicate_matched" => Ok(GeneratedField::PredicateMatched),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckAndMutateRowResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.CheckAndMutateRowResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckAndMutateRowResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut predicate_matched__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PredicateMatched => {
                            if predicate_matched__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicateMatched"));
                            }
                            predicate_matched__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckAndMutateRowResponse {
                    predicate_matched: predicate_matched__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.CheckAndMutateRowResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Column {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.qualifier.is_empty() {
            len += 1;
        }
        if !self.cells.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Column", len)?;
        if !self.qualifier.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("qualifier", pbjson::private::base64::encode(&self.qualifier).as_str())?;
        }
        if !self.cells.is_empty() {
            struct_ser.serialize_field("cells", &self.cells)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Column {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "qualifier",
            "cells",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Qualifier,
            Cells,
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
                            "qualifier" => Ok(GeneratedField::Qualifier),
                            "cells" => Ok(GeneratedField::Cells),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Column;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Column")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Column, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut qualifier__ = None;
                let mut cells__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Qualifier => {
                            if qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("qualifier"));
                            }
                            qualifier__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cells => {
                            if cells__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cells"));
                            }
                            cells__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Column {
                    qualifier: qualifier__.unwrap_or_default(),
                    cells: cells__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Column", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnMetadata {
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
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ColumnMetadata", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Type,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ColumnMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ColumnMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
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
                            r#type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ColumnMetadata {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ColumnMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_name.is_empty() {
            len += 1;
        }
        if self.start_qualifier.is_some() {
            len += 1;
        }
        if self.end_qualifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ColumnRange", len)?;
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        if let Some(v) = self.start_qualifier.as_ref() {
            match v {
                column_range::StartQualifier::StartQualifierClosed(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("startQualifierClosed", pbjson::private::base64::encode(&v).as_str())?;
                }
                column_range::StartQualifier::StartQualifierOpen(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("startQualifierOpen", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        if let Some(v) = self.end_qualifier.as_ref() {
            match v {
                column_range::EndQualifier::EndQualifierClosed(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("endQualifierClosed", pbjson::private::base64::encode(&v).as_str())?;
                }
                column_range::EndQualifier::EndQualifierOpen(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("endQualifierOpen", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_name",
            "familyName",
            "start_qualifier_closed",
            "startQualifierClosed",
            "start_qualifier_open",
            "startQualifierOpen",
            "end_qualifier_closed",
            "endQualifierClosed",
            "end_qualifier_open",
            "endQualifierOpen",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyName,
            StartQualifierClosed,
            StartQualifierOpen,
            EndQualifierClosed,
            EndQualifierOpen,
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
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
                            "startQualifierClosed" | "start_qualifier_closed" => Ok(GeneratedField::StartQualifierClosed),
                            "startQualifierOpen" | "start_qualifier_open" => Ok(GeneratedField::StartQualifierOpen),
                            "endQualifierClosed" | "end_qualifier_closed" => Ok(GeneratedField::EndQualifierClosed),
                            "endQualifierOpen" | "end_qualifier_open" => Ok(GeneratedField::EndQualifierOpen),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ColumnRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ColumnRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_name__ = None;
                let mut start_qualifier__ = None;
                let mut end_qualifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartQualifierClosed => {
                            if start_qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startQualifierClosed"));
                            }
                            start_qualifier__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| column_range::StartQualifier::StartQualifierClosed(x.0));
                        }
                        GeneratedField::StartQualifierOpen => {
                            if start_qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startQualifierOpen"));
                            }
                            start_qualifier__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| column_range::StartQualifier::StartQualifierOpen(x.0));
                        }
                        GeneratedField::EndQualifierClosed => {
                            if end_qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endQualifierClosed"));
                            }
                            end_qualifier__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| column_range::EndQualifier::EndQualifierClosed(x.0));
                        }
                        GeneratedField::EndQualifierOpen => {
                            if end_qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endQualifierOpen"));
                            }
                            end_qualifier__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| column_range::EndQualifier::EndQualifierOpen(x.0));
                        }
                    }
                }
                Ok(ColumnRange {
                    family_name: family_name__.unwrap_or_default(),
                    start_qualifier: start_qualifier__,
                    end_qualifier: end_qualifier__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ColumnRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteQueryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.instance_name.is_empty() {
            len += 1;
        }
        if !self.app_profile_id.is_empty() {
            len += 1;
        }
        if !self.query.is_empty() {
            len += 1;
        }
        if !self.resume_token.is_empty() {
            len += 1;
        }
        if !self.params.is_empty() {
            len += 1;
        }
        if self.data_format.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ExecuteQueryRequest", len)?;
        if !self.instance_name.is_empty() {
            struct_ser.serialize_field("instanceName", &self.instance_name)?;
        }
        if !self.app_profile_id.is_empty() {
            struct_ser.serialize_field("appProfileId", &self.app_profile_id)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if !self.resume_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("resumeToken", pbjson::private::base64::encode(&self.resume_token).as_str())?;
        }
        if !self.params.is_empty() {
            struct_ser.serialize_field("params", &self.params)?;
        }
        if let Some(v) = self.data_format.as_ref() {
            match v {
                execute_query_request::DataFormat::ProtoFormat(v) => {
                    struct_ser.serialize_field("protoFormat", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteQueryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_name",
            "instanceName",
            "app_profile_id",
            "appProfileId",
            "query",
            "resume_token",
            "resumeToken",
            "params",
            "proto_format",
            "protoFormat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceName,
            AppProfileId,
            Query,
            ResumeToken,
            Params,
            ProtoFormat,
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
                            "instanceName" | "instance_name" => Ok(GeneratedField::InstanceName),
                            "appProfileId" | "app_profile_id" => Ok(GeneratedField::AppProfileId),
                            "query" => Ok(GeneratedField::Query),
                            "resumeToken" | "resume_token" => Ok(GeneratedField::ResumeToken),
                            "params" => Ok(GeneratedField::Params),
                            "protoFormat" | "proto_format" => Ok(GeneratedField::ProtoFormat),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteQueryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ExecuteQueryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteQueryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_name__ = None;
                let mut app_profile_id__ = None;
                let mut query__ = None;
                let mut resume_token__ = None;
                let mut params__ = None;
                let mut data_format__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceName => {
                            if instance_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceName"));
                            }
                            instance_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppProfileId => {
                            if app_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appProfileId"));
                            }
                            app_profile_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResumeToken => {
                            if resume_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resumeToken"));
                            }
                            resume_token__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::ProtoFormat => {
                            if data_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoFormat"));
                            }
                            data_format__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_query_request::DataFormat::ProtoFormat)
;
                        }
                    }
                }
                Ok(ExecuteQueryRequest {
                    instance_name: instance_name__.unwrap_or_default(),
                    app_profile_id: app_profile_id__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                    resume_token: resume_token__.unwrap_or_default(),
                    params: params__.unwrap_or_default(),
                    data_format: data_format__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ExecuteQueryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteQueryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ExecuteQueryResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                execute_query_response::Response::Metadata(v) => {
                    struct_ser.serialize_field("metadata", v)?;
                }
                execute_query_response::Response::Results(v) => {
                    struct_ser.serialize_field("results", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteQueryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "results",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
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
                            "metadata" => Ok(GeneratedField::Metadata),
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
            type Value = ExecuteQueryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ExecuteQueryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteQueryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_query_response::Response::Metadata)
;
                        }
                        GeneratedField::Results => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_query_response::Response::Results)
;
                        }
                    }
                }
                Ok(ExecuteQueryResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ExecuteQueryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Family {
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
        if !self.columns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Family", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Family {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Columns,
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
                            "columns" => Ok(GeneratedField::Columns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Family;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Family")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Family, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut columns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Family {
                    name: name__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Family", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FeatureFlags {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reverse_scans {
            len += 1;
        }
        if self.mutate_rows_rate_limit {
            len += 1;
        }
        if self.mutate_rows_rate_limit2 {
            len += 1;
        }
        if self.last_scanned_row_responses {
            len += 1;
        }
        if self.routing_cookie {
            len += 1;
        }
        if self.retry_info {
            len += 1;
        }
        if self.client_side_metrics_enabled {
            len += 1;
        }
        if self.traffic_director_enabled {
            len += 1;
        }
        if self.direct_access_requested {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.FeatureFlags", len)?;
        if self.reverse_scans {
            struct_ser.serialize_field("reverseScans", &self.reverse_scans)?;
        }
        if self.mutate_rows_rate_limit {
            struct_ser.serialize_field("mutateRowsRateLimit", &self.mutate_rows_rate_limit)?;
        }
        if self.mutate_rows_rate_limit2 {
            struct_ser.serialize_field("mutateRowsRateLimit2", &self.mutate_rows_rate_limit2)?;
        }
        if self.last_scanned_row_responses {
            struct_ser.serialize_field("lastScannedRowResponses", &self.last_scanned_row_responses)?;
        }
        if self.routing_cookie {
            struct_ser.serialize_field("routingCookie", &self.routing_cookie)?;
        }
        if self.retry_info {
            struct_ser.serialize_field("retryInfo", &self.retry_info)?;
        }
        if self.client_side_metrics_enabled {
            struct_ser.serialize_field("clientSideMetricsEnabled", &self.client_side_metrics_enabled)?;
        }
        if self.traffic_director_enabled {
            struct_ser.serialize_field("trafficDirectorEnabled", &self.traffic_director_enabled)?;
        }
        if self.direct_access_requested {
            struct_ser.serialize_field("directAccessRequested", &self.direct_access_requested)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FeatureFlags {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reverse_scans",
            "reverseScans",
            "mutate_rows_rate_limit",
            "mutateRowsRateLimit",
            "mutate_rows_rate_limit2",
            "mutateRowsRateLimit2",
            "last_scanned_row_responses",
            "lastScannedRowResponses",
            "routing_cookie",
            "routingCookie",
            "retry_info",
            "retryInfo",
            "client_side_metrics_enabled",
            "clientSideMetricsEnabled",
            "traffic_director_enabled",
            "trafficDirectorEnabled",
            "direct_access_requested",
            "directAccessRequested",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReverseScans,
            MutateRowsRateLimit,
            MutateRowsRateLimit2,
            LastScannedRowResponses,
            RoutingCookie,
            RetryInfo,
            ClientSideMetricsEnabled,
            TrafficDirectorEnabled,
            DirectAccessRequested,
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
                            "reverseScans" | "reverse_scans" => Ok(GeneratedField::ReverseScans),
                            "mutateRowsRateLimit" | "mutate_rows_rate_limit" => Ok(GeneratedField::MutateRowsRateLimit),
                            "mutateRowsRateLimit2" | "mutate_rows_rate_limit2" => Ok(GeneratedField::MutateRowsRateLimit2),
                            "lastScannedRowResponses" | "last_scanned_row_responses" => Ok(GeneratedField::LastScannedRowResponses),
                            "routingCookie" | "routing_cookie" => Ok(GeneratedField::RoutingCookie),
                            "retryInfo" | "retry_info" => Ok(GeneratedField::RetryInfo),
                            "clientSideMetricsEnabled" | "client_side_metrics_enabled" => Ok(GeneratedField::ClientSideMetricsEnabled),
                            "trafficDirectorEnabled" | "traffic_director_enabled" => Ok(GeneratedField::TrafficDirectorEnabled),
                            "directAccessRequested" | "direct_access_requested" => Ok(GeneratedField::DirectAccessRequested),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeatureFlags;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.FeatureFlags")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeatureFlags, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reverse_scans__ = None;
                let mut mutate_rows_rate_limit__ = None;
                let mut mutate_rows_rate_limit2__ = None;
                let mut last_scanned_row_responses__ = None;
                let mut routing_cookie__ = None;
                let mut retry_info__ = None;
                let mut client_side_metrics_enabled__ = None;
                let mut traffic_director_enabled__ = None;
                let mut direct_access_requested__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReverseScans => {
                            if reverse_scans__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reverseScans"));
                            }
                            reverse_scans__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MutateRowsRateLimit => {
                            if mutate_rows_rate_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutateRowsRateLimit"));
                            }
                            mutate_rows_rate_limit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MutateRowsRateLimit2 => {
                            if mutate_rows_rate_limit2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutateRowsRateLimit2"));
                            }
                            mutate_rows_rate_limit2__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastScannedRowResponses => {
                            if last_scanned_row_responses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastScannedRowResponses"));
                            }
                            last_scanned_row_responses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoutingCookie => {
                            if routing_cookie__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routingCookie"));
                            }
                            routing_cookie__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetryInfo => {
                            if retry_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryInfo"));
                            }
                            retry_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientSideMetricsEnabled => {
                            if client_side_metrics_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSideMetricsEnabled"));
                            }
                            client_side_metrics_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TrafficDirectorEnabled => {
                            if traffic_director_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trafficDirectorEnabled"));
                            }
                            traffic_director_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DirectAccessRequested => {
                            if direct_access_requested__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directAccessRequested"));
                            }
                            direct_access_requested__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FeatureFlags {
                    reverse_scans: reverse_scans__.unwrap_or_default(),
                    mutate_rows_rate_limit: mutate_rows_rate_limit__.unwrap_or_default(),
                    mutate_rows_rate_limit2: mutate_rows_rate_limit2__.unwrap_or_default(),
                    last_scanned_row_responses: last_scanned_row_responses__.unwrap_or_default(),
                    routing_cookie: routing_cookie__.unwrap_or_default(),
                    retry_info: retry_info__.unwrap_or_default(),
                    client_side_metrics_enabled: client_side_metrics_enabled__.unwrap_or_default(),
                    traffic_director_enabled: traffic_director_enabled__.unwrap_or_default(),
                    direct_access_requested: direct_access_requested__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.FeatureFlags", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FullReadStatsView {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.read_iteration_stats.is_some() {
            len += 1;
        }
        if self.request_latency_stats.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.FullReadStatsView", len)?;
        if let Some(v) = self.read_iteration_stats.as_ref() {
            struct_ser.serialize_field("readIterationStats", v)?;
        }
        if let Some(v) = self.request_latency_stats.as_ref() {
            struct_ser.serialize_field("requestLatencyStats", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FullReadStatsView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "read_iteration_stats",
            "readIterationStats",
            "request_latency_stats",
            "requestLatencyStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadIterationStats,
            RequestLatencyStats,
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
                            "readIterationStats" | "read_iteration_stats" => Ok(GeneratedField::ReadIterationStats),
                            "requestLatencyStats" | "request_latency_stats" => Ok(GeneratedField::RequestLatencyStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FullReadStatsView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.FullReadStatsView")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FullReadStatsView, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut read_iteration_stats__ = None;
                let mut request_latency_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReadIterationStats => {
                            if read_iteration_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readIterationStats"));
                            }
                            read_iteration_stats__ = map_.next_value()?;
                        }
                        GeneratedField::RequestLatencyStats => {
                            if request_latency_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestLatencyStats"));
                            }
                            request_latency_stats__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FullReadStatsView {
                    read_iteration_stats: read_iteration_stats__,
                    request_latency_stats: request_latency_stats__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.FullReadStatsView", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateInitialChangeStreamPartitionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.app_profile_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.GenerateInitialChangeStreamPartitionsRequest", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.app_profile_id.is_empty() {
            struct_ser.serialize_field("appProfileId", &self.app_profile_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateInitialChangeStreamPartitionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_name",
            "tableName",
            "app_profile_id",
            "appProfileId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            AppProfileId,
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
                            "tableName" | "table_name" => Ok(GeneratedField::TableName),
                            "appProfileId" | "app_profile_id" => Ok(GeneratedField::AppProfileId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateInitialChangeStreamPartitionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.GenerateInitialChangeStreamPartitionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateInitialChangeStreamPartitionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name__ = None;
                let mut app_profile_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppProfileId => {
                            if app_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appProfileId"));
                            }
                            app_profile_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenerateInitialChangeStreamPartitionsRequest {
                    table_name: table_name__.unwrap_or_default(),
                    app_profile_id: app_profile_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.GenerateInitialChangeStreamPartitionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateInitialChangeStreamPartitionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.partition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.GenerateInitialChangeStreamPartitionsResponse", len)?;
        if let Some(v) = self.partition.as_ref() {
            struct_ser.serialize_field("partition", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateInitialChangeStreamPartitionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "partition",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Partition,
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
                            "partition" => Ok(GeneratedField::Partition),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateInitialChangeStreamPartitionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.GenerateInitialChangeStreamPartitionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateInitialChangeStreamPartitionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut partition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Partition => {
                            if partition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partition"));
                            }
                            partition__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GenerateInitialChangeStreamPartitionsResponse {
                    partition: partition__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.GenerateInitialChangeStreamPartitionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MutateRowRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.authorized_view_name.is_empty() {
            len += 1;
        }
        if !self.app_profile_id.is_empty() {
            len += 1;
        }
        if !self.row_key.is_empty() {
            len += 1;
        }
        if !self.mutations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.MutateRowRequest", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.authorized_view_name.is_empty() {
            struct_ser.serialize_field("authorizedViewName", &self.authorized_view_name)?;
        }
        if !self.app_profile_id.is_empty() {
            struct_ser.serialize_field("appProfileId", &self.app_profile_id)?;
        }
        if !self.row_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rowKey", pbjson::private::base64::encode(&self.row_key).as_str())?;
        }
        if !self.mutations.is_empty() {
            struct_ser.serialize_field("mutations", &self.mutations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MutateRowRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_name",
            "tableName",
            "authorized_view_name",
            "authorizedViewName",
            "app_profile_id",
            "appProfileId",
            "row_key",
            "rowKey",
            "mutations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            AuthorizedViewName,
            AppProfileId,
            RowKey,
            Mutations,
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
                            "tableName" | "table_name" => Ok(GeneratedField::TableName),
                            "authorizedViewName" | "authorized_view_name" => Ok(GeneratedField::AuthorizedViewName),
                            "appProfileId" | "app_profile_id" => Ok(GeneratedField::AppProfileId),
                            "rowKey" | "row_key" => Ok(GeneratedField::RowKey),
                            "mutations" => Ok(GeneratedField::Mutations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MutateRowRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.MutateRowRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MutateRowRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name__ = None;
                let mut authorized_view_name__ = None;
                let mut app_profile_id__ = None;
                let mut row_key__ = None;
                let mut mutations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizedViewName => {
                            if authorized_view_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizedViewName"));
                            }
                            authorized_view_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppProfileId => {
                            if app_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appProfileId"));
                            }
                            app_profile_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RowKey => {
                            if row_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowKey"));
                            }
                            row_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mutations => {
                            if mutations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutations"));
                            }
                            mutations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MutateRowRequest {
                    table_name: table_name__.unwrap_or_default(),
                    authorized_view_name: authorized_view_name__.unwrap_or_default(),
                    app_profile_id: app_profile_id__.unwrap_or_default(),
                    row_key: row_key__.unwrap_or_default(),
                    mutations: mutations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.MutateRowRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MutateRowResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.MutateRowResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MutateRowResponse {
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
            type Value = MutateRowResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.MutateRowResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MutateRowResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MutateRowResponse {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.MutateRowResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MutateRowsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.authorized_view_name.is_empty() {
            len += 1;
        }
        if !self.app_profile_id.is_empty() {
            len += 1;
        }
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.MutateRowsRequest", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.authorized_view_name.is_empty() {
            struct_ser.serialize_field("authorizedViewName", &self.authorized_view_name)?;
        }
        if !self.app_profile_id.is_empty() {
            struct_ser.serialize_field("appProfileId", &self.app_profile_id)?;
        }
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MutateRowsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_name",
            "tableName",
            "authorized_view_name",
            "authorizedViewName",
            "app_profile_id",
            "appProfileId",
            "entries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            AuthorizedViewName,
            AppProfileId,
            Entries,
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
                            "tableName" | "table_name" => Ok(GeneratedField::TableName),
                            "authorizedViewName" | "authorized_view_name" => Ok(GeneratedField::AuthorizedViewName),
                            "appProfileId" | "app_profile_id" => Ok(GeneratedField::AppProfileId),
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MutateRowsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.MutateRowsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MutateRowsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name__ = None;
                let mut authorized_view_name__ = None;
                let mut app_profile_id__ = None;
                let mut entries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizedViewName => {
                            if authorized_view_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizedViewName"));
                            }
                            authorized_view_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppProfileId => {
                            if app_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appProfileId"));
                            }
                            app_profile_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MutateRowsRequest {
                    table_name: table_name__.unwrap_or_default(),
                    authorized_view_name: authorized_view_name__.unwrap_or_default(),
                    app_profile_id: app_profile_id__.unwrap_or_default(),
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.MutateRowsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for mutate_rows_request::Entry {
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
        if !self.mutations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.MutateRowsRequest.Entry", len)?;
        if !self.row_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rowKey", pbjson::private::base64::encode(&self.row_key).as_str())?;
        }
        if !self.mutations.is_empty() {
            struct_ser.serialize_field("mutations", &self.mutations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for mutate_rows_request::Entry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "row_key",
            "rowKey",
            "mutations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RowKey,
            Mutations,
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
                            "mutations" => Ok(GeneratedField::Mutations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = mutate_rows_request::Entry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.MutateRowsRequest.Entry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<mutate_rows_request::Entry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut row_key__ = None;
                let mut mutations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RowKey => {
                            if row_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowKey"));
                            }
                            row_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mutations => {
                            if mutations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutations"));
                            }
                            mutations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(mutate_rows_request::Entry {
                    row_key: row_key__.unwrap_or_default(),
                    mutations: mutations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.MutateRowsRequest.Entry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MutateRowsResponse {
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
        if self.rate_limit_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.MutateRowsResponse", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if let Some(v) = self.rate_limit_info.as_ref() {
            struct_ser.serialize_field("rateLimitInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MutateRowsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entries",
            "rate_limit_info",
            "rateLimitInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
            RateLimitInfo,
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
                            "rateLimitInfo" | "rate_limit_info" => Ok(GeneratedField::RateLimitInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MutateRowsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.MutateRowsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MutateRowsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                let mut rate_limit_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RateLimitInfo => {
                            if rate_limit_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimitInfo"));
                            }
                            rate_limit_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MutateRowsResponse {
                    entries: entries__.unwrap_or_default(),
                    rate_limit_info: rate_limit_info__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.MutateRowsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for mutate_rows_response::Entry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.index != 0 {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.MutateRowsResponse.Entry", len)?;
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for mutate_rows_response::Entry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "index",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Index,
            Status,
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
                            "index" => Ok(GeneratedField::Index),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = mutate_rows_response::Entry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.MutateRowsResponse.Entry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<mutate_rows_response::Entry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut index__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(mutate_rows_response::Entry {
                    index: index__.unwrap_or_default(),
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.MutateRowsResponse.Entry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Mutation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mutation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Mutation", len)?;
        if let Some(v) = self.mutation.as_ref() {
            match v {
                mutation::Mutation::SetCell(v) => {
                    struct_ser.serialize_field("setCell", v)?;
                }
                mutation::Mutation::AddToCell(v) => {
                    struct_ser.serialize_field("addToCell", v)?;
                }
                mutation::Mutation::MergeToCell(v) => {
                    struct_ser.serialize_field("mergeToCell", v)?;
                }
                mutation::Mutation::DeleteFromColumn(v) => {
                    struct_ser.serialize_field("deleteFromColumn", v)?;
                }
                mutation::Mutation::DeleteFromFamily(v) => {
                    struct_ser.serialize_field("deleteFromFamily", v)?;
                }
                mutation::Mutation::DeleteFromRow(v) => {
                    struct_ser.serialize_field("deleteFromRow", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Mutation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "set_cell",
            "setCell",
            "add_to_cell",
            "addToCell",
            "merge_to_cell",
            "mergeToCell",
            "delete_from_column",
            "deleteFromColumn",
            "delete_from_family",
            "deleteFromFamily",
            "delete_from_row",
            "deleteFromRow",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SetCell,
            AddToCell,
            MergeToCell,
            DeleteFromColumn,
            DeleteFromFamily,
            DeleteFromRow,
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
                            "setCell" | "set_cell" => Ok(GeneratedField::SetCell),
                            "addToCell" | "add_to_cell" => Ok(GeneratedField::AddToCell),
                            "mergeToCell" | "merge_to_cell" => Ok(GeneratedField::MergeToCell),
                            "deleteFromColumn" | "delete_from_column" => Ok(GeneratedField::DeleteFromColumn),
                            "deleteFromFamily" | "delete_from_family" => Ok(GeneratedField::DeleteFromFamily),
                            "deleteFromRow" | "delete_from_row" => Ok(GeneratedField::DeleteFromRow),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Mutation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Mutation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Mutation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mutation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SetCell => {
                            if mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setCell"));
                            }
                            mutation__ = map_.next_value::<::std::option::Option<_>>()?.map(mutation::Mutation::SetCell)
;
                        }
                        GeneratedField::AddToCell => {
                            if mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addToCell"));
                            }
                            mutation__ = map_.next_value::<::std::option::Option<_>>()?.map(mutation::Mutation::AddToCell)
;
                        }
                        GeneratedField::MergeToCell => {
                            if mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mergeToCell"));
                            }
                            mutation__ = map_.next_value::<::std::option::Option<_>>()?.map(mutation::Mutation::MergeToCell)
;
                        }
                        GeneratedField::DeleteFromColumn => {
                            if mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteFromColumn"));
                            }
                            mutation__ = map_.next_value::<::std::option::Option<_>>()?.map(mutation::Mutation::DeleteFromColumn)
;
                        }
                        GeneratedField::DeleteFromFamily => {
                            if mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteFromFamily"));
                            }
                            mutation__ = map_.next_value::<::std::option::Option<_>>()?.map(mutation::Mutation::DeleteFromFamily)
;
                        }
                        GeneratedField::DeleteFromRow => {
                            if mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteFromRow"));
                            }
                            mutation__ = map_.next_value::<::std::option::Option<_>>()?.map(mutation::Mutation::DeleteFromRow)
;
                        }
                    }
                }
                Ok(Mutation {
                    mutation: mutation__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Mutation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for mutation::AddToCell {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_name.is_empty() {
            len += 1;
        }
        if self.column_qualifier.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.input.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Mutation.AddToCell", len)?;
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        if let Some(v) = self.column_qualifier.as_ref() {
            struct_ser.serialize_field("columnQualifier", v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.input.as_ref() {
            struct_ser.serialize_field("input", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for mutation::AddToCell {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_name",
            "familyName",
            "column_qualifier",
            "columnQualifier",
            "timestamp",
            "input",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyName,
            ColumnQualifier,
            Timestamp,
            Input,
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
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
                            "columnQualifier" | "column_qualifier" => Ok(GeneratedField::ColumnQualifier),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "input" => Ok(GeneratedField::Input),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = mutation::AddToCell;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Mutation.AddToCell")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<mutation::AddToCell, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_name__ = None;
                let mut column_qualifier__ = None;
                let mut timestamp__ = None;
                let mut input__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnQualifier => {
                            if column_qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnQualifier"));
                            }
                            column_qualifier__ = map_.next_value()?;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::Input => {
                            if input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            input__ = map_.next_value()?;
                        }
                    }
                }
                Ok(mutation::AddToCell {
                    family_name: family_name__.unwrap_or_default(),
                    column_qualifier: column_qualifier__,
                    timestamp: timestamp__,
                    input: input__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Mutation.AddToCell", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for mutation::DeleteFromColumn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_name.is_empty() {
            len += 1;
        }
        if !self.column_qualifier.is_empty() {
            len += 1;
        }
        if self.time_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Mutation.DeleteFromColumn", len)?;
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        if !self.column_qualifier.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("columnQualifier", pbjson::private::base64::encode(&self.column_qualifier).as_str())?;
        }
        if let Some(v) = self.time_range.as_ref() {
            struct_ser.serialize_field("timeRange", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for mutation::DeleteFromColumn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_name",
            "familyName",
            "column_qualifier",
            "columnQualifier",
            "time_range",
            "timeRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyName,
            ColumnQualifier,
            TimeRange,
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
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
                            "columnQualifier" | "column_qualifier" => Ok(GeneratedField::ColumnQualifier),
                            "timeRange" | "time_range" => Ok(GeneratedField::TimeRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = mutation::DeleteFromColumn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Mutation.DeleteFromColumn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<mutation::DeleteFromColumn, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_name__ = None;
                let mut column_qualifier__ = None;
                let mut time_range__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnQualifier => {
                            if column_qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnQualifier"));
                            }
                            column_qualifier__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TimeRange => {
                            if time_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeRange"));
                            }
                            time_range__ = map_.next_value()?;
                        }
                    }
                }
                Ok(mutation::DeleteFromColumn {
                    family_name: family_name__.unwrap_or_default(),
                    column_qualifier: column_qualifier__.unwrap_or_default(),
                    time_range: time_range__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Mutation.DeleteFromColumn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for mutation::DeleteFromFamily {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Mutation.DeleteFromFamily", len)?;
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for mutation::DeleteFromFamily {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_name",
            "familyName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyName,
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
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = mutation::DeleteFromFamily;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Mutation.DeleteFromFamily")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<mutation::DeleteFromFamily, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(mutation::DeleteFromFamily {
                    family_name: family_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Mutation.DeleteFromFamily", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for mutation::DeleteFromRow {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Mutation.DeleteFromRow", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for mutation::DeleteFromRow {
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
            type Value = mutation::DeleteFromRow;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Mutation.DeleteFromRow")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<mutation::DeleteFromRow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(mutation::DeleteFromRow {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Mutation.DeleteFromRow", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for mutation::MergeToCell {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_name.is_empty() {
            len += 1;
        }
        if self.column_qualifier.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.input.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Mutation.MergeToCell", len)?;
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        if let Some(v) = self.column_qualifier.as_ref() {
            struct_ser.serialize_field("columnQualifier", v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.input.as_ref() {
            struct_ser.serialize_field("input", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for mutation::MergeToCell {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_name",
            "familyName",
            "column_qualifier",
            "columnQualifier",
            "timestamp",
            "input",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyName,
            ColumnQualifier,
            Timestamp,
            Input,
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
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
                            "columnQualifier" | "column_qualifier" => Ok(GeneratedField::ColumnQualifier),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "input" => Ok(GeneratedField::Input),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = mutation::MergeToCell;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Mutation.MergeToCell")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<mutation::MergeToCell, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_name__ = None;
                let mut column_qualifier__ = None;
                let mut timestamp__ = None;
                let mut input__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnQualifier => {
                            if column_qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnQualifier"));
                            }
                            column_qualifier__ = map_.next_value()?;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::Input => {
                            if input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            input__ = map_.next_value()?;
                        }
                    }
                }
                Ok(mutation::MergeToCell {
                    family_name: family_name__.unwrap_or_default(),
                    column_qualifier: column_qualifier__,
                    timestamp: timestamp__,
                    input: input__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Mutation.MergeToCell", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for mutation::SetCell {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_name.is_empty() {
            len += 1;
        }
        if !self.column_qualifier.is_empty() {
            len += 1;
        }
        if self.timestamp_micros != 0 {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Mutation.SetCell", len)?;
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        if !self.column_qualifier.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("columnQualifier", pbjson::private::base64::encode(&self.column_qualifier).as_str())?;
        }
        if self.timestamp_micros != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("timestampMicros", ToString::to_string(&self.timestamp_micros).as_str())?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for mutation::SetCell {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_name",
            "familyName",
            "column_qualifier",
            "columnQualifier",
            "timestamp_micros",
            "timestampMicros",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyName,
            ColumnQualifier,
            TimestampMicros,
            Value,
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
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
                            "columnQualifier" | "column_qualifier" => Ok(GeneratedField::ColumnQualifier),
                            "timestampMicros" | "timestamp_micros" => Ok(GeneratedField::TimestampMicros),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = mutation::SetCell;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Mutation.SetCell")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<mutation::SetCell, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_name__ = None;
                let mut column_qualifier__ = None;
                let mut timestamp_micros__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnQualifier => {
                            if column_qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnQualifier"));
                            }
                            column_qualifier__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
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
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(mutation::SetCell {
                    family_name: family_name__.unwrap_or_default(),
                    column_qualifier: column_qualifier__.unwrap_or_default(),
                    timestamp_micros: timestamp_micros__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Mutation.SetCell", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PartialResultSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resume_token.is_empty() {
            len += 1;
        }
        if self.estimated_batch_size != 0 {
            len += 1;
        }
        if self.partial_rows.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.PartialResultSet", len)?;
        if !self.resume_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("resumeToken", pbjson::private::base64::encode(&self.resume_token).as_str())?;
        }
        if self.estimated_batch_size != 0 {
            struct_ser.serialize_field("estimatedBatchSize", &self.estimated_batch_size)?;
        }
        if let Some(v) = self.partial_rows.as_ref() {
            match v {
                partial_result_set::PartialRows::ProtoRowsBatch(v) => {
                    struct_ser.serialize_field("protoRowsBatch", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PartialResultSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resume_token",
            "resumeToken",
            "estimated_batch_size",
            "estimatedBatchSize",
            "proto_rows_batch",
            "protoRowsBatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResumeToken,
            EstimatedBatchSize,
            ProtoRowsBatch,
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
                            "resumeToken" | "resume_token" => Ok(GeneratedField::ResumeToken),
                            "estimatedBatchSize" | "estimated_batch_size" => Ok(GeneratedField::EstimatedBatchSize),
                            "protoRowsBatch" | "proto_rows_batch" => Ok(GeneratedField::ProtoRowsBatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PartialResultSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.PartialResultSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PartialResultSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resume_token__ = None;
                let mut estimated_batch_size__ = None;
                let mut partial_rows__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResumeToken => {
                            if resume_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resumeToken"));
                            }
                            resume_token__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EstimatedBatchSize => {
                            if estimated_batch_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("estimatedBatchSize"));
                            }
                            estimated_batch_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProtoRowsBatch => {
                            if partial_rows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoRowsBatch"));
                            }
                            partial_rows__ = map_.next_value::<::std::option::Option<_>>()?.map(partial_result_set::PartialRows::ProtoRowsBatch)
;
                        }
                    }
                }
                Ok(PartialResultSet {
                    resume_token: resume_token__.unwrap_or_default(),
                    estimated_batch_size: estimated_batch_size__.unwrap_or_default(),
                    partial_rows: partial_rows__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.PartialResultSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingAndWarmRequest {
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
        if !self.app_profile_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.PingAndWarmRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.app_profile_id.is_empty() {
            struct_ser.serialize_field("appProfileId", &self.app_profile_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingAndWarmRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "app_profile_id",
            "appProfileId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            AppProfileId,
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
                            "appProfileId" | "app_profile_id" => Ok(GeneratedField::AppProfileId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PingAndWarmRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.PingAndWarmRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PingAndWarmRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut app_profile_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppProfileId => {
                            if app_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appProfileId"));
                            }
                            app_profile_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PingAndWarmRequest {
                    name: name__.unwrap_or_default(),
                    app_profile_id: app_profile_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.PingAndWarmRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingAndWarmResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.PingAndWarmResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingAndWarmResponse {
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
            type Value = PingAndWarmResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.PingAndWarmResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PingAndWarmResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PingAndWarmResponse {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.PingAndWarmResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProtoFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.ProtoFormat", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProtoFormat {
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
            type Value = ProtoFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ProtoFormat")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProtoFormat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ProtoFormat {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ProtoFormat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProtoRows {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ProtoRows", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProtoRows {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
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
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProtoRows;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ProtoRows")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProtoRows, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProtoRows {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ProtoRows", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProtoRowsBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.batch_data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ProtoRowsBatch", len)?;
        if !self.batch_data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("batchData", pbjson::private::base64::encode(&self.batch_data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProtoRowsBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_data",
            "batchData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchData,
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
                            "batchData" | "batch_data" => Ok(GeneratedField::BatchData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProtoRowsBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ProtoRowsBatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProtoRowsBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchData => {
                            if batch_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchData"));
                            }
                            batch_data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProtoRowsBatch {
                    batch_data: batch_data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ProtoRowsBatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProtoSchema {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.columns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ProtoSchema", len)?;
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProtoSchema {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Columns,
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
                            "columns" => Ok(GeneratedField::Columns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProtoSchema;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ProtoSchema")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProtoSchema, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut columns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProtoSchema {
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ProtoSchema", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.period.is_some() {
            len += 1;
        }
        if self.factor != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.RateLimitInfo", len)?;
        if let Some(v) = self.period.as_ref() {
            struct_ser.serialize_field("period", v)?;
        }
        if self.factor != 0. {
            struct_ser.serialize_field("factor", &self.factor)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "period",
            "factor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Period,
            Factor,
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
                            "period" => Ok(GeneratedField::Period),
                            "factor" => Ok(GeneratedField::Factor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.RateLimitInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RateLimitInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut period__ = None;
                let mut factor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Period => {
                            if period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            period__ = map_.next_value()?;
                        }
                        GeneratedField::Factor => {
                            if factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            factor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RateLimitInfo {
                    period: period__,
                    factor: factor__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.RateLimitInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadChangeStreamRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.app_profile_id.is_empty() {
            len += 1;
        }
        if self.partition.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if self.heartbeat_duration.is_some() {
            len += 1;
        }
        if self.start_from.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadChangeStreamRequest", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.app_profile_id.is_empty() {
            struct_ser.serialize_field("appProfileId", &self.app_profile_id)?;
        }
        if let Some(v) = self.partition.as_ref() {
            struct_ser.serialize_field("partition", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if let Some(v) = self.heartbeat_duration.as_ref() {
            struct_ser.serialize_field("heartbeatDuration", v)?;
        }
        if let Some(v) = self.start_from.as_ref() {
            match v {
                read_change_stream_request::StartFrom::StartTime(v) => {
                    struct_ser.serialize_field("startTime", v)?;
                }
                read_change_stream_request::StartFrom::ContinuationTokens(v) => {
                    struct_ser.serialize_field("continuationTokens", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadChangeStreamRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_name",
            "tableName",
            "app_profile_id",
            "appProfileId",
            "partition",
            "end_time",
            "endTime",
            "heartbeat_duration",
            "heartbeatDuration",
            "start_time",
            "startTime",
            "continuation_tokens",
            "continuationTokens",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            AppProfileId,
            Partition,
            EndTime,
            HeartbeatDuration,
            StartTime,
            ContinuationTokens,
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
                            "tableName" | "table_name" => Ok(GeneratedField::TableName),
                            "appProfileId" | "app_profile_id" => Ok(GeneratedField::AppProfileId),
                            "partition" => Ok(GeneratedField::Partition),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "heartbeatDuration" | "heartbeat_duration" => Ok(GeneratedField::HeartbeatDuration),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "continuationTokens" | "continuation_tokens" => Ok(GeneratedField::ContinuationTokens),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadChangeStreamRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadChangeStreamRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadChangeStreamRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name__ = None;
                let mut app_profile_id__ = None;
                let mut partition__ = None;
                let mut end_time__ = None;
                let mut heartbeat_duration__ = None;
                let mut start_from__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppProfileId => {
                            if app_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appProfileId"));
                            }
                            app_profile_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Partition => {
                            if partition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partition"));
                            }
                            partition__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::HeartbeatDuration => {
                            if heartbeat_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heartbeatDuration"));
                            }
                            heartbeat_duration__ = map_.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_from__ = map_.next_value::<::std::option::Option<_>>()?.map(read_change_stream_request::StartFrom::StartTime)
;
                        }
                        GeneratedField::ContinuationTokens => {
                            if start_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuationTokens"));
                            }
                            start_from__ = map_.next_value::<::std::option::Option<_>>()?.map(read_change_stream_request::StartFrom::ContinuationTokens)
;
                        }
                    }
                }
                Ok(ReadChangeStreamRequest {
                    table_name: table_name__.unwrap_or_default(),
                    app_profile_id: app_profile_id__.unwrap_or_default(),
                    partition: partition__,
                    end_time: end_time__,
                    heartbeat_duration: heartbeat_duration__,
                    start_from: start_from__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadChangeStreamRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadChangeStreamResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stream_record.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadChangeStreamResponse", len)?;
        if let Some(v) = self.stream_record.as_ref() {
            match v {
                read_change_stream_response::StreamRecord::DataChange(v) => {
                    struct_ser.serialize_field("dataChange", v)?;
                }
                read_change_stream_response::StreamRecord::Heartbeat(v) => {
                    struct_ser.serialize_field("heartbeat", v)?;
                }
                read_change_stream_response::StreamRecord::CloseStream(v) => {
                    struct_ser.serialize_field("closeStream", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadChangeStreamResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_change",
            "dataChange",
            "heartbeat",
            "close_stream",
            "closeStream",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataChange,
            Heartbeat,
            CloseStream,
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
                            "dataChange" | "data_change" => Ok(GeneratedField::DataChange),
                            "heartbeat" => Ok(GeneratedField::Heartbeat),
                            "closeStream" | "close_stream" => Ok(GeneratedField::CloseStream),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadChangeStreamResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadChangeStreamResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadChangeStreamResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stream_record__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DataChange => {
                            if stream_record__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataChange"));
                            }
                            stream_record__ = map_.next_value::<::std::option::Option<_>>()?.map(read_change_stream_response::StreamRecord::DataChange)
;
                        }
                        GeneratedField::Heartbeat => {
                            if stream_record__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heartbeat"));
                            }
                            stream_record__ = map_.next_value::<::std::option::Option<_>>()?.map(read_change_stream_response::StreamRecord::Heartbeat)
;
                        }
                        GeneratedField::CloseStream => {
                            if stream_record__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closeStream"));
                            }
                            stream_record__ = map_.next_value::<::std::option::Option<_>>()?.map(read_change_stream_response::StreamRecord::CloseStream)
;
                        }
                    }
                }
                Ok(ReadChangeStreamResponse {
                    stream_record: stream_record__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadChangeStreamResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_change_stream_response::CloseStream {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        if !self.continuation_tokens.is_empty() {
            len += 1;
        }
        if !self.new_partitions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadChangeStreamResponse.CloseStream", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if !self.continuation_tokens.is_empty() {
            struct_ser.serialize_field("continuationTokens", &self.continuation_tokens)?;
        }
        if !self.new_partitions.is_empty() {
            struct_ser.serialize_field("newPartitions", &self.new_partitions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_change_stream_response::CloseStream {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "continuation_tokens",
            "continuationTokens",
            "new_partitions",
            "newPartitions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            ContinuationTokens,
            NewPartitions,
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
                            "status" => Ok(GeneratedField::Status),
                            "continuationTokens" | "continuation_tokens" => Ok(GeneratedField::ContinuationTokens),
                            "newPartitions" | "new_partitions" => Ok(GeneratedField::NewPartitions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_change_stream_response::CloseStream;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadChangeStreamResponse.CloseStream")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<read_change_stream_response::CloseStream, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut continuation_tokens__ = None;
                let mut new_partitions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::ContinuationTokens => {
                            if continuation_tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuationTokens"));
                            }
                            continuation_tokens__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPartitions => {
                            if new_partitions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPartitions"));
                            }
                            new_partitions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(read_change_stream_response::CloseStream {
                    status: status__,
                    continuation_tokens: continuation_tokens__.unwrap_or_default(),
                    new_partitions: new_partitions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadChangeStreamResponse.CloseStream", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_change_stream_response::DataChange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if !self.source_cluster_id.is_empty() {
            len += 1;
        }
        if !self.row_key.is_empty() {
            len += 1;
        }
        if self.commit_timestamp.is_some() {
            len += 1;
        }
        if self.tiebreaker != 0 {
            len += 1;
        }
        if !self.chunks.is_empty() {
            len += 1;
        }
        if self.done {
            len += 1;
        }
        if !self.token.is_empty() {
            len += 1;
        }
        if self.estimated_low_watermark.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadChangeStreamResponse.DataChange", len)?;
        if self.r#type != 0 {
            let v = read_change_stream_response::data_change::Type::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.source_cluster_id.is_empty() {
            struct_ser.serialize_field("sourceClusterId", &self.source_cluster_id)?;
        }
        if !self.row_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rowKey", pbjson::private::base64::encode(&self.row_key).as_str())?;
        }
        if let Some(v) = self.commit_timestamp.as_ref() {
            struct_ser.serialize_field("commitTimestamp", v)?;
        }
        if self.tiebreaker != 0 {
            struct_ser.serialize_field("tiebreaker", &self.tiebreaker)?;
        }
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        if self.done {
            struct_ser.serialize_field("done", &self.done)?;
        }
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        if let Some(v) = self.estimated_low_watermark.as_ref() {
            struct_ser.serialize_field("estimatedLowWatermark", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_change_stream_response::DataChange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "source_cluster_id",
            "sourceClusterId",
            "row_key",
            "rowKey",
            "commit_timestamp",
            "commitTimestamp",
            "tiebreaker",
            "chunks",
            "done",
            "token",
            "estimated_low_watermark",
            "estimatedLowWatermark",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            SourceClusterId,
            RowKey,
            CommitTimestamp,
            Tiebreaker,
            Chunks,
            Done,
            Token,
            EstimatedLowWatermark,
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
                            "sourceClusterId" | "source_cluster_id" => Ok(GeneratedField::SourceClusterId),
                            "rowKey" | "row_key" => Ok(GeneratedField::RowKey),
                            "commitTimestamp" | "commit_timestamp" => Ok(GeneratedField::CommitTimestamp),
                            "tiebreaker" => Ok(GeneratedField::Tiebreaker),
                            "chunks" => Ok(GeneratedField::Chunks),
                            "done" => Ok(GeneratedField::Done),
                            "token" => Ok(GeneratedField::Token),
                            "estimatedLowWatermark" | "estimated_low_watermark" => Ok(GeneratedField::EstimatedLowWatermark),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_change_stream_response::DataChange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadChangeStreamResponse.DataChange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<read_change_stream_response::DataChange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut source_cluster_id__ = None;
                let mut row_key__ = None;
                let mut commit_timestamp__ = None;
                let mut tiebreaker__ = None;
                let mut chunks__ = None;
                let mut done__ = None;
                let mut token__ = None;
                let mut estimated_low_watermark__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<read_change_stream_response::data_change::Type>()? as i32);
                        }
                        GeneratedField::SourceClusterId => {
                            if source_cluster_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceClusterId"));
                            }
                            source_cluster_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RowKey => {
                            if row_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowKey"));
                            }
                            row_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CommitTimestamp => {
                            if commit_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitTimestamp"));
                            }
                            commit_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::Tiebreaker => {
                            if tiebreaker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tiebreaker"));
                            }
                            tiebreaker__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Chunks => {
                            if chunks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Done => {
                            if done__.is_some() {
                                return Err(serde::de::Error::duplicate_field("done"));
                            }
                            done__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EstimatedLowWatermark => {
                            if estimated_low_watermark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("estimatedLowWatermark"));
                            }
                            estimated_low_watermark__ = map_.next_value()?;
                        }
                    }
                }
                Ok(read_change_stream_response::DataChange {
                    r#type: r#type__.unwrap_or_default(),
                    source_cluster_id: source_cluster_id__.unwrap_or_default(),
                    row_key: row_key__.unwrap_or_default(),
                    commit_timestamp: commit_timestamp__,
                    tiebreaker: tiebreaker__.unwrap_or_default(),
                    chunks: chunks__.unwrap_or_default(),
                    done: done__.unwrap_or_default(),
                    token: token__.unwrap_or_default(),
                    estimated_low_watermark: estimated_low_watermark__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadChangeStreamResponse.DataChange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_change_stream_response::data_change::Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TYPE_UNSPECIFIED",
            Self::User => "USER",
            Self::GarbageCollection => "GARBAGE_COLLECTION",
            Self::Continuation => "CONTINUATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for read_change_stream_response::data_change::Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TYPE_UNSPECIFIED",
            "USER",
            "GARBAGE_COLLECTION",
            "CONTINUATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_change_stream_response::data_change::Type;

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
                    "TYPE_UNSPECIFIED" => Ok(read_change_stream_response::data_change::Type::Unspecified),
                    "USER" => Ok(read_change_stream_response::data_change::Type::User),
                    "GARBAGE_COLLECTION" => Ok(read_change_stream_response::data_change::Type::GarbageCollection),
                    "CONTINUATION" => Ok(read_change_stream_response::data_change::Type::Continuation),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for read_change_stream_response::Heartbeat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.continuation_token.is_some() {
            len += 1;
        }
        if self.estimated_low_watermark.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadChangeStreamResponse.Heartbeat", len)?;
        if let Some(v) = self.continuation_token.as_ref() {
            struct_ser.serialize_field("continuationToken", v)?;
        }
        if let Some(v) = self.estimated_low_watermark.as_ref() {
            struct_ser.serialize_field("estimatedLowWatermark", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_change_stream_response::Heartbeat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "continuation_token",
            "continuationToken",
            "estimated_low_watermark",
            "estimatedLowWatermark",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContinuationToken,
            EstimatedLowWatermark,
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
                            "continuationToken" | "continuation_token" => Ok(GeneratedField::ContinuationToken),
                            "estimatedLowWatermark" | "estimated_low_watermark" => Ok(GeneratedField::EstimatedLowWatermark),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_change_stream_response::Heartbeat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadChangeStreamResponse.Heartbeat")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<read_change_stream_response::Heartbeat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut continuation_token__ = None;
                let mut estimated_low_watermark__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContinuationToken => {
                            if continuation_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuationToken"));
                            }
                            continuation_token__ = map_.next_value()?;
                        }
                        GeneratedField::EstimatedLowWatermark => {
                            if estimated_low_watermark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("estimatedLowWatermark"));
                            }
                            estimated_low_watermark__ = map_.next_value()?;
                        }
                    }
                }
                Ok(read_change_stream_response::Heartbeat {
                    continuation_token: continuation_token__,
                    estimated_low_watermark: estimated_low_watermark__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadChangeStreamResponse.Heartbeat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_change_stream_response::MutationChunk {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chunk_info.is_some() {
            len += 1;
        }
        if self.mutation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadChangeStreamResponse.MutationChunk", len)?;
        if let Some(v) = self.chunk_info.as_ref() {
            struct_ser.serialize_field("chunkInfo", v)?;
        }
        if let Some(v) = self.mutation.as_ref() {
            struct_ser.serialize_field("mutation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_change_stream_response::MutationChunk {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chunk_info",
            "chunkInfo",
            "mutation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChunkInfo,
            Mutation,
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
                            "chunkInfo" | "chunk_info" => Ok(GeneratedField::ChunkInfo),
                            "mutation" => Ok(GeneratedField::Mutation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_change_stream_response::MutationChunk;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadChangeStreamResponse.MutationChunk")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<read_change_stream_response::MutationChunk, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chunk_info__ = None;
                let mut mutation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChunkInfo => {
                            if chunk_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkInfo"));
                            }
                            chunk_info__ = map_.next_value()?;
                        }
                        GeneratedField::Mutation => {
                            if mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutation"));
                            }
                            mutation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(read_change_stream_response::MutationChunk {
                    chunk_info: chunk_info__,
                    mutation: mutation__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadChangeStreamResponse.MutationChunk", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_change_stream_response::mutation_chunk::ChunkInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chunked_value_size != 0 {
            len += 1;
        }
        if self.chunked_value_offset != 0 {
            len += 1;
        }
        if self.last_chunk {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadChangeStreamResponse.MutationChunk.ChunkInfo", len)?;
        if self.chunked_value_size != 0 {
            struct_ser.serialize_field("chunkedValueSize", &self.chunked_value_size)?;
        }
        if self.chunked_value_offset != 0 {
            struct_ser.serialize_field("chunkedValueOffset", &self.chunked_value_offset)?;
        }
        if self.last_chunk {
            struct_ser.serialize_field("lastChunk", &self.last_chunk)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_change_stream_response::mutation_chunk::ChunkInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chunked_value_size",
            "chunkedValueSize",
            "chunked_value_offset",
            "chunkedValueOffset",
            "last_chunk",
            "lastChunk",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChunkedValueSize,
            ChunkedValueOffset,
            LastChunk,
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
                            "chunkedValueSize" | "chunked_value_size" => Ok(GeneratedField::ChunkedValueSize),
                            "chunkedValueOffset" | "chunked_value_offset" => Ok(GeneratedField::ChunkedValueOffset),
                            "lastChunk" | "last_chunk" => Ok(GeneratedField::LastChunk),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_change_stream_response::mutation_chunk::ChunkInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadChangeStreamResponse.MutationChunk.ChunkInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<read_change_stream_response::mutation_chunk::ChunkInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chunked_value_size__ = None;
                let mut chunked_value_offset__ = None;
                let mut last_chunk__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChunkedValueSize => {
                            if chunked_value_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkedValueSize"));
                            }
                            chunked_value_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ChunkedValueOffset => {
                            if chunked_value_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkedValueOffset"));
                            }
                            chunked_value_offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastChunk => {
                            if last_chunk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastChunk"));
                            }
                            last_chunk__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(read_change_stream_response::mutation_chunk::ChunkInfo {
                    chunked_value_size: chunked_value_size__.unwrap_or_default(),
                    chunked_value_offset: chunked_value_offset__.unwrap_or_default(),
                    last_chunk: last_chunk__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadChangeStreamResponse.MutationChunk.ChunkInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadIterationStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rows_seen_count != 0 {
            len += 1;
        }
        if self.rows_returned_count != 0 {
            len += 1;
        }
        if self.cells_seen_count != 0 {
            len += 1;
        }
        if self.cells_returned_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadIterationStats", len)?;
        if self.rows_seen_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rowsSeenCount", ToString::to_string(&self.rows_seen_count).as_str())?;
        }
        if self.rows_returned_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rowsReturnedCount", ToString::to_string(&self.rows_returned_count).as_str())?;
        }
        if self.cells_seen_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("cellsSeenCount", ToString::to_string(&self.cells_seen_count).as_str())?;
        }
        if self.cells_returned_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("cellsReturnedCount", ToString::to_string(&self.cells_returned_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadIterationStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rows_seen_count",
            "rowsSeenCount",
            "rows_returned_count",
            "rowsReturnedCount",
            "cells_seen_count",
            "cellsSeenCount",
            "cells_returned_count",
            "cellsReturnedCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RowsSeenCount,
            RowsReturnedCount,
            CellsSeenCount,
            CellsReturnedCount,
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
                            "rowsSeenCount" | "rows_seen_count" => Ok(GeneratedField::RowsSeenCount),
                            "rowsReturnedCount" | "rows_returned_count" => Ok(GeneratedField::RowsReturnedCount),
                            "cellsSeenCount" | "cells_seen_count" => Ok(GeneratedField::CellsSeenCount),
                            "cellsReturnedCount" | "cells_returned_count" => Ok(GeneratedField::CellsReturnedCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadIterationStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadIterationStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadIterationStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rows_seen_count__ = None;
                let mut rows_returned_count__ = None;
                let mut cells_seen_count__ = None;
                let mut cells_returned_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RowsSeenCount => {
                            if rows_seen_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowsSeenCount"));
                            }
                            rows_seen_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RowsReturnedCount => {
                            if rows_returned_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowsReturnedCount"));
                            }
                            rows_returned_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CellsSeenCount => {
                            if cells_seen_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cellsSeenCount"));
                            }
                            cells_seen_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CellsReturnedCount => {
                            if cells_returned_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cellsReturnedCount"));
                            }
                            cells_returned_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ReadIterationStats {
                    rows_seen_count: rows_seen_count__.unwrap_or_default(),
                    rows_returned_count: rows_returned_count__.unwrap_or_default(),
                    cells_seen_count: cells_seen_count__.unwrap_or_default(),
                    cells_returned_count: cells_returned_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadIterationStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadModifyWriteRowRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.authorized_view_name.is_empty() {
            len += 1;
        }
        if !self.app_profile_id.is_empty() {
            len += 1;
        }
        if !self.row_key.is_empty() {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadModifyWriteRowRequest", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.authorized_view_name.is_empty() {
            struct_ser.serialize_field("authorizedViewName", &self.authorized_view_name)?;
        }
        if !self.app_profile_id.is_empty() {
            struct_ser.serialize_field("appProfileId", &self.app_profile_id)?;
        }
        if !self.row_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rowKey", pbjson::private::base64::encode(&self.row_key).as_str())?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadModifyWriteRowRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_name",
            "tableName",
            "authorized_view_name",
            "authorizedViewName",
            "app_profile_id",
            "appProfileId",
            "row_key",
            "rowKey",
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            AuthorizedViewName,
            AppProfileId,
            RowKey,
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
                            "tableName" | "table_name" => Ok(GeneratedField::TableName),
                            "authorizedViewName" | "authorized_view_name" => Ok(GeneratedField::AuthorizedViewName),
                            "appProfileId" | "app_profile_id" => Ok(GeneratedField::AppProfileId),
                            "rowKey" | "row_key" => Ok(GeneratedField::RowKey),
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
            type Value = ReadModifyWriteRowRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadModifyWriteRowRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadModifyWriteRowRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name__ = None;
                let mut authorized_view_name__ = None;
                let mut app_profile_id__ = None;
                let mut row_key__ = None;
                let mut rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizedViewName => {
                            if authorized_view_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizedViewName"));
                            }
                            authorized_view_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppProfileId => {
                            if app_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appProfileId"));
                            }
                            app_profile_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RowKey => {
                            if row_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowKey"));
                            }
                            row_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadModifyWriteRowRequest {
                    table_name: table_name__.unwrap_or_default(),
                    authorized_view_name: authorized_view_name__.unwrap_or_default(),
                    app_profile_id: app_profile_id__.unwrap_or_default(),
                    row_key: row_key__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadModifyWriteRowRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadModifyWriteRowResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.row.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadModifyWriteRowResponse", len)?;
        if let Some(v) = self.row.as_ref() {
            struct_ser.serialize_field("row", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadModifyWriteRowResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "row",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Row,
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
                            "row" => Ok(GeneratedField::Row),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadModifyWriteRowResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadModifyWriteRowResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadModifyWriteRowResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut row__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Row => {
                            if row__.is_some() {
                                return Err(serde::de::Error::duplicate_field("row"));
                            }
                            row__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadModifyWriteRowResponse {
                    row: row__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadModifyWriteRowResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadModifyWriteRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_name.is_empty() {
            len += 1;
        }
        if !self.column_qualifier.is_empty() {
            len += 1;
        }
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadModifyWriteRule", len)?;
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        if !self.column_qualifier.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("columnQualifier", pbjson::private::base64::encode(&self.column_qualifier).as_str())?;
        }
        if let Some(v) = self.rule.as_ref() {
            match v {
                read_modify_write_rule::Rule::AppendValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("appendValue", pbjson::private::base64::encode(&v).as_str())?;
                }
                read_modify_write_rule::Rule::IncrementAmount(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("incrementAmount", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadModifyWriteRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_name",
            "familyName",
            "column_qualifier",
            "columnQualifier",
            "append_value",
            "appendValue",
            "increment_amount",
            "incrementAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyName,
            ColumnQualifier,
            AppendValue,
            IncrementAmount,
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
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
                            "columnQualifier" | "column_qualifier" => Ok(GeneratedField::ColumnQualifier),
                            "appendValue" | "append_value" => Ok(GeneratedField::AppendValue),
                            "incrementAmount" | "increment_amount" => Ok(GeneratedField::IncrementAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadModifyWriteRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadModifyWriteRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadModifyWriteRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_name__ = None;
                let mut column_qualifier__ = None;
                let mut rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnQualifier => {
                            if column_qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnQualifier"));
                            }
                            column_qualifier__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AppendValue => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appendValue"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| read_modify_write_rule::Rule::AppendValue(x.0));
                        }
                        GeneratedField::IncrementAmount => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("incrementAmount"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| read_modify_write_rule::Rule::IncrementAmount(x.0));
                        }
                    }
                }
                Ok(ReadModifyWriteRule {
                    family_name: family_name__.unwrap_or_default(),
                    column_qualifier: column_qualifier__.unwrap_or_default(),
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadModifyWriteRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadRowsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.authorized_view_name.is_empty() {
            len += 1;
        }
        if !self.app_profile_id.is_empty() {
            len += 1;
        }
        if self.rows.is_some() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        if self.rows_limit != 0 {
            len += 1;
        }
        if self.request_stats_view != 0 {
            len += 1;
        }
        if self.reversed {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadRowsRequest", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.authorized_view_name.is_empty() {
            struct_ser.serialize_field("authorizedViewName", &self.authorized_view_name)?;
        }
        if !self.app_profile_id.is_empty() {
            struct_ser.serialize_field("appProfileId", &self.app_profile_id)?;
        }
        if let Some(v) = self.rows.as_ref() {
            struct_ser.serialize_field("rows", v)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if self.rows_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rowsLimit", ToString::to_string(&self.rows_limit).as_str())?;
        }
        if self.request_stats_view != 0 {
            let v = read_rows_request::RequestStatsView::try_from(self.request_stats_view)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.request_stats_view)))?;
            struct_ser.serialize_field("requestStatsView", &v)?;
        }
        if self.reversed {
            struct_ser.serialize_field("reversed", &self.reversed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadRowsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_name",
            "tableName",
            "authorized_view_name",
            "authorizedViewName",
            "app_profile_id",
            "appProfileId",
            "rows",
            "filter",
            "rows_limit",
            "rowsLimit",
            "request_stats_view",
            "requestStatsView",
            "reversed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            AuthorizedViewName,
            AppProfileId,
            Rows,
            Filter,
            RowsLimit,
            RequestStatsView,
            Reversed,
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
                            "tableName" | "table_name" => Ok(GeneratedField::TableName),
                            "authorizedViewName" | "authorized_view_name" => Ok(GeneratedField::AuthorizedViewName),
                            "appProfileId" | "app_profile_id" => Ok(GeneratedField::AppProfileId),
                            "rows" => Ok(GeneratedField::Rows),
                            "filter" => Ok(GeneratedField::Filter),
                            "rowsLimit" | "rows_limit" => Ok(GeneratedField::RowsLimit),
                            "requestStatsView" | "request_stats_view" => Ok(GeneratedField::RequestStatsView),
                            "reversed" => Ok(GeneratedField::Reversed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadRowsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadRowsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadRowsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name__ = None;
                let mut authorized_view_name__ = None;
                let mut app_profile_id__ = None;
                let mut rows__ = None;
                let mut filter__ = None;
                let mut rows_limit__ = None;
                let mut request_stats_view__ = None;
                let mut reversed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizedViewName => {
                            if authorized_view_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizedViewName"));
                            }
                            authorized_view_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppProfileId => {
                            if app_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appProfileId"));
                            }
                            app_profile_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rows => {
                            if rows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rows"));
                            }
                            rows__ = map_.next_value()?;
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                        GeneratedField::RowsLimit => {
                            if rows_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowsLimit"));
                            }
                            rows_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequestStatsView => {
                            if request_stats_view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestStatsView"));
                            }
                            request_stats_view__ = Some(map_.next_value::<read_rows_request::RequestStatsView>()? as i32);
                        }
                        GeneratedField::Reversed => {
                            if reversed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reversed"));
                            }
                            reversed__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadRowsRequest {
                    table_name: table_name__.unwrap_or_default(),
                    authorized_view_name: authorized_view_name__.unwrap_or_default(),
                    app_profile_id: app_profile_id__.unwrap_or_default(),
                    rows: rows__,
                    filter: filter__,
                    rows_limit: rows_limit__.unwrap_or_default(),
                    request_stats_view: request_stats_view__.unwrap_or_default(),
                    reversed: reversed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadRowsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_rows_request::RequestStatsView {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REQUEST_STATS_VIEW_UNSPECIFIED",
            Self::RequestStatsNone => "REQUEST_STATS_NONE",
            Self::RequestStatsFull => "REQUEST_STATS_FULL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for read_rows_request::RequestStatsView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REQUEST_STATS_VIEW_UNSPECIFIED",
            "REQUEST_STATS_NONE",
            "REQUEST_STATS_FULL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_rows_request::RequestStatsView;

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
                    "REQUEST_STATS_VIEW_UNSPECIFIED" => Ok(read_rows_request::RequestStatsView::Unspecified),
                    "REQUEST_STATS_NONE" => Ok(read_rows_request::RequestStatsView::RequestStatsNone),
                    "REQUEST_STATS_FULL" => Ok(read_rows_request::RequestStatsView::RequestStatsFull),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReadRowsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chunks.is_empty() {
            len += 1;
        }
        if !self.last_scanned_row_key.is_empty() {
            len += 1;
        }
        if self.request_stats.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadRowsResponse", len)?;
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        if !self.last_scanned_row_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lastScannedRowKey", pbjson::private::base64::encode(&self.last_scanned_row_key).as_str())?;
        }
        if let Some(v) = self.request_stats.as_ref() {
            struct_ser.serialize_field("requestStats", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadRowsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chunks",
            "last_scanned_row_key",
            "lastScannedRowKey",
            "request_stats",
            "requestStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Chunks,
            LastScannedRowKey,
            RequestStats,
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
                            "chunks" => Ok(GeneratedField::Chunks),
                            "lastScannedRowKey" | "last_scanned_row_key" => Ok(GeneratedField::LastScannedRowKey),
                            "requestStats" | "request_stats" => Ok(GeneratedField::RequestStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadRowsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadRowsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadRowsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chunks__ = None;
                let mut last_scanned_row_key__ = None;
                let mut request_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Chunks => {
                            if chunks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastScannedRowKey => {
                            if last_scanned_row_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastScannedRowKey"));
                            }
                            last_scanned_row_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequestStats => {
                            if request_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestStats"));
                            }
                            request_stats__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadRowsResponse {
                    chunks: chunks__.unwrap_or_default(),
                    last_scanned_row_key: last_scanned_row_key__.unwrap_or_default(),
                    request_stats: request_stats__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadRowsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_rows_response::CellChunk {
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
        if self.family_name.is_some() {
            len += 1;
        }
        if self.qualifier.is_some() {
            len += 1;
        }
        if self.timestamp_micros != 0 {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if self.value_size != 0 {
            len += 1;
        }
        if self.row_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ReadRowsResponse.CellChunk", len)?;
        if !self.row_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rowKey", pbjson::private::base64::encode(&self.row_key).as_str())?;
        }
        if let Some(v) = self.family_name.as_ref() {
            struct_ser.serialize_field("familyName", v)?;
        }
        if let Some(v) = self.qualifier.as_ref() {
            struct_ser.serialize_field("qualifier", v)?;
        }
        if self.timestamp_micros != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("timestampMicros", ToString::to_string(&self.timestamp_micros).as_str())?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        if self.value_size != 0 {
            struct_ser.serialize_field("valueSize", &self.value_size)?;
        }
        if let Some(v) = self.row_status.as_ref() {
            match v {
                read_rows_response::cell_chunk::RowStatus::ResetRow(v) => {
                    struct_ser.serialize_field("resetRow", v)?;
                }
                read_rows_response::cell_chunk::RowStatus::CommitRow(v) => {
                    struct_ser.serialize_field("commitRow", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_rows_response::CellChunk {
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
            "labels",
            "value",
            "value_size",
            "valueSize",
            "reset_row",
            "resetRow",
            "commit_row",
            "commitRow",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RowKey,
            FamilyName,
            Qualifier,
            TimestampMicros,
            Labels,
            Value,
            ValueSize,
            ResetRow,
            CommitRow,
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
                            "labels" => Ok(GeneratedField::Labels),
                            "value" => Ok(GeneratedField::Value),
                            "valueSize" | "value_size" => Ok(GeneratedField::ValueSize),
                            "resetRow" | "reset_row" => Ok(GeneratedField::ResetRow),
                            "commitRow" | "commit_row" => Ok(GeneratedField::CommitRow),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_rows_response::CellChunk;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ReadRowsResponse.CellChunk")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<read_rows_response::CellChunk, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut row_key__ = None;
                let mut family_name__ = None;
                let mut qualifier__ = None;
                let mut timestamp_micros__ = None;
                let mut labels__ = None;
                let mut value__ = None;
                let mut value_size__ = None;
                let mut row_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RowKey => {
                            if row_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowKey"));
                            }
                            row_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = map_.next_value()?;
                        }
                        GeneratedField::Qualifier => {
                            if qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("qualifier"));
                            }
                            qualifier__ = map_.next_value()?;
                        }
                        GeneratedField::TimestampMicros => {
                            if timestamp_micros__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampMicros"));
                            }
                            timestamp_micros__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ValueSize => {
                            if value_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSize"));
                            }
                            value_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResetRow => {
                            if row_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resetRow"));
                            }
                            row_status__ = map_.next_value::<::std::option::Option<_>>()?.map(read_rows_response::cell_chunk::RowStatus::ResetRow);
                        }
                        GeneratedField::CommitRow => {
                            if row_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitRow"));
                            }
                            row_status__ = map_.next_value::<::std::option::Option<_>>()?.map(read_rows_response::cell_chunk::RowStatus::CommitRow);
                        }
                    }
                }
                Ok(read_rows_response::CellChunk {
                    row_key: row_key__.unwrap_or_default(),
                    family_name: family_name__,
                    qualifier: qualifier__,
                    timestamp_micros: timestamp_micros__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    value_size: value_size__.unwrap_or_default(),
                    row_status: row_status__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ReadRowsResponse.CellChunk", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestLatencyStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.frontend_server_latency.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.RequestLatencyStats", len)?;
        if let Some(v) = self.frontend_server_latency.as_ref() {
            struct_ser.serialize_field("frontendServerLatency", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestLatencyStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "frontend_server_latency",
            "frontendServerLatency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FrontendServerLatency,
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
                            "frontendServerLatency" | "frontend_server_latency" => Ok(GeneratedField::FrontendServerLatency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestLatencyStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.RequestLatencyStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestLatencyStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut frontend_server_latency__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FrontendServerLatency => {
                            if frontend_server_latency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frontendServerLatency"));
                            }
                            frontend_server_latency__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RequestLatencyStats {
                    frontend_server_latency: frontend_server_latency__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.RequestLatencyStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stats_view.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.RequestStats", len)?;
        if let Some(v) = self.stats_view.as_ref() {
            match v {
                request_stats::StatsView::FullReadStatsView(v) => {
                    struct_ser.serialize_field("fullReadStatsView", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full_read_stats_view",
            "fullReadStatsView",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullReadStatsView,
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
                            "fullReadStatsView" | "full_read_stats_view" => Ok(GeneratedField::FullReadStatsView),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.RequestStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stats_view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullReadStatsView => {
                            if stats_view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullReadStatsView"));
                            }
                            stats_view__ = map_.next_value::<::std::option::Option<_>>()?.map(request_stats::StatsView::FullReadStatsView)
;
                        }
                    }
                }
                Ok(RequestStats {
                    stats_view: stats_view__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.RequestStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.zone_id.is_some() {
            len += 1;
        }
        if self.cluster_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ResponseParams", len)?;
        if let Some(v) = self.zone_id.as_ref() {
            struct_ser.serialize_field("zoneId", v)?;
        }
        if let Some(v) = self.cluster_id.as_ref() {
            struct_ser.serialize_field("clusterId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
            "cluster_id",
            "clusterId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
            ClusterId,
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
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "clusterId" | "cluster_id" => Ok(GeneratedField::ClusterId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ResponseParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                let mut cluster_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = map_.next_value()?;
                        }
                        GeneratedField::ClusterId => {
                            if cluster_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterId"));
                            }
                            cluster_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResponseParams {
                    zone_id: zone_id__,
                    cluster_id: cluster_id__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ResponseParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResultSetMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.schema.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ResultSetMetadata", len)?;
        if let Some(v) = self.schema.as_ref() {
            match v {
                result_set_metadata::Schema::ProtoSchema(v) => {
                    struct_ser.serialize_field("protoSchema", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResultSetMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proto_schema",
            "protoSchema",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProtoSchema,
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
                            "protoSchema" | "proto_schema" => Ok(GeneratedField::ProtoSchema),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResultSetMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ResultSetMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResultSetMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schema__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProtoSchema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoSchema"));
                            }
                            schema__ = map_.next_value::<::std::option::Option<_>>()?.map(result_set_metadata::Schema::ProtoSchema)
;
                        }
                    }
                }
                Ok(ResultSetMetadata {
                    schema: schema__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ResultSetMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Row {
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
        if !self.families.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Row", len)?;
        if !self.key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if !self.families.is_empty() {
            struct_ser.serialize_field("families", &self.families)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Row {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "families",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Families,
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
                            "families" => Ok(GeneratedField::Families),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Row;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Row")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Row, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut families__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Families => {
                            if families__.is_some() {
                                return Err(serde::de::Error::duplicate_field("families"));
                            }
                            families__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Row {
                    key: key__.unwrap_or_default(),
                    families: families__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Row", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RowFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.RowFilter", len)?;
        if let Some(v) = self.filter.as_ref() {
            match v {
                row_filter::Filter::Chain(v) => {
                    struct_ser.serialize_field("chain", v)?;
                }
                row_filter::Filter::Interleave(v) => {
                    struct_ser.serialize_field("interleave", v)?;
                }
                row_filter::Filter::Condition(v) => {
                    struct_ser.serialize_field("condition", v)?;
                }
                row_filter::Filter::Sink(v) => {
                    struct_ser.serialize_field("sink", v)?;
                }
                row_filter::Filter::PassAllFilter(v) => {
                    struct_ser.serialize_field("passAllFilter", v)?;
                }
                row_filter::Filter::BlockAllFilter(v) => {
                    struct_ser.serialize_field("blockAllFilter", v)?;
                }
                row_filter::Filter::RowKeyRegexFilter(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("rowKeyRegexFilter", pbjson::private::base64::encode(&v).as_str())?;
                }
                row_filter::Filter::RowSampleFilter(v) => {
                    struct_ser.serialize_field("rowSampleFilter", v)?;
                }
                row_filter::Filter::FamilyNameRegexFilter(v) => {
                    struct_ser.serialize_field("familyNameRegexFilter", v)?;
                }
                row_filter::Filter::ColumnQualifierRegexFilter(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("columnQualifierRegexFilter", pbjson::private::base64::encode(&v).as_str())?;
                }
                row_filter::Filter::ColumnRangeFilter(v) => {
                    struct_ser.serialize_field("columnRangeFilter", v)?;
                }
                row_filter::Filter::TimestampRangeFilter(v) => {
                    struct_ser.serialize_field("timestampRangeFilter", v)?;
                }
                row_filter::Filter::ValueRegexFilter(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("valueRegexFilter", pbjson::private::base64::encode(&v).as_str())?;
                }
                row_filter::Filter::ValueRangeFilter(v) => {
                    struct_ser.serialize_field("valueRangeFilter", v)?;
                }
                row_filter::Filter::CellsPerRowOffsetFilter(v) => {
                    struct_ser.serialize_field("cellsPerRowOffsetFilter", v)?;
                }
                row_filter::Filter::CellsPerRowLimitFilter(v) => {
                    struct_ser.serialize_field("cellsPerRowLimitFilter", v)?;
                }
                row_filter::Filter::CellsPerColumnLimitFilter(v) => {
                    struct_ser.serialize_field("cellsPerColumnLimitFilter", v)?;
                }
                row_filter::Filter::StripValueTransformer(v) => {
                    struct_ser.serialize_field("stripValueTransformer", v)?;
                }
                row_filter::Filter::ApplyLabelTransformer(v) => {
                    struct_ser.serialize_field("applyLabelTransformer", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RowFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain",
            "interleave",
            "condition",
            "sink",
            "pass_all_filter",
            "passAllFilter",
            "block_all_filter",
            "blockAllFilter",
            "row_key_regex_filter",
            "rowKeyRegexFilter",
            "row_sample_filter",
            "rowSampleFilter",
            "family_name_regex_filter",
            "familyNameRegexFilter",
            "column_qualifier_regex_filter",
            "columnQualifierRegexFilter",
            "column_range_filter",
            "columnRangeFilter",
            "timestamp_range_filter",
            "timestampRangeFilter",
            "value_regex_filter",
            "valueRegexFilter",
            "value_range_filter",
            "valueRangeFilter",
            "cells_per_row_offset_filter",
            "cellsPerRowOffsetFilter",
            "cells_per_row_limit_filter",
            "cellsPerRowLimitFilter",
            "cells_per_column_limit_filter",
            "cellsPerColumnLimitFilter",
            "strip_value_transformer",
            "stripValueTransformer",
            "apply_label_transformer",
            "applyLabelTransformer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Chain,
            Interleave,
            Condition,
            Sink,
            PassAllFilter,
            BlockAllFilter,
            RowKeyRegexFilter,
            RowSampleFilter,
            FamilyNameRegexFilter,
            ColumnQualifierRegexFilter,
            ColumnRangeFilter,
            TimestampRangeFilter,
            ValueRegexFilter,
            ValueRangeFilter,
            CellsPerRowOffsetFilter,
            CellsPerRowLimitFilter,
            CellsPerColumnLimitFilter,
            StripValueTransformer,
            ApplyLabelTransformer,
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
                            "chain" => Ok(GeneratedField::Chain),
                            "interleave" => Ok(GeneratedField::Interleave),
                            "condition" => Ok(GeneratedField::Condition),
                            "sink" => Ok(GeneratedField::Sink),
                            "passAllFilter" | "pass_all_filter" => Ok(GeneratedField::PassAllFilter),
                            "blockAllFilter" | "block_all_filter" => Ok(GeneratedField::BlockAllFilter),
                            "rowKeyRegexFilter" | "row_key_regex_filter" => Ok(GeneratedField::RowKeyRegexFilter),
                            "rowSampleFilter" | "row_sample_filter" => Ok(GeneratedField::RowSampleFilter),
                            "familyNameRegexFilter" | "family_name_regex_filter" => Ok(GeneratedField::FamilyNameRegexFilter),
                            "columnQualifierRegexFilter" | "column_qualifier_regex_filter" => Ok(GeneratedField::ColumnQualifierRegexFilter),
                            "columnRangeFilter" | "column_range_filter" => Ok(GeneratedField::ColumnRangeFilter),
                            "timestampRangeFilter" | "timestamp_range_filter" => Ok(GeneratedField::TimestampRangeFilter),
                            "valueRegexFilter" | "value_regex_filter" => Ok(GeneratedField::ValueRegexFilter),
                            "valueRangeFilter" | "value_range_filter" => Ok(GeneratedField::ValueRangeFilter),
                            "cellsPerRowOffsetFilter" | "cells_per_row_offset_filter" => Ok(GeneratedField::CellsPerRowOffsetFilter),
                            "cellsPerRowLimitFilter" | "cells_per_row_limit_filter" => Ok(GeneratedField::CellsPerRowLimitFilter),
                            "cellsPerColumnLimitFilter" | "cells_per_column_limit_filter" => Ok(GeneratedField::CellsPerColumnLimitFilter),
                            "stripValueTransformer" | "strip_value_transformer" => Ok(GeneratedField::StripValueTransformer),
                            "applyLabelTransformer" | "apply_label_transformer" => Ok(GeneratedField::ApplyLabelTransformer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RowFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.RowFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RowFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Chain => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chain"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::Chain)
;
                        }
                        GeneratedField::Interleave => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interleave"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::Interleave)
;
                        }
                        GeneratedField::Condition => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::Condition)
;
                        }
                        GeneratedField::Sink => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sink"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::Sink);
                        }
                        GeneratedField::PassAllFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passAllFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::PassAllFilter);
                        }
                        GeneratedField::BlockAllFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockAllFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::BlockAllFilter);
                        }
                        GeneratedField::RowKeyRegexFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowKeyRegexFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| row_filter::Filter::RowKeyRegexFilter(x.0));
                        }
                        GeneratedField::RowSampleFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowSampleFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| row_filter::Filter::RowSampleFilter(x.0));
                        }
                        GeneratedField::FamilyNameRegexFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyNameRegexFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::FamilyNameRegexFilter);
                        }
                        GeneratedField::ColumnQualifierRegexFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnQualifierRegexFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| row_filter::Filter::ColumnQualifierRegexFilter(x.0));
                        }
                        GeneratedField::ColumnRangeFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnRangeFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::ColumnRangeFilter)
;
                        }
                        GeneratedField::TimestampRangeFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampRangeFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::TimestampRangeFilter)
;
                        }
                        GeneratedField::ValueRegexFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRegexFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| row_filter::Filter::ValueRegexFilter(x.0));
                        }
                        GeneratedField::ValueRangeFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRangeFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::ValueRangeFilter)
;
                        }
                        GeneratedField::CellsPerRowOffsetFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cellsPerRowOffsetFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| row_filter::Filter::CellsPerRowOffsetFilter(x.0));
                        }
                        GeneratedField::CellsPerRowLimitFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cellsPerRowLimitFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| row_filter::Filter::CellsPerRowLimitFilter(x.0));
                        }
                        GeneratedField::CellsPerColumnLimitFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cellsPerColumnLimitFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| row_filter::Filter::CellsPerColumnLimitFilter(x.0));
                        }
                        GeneratedField::StripValueTransformer => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripValueTransformer"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::StripValueTransformer);
                        }
                        GeneratedField::ApplyLabelTransformer => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applyLabelTransformer"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(row_filter::Filter::ApplyLabelTransformer);
                        }
                    }
                }
                Ok(RowFilter {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.RowFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for row_filter::Chain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.RowFilter.Chain", len)?;
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for row_filter::Chain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filters,
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
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = row_filter::Chain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.RowFilter.Chain")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<row_filter::Chain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(row_filter::Chain {
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.RowFilter.Chain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for row_filter::Condition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.predicate_filter.is_some() {
            len += 1;
        }
        if self.true_filter.is_some() {
            len += 1;
        }
        if self.false_filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.RowFilter.Condition", len)?;
        if let Some(v) = self.predicate_filter.as_ref() {
            struct_ser.serialize_field("predicateFilter", v)?;
        }
        if let Some(v) = self.true_filter.as_ref() {
            struct_ser.serialize_field("trueFilter", v)?;
        }
        if let Some(v) = self.false_filter.as_ref() {
            struct_ser.serialize_field("falseFilter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for row_filter::Condition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "predicate_filter",
            "predicateFilter",
            "true_filter",
            "trueFilter",
            "false_filter",
            "falseFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PredicateFilter,
            TrueFilter,
            FalseFilter,
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
                            "predicateFilter" | "predicate_filter" => Ok(GeneratedField::PredicateFilter),
                            "trueFilter" | "true_filter" => Ok(GeneratedField::TrueFilter),
                            "falseFilter" | "false_filter" => Ok(GeneratedField::FalseFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = row_filter::Condition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.RowFilter.Condition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<row_filter::Condition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut predicate_filter__ = None;
                let mut true_filter__ = None;
                let mut false_filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PredicateFilter => {
                            if predicate_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicateFilter"));
                            }
                            predicate_filter__ = map_.next_value()?;
                        }
                        GeneratedField::TrueFilter => {
                            if true_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trueFilter"));
                            }
                            true_filter__ = map_.next_value()?;
                        }
                        GeneratedField::FalseFilter => {
                            if false_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("falseFilter"));
                            }
                            false_filter__ = map_.next_value()?;
                        }
                    }
                }
                Ok(row_filter::Condition {
                    predicate_filter: predicate_filter__,
                    true_filter: true_filter__,
                    false_filter: false_filter__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.RowFilter.Condition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for row_filter::Interleave {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.RowFilter.Interleave", len)?;
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for row_filter::Interleave {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filters,
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
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = row_filter::Interleave;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.RowFilter.Interleave")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<row_filter::Interleave, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(row_filter::Interleave {
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.RowFilter.Interleave", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RowRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_key.is_some() {
            len += 1;
        }
        if self.end_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.RowRange", len)?;
        if let Some(v) = self.start_key.as_ref() {
            match v {
                row_range::StartKey::StartKeyClosed(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("startKeyClosed", pbjson::private::base64::encode(&v).as_str())?;
                }
                row_range::StartKey::StartKeyOpen(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("startKeyOpen", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        if let Some(v) = self.end_key.as_ref() {
            match v {
                row_range::EndKey::EndKeyOpen(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("endKeyOpen", pbjson::private::base64::encode(&v).as_str())?;
                }
                row_range::EndKey::EndKeyClosed(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("endKeyClosed", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RowRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_key_closed",
            "startKeyClosed",
            "start_key_open",
            "startKeyOpen",
            "end_key_open",
            "endKeyOpen",
            "end_key_closed",
            "endKeyClosed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartKeyClosed,
            StartKeyOpen,
            EndKeyOpen,
            EndKeyClosed,
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
                            "startKeyClosed" | "start_key_closed" => Ok(GeneratedField::StartKeyClosed),
                            "startKeyOpen" | "start_key_open" => Ok(GeneratedField::StartKeyOpen),
                            "endKeyOpen" | "end_key_open" => Ok(GeneratedField::EndKeyOpen),
                            "endKeyClosed" | "end_key_closed" => Ok(GeneratedField::EndKeyClosed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RowRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.RowRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RowRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_key__ = None;
                let mut end_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartKeyClosed => {
                            if start_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startKeyClosed"));
                            }
                            start_key__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| row_range::StartKey::StartKeyClosed(x.0));
                        }
                        GeneratedField::StartKeyOpen => {
                            if start_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startKeyOpen"));
                            }
                            start_key__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| row_range::StartKey::StartKeyOpen(x.0));
                        }
                        GeneratedField::EndKeyOpen => {
                            if end_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endKeyOpen"));
                            }
                            end_key__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| row_range::EndKey::EndKeyOpen(x.0));
                        }
                        GeneratedField::EndKeyClosed => {
                            if end_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endKeyClosed"));
                            }
                            end_key__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| row_range::EndKey::EndKeyClosed(x.0));
                        }
                    }
                }
                Ok(RowRange {
                    start_key: start_key__,
                    end_key: end_key__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.RowRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RowSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.row_keys.is_empty() {
            len += 1;
        }
        if !self.row_ranges.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.RowSet", len)?;
        if !self.row_keys.is_empty() {
            struct_ser.serialize_field("rowKeys", &self.row_keys.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.row_ranges.is_empty() {
            struct_ser.serialize_field("rowRanges", &self.row_ranges)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RowSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "row_keys",
            "rowKeys",
            "row_ranges",
            "rowRanges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RowKeys,
            RowRanges,
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
                            "rowKeys" | "row_keys" => Ok(GeneratedField::RowKeys),
                            "rowRanges" | "row_ranges" => Ok(GeneratedField::RowRanges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RowSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.RowSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RowSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut row_keys__ = None;
                let mut row_ranges__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RowKeys => {
                            if row_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowKeys"));
                            }
                            row_keys__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::RowRanges => {
                            if row_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowRanges"));
                            }
                            row_ranges__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RowSet {
                    row_keys: row_keys__.unwrap_or_default(),
                    row_ranges: row_ranges__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.RowSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SampleRowKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.authorized_view_name.is_empty() {
            len += 1;
        }
        if !self.app_profile_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.SampleRowKeysRequest", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.authorized_view_name.is_empty() {
            struct_ser.serialize_field("authorizedViewName", &self.authorized_view_name)?;
        }
        if !self.app_profile_id.is_empty() {
            struct_ser.serialize_field("appProfileId", &self.app_profile_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SampleRowKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_name",
            "tableName",
            "authorized_view_name",
            "authorizedViewName",
            "app_profile_id",
            "appProfileId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            AuthorizedViewName,
            AppProfileId,
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
                            "tableName" | "table_name" => Ok(GeneratedField::TableName),
                            "authorizedViewName" | "authorized_view_name" => Ok(GeneratedField::AuthorizedViewName),
                            "appProfileId" | "app_profile_id" => Ok(GeneratedField::AppProfileId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SampleRowKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.SampleRowKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SampleRowKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name__ = None;
                let mut authorized_view_name__ = None;
                let mut app_profile_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizedViewName => {
                            if authorized_view_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizedViewName"));
                            }
                            authorized_view_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppProfileId => {
                            if app_profile_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appProfileId"));
                            }
                            app_profile_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SampleRowKeysRequest {
                    table_name: table_name__.unwrap_or_default(),
                    authorized_view_name: authorized_view_name__.unwrap_or_default(),
                    app_profile_id: app_profile_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.SampleRowKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SampleRowKeysResponse {
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
        if self.offset_bytes != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.SampleRowKeysResponse", len)?;
        if !self.row_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rowKey", pbjson::private::base64::encode(&self.row_key).as_str())?;
        }
        if self.offset_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("offsetBytes", ToString::to_string(&self.offset_bytes).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SampleRowKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "row_key",
            "rowKey",
            "offset_bytes",
            "offsetBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RowKey,
            OffsetBytes,
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
                            "offsetBytes" | "offset_bytes" => Ok(GeneratedField::OffsetBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SampleRowKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.SampleRowKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SampleRowKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut row_key__ = None;
                let mut offset_bytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RowKey => {
                            if row_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowKey"));
                            }
                            row_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OffsetBytes => {
                            if offset_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offsetBytes"));
                            }
                            offset_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SampleRowKeysResponse {
                    row_key: row_key__.unwrap_or_default(),
                    offset_bytes: offset_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.SampleRowKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamContinuationToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.partition.is_some() {
            len += 1;
        }
        if !self.token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.StreamContinuationToken", len)?;
        if let Some(v) = self.partition.as_ref() {
            struct_ser.serialize_field("partition", v)?;
        }
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamContinuationToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "partition",
            "token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Partition,
            Token,
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
                            "partition" => Ok(GeneratedField::Partition),
                            "token" => Ok(GeneratedField::Token),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamContinuationToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.StreamContinuationToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamContinuationToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut partition__ = None;
                let mut token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Partition => {
                            if partition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partition"));
                            }
                            partition__ = map_.next_value()?;
                        }
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamContinuationToken {
                    partition: partition__,
                    token: token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.StreamContinuationToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamContinuationTokens {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tokens.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.StreamContinuationTokens", len)?;
        if !self.tokens.is_empty() {
            struct_ser.serialize_field("tokens", &self.tokens)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamContinuationTokens {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tokens",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tokens,
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
                            "tokens" => Ok(GeneratedField::Tokens),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamContinuationTokens;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.StreamContinuationTokens")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamContinuationTokens, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tokens__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tokens => {
                            if tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            tokens__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamContinuationTokens {
                    tokens: tokens__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.StreamContinuationTokens", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamPartition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.row_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.StreamPartition", len)?;
        if let Some(v) = self.row_range.as_ref() {
            struct_ser.serialize_field("rowRange", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamPartition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "row_range",
            "rowRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RowRange,
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
                            "rowRange" | "row_range" => Ok(GeneratedField::RowRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamPartition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.StreamPartition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamPartition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut row_range__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RowRange => {
                            if row_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowRange"));
                            }
                            row_range__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StreamPartition {
                    row_range: row_range__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.StreamPartition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimestampRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_timestamp_micros != 0 {
            len += 1;
        }
        if self.end_timestamp_micros != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.TimestampRange", len)?;
        if self.start_timestamp_micros != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("startTimestampMicros", ToString::to_string(&self.start_timestamp_micros).as_str())?;
        }
        if self.end_timestamp_micros != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("endTimestampMicros", ToString::to_string(&self.end_timestamp_micros).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimestampRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_timestamp_micros",
            "startTimestampMicros",
            "end_timestamp_micros",
            "endTimestampMicros",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartTimestampMicros,
            EndTimestampMicros,
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
                            "startTimestampMicros" | "start_timestamp_micros" => Ok(GeneratedField::StartTimestampMicros),
                            "endTimestampMicros" | "end_timestamp_micros" => Ok(GeneratedField::EndTimestampMicros),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimestampRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.TimestampRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimestampRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_timestamp_micros__ = None;
                let mut end_timestamp_micros__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartTimestampMicros => {
                            if start_timestamp_micros__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTimestampMicros"));
                            }
                            start_timestamp_micros__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EndTimestampMicros => {
                            if end_timestamp_micros__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTimestampMicros"));
                            }
                            end_timestamp_micros__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TimestampRange {
                    start_timestamp_micros: start_timestamp_micros__.unwrap_or_default(),
                    end_timestamp_micros: end_timestamp_micros__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.TimestampRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type", len)?;
        if let Some(v) = self.kind.as_ref() {
            match v {
                r#type::Kind::BytesType(v) => {
                    struct_ser.serialize_field("bytesType", v)?;
                }
                r#type::Kind::StringType(v) => {
                    struct_ser.serialize_field("stringType", v)?;
                }
                r#type::Kind::Int64Type(v) => {
                    struct_ser.serialize_field("int64Type", v)?;
                }
                r#type::Kind::Float32Type(v) => {
                    struct_ser.serialize_field("float32Type", v)?;
                }
                r#type::Kind::Float64Type(v) => {
                    struct_ser.serialize_field("float64Type", v)?;
                }
                r#type::Kind::BoolType(v) => {
                    struct_ser.serialize_field("boolType", v)?;
                }
                r#type::Kind::TimestampType(v) => {
                    struct_ser.serialize_field("timestampType", v)?;
                }
                r#type::Kind::DateType(v) => {
                    struct_ser.serialize_field("dateType", v)?;
                }
                r#type::Kind::AggregateType(v) => {
                    struct_ser.serialize_field("aggregateType", v)?;
                }
                r#type::Kind::StructType(v) => {
                    struct_ser.serialize_field("structType", v)?;
                }
                r#type::Kind::ArrayType(v) => {
                    struct_ser.serialize_field("arrayType", v)?;
                }
                r#type::Kind::MapType(v) => {
                    struct_ser.serialize_field("mapType", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bytes_type",
            "bytesType",
            "string_type",
            "stringType",
            "int64_type",
            "int64Type",
            "float32_type",
            "float32Type",
            "float64_type",
            "float64Type",
            "bool_type",
            "boolType",
            "timestamp_type",
            "timestampType",
            "date_type",
            "dateType",
            "aggregate_type",
            "aggregateType",
            "struct_type",
            "structType",
            "array_type",
            "arrayType",
            "map_type",
            "mapType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BytesType,
            StringType,
            Int64Type,
            Float32Type,
            Float64Type,
            BoolType,
            TimestampType,
            DateType,
            AggregateType,
            StructType,
            ArrayType,
            MapType,
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
                            "bytesType" | "bytes_type" => Ok(GeneratedField::BytesType),
                            "stringType" | "string_type" => Ok(GeneratedField::StringType),
                            "int64Type" | "int64_type" => Ok(GeneratedField::Int64Type),
                            "float32Type" | "float32_type" => Ok(GeneratedField::Float32Type),
                            "float64Type" | "float64_type" => Ok(GeneratedField::Float64Type),
                            "boolType" | "bool_type" => Ok(GeneratedField::BoolType),
                            "timestampType" | "timestamp_type" => Ok(GeneratedField::TimestampType),
                            "dateType" | "date_type" => Ok(GeneratedField::DateType),
                            "aggregateType" | "aggregate_type" => Ok(GeneratedField::AggregateType),
                            "structType" | "struct_type" => Ok(GeneratedField::StructType),
                            "arrayType" | "array_type" => Ok(GeneratedField::ArrayType),
                            "mapType" | "map_type" => Ok(GeneratedField::MapType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Type, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BytesType => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesType"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::BytesType)
;
                        }
                        GeneratedField::StringType => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringType"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::StringType)
;
                        }
                        GeneratedField::Int64Type => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64Type"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::Int64Type)
;
                        }
                        GeneratedField::Float32Type => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("float32Type"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::Float32Type)
;
                        }
                        GeneratedField::Float64Type => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("float64Type"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::Float64Type)
;
                        }
                        GeneratedField::BoolType => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolType"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::BoolType)
;
                        }
                        GeneratedField::TimestampType => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampType"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::TimestampType)
;
                        }
                        GeneratedField::DateType => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateType"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::DateType)
;
                        }
                        GeneratedField::AggregateType => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregateType"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::AggregateType)
;
                        }
                        GeneratedField::StructType => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structType"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::StructType)
;
                        }
                        GeneratedField::ArrayType => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arrayType"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::ArrayType)
;
                        }
                        GeneratedField::MapType => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mapType"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::Kind::MapType)
;
                        }
                    }
                }
                Ok(Type {
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Aggregate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.input_type.is_some() {
            len += 1;
        }
        if self.state_type.is_some() {
            len += 1;
        }
        if self.aggregator.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Aggregate", len)?;
        if let Some(v) = self.input_type.as_ref() {
            struct_ser.serialize_field("inputType", v)?;
        }
        if let Some(v) = self.state_type.as_ref() {
            struct_ser.serialize_field("stateType", v)?;
        }
        if let Some(v) = self.aggregator.as_ref() {
            match v {
                r#type::aggregate::Aggregator::Sum(v) => {
                    struct_ser.serialize_field("sum", v)?;
                }
                r#type::aggregate::Aggregator::HllppUniqueCount(v) => {
                    struct_ser.serialize_field("hllppUniqueCount", v)?;
                }
                r#type::aggregate::Aggregator::Max(v) => {
                    struct_ser.serialize_field("max", v)?;
                }
                r#type::aggregate::Aggregator::Min(v) => {
                    struct_ser.serialize_field("min", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Aggregate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "input_type",
            "inputType",
            "state_type",
            "stateType",
            "sum",
            "hllpp_unique_count",
            "hllppUniqueCount",
            "max",
            "min",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InputType,
            StateType,
            Sum,
            HllppUniqueCount,
            Max,
            Min,
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
                            "inputType" | "input_type" => Ok(GeneratedField::InputType),
                            "stateType" | "state_type" => Ok(GeneratedField::StateType),
                            "sum" => Ok(GeneratedField::Sum),
                            "hllppUniqueCount" | "hllpp_unique_count" => Ok(GeneratedField::HllppUniqueCount),
                            "max" => Ok(GeneratedField::Max),
                            "min" => Ok(GeneratedField::Min),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::Aggregate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Aggregate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Aggregate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut input_type__ = None;
                let mut state_type__ = None;
                let mut aggregator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InputType => {
                            if input_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputType"));
                            }
                            input_type__ = map_.next_value()?;
                        }
                        GeneratedField::StateType => {
                            if state_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stateType"));
                            }
                            state_type__ = map_.next_value()?;
                        }
                        GeneratedField::Sum => {
                            if aggregator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sum"));
                            }
                            aggregator__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::aggregate::Aggregator::Sum)
;
                        }
                        GeneratedField::HllppUniqueCount => {
                            if aggregator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hllppUniqueCount"));
                            }
                            aggregator__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::aggregate::Aggregator::HllppUniqueCount)
;
                        }
                        GeneratedField::Max => {
                            if aggregator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            aggregator__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::aggregate::Aggregator::Max)
;
                        }
                        GeneratedField::Min => {
                            if aggregator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            aggregator__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::aggregate::Aggregator::Min)
;
                        }
                    }
                }
                Ok(r#type::Aggregate {
                    input_type: input_type__,
                    state_type: state_type__,
                    aggregator: aggregator__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Aggregate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::aggregate::HyperLogLogPlusPlusUniqueCount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Aggregate.HyperLogLogPlusPlusUniqueCount", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::aggregate::HyperLogLogPlusPlusUniqueCount {
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
            type Value = r#type::aggregate::HyperLogLogPlusPlusUniqueCount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Aggregate.HyperLogLogPlusPlusUniqueCount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::aggregate::HyperLogLogPlusPlusUniqueCount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::aggregate::HyperLogLogPlusPlusUniqueCount {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Aggregate.HyperLogLogPlusPlusUniqueCount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::aggregate::Max {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Aggregate.Max", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::aggregate::Max {
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
            type Value = r#type::aggregate::Max;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Aggregate.Max")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::aggregate::Max, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::aggregate::Max {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Aggregate.Max", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::aggregate::Min {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Aggregate.Min", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::aggregate::Min {
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
            type Value = r#type::aggregate::Min;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Aggregate.Min")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::aggregate::Min, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::aggregate::Min {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Aggregate.Min", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::aggregate::Sum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Aggregate.Sum", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::aggregate::Sum {
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
            type Value = r#type::aggregate::Sum;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Aggregate.Sum")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::aggregate::Sum, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::aggregate::Sum {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Aggregate.Sum", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Array {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.element_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Array", len)?;
        if let Some(v) = self.element_type.as_ref() {
            struct_ser.serialize_field("elementType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Array {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "element_type",
            "elementType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ElementType,
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
                            "elementType" | "element_type" => Ok(GeneratedField::ElementType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::Array;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Array")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Array, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut element_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ElementType => {
                            if element_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("elementType"));
                            }
                            element_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(r#type::Array {
                    element_type: element_type__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Array", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Bool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Bool", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Bool {
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
            type Value = r#type::Bool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Bool")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Bool, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::Bool {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Bool", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Bytes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.encoding.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Bytes", len)?;
        if let Some(v) = self.encoding.as_ref() {
            struct_ser.serialize_field("encoding", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Bytes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "encoding",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Encoding,
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
                            "encoding" => Ok(GeneratedField::Encoding),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::Bytes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Bytes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Bytes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut encoding__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Encoding => {
                            if encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encoding"));
                            }
                            encoding__ = map_.next_value()?;
                        }
                    }
                }
                Ok(r#type::Bytes {
                    encoding: encoding__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Bytes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::bytes::Encoding {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.encoding.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Bytes.Encoding", len)?;
        if let Some(v) = self.encoding.as_ref() {
            match v {
                r#type::bytes::encoding::Encoding::Raw(v) => {
                    struct_ser.serialize_field("raw", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::bytes::Encoding {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "raw",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Raw,
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
                            "raw" => Ok(GeneratedField::Raw),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::bytes::Encoding;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Bytes.Encoding")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::bytes::Encoding, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut encoding__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Raw => {
                            if encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("raw"));
                            }
                            encoding__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::bytes::encoding::Encoding::Raw)
;
                        }
                    }
                }
                Ok(r#type::bytes::Encoding {
                    encoding: encoding__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Bytes.Encoding", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::bytes::encoding::Raw {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Bytes.Encoding.Raw", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::bytes::encoding::Raw {
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
            type Value = r#type::bytes::encoding::Raw;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Bytes.Encoding.Raw")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::bytes::encoding::Raw, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::bytes::encoding::Raw {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Bytes.Encoding.Raw", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Date {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Date", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Date {
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
            type Value = r#type::Date;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Date")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Date, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::Date {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Date", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Float32 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Float32", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Float32 {
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
            type Value = r#type::Float32;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Float32")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Float32, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::Float32 {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Float32", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Float64 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Float64", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Float64 {
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
            type Value = r#type::Float64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Float64")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Float64, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::Float64 {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Float64", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Int64 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.encoding.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Int64", len)?;
        if let Some(v) = self.encoding.as_ref() {
            struct_ser.serialize_field("encoding", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Int64 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "encoding",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Encoding,
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
                            "encoding" => Ok(GeneratedField::Encoding),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::Int64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Int64")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Int64, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut encoding__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Encoding => {
                            if encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encoding"));
                            }
                            encoding__ = map_.next_value()?;
                        }
                    }
                }
                Ok(r#type::Int64 {
                    encoding: encoding__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Int64", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::int64::Encoding {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.encoding.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Int64.Encoding", len)?;
        if let Some(v) = self.encoding.as_ref() {
            match v {
                r#type::int64::encoding::Encoding::BigEndianBytes(v) => {
                    struct_ser.serialize_field("bigEndianBytes", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::int64::Encoding {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "big_endian_bytes",
            "bigEndianBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BigEndianBytes,
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
                            "bigEndianBytes" | "big_endian_bytes" => Ok(GeneratedField::BigEndianBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::int64::Encoding;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Int64.Encoding")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::int64::Encoding, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut encoding__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BigEndianBytes => {
                            if encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bigEndianBytes"));
                            }
                            encoding__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::int64::encoding::Encoding::BigEndianBytes)
;
                        }
                    }
                }
                Ok(r#type::int64::Encoding {
                    encoding: encoding__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Int64.Encoding", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::int64::encoding::BigEndianBytes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bytes_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Int64.Encoding.BigEndianBytes", len)?;
        if let Some(v) = self.bytes_type.as_ref() {
            struct_ser.serialize_field("bytesType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::int64::encoding::BigEndianBytes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bytes_type",
            "bytesType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BytesType,
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
                            "bytesType" | "bytes_type" => Ok(GeneratedField::BytesType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::int64::encoding::BigEndianBytes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Int64.Encoding.BigEndianBytes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::int64::encoding::BigEndianBytes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bytes_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BytesType => {
                            if bytes_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesType"));
                            }
                            bytes_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(r#type::int64::encoding::BigEndianBytes {
                    bytes_type: bytes_type__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Int64.Encoding.BigEndianBytes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Map {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_type.is_some() {
            len += 1;
        }
        if self.value_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Map", len)?;
        if let Some(v) = self.key_type.as_ref() {
            struct_ser.serialize_field("keyType", v)?;
        }
        if let Some(v) = self.value_type.as_ref() {
            struct_ser.serialize_field("valueType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Map {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_type",
            "keyType",
            "value_type",
            "valueType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyType,
            ValueType,
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
                            "keyType" | "key_type" => Ok(GeneratedField::KeyType),
                            "valueType" | "value_type" => Ok(GeneratedField::ValueType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::Map;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Map")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Map, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_type__ = None;
                let mut value_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyType => {
                            if key_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyType"));
                            }
                            key_type__ = map_.next_value()?;
                        }
                        GeneratedField::ValueType => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueType"));
                            }
                            value_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(r#type::Map {
                    key_type: key_type__,
                    value_type: value_type__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Map", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::String {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.encoding.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.String", len)?;
        if let Some(v) = self.encoding.as_ref() {
            struct_ser.serialize_field("encoding", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::String {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "encoding",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Encoding,
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
                            "encoding" => Ok(GeneratedField::Encoding),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::String;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.String")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::String, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut encoding__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Encoding => {
                            if encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encoding"));
                            }
                            encoding__ = map_.next_value()?;
                        }
                    }
                }
                Ok(r#type::String {
                    encoding: encoding__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.String", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::string::Encoding {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.encoding.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.String.Encoding", len)?;
        if let Some(v) = self.encoding.as_ref() {
            match v {
                r#type::string::encoding::Encoding::Utf8Raw(v) => {
                    struct_ser.serialize_field("utf8Raw", v)?;
                }
                r#type::string::encoding::Encoding::Utf8Bytes(v) => {
                    struct_ser.serialize_field("utf8Bytes", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::string::Encoding {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "utf8_raw",
            "utf8Raw",
            "utf8_bytes",
            "utf8Bytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Utf8Raw,
            Utf8Bytes,
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
                            "utf8Raw" | "utf8_raw" => Ok(GeneratedField::Utf8Raw),
                            "utf8Bytes" | "utf8_bytes" => Ok(GeneratedField::Utf8Bytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::string::Encoding;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.String.Encoding")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::string::Encoding, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut encoding__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Utf8Raw => {
                            if encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("utf8Raw"));
                            }
                            encoding__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::string::encoding::Encoding::Utf8Raw)
;
                        }
                        GeneratedField::Utf8Bytes => {
                            if encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("utf8Bytes"));
                            }
                            encoding__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::string::encoding::Encoding::Utf8Bytes)
;
                        }
                    }
                }
                Ok(r#type::string::Encoding {
                    encoding: encoding__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.String.Encoding", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::string::encoding::Utf8Bytes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.String.Encoding.Utf8Bytes", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::string::encoding::Utf8Bytes {
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
            type Value = r#type::string::encoding::Utf8Bytes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.String.Encoding.Utf8Bytes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::string::encoding::Utf8Bytes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::string::encoding::Utf8Bytes {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.String.Encoding.Utf8Bytes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::string::encoding::Utf8Raw {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.String.Encoding.Utf8Raw", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::string::encoding::Utf8Raw {
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
            type Value = r#type::string::encoding::Utf8Raw;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.String.Encoding.Utf8Raw")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::string::encoding::Utf8Raw, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::string::encoding::Utf8Raw {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.String.Encoding.Utf8Raw", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Struct {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fields.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Struct", len)?;
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Struct {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fields,
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
                            "fields" => Ok(GeneratedField::Fields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::Struct;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Struct")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Struct, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(r#type::Struct {
                    fields: fields__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Struct", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::r#struct::Field {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field_name.is_empty() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Struct.Field", len)?;
        if !self.field_name.is_empty() {
            struct_ser.serialize_field("fieldName", &self.field_name)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::r#struct::Field {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_name",
            "fieldName",
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldName,
            Type,
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
                            "fieldName" | "field_name" => Ok(GeneratedField::FieldName),
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::r#struct::Field;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Struct.Field")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::r#struct::Field, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_name__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldName => {
                            if field_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldName"));
                            }
                            field_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(r#type::r#struct::Field {
                    field_name: field_name__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Struct.Field", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::Timestamp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.bigtable.v2.Type.Timestamp", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::Timestamp {
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
            type Value = r#type::Timestamp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Type.Timestamp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::Timestamp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(r#type::Timestamp {
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Type.Timestamp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Value {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.Value", len)?;
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            match v {
                value::Kind::RawValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("rawValue", pbjson::private::base64::encode(&v).as_str())?;
                }
                value::Kind::RawTimestampMicros(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("rawTimestampMicros", ToString::to_string(&v).as_str())?;
                }
                value::Kind::BytesValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("bytesValue", pbjson::private::base64::encode(&v).as_str())?;
                }
                value::Kind::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                value::Kind::IntValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("intValue", ToString::to_string(&v).as_str())?;
                }
                value::Kind::BoolValue(v) => {
                    struct_ser.serialize_field("boolValue", v)?;
                }
                value::Kind::FloatValue(v) => {
                    struct_ser.serialize_field("floatValue", v)?;
                }
                value::Kind::TimestampValue(v) => {
                    struct_ser.serialize_field("timestampValue", v)?;
                }
                value::Kind::DateValue(v) => {
                    struct_ser.serialize_field("dateValue", v)?;
                }
                value::Kind::ArrayValue(v) => {
                    struct_ser.serialize_field("arrayValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Value {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "raw_value",
            "rawValue",
            "raw_timestamp_micros",
            "rawTimestampMicros",
            "bytes_value",
            "bytesValue",
            "string_value",
            "stringValue",
            "int_value",
            "intValue",
            "bool_value",
            "boolValue",
            "float_value",
            "floatValue",
            "timestamp_value",
            "timestampValue",
            "date_value",
            "dateValue",
            "array_value",
            "arrayValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            RawValue,
            RawTimestampMicros,
            BytesValue,
            StringValue,
            IntValue,
            BoolValue,
            FloatValue,
            TimestampValue,
            DateValue,
            ArrayValue,
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
                            "rawValue" | "raw_value" => Ok(GeneratedField::RawValue),
                            "rawTimestampMicros" | "raw_timestamp_micros" => Ok(GeneratedField::RawTimestampMicros),
                            "bytesValue" | "bytes_value" => Ok(GeneratedField::BytesValue),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "intValue" | "int_value" => Ok(GeneratedField::IntValue),
                            "boolValue" | "bool_value" => Ok(GeneratedField::BoolValue),
                            "floatValue" | "float_value" => Ok(GeneratedField::FloatValue),
                            "timestampValue" | "timestamp_value" => Ok(GeneratedField::TimestampValue),
                            "dateValue" | "date_value" => Ok(GeneratedField::DateValue),
                            "arrayValue" | "array_value" => Ok(GeneratedField::ArrayValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.Value")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Value, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map_.next_value()?;
                        }
                        GeneratedField::RawValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rawValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| value::Kind::RawValue(x.0));
                        }
                        GeneratedField::RawTimestampMicros => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rawTimestampMicros"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| value::Kind::RawTimestampMicros(x.0));
                        }
                        GeneratedField::BytesValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| value::Kind::BytesValue(x.0));
                        }
                        GeneratedField::StringValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::StringValue);
                        }
                        GeneratedField::IntValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| value::Kind::IntValue(x.0));
                        }
                        GeneratedField::BoolValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::BoolValue);
                        }
                        GeneratedField::FloatValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| value::Kind::FloatValue(x.0));
                        }
                        GeneratedField::TimestampValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::TimestampValue)
;
                        }
                        GeneratedField::DateValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::DateValue)
;
                        }
                        GeneratedField::ArrayValue => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arrayValue"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Kind::ArrayValue)
;
                        }
                    }
                }
                Ok(Value {
                    r#type: r#type__,
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.Value", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValueRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_value.is_some() {
            len += 1;
        }
        if self.end_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.bigtable.v2.ValueRange", len)?;
        if let Some(v) = self.start_value.as_ref() {
            match v {
                value_range::StartValue::StartValueClosed(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("startValueClosed", pbjson::private::base64::encode(&v).as_str())?;
                }
                value_range::StartValue::StartValueOpen(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("startValueOpen", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        if let Some(v) = self.end_value.as_ref() {
            match v {
                value_range::EndValue::EndValueClosed(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("endValueClosed", pbjson::private::base64::encode(&v).as_str())?;
                }
                value_range::EndValue::EndValueOpen(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("endValueOpen", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValueRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_value_closed",
            "startValueClosed",
            "start_value_open",
            "startValueOpen",
            "end_value_closed",
            "endValueClosed",
            "end_value_open",
            "endValueOpen",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartValueClosed,
            StartValueOpen,
            EndValueClosed,
            EndValueOpen,
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
                            "startValueClosed" | "start_value_closed" => Ok(GeneratedField::StartValueClosed),
                            "startValueOpen" | "start_value_open" => Ok(GeneratedField::StartValueOpen),
                            "endValueClosed" | "end_value_closed" => Ok(GeneratedField::EndValueClosed),
                            "endValueOpen" | "end_value_open" => Ok(GeneratedField::EndValueOpen),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValueRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.bigtable.v2.ValueRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValueRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_value__ = None;
                let mut end_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartValueClosed => {
                            if start_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startValueClosed"));
                            }
                            start_value__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| value_range::StartValue::StartValueClosed(x.0));
                        }
                        GeneratedField::StartValueOpen => {
                            if start_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startValueOpen"));
                            }
                            start_value__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| value_range::StartValue::StartValueOpen(x.0));
                        }
                        GeneratedField::EndValueClosed => {
                            if end_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endValueClosed"));
                            }
                            end_value__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| value_range::EndValue::EndValueClosed(x.0));
                        }
                        GeneratedField::EndValueOpen => {
                            if end_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endValueOpen"));
                            }
                            end_value__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| value_range::EndValue::EndValueOpen(x.0));
                        }
                    }
                }
                Ok(ValueRange {
                    start_value: start_value__,
                    end_value: end_value__,
                })
            }
        }
        deserializer.deserialize_struct("google.bigtable.v2.ValueRange", FIELDS, GeneratedVisitor)
    }
}
