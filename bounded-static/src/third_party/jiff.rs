use crate::{macros, IntoBoundedStatic, ToBoundedStatic};

macros::make_copy_impl!(jiff::SignedDuration);
macros::make_copy_impl!(jiff::Span);
macros::make_copy_impl!(jiff::SpanFieldwise);
macros::make_copy_impl!(jiff::Timestamp);
macros::make_copy_impl!(jiff::civil::Date);
macros::make_copy_impl!(jiff::civil::DateTime);
macros::make_copy_impl!(jiff::civil::Era);
macros::make_copy_impl!(jiff::civil::ISOWeekDate);
macros::make_copy_impl!(jiff::civil::Time);
macros::make_copy_impl!(jiff::civil::Weekday);
macros::make_copy_impl!(jiff::tz::AmbiguousOffset);
macros::make_copy_impl!(jiff::tz::AmbiguousTimestamp);
macros::make_copy_impl!(jiff::tz::Dst);
macros::make_copy_impl!(jiff::tz::Offset);
macros::make_clone_impl!(jiff::Zoned);
macros::make_clone_impl!(jiff::tz::AmbiguousZoned);
macros::make_clone_impl!(jiff::tz::TimeZone);

#[cfg(test)]
mod tests {
    use super::*;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_jiff_signed_duration() {
        let value = jiff::SignedDuration::ZERO;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_span() {
        let value = jiff::Span::new();
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_span_fieldwise() {
        let value = jiff::Span::new().fieldwise();
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_timestamp() {
        let value = jiff::Timestamp::UNIX_EPOCH;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_zoned() {
        let value = jiff::Timestamp::UNIX_EPOCH.to_zoned(jiff::tz::TimeZone::UTC);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_civil_date() {
        let value = jiff::civil::Date::default();
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_civil_date_time() {
        let value = jiff::civil::DateTime::default();
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_civil_era() {
        let value = jiff::civil::Era::CE;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_civil_iso_week_date() {
        let value = jiff::civil::ISOWeekDate::MIN;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_civil_time() {
        let value = jiff::civil::Time::default();
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_civil_weekday() {
        let value = jiff::civil::Weekday::Monday;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_tz_ambiguous_offset() {
        let datetime = jiff::civil::DateTime::default();
        let ambiguous_timestamp = jiff::tz::TimeZone::UTC.to_ambiguous_timestamp(datetime);
        let value = ambiguous_timestamp.offset();
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_tz_ambiguous_timestamp() {
        let datetime = jiff::civil::DateTime::default();
        let value = jiff::tz::TimeZone::UTC.to_ambiguous_timestamp(datetime);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_tz_ambiguous_zoned() {
        let datetime = jiff::civil::DateTime::default();
        let value = jiff::tz::TimeZone::UTC.to_ambiguous_zoned(datetime);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_tz_dst() {
        let value = jiff::tz::Dst::No;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_tz_offset() {
        let value = jiff::tz::Offset::UTC;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_jiff_tz_time_zone() {
        let value = jiff::tz::TimeZone::UTC;
        let to_static = value.to_static();
        ensure_static(to_static);
    }
}
