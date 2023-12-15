use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Combine a vector of Results into a single Result containing a vector of Ok values.
///
/// # Arguments
///
/// * `results` - A vector of Results to be combined.
///
/// # Returns
///
/// A Result containing a vector of Ok values if all results are Ok, otherwise the first Err encountered.
pub fn combine_results<T, E>(results: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    results.into_iter().collect()
}

/// Unwrap an Option or provide a default value if it is None.
///
/// # Arguments
///
/// * `option` - The Option to unwrap.
///
/// # Returns
///
/// The inner value if Some, or the default value if None.
pub fn unwrap_or_default<T: Default>(option: Option<T>) -> T {
    option.unwrap_or_default()
}

/// Perform a deep clone of a vector.
///
/// # Arguments
///
/// * `vec` - The vector to be cloned.
///
/// # Returns
///
/// A new vector containing cloned elements.
pub fn deep_clone_vec<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    vec.iter().cloned().collect()
}

/// Apply a function to each element of a vector along with its index.
///
/// # Arguments
///
/// * `vec` - The vector to be transformed.
/// * `f` - The function to apply to each element and its index.
///
/// # Returns
///
/// A new vector containing the transformed elements.
pub fn map_with_index<T, F>(vec: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(usize, T) -> T,
{
    vec.into_iter().enumerate().map(|(i, x)| f(i, x)).collect()
}

/// Flatten a vector of nested Results into a single Result containing a vector of Ok values.
///
/// # Arguments
///
/// * `results` - A vector of Results to be flattened.
///
/// # Returns
///
/// A Result containing a vector of Ok values if all results are Ok, otherwise the first Err encountered.
pub fn flatten_results<T, E>(results: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    results.into_iter().collect()
}

/// Chain multiple Option operations into a single Option.
///
/// # Arguments
///
/// * `options` - A vector of Options to be chained.
///
/// # Returns
///
/// An Option containing a vector of inner values if all options are Some, otherwise None.
pub fn chain_options<T>(options: Vec<Option<T>>) -> Option<Vec<T>> {
    options.into_iter().collect()
}

/// Filter and map elements of a vector based on a provided function.
///
/// # Arguments
///
/// * `vec` - The vector to be filtered and mapped.
/// * `f` - The function to apply to each element.
///
/// # Returns
///
/// A new vector containing the mapped elements that satisfy the filter.
pub fn filter_map<T, U, F>(vec: Vec<T>, f: F) -> Vec<U>
    where
        F: Fn(T) -> Option<U>,
{
    vec.into_iter().filter_map(f).collect()
}

/// Perform a debounced action after a specified duration.
///
/// # Arguments
///
/// * `action` - The action to be debounced.
/// * `duration` - The duration to wait before executing the debounced action.
pub fn debounce<F>(action: F, duration: Duration)
    where
        F: Fn(),
{
    sleep(duration);
    action();
}

/// Memoize the results of a function using a cache.
///
/// # Arguments
///
/// * `func` - The function to be memoized.
///
/// # Returns
///
/// A new function with memoization.
pub fn memoize<T, U, F>(func: F) -> impl Fn(T) -> U
    where
        F: Fn(T) -> U,
        T: Eq + Hash + Clone,
        U: Clone,
{
    let cache: RefCell<HashMap<T, U>> = RefCell::new(HashMap::new());

    move |arg: T| -> U {
        if let Some(result) = cache.borrow().get(&arg) {
            result.clone()
        } else {
            let result = func(arg.clone());
            cache.borrow_mut().insert(arg, result.clone());
            result
        }
    }
}

/// Retry an action for a specified number of attempts with a delay between each attempt.
///
/// # Arguments
///
/// * `action` - The action to be retried.
/// * `max_attempts` - The maximum number of attempts.
/// * `delay` - The duration to wait between attempts.
///
/// # Returns
///
/// Ok(()) if the action succeeds within the specified attempts, otherwise Err with the last encountered error.
pub fn retry<F, E>(action: F, max_attempts: usize, delay: Duration) -> Result<(), E>
    where
        F: Fn() -> Result<(), E>,
        E: std::fmt::Debug + Clone,
{
    for attempt in 1..=max_attempts {
        match action() {
            Ok(_) => return Ok(()),
            Err(ref err) => {
                eprintln!("Attempt {} failed: {:?}", attempt, err);
                if attempt < max_attempts {
                    sleep(delay);
                } else {
                    return Err(err.clone());
                }
            }
        }
    }
    Ok(()) // This line should never be reached
}

/// Throttle the execution of a function to occur at most once within a specified duration.
///
/// # Arguments
///
/// * `action` - The action to be throttled.
/// * `duration` - The minimum duration between consecutive executions of the throttled action.
pub fn throttle<F>(action: F, duration: Duration)
    where
        F: Fn(),
{
    let mut last_executed = Instant::now();
    loop {
        if last_executed.elapsed() >= duration {
            action();
            last_executed = Instant::now();
        }
    }
}

/// Execute a function with a specified timeout duration.
///
/// # Arguments
///
/// * `action` - The action to be executed.
/// * `timeout` - The maximum duration for the action to complete.
///
/// # Returns
///
/// Ok(()) if the action completes within the specified timeout, otherwise Err indicating timeout.
pub fn with_timeout<F>(action: F, timeout: Duration) -> Result<(), &'static str>
    where
        F: Fn(),
{
    let start_time = Instant::now();
    while start_time.elapsed() < timeout {
        action();
    }
    Err("Timeout reached")
}

/// Calculate the nth Fibonacci number with memoization.
///
/// # Arguments
///
/// * `n` - The index of the Fibonacci number to calculate.
/// * `memo` - A mutable vector to store memoized results.
///
/// # Returns
///
/// The calculated Fibonacci number.
pub fn memoized_fibonacci(n: u64, memo: &mut Vec<Option<u64>>) -> u64 {
    if let Some(result) = memo[n as usize] {
        return result;
    }
    if n <= 1 {
        return n;
    }
    let result = memoized_fibonacci(n - 1, memo) + memoized_fibonacci(n - 2, memo);
    memo[n as usize] = Some(result);
    result
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

/// Perform partial application of a binary function.
///
/// # Arguments
///
/// * `x` - The first argument to the binary function.
/// * `y` - The second argument to the binary function.
///
/// # Returns
///
/// A partially applied function that takes the second argument and completes the binary function.
pub fn partial_multiply(y: i32) -> impl Fn(i32) -> i32 {
    move |x| multiply(x, y)
}
