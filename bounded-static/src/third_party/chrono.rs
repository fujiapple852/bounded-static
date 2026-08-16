use crate::{macros, IntoBoundedStatic, ToBoundedStatic};

/// Blanket [`ToBoundedStatic`] impl for converting `chrono::DateTime<Tz>` into `chrono::DateTime<Tz>: 'static`.
impl<Tz> ToBoundedStatic for chrono::DateTime<Tz>
where
    Tz: ToBoundedStatic + chrono::TimeZone,
    Tz::Static: chrono::TimeZone,
{
    type Static = chrono::DateTime<Tz::Static>;

    fn to_static(&self) -> Self::Static {
        self.with_timezone(&self.timezone().to_static())
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `chrono::DateTime<Tz>` into `chrono::DateTime<Tz>: 'static`.
impl<Tz> IntoBoundedStatic for chrono::DateTime<Tz>
where
    Tz: IntoBoundedStatic + chrono::TimeZone,
    Tz::Static: chrono::TimeZone,
{
    type Static = chrono::DateTime<Tz::Static>;

    fn into_static(self) -> Self::Static {
        self.with_timezone(&self.timezone().into_static())
    }
}

macros::make_copy_impl!(chrono::FixedOffset);
macros::make_copy_impl!(chrono::Months);
macros::make_copy_impl!(chrono::TimeDelta);
macros::make_copy_impl!(chrono::Utc);
macros::make_copy_impl!(chrono::Month);
macros::make_copy_impl!(chrono::Weekday);
macros::make_copy_impl!(chrono::naive::Days);
macros::make_copy_impl!(chrono::naive::IsoWeek);
macros::make_copy_impl!(chrono::naive::NaiveDate);
macros::make_copy_impl!(chrono::naive::NaiveDateTime);
macros::make_copy_impl!(chrono::naive::NaiveTime);
#[cfg(feature = "chrono-clock")]
macros::make_copy_impl!(chrono::Local);
// No implementation for chrono::NaiveWeek as it's not Copy nor Clone.

#[cfg(test)]
mod tests {
    use super::*;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_chrono_datetime() {
        let value = chrono::Utc::now();
        let to_static = value.to_static();
        assert_eq!(value, to_static);
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_datetime_with_custom_tz() {
        use chrono::{
            DateTime, FixedOffset, MappedLocalTime, NaiveDate, NaiveDateTime, Offset, TimeZone,
        };
        #[derive(Debug, Clone)]
        struct MyOffset;
        impl Offset for MyOffset {
            fn fix(&self) -> FixedOffset {
                FixedOffset::east_opt(1).unwrap()
            }
        }
        #[derive(Clone)]
        struct MyTz;
        impl TimeZone for MyTz {
            type Offset = MyOffset;

            fn from_offset(_offset: &Self::Offset) -> Self {
                Self
            }

            fn offset_from_local_date(&self, _local: &NaiveDate) -> MappedLocalTime<Self::Offset> {
                MappedLocalTime::None
            }

            fn offset_from_local_datetime(
                &self,
                _local: &NaiveDateTime,
            ) -> MappedLocalTime<Self::Offset> {
                MappedLocalTime::None
            }

            fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Self::Offset {
                MyOffset
            }

            fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Self::Offset {
                MyOffset
            }
        }

        impl ToBoundedStatic for MyTz {
            type Static = Self;

            fn to_static(&self) -> Self::Static {
                self.clone()
            }
        }

        let value = DateTime::from_timestamp(0, 0).unwrap().with_timezone(&MyTz);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_fixed_offset() {
        let value = chrono::FixedOffset::east_opt(1).unwrap();
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_months() {
        let value = chrono::Months::new(1);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_time_delta() {
        let value = chrono::TimeDelta::days(10);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_utc() {
        let value = chrono::Utc;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_month() {
        let value = chrono::Month::January;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_weekday() {
        let value = chrono::Weekday::Mon;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_naive_days() {
        let value = chrono::naive::Days::new(1);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_naive_iso_week() {
        use chrono::Datelike;
        let value = chrono::naive::NaiveDate::from_ymd_opt(2024, 6, 1)
            .unwrap()
            .iso_week();
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_naive_date() {
        let value = chrono::naive::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_naive_date_time() {
        let value = chrono::naive::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            chrono::NaiveTime::from_hms_opt(22, 33, 44).unwrap(),
        );
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_chrono_naive_time() {
        let value = chrono::naive::NaiveTime::from_hms_opt(22, 33, 44).unwrap();
        let to_static = value.to_static();
        ensure_static(to_static);
    }
}

#[cfg(feature = "chrono-clock")]
#[cfg(test)]
mod clock_tests {
    use super::*;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_chrono_local() {
        let value = chrono::Local::now();
        let to_static = value.to_static();
        ensure_static(to_static);
    }
}
