// @generated
impl serde::Serialize for CalendarPeriod {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CALENDAR_PERIOD_UNSPECIFIED",
            Self::Day => "DAY",
            Self::Week => "WEEK",
            Self::Fortnight => "FORTNIGHT",
            Self::Month => "MONTH",
            Self::Quarter => "QUARTER",
            Self::Half => "HALF",
            Self::Year => "YEAR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CalendarPeriod {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CALENDAR_PERIOD_UNSPECIFIED",
            "DAY",
            "WEEK",
            "FORTNIGHT",
            "MONTH",
            "QUARTER",
            "HALF",
            "YEAR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CalendarPeriod;

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
                    "CALENDAR_PERIOD_UNSPECIFIED" => Ok(CalendarPeriod::Unspecified),
                    "DAY" => Ok(CalendarPeriod::Day),
                    "WEEK" => Ok(CalendarPeriod::Week),
                    "FORTNIGHT" => Ok(CalendarPeriod::Fortnight),
                    "MONTH" => Ok(CalendarPeriod::Month),
                    "QUARTER" => Ok(CalendarPeriod::Quarter),
                    "HALF" => Ok(CalendarPeriod::Half),
                    "YEAR" => Ok(CalendarPeriod::Year),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Color {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.red != 0. {
            len += 1;
        }
        if self.green != 0. {
            len += 1;
        }
        if self.blue != 0. {
            len += 1;
        }
        if self.alpha.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.Color", len)?;
        if self.red != 0. {
            struct_ser.serialize_field("red", &self.red)?;
        }
        if self.green != 0. {
            struct_ser.serialize_field("green", &self.green)?;
        }
        if self.blue != 0. {
            struct_ser.serialize_field("blue", &self.blue)?;
        }
        if let Some(v) = self.alpha.as_ref() {
            struct_ser.serialize_field("alpha", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Color {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "red",
            "green",
            "blue",
            "alpha",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Red,
            Green,
            Blue,
            Alpha,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "red" => Ok(GeneratedField::Red),
                            "green" => Ok(GeneratedField::Green),
                            "blue" => Ok(GeneratedField::Blue),
                            "alpha" => Ok(GeneratedField::Alpha),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.Color")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Color, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut red__ = None;
                let mut green__ = None;
                let mut blue__ = None;
                let mut alpha__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Red => {
                            if red__.is_some() {
                                return Err(serde::de::Error::duplicate_field("red"));
                            }
                            red__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Green => {
                            if green__.is_some() {
                                return Err(serde::de::Error::duplicate_field("green"));
                            }
                            green__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Blue => {
                            if blue__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blue"));
                            }
                            blue__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Alpha => {
                            if alpha__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alpha"));
                            }
                            alpha__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Color {
                    red: red__.unwrap_or_default(),
                    green: green__.unwrap_or_default(),
                    blue: blue__.unwrap_or_default(),
                    alpha: alpha__,
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.Color", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Date {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.year != 0 {
            len += 1;
        }
        if self.month != 0 {
            len += 1;
        }
        if self.day != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.Date", len)?;
        if self.year != 0 {
            struct_ser.serialize_field("year", &self.year)?;
        }
        if self.month != 0 {
            struct_ser.serialize_field("month", &self.month)?;
        }
        if self.day != 0 {
            struct_ser.serialize_field("day", &self.day)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Date {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "year",
            "month",
            "day",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Year,
            Month,
            Day,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "year" => Ok(GeneratedField::Year),
                            "month" => Ok(GeneratedField::Month),
                            "day" => Ok(GeneratedField::Day),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Date;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.Date")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Date, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut year__ = None;
                let mut month__ = None;
                let mut day__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Year => {
                            if year__.is_some() {
                                return Err(serde::de::Error::duplicate_field("year"));
                            }
                            year__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Month => {
                            if month__.is_some() {
                                return Err(serde::de::Error::duplicate_field("month"));
                            }
                            month__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Day => {
                            if day__.is_some() {
                                return Err(serde::de::Error::duplicate_field("day"));
                            }
                            day__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Date {
                    year: year__.unwrap_or_default(),
                    month: month__.unwrap_or_default(),
                    day: day__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.Date", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DateTime {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.year != 0 {
            len += 1;
        }
        if self.month != 0 {
            len += 1;
        }
        if self.day != 0 {
            len += 1;
        }
        if self.hours != 0 {
            len += 1;
        }
        if self.minutes != 0 {
            len += 1;
        }
        if self.seconds != 0 {
            len += 1;
        }
        if self.nanos != 0 {
            len += 1;
        }
        if self.time_offset.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.DateTime", len)?;
        if self.year != 0 {
            struct_ser.serialize_field("year", &self.year)?;
        }
        if self.month != 0 {
            struct_ser.serialize_field("month", &self.month)?;
        }
        if self.day != 0 {
            struct_ser.serialize_field("day", &self.day)?;
        }
        if self.hours != 0 {
            struct_ser.serialize_field("hours", &self.hours)?;
        }
        if self.minutes != 0 {
            struct_ser.serialize_field("minutes", &self.minutes)?;
        }
        if self.seconds != 0 {
            struct_ser.serialize_field("seconds", &self.seconds)?;
        }
        if self.nanos != 0 {
            struct_ser.serialize_field("nanos", &self.nanos)?;
        }
        if let Some(v) = self.time_offset.as_ref() {
            match v {
                date_time::TimeOffset::UtcOffset(v) => {
                    struct_ser.serialize_field("utcOffset", v)?;
                }
                date_time::TimeOffset::TimeZone(v) => {
                    struct_ser.serialize_field("timeZone", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DateTime {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "year",
            "month",
            "day",
            "hours",
            "minutes",
            "seconds",
            "nanos",
            "utc_offset",
            "utcOffset",
            "time_zone",
            "timeZone",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Year,
            Month,
            Day,
            Hours,
            Minutes,
            Seconds,
            Nanos,
            UtcOffset,
            TimeZone,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "year" => Ok(GeneratedField::Year),
                            "month" => Ok(GeneratedField::Month),
                            "day" => Ok(GeneratedField::Day),
                            "hours" => Ok(GeneratedField::Hours),
                            "minutes" => Ok(GeneratedField::Minutes),
                            "seconds" => Ok(GeneratedField::Seconds),
                            "nanos" => Ok(GeneratedField::Nanos),
                            "utcOffset" | "utc_offset" => Ok(GeneratedField::UtcOffset),
                            "timeZone" | "time_zone" => Ok(GeneratedField::TimeZone),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DateTime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.DateTime")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DateTime, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut year__ = None;
                let mut month__ = None;
                let mut day__ = None;
                let mut hours__ = None;
                let mut minutes__ = None;
                let mut seconds__ = None;
                let mut nanos__ = None;
                let mut time_offset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Year => {
                            if year__.is_some() {
                                return Err(serde::de::Error::duplicate_field("year"));
                            }
                            year__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Month => {
                            if month__.is_some() {
                                return Err(serde::de::Error::duplicate_field("month"));
                            }
                            month__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Day => {
                            if day__.is_some() {
                                return Err(serde::de::Error::duplicate_field("day"));
                            }
                            day__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hours => {
                            if hours__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hours"));
                            }
                            hours__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Minutes => {
                            if minutes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minutes"));
                            }
                            minutes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Seconds => {
                            if seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seconds"));
                            }
                            seconds__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UtcOffset => {
                            if time_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("utcOffset"));
                            }
                            time_offset__ = map_.next_value::<::std::option::Option<_>>()?.map(date_time::TimeOffset::UtcOffset)
;
                        }
                        GeneratedField::TimeZone => {
                            if time_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeZone"));
                            }
                            time_offset__ = map_.next_value::<::std::option::Option<_>>()?.map(date_time::TimeOffset::TimeZone)
;
                        }
                    }
                }
                Ok(DateTime {
                    year: year__.unwrap_or_default(),
                    month: month__.unwrap_or_default(),
                    day: day__.unwrap_or_default(),
                    hours: hours__.unwrap_or_default(),
                    minutes: minutes__.unwrap_or_default(),
                    seconds: seconds__.unwrap_or_default(),
                    nanos: nanos__.unwrap_or_default(),
                    time_offset: time_offset__,
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.DateTime", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DayOfWeek {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DAY_OF_WEEK_UNSPECIFIED",
            Self::Monday => "MONDAY",
            Self::Tuesday => "TUESDAY",
            Self::Wednesday => "WEDNESDAY",
            Self::Thursday => "THURSDAY",
            Self::Friday => "FRIDAY",
            Self::Saturday => "SATURDAY",
            Self::Sunday => "SUNDAY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for DayOfWeek {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DAY_OF_WEEK_UNSPECIFIED",
            "MONDAY",
            "TUESDAY",
            "WEDNESDAY",
            "THURSDAY",
            "FRIDAY",
            "SATURDAY",
            "SUNDAY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DayOfWeek;

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
                    "DAY_OF_WEEK_UNSPECIFIED" => Ok(DayOfWeek::Unspecified),
                    "MONDAY" => Ok(DayOfWeek::Monday),
                    "TUESDAY" => Ok(DayOfWeek::Tuesday),
                    "WEDNESDAY" => Ok(DayOfWeek::Wednesday),
                    "THURSDAY" => Ok(DayOfWeek::Thursday),
                    "FRIDAY" => Ok(DayOfWeek::Friday),
                    "SATURDAY" => Ok(DayOfWeek::Saturday),
                    "SUNDAY" => Ok(DayOfWeek::Sunday),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Decimal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.Decimal", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Decimal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = Decimal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.Decimal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Decimal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Decimal {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.Decimal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Expr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.expression.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.Expr", len)?;
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Expr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expression",
            "title",
            "description",
            "location",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expression,
            Title,
            Description,
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
                            "expression" => Ok(GeneratedField::Expression),
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
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
            type Value = Expr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.Expr")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Expr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expression__ = None;
                let mut title__ = None;
                let mut description__ = None;
                let mut location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Expr {
                    expression: expression__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.Expr", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Fraction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.numerator != 0 {
            len += 1;
        }
        if self.denominator != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.Fraction", len)?;
        if self.numerator != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("numerator", ToString::to_string(&self.numerator).as_str())?;
        }
        if self.denominator != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("denominator", ToString::to_string(&self.denominator).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Fraction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "numerator",
            "denominator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Numerator,
            Denominator,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "numerator" => Ok(GeneratedField::Numerator),
                            "denominator" => Ok(GeneratedField::Denominator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Fraction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.Fraction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Fraction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut numerator__ = None;
                let mut denominator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Numerator => {
                            if numerator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numerator"));
                            }
                            numerator__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Denominator => {
                            if denominator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denominator"));
                            }
                            denominator__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Fraction {
                    numerator: numerator__.unwrap_or_default(),
                    denominator: denominator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.Fraction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Interval {
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
        let mut struct_ser = serializer.serialize_struct("google.r#type.Interval", len)?;
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Interval {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartTime,
            EndTime,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Interval;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.Interval")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Interval, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_time__ = None;
                let mut end_time__ = None;
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
                    }
                }
                Ok(Interval {
                    start_time: start_time__,
                    end_time: end_time__,
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.Interval", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LatLng {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.latitude != 0. {
            len += 1;
        }
        if self.longitude != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.LatLng", len)?;
        if self.latitude != 0. {
            struct_ser.serialize_field("latitude", &self.latitude)?;
        }
        if self.longitude != 0. {
            struct_ser.serialize_field("longitude", &self.longitude)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LatLng {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "latitude",
            "longitude",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Latitude,
            Longitude,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "latitude" => Ok(GeneratedField::Latitude),
                            "longitude" => Ok(GeneratedField::Longitude),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LatLng;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.LatLng")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LatLng, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut latitude__ = None;
                let mut longitude__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Latitude => {
                            if latitude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latitude"));
                            }
                            latitude__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Longitude => {
                            if longitude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("longitude"));
                            }
                            longitude__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LatLng {
                    latitude: latitude__.unwrap_or_default(),
                    longitude: longitude__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.LatLng", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalizedText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        if !self.language_code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.LocalizedText", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if !self.language_code.is_empty() {
            struct_ser.serialize_field("languageCode", &self.language_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalizedText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "language_code",
            "languageCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            LanguageCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "text" => Ok(GeneratedField::Text),
                            "languageCode" | "language_code" => Ok(GeneratedField::LanguageCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalizedText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.LocalizedText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocalizedText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut language_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LanguageCode => {
                            if language_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("languageCode"));
                            }
                            language_code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LocalizedText {
                    text: text__.unwrap_or_default(),
                    language_code: language_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.LocalizedText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Money {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.currency_code.is_empty() {
            len += 1;
        }
        if self.units != 0 {
            len += 1;
        }
        if self.nanos != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.Money", len)?;
        if !self.currency_code.is_empty() {
            struct_ser.serialize_field("currencyCode", &self.currency_code)?;
        }
        if self.units != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("units", ToString::to_string(&self.units).as_str())?;
        }
        if self.nanos != 0 {
            struct_ser.serialize_field("nanos", &self.nanos)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Money {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "currency_code",
            "currencyCode",
            "units",
            "nanos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CurrencyCode,
            Units,
            Nanos,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "currencyCode" | "currency_code" => Ok(GeneratedField::CurrencyCode),
                            "units" => Ok(GeneratedField::Units),
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Money;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.Money")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Money, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut currency_code__ = None;
                let mut units__ = None;
                let mut nanos__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CurrencyCode => {
                            if currency_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currencyCode"));
                            }
                            currency_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Units => {
                            if units__.is_some() {
                                return Err(serde::de::Error::duplicate_field("units"));
                            }
                            units__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Money {
                    currency_code: currency_code__.unwrap_or_default(),
                    units: units__.unwrap_or_default(),
                    nanos: nanos__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.Money", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Month {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MONTH_UNSPECIFIED",
            Self::January => "JANUARY",
            Self::February => "FEBRUARY",
            Self::March => "MARCH",
            Self::April => "APRIL",
            Self::May => "MAY",
            Self::June => "JUNE",
            Self::July => "JULY",
            Self::August => "AUGUST",
            Self::September => "SEPTEMBER",
            Self::October => "OCTOBER",
            Self::November => "NOVEMBER",
            Self::December => "DECEMBER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Month {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MONTH_UNSPECIFIED",
            "JANUARY",
            "FEBRUARY",
            "MARCH",
            "APRIL",
            "MAY",
            "JUNE",
            "JULY",
            "AUGUST",
            "SEPTEMBER",
            "OCTOBER",
            "NOVEMBER",
            "DECEMBER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Month;

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
                    "MONTH_UNSPECIFIED" => Ok(Month::Unspecified),
                    "JANUARY" => Ok(Month::January),
                    "FEBRUARY" => Ok(Month::February),
                    "MARCH" => Ok(Month::March),
                    "APRIL" => Ok(Month::April),
                    "MAY" => Ok(Month::May),
                    "JUNE" => Ok(Month::June),
                    "JULY" => Ok(Month::July),
                    "AUGUST" => Ok(Month::August),
                    "SEPTEMBER" => Ok(Month::September),
                    "OCTOBER" => Ok(Month::October),
                    "NOVEMBER" => Ok(Month::November),
                    "DECEMBER" => Ok(Month::December),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PhoneNumber {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.extension.is_empty() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.PhoneNumber", len)?;
        if !self.extension.is_empty() {
            struct_ser.serialize_field("extension", &self.extension)?;
        }
        if let Some(v) = self.kind.as_ref() {
            match v {
                phone_number::Kind::E164Number(v) => {
                    struct_ser.serialize_field("e164Number", v)?;
                }
                phone_number::Kind::ShortCode(v) => {
                    struct_ser.serialize_field("shortCode", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PhoneNumber {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extension",
            "e164_number",
            "e164Number",
            "short_code",
            "shortCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Extension,
            E164Number,
            ShortCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "extension" => Ok(GeneratedField::Extension),
                            "e164Number" | "e164_number" => Ok(GeneratedField::E164Number),
                            "shortCode" | "short_code" => Ok(GeneratedField::ShortCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PhoneNumber;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.PhoneNumber")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PhoneNumber, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extension__ = None;
                let mut kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Extension => {
                            if extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            extension__ = Some(map_.next_value()?);
                        }
                        GeneratedField::E164Number => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("e164Number"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(phone_number::Kind::E164Number);
                        }
                        GeneratedField::ShortCode => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shortCode"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(phone_number::Kind::ShortCode)
;
                        }
                    }
                }
                Ok(PhoneNumber {
                    extension: extension__.unwrap_or_default(),
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.PhoneNumber", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for phone_number::ShortCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.region_code.is_empty() {
            len += 1;
        }
        if !self.number.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.PhoneNumber.ShortCode", len)?;
        if !self.region_code.is_empty() {
            struct_ser.serialize_field("regionCode", &self.region_code)?;
        }
        if !self.number.is_empty() {
            struct_ser.serialize_field("number", &self.number)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for phone_number::ShortCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "region_code",
            "regionCode",
            "number",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegionCode,
            Number,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "regionCode" | "region_code" => Ok(GeneratedField::RegionCode),
                            "number" => Ok(GeneratedField::Number),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = phone_number::ShortCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.PhoneNumber.ShortCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<phone_number::ShortCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut region_code__ = None;
                let mut number__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RegionCode => {
                            if region_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regionCode"));
                            }
                            region_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(phone_number::ShortCode {
                    region_code: region_code__.unwrap_or_default(),
                    number: number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.PhoneNumber.ShortCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PostalAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.revision != 0 {
            len += 1;
        }
        if !self.region_code.is_empty() {
            len += 1;
        }
        if !self.language_code.is_empty() {
            len += 1;
        }
        if !self.postal_code.is_empty() {
            len += 1;
        }
        if !self.sorting_code.is_empty() {
            len += 1;
        }
        if !self.administrative_area.is_empty() {
            len += 1;
        }
        if !self.locality.is_empty() {
            len += 1;
        }
        if !self.sublocality.is_empty() {
            len += 1;
        }
        if !self.address_lines.is_empty() {
            len += 1;
        }
        if !self.recipients.is_empty() {
            len += 1;
        }
        if !self.organization.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.PostalAddress", len)?;
        if self.revision != 0 {
            struct_ser.serialize_field("revision", &self.revision)?;
        }
        if !self.region_code.is_empty() {
            struct_ser.serialize_field("regionCode", &self.region_code)?;
        }
        if !self.language_code.is_empty() {
            struct_ser.serialize_field("languageCode", &self.language_code)?;
        }
        if !self.postal_code.is_empty() {
            struct_ser.serialize_field("postalCode", &self.postal_code)?;
        }
        if !self.sorting_code.is_empty() {
            struct_ser.serialize_field("sortingCode", &self.sorting_code)?;
        }
        if !self.administrative_area.is_empty() {
            struct_ser.serialize_field("administrativeArea", &self.administrative_area)?;
        }
        if !self.locality.is_empty() {
            struct_ser.serialize_field("locality", &self.locality)?;
        }
        if !self.sublocality.is_empty() {
            struct_ser.serialize_field("sublocality", &self.sublocality)?;
        }
        if !self.address_lines.is_empty() {
            struct_ser.serialize_field("addressLines", &self.address_lines)?;
        }
        if !self.recipients.is_empty() {
            struct_ser.serialize_field("recipients", &self.recipients)?;
        }
        if !self.organization.is_empty() {
            struct_ser.serialize_field("organization", &self.organization)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PostalAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "revision",
            "region_code",
            "regionCode",
            "language_code",
            "languageCode",
            "postal_code",
            "postalCode",
            "sorting_code",
            "sortingCode",
            "administrative_area",
            "administrativeArea",
            "locality",
            "sublocality",
            "address_lines",
            "addressLines",
            "recipients",
            "organization",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Revision,
            RegionCode,
            LanguageCode,
            PostalCode,
            SortingCode,
            AdministrativeArea,
            Locality,
            Sublocality,
            AddressLines,
            Recipients,
            Organization,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "revision" => Ok(GeneratedField::Revision),
                            "regionCode" | "region_code" => Ok(GeneratedField::RegionCode),
                            "languageCode" | "language_code" => Ok(GeneratedField::LanguageCode),
                            "postalCode" | "postal_code" => Ok(GeneratedField::PostalCode),
                            "sortingCode" | "sorting_code" => Ok(GeneratedField::SortingCode),
                            "administrativeArea" | "administrative_area" => Ok(GeneratedField::AdministrativeArea),
                            "locality" => Ok(GeneratedField::Locality),
                            "sublocality" => Ok(GeneratedField::Sublocality),
                            "addressLines" | "address_lines" => Ok(GeneratedField::AddressLines),
                            "recipients" => Ok(GeneratedField::Recipients),
                            "organization" => Ok(GeneratedField::Organization),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PostalAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.PostalAddress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PostalAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut revision__ = None;
                let mut region_code__ = None;
                let mut language_code__ = None;
                let mut postal_code__ = None;
                let mut sorting_code__ = None;
                let mut administrative_area__ = None;
                let mut locality__ = None;
                let mut sublocality__ = None;
                let mut address_lines__ = None;
                let mut recipients__ = None;
                let mut organization__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Revision => {
                            if revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("revision"));
                            }
                            revision__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RegionCode => {
                            if region_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regionCode"));
                            }
                            region_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LanguageCode => {
                            if language_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("languageCode"));
                            }
                            language_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PostalCode => {
                            if postal_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postalCode"));
                            }
                            postal_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortingCode => {
                            if sorting_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingCode"));
                            }
                            sorting_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdministrativeArea => {
                            if administrative_area__.is_some() {
                                return Err(serde::de::Error::duplicate_field("administrativeArea"));
                            }
                            administrative_area__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Locality => {
                            if locality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locality"));
                            }
                            locality__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sublocality => {
                            if sublocality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sublocality"));
                            }
                            sublocality__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddressLines => {
                            if address_lines__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressLines"));
                            }
                            address_lines__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipients => {
                            if recipients__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipients"));
                            }
                            recipients__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Organization => {
                            if organization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            organization__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PostalAddress {
                    revision: revision__.unwrap_or_default(),
                    region_code: region_code__.unwrap_or_default(),
                    language_code: language_code__.unwrap_or_default(),
                    postal_code: postal_code__.unwrap_or_default(),
                    sorting_code: sorting_code__.unwrap_or_default(),
                    administrative_area: administrative_area__.unwrap_or_default(),
                    locality: locality__.unwrap_or_default(),
                    sublocality: sublocality__.unwrap_or_default(),
                    address_lines: address_lines__.unwrap_or_default(),
                    recipients: recipients__.unwrap_or_default(),
                    organization: organization__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.PostalAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Quaternion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.x != 0. {
            len += 1;
        }
        if self.y != 0. {
            len += 1;
        }
        if self.z != 0. {
            len += 1;
        }
        if self.w != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.Quaternion", len)?;
        if self.x != 0. {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0. {
            struct_ser.serialize_field("y", &self.y)?;
        }
        if self.z != 0. {
            struct_ser.serialize_field("z", &self.z)?;
        }
        if self.w != 0. {
            struct_ser.serialize_field("w", &self.w)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Quaternion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
            "z",
            "w",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
            Z,
            W,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "x" => Ok(GeneratedField::X),
                            "y" => Ok(GeneratedField::Y),
                            "z" => Ok(GeneratedField::Z),
                            "w" => Ok(GeneratedField::W),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Quaternion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.Quaternion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Quaternion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
                let mut z__ = None;
                let mut w__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::X => {
                            if x__.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            x__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Y => {
                            if y__.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            y__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Z => {
                            if z__.is_some() {
                                return Err(serde::de::Error::duplicate_field("z"));
                            }
                            z__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::W => {
                            if w__.is_some() {
                                return Err(serde::de::Error::duplicate_field("w"));
                            }
                            w__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Quaternion {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                    z: z__.unwrap_or_default(),
                    w: w__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.Quaternion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeOfDay {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.hours != 0 {
            len += 1;
        }
        if self.minutes != 0 {
            len += 1;
        }
        if self.seconds != 0 {
            len += 1;
        }
        if self.nanos != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.TimeOfDay", len)?;
        if self.hours != 0 {
            struct_ser.serialize_field("hours", &self.hours)?;
        }
        if self.minutes != 0 {
            struct_ser.serialize_field("minutes", &self.minutes)?;
        }
        if self.seconds != 0 {
            struct_ser.serialize_field("seconds", &self.seconds)?;
        }
        if self.nanos != 0 {
            struct_ser.serialize_field("nanos", &self.nanos)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimeOfDay {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hours",
            "minutes",
            "seconds",
            "nanos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hours,
            Minutes,
            Seconds,
            Nanos,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "hours" => Ok(GeneratedField::Hours),
                            "minutes" => Ok(GeneratedField::Minutes),
                            "seconds" => Ok(GeneratedField::Seconds),
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimeOfDay;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.TimeOfDay")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimeOfDay, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hours__ = None;
                let mut minutes__ = None;
                let mut seconds__ = None;
                let mut nanos__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hours => {
                            if hours__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hours"));
                            }
                            hours__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Minutes => {
                            if minutes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minutes"));
                            }
                            minutes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Seconds => {
                            if seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seconds"));
                            }
                            seconds__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TimeOfDay {
                    hours: hours__.unwrap_or_default(),
                    minutes: minutes__.unwrap_or_default(),
                    seconds: seconds__.unwrap_or_default(),
                    nanos: nanos__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.TimeOfDay", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeZone {
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
        if !self.version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.r#type.TimeZone", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimeZone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
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
            type Value = TimeZone;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.r#type.TimeZone")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimeZone, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TimeZone {
                    id: id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.r#type.TimeZone", FIELDS, GeneratedVisitor)
    }
}
