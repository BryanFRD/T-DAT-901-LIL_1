#[macro_export]
macro_rules! info {
    ($src:expr, $($arg:tt)+) => {
        $crate::logger::log(
            $crate::logger::LogType::INFO,
            Some($src),
            &format!($($arg)+)
        )
    };
    ($($arg:tt)+) => {
        $crate::logger::log(
            $crate::logger::LogType::INFO,
            None,
            &format!($($arg)+)
        )
    };
}

#[macro_export]
macro_rules! warn {
    ($src:expr, $($arg:tt)+) => {
        $crate::logger::log(
            $crate::logger::LogType::WARN,
            Some($src),
            &format!($($arg)+)
        )
    };
    ($($arg:tt)+) => {
        $crate::logger::log(
            $crate::logger::LogType::WARN,
            None,
            &format!($($arg)+)
        )
    };
}

#[macro_export]
macro_rules! error {
    ($src:expr, $($arg:tt)+) => {
        $crate::logger::log(
            $crate::logger::LogType::ERROR,
            Some($src),
            &format!($($arg)+)
        )
    };
    ($($arg:tt)+) => {
        $crate::logger::log(
            $crate::logger::LogType::ERROR,
            None,
            &format!($($arg)+)
        )
    };
}

#[macro_export]
macro_rules! debug {
    ($src:expr, $($arg:tt)+) => {
        $crate::logger::log(
            $crate::logger::LogType::DEBUG,
            Some($src),
            &format!($($arg)+)
        )
    };
    ($($arg:tt)+) => {
        $crate::logger::log(
            $crate::logger::LogType::DEBUG,
            None,
            &format!($($arg)+)
        )
    };
}
