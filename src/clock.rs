//! The wall clock in local time: the system clock (set by chrony) in the time zone of the system
//! (`persist.sys.timezone`, which bionic's localtime_r reads with the platform's tzdata).
//! Android's chrono build has no clock feature (no chrono::Local).

/// What the screen shows of the time.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Clock {
    /// "10:33" (#template/time)
    pub time: String,
}

/// The local time now.
pub fn now() -> Clock {
    // SAFETY: time(NULL) only reads the clock; localtime_r writes into the tm we own and
    // returns NULL on failure, which leaves the default (empty) clock.
    unsafe {
        let t = libc::time(std::ptr::null_mut());
        let mut tm: libc::tm = std::mem::zeroed();
        if libc::localtime_r(&t, &mut tm).is_null() {
            return Clock::default();
        }
        format(tm.tm_hour, tm.tm_min)
    }
}

/// The text from the fields of a `struct tm`.
fn format(hour: i32, min: i32) -> Clock {
    Clock { time: format!("{hour:02}:{min:02}") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_text_of_a_time() {
        assert_eq!(format(9, 5).time, "09:05");
    }
}
