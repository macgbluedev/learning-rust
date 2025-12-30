/* Let’s design a simple logging utility, using a trait Logger with a log method. Code that
might log its progress can then take an &impl Logger . In testing, this might put messages in
the test logfile, while in a production build it would send messages to a log server.

However, the StderrLogger given below logs all messages, regardless of verbosity. Your
task is to write a VerbosityFilter type that will ignore messages above a maximum
verbosity.

This is a common pattern: a struct wrapping a trait implementation and implementing that
same trait, adding behavior in the process. In the “Generics” segment, we will see how to
make the wrapper generic over the wrapped type.*/

/// Trait for logging messages at a given verbosity level.
trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

/// Simple logger that writes everything to `stderr`.
struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Wraps a logger and suppresses any messages that exceed `max_verbosity`.
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

///TODO
impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if(verbosity<=self.max_verbosity ){
            self.inner.log(verbosity,message);
        }
    }
}

fn main() {
    let logger = VerbosityFilter {
        max_verbosity: 3,
        inner: StderrLogger,
    };

    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}
