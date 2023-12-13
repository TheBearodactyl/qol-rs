/// Print a formatted message with file name and line number for debugging.
///
/// # Example
///
/// ```rust
/// printl!("This is a debug message: {}", variable);
/// ```
#[macro_export]
macro_rules! printl {
    ($($arg:tt)*) => {
        println!("[{}:{}]\t{}", file!(), line!(), format_args!($($arg)*));
    };
}

/// Return an error with a formatted message.
///
/// # Example
///
/// ```rust
/// error!("Failed to perform operation");
/// ```
#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        return Err(format!("{}", $msg).into());
    };
}

/// Create a vector with given elements.
///
/// # Example
///
/// ```rust
/// let my_vec = vec_of![1, 2, 3];
/// ```
#[macro_export]
macro_rules! vec_of {
    ($($x:expr),*) => (vec![$($x),*]);
}

/// Measure the time taken by a code block to execute.
///
/// # Example
///
/// ```rust
/// let (result, elapsed_time) = measure_time!({
///     // code to be measured
/// });
/// ```
#[macro_export]
macro_rules! measure_time {
    ($code:expr) => {{
        let start_time = std::time::Instant::now();
        let result = $code;
        let elapsed = start_time.elapsed();
        (result, elapsed)
    }};
}

/// Unwrap an Option or return early with an error message.
///
/// # Example
///
/// ```rust
/// let result = some_option.unwrap_or_return("Option is None");
/// ```
#[macro_export]
macro_rules! unwrap_or_return {
    ($result:expr, $msg:expr) => {
        match $result {
            Some(value) => value,
            None => {
                eprintln!("{}", $msg);
                return;
            }
        }
    };
}

/// Match an enum variant and execute corresponding code.
///
/// # Example
///
/// ```rust
/// match_enum!(my_enum,
///     Variant1 => {
///         // code for Variant1
///     },
///     Variant2 => {
///         // code for Variant2
///     }
/// );
/// ```
#[macro_export]
macro_rules! match_enum {
    ($enum:expr, $($variant:ident => $code:block),*) => {
        match $enum {
            $(Enum::$variant => $code),*
        }
    };
}

/// Lock a mutex and execute a code block.
///
/// # Example
///
/// ```rust
/// lock!(my_mutex, {
///     // code inside the locked section
/// });
/// ```
#[macro_export]
macro_rules! lock {
    ($lock:expr, $code:block) => {{
        let _lock = $lock.lock().unwrap();
        $code
    }};
}

/// Unwrap a Result or return early with a custom error message.
///
/// # Example
///
/// ```rust
/// let result = some_result.unwrap_or_return("Failed to get result");
/// ```
#[macro_export]
macro_rules! custom_result {
    ($result:expr, $msg:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Error: {}: {}", $msg, err);
                return;
            }
        }
    };
}

/// Repeat a code block a specified number of times.
///
/// # Example
///
/// ```rust
/// repeat_n!(5, {
///     // code to be repeated 5 times
/// });
/// ```
#[macro_export]
macro_rules! repeat_n {
    ($n:expr, $code:block) => {
        (0..$n).for_each(|_| $code);
    };
}

/// Log a message and return early from the current function.
///
/// # Example
///
/// ```rust
/// log_and_return!("Error: Something went wrong");
/// ```
#[macro_export]
macro_rules! log_and_return {
    ($msg:expr) => {{
        eprintln!("{}", $msg);
        return;
    }};
}

/// Concatenate multiple strings into an HTML-formatted string.
///
/// # Example
///
/// ```rust
/// let html_string = html!("<p>", "This is", "an example", "</p>");
/// ```
#[macro_export]
macro_rules! html {
    ($($content:expr),*) => {
        format!($("<html>{}</html>")+, $($content),*);
    };
}
