use pgx::pg_sys::int32;
use pgx::prelude::*;
use pgx::*;
use pgx::{FromDatum, IntoDatum, PgOid};
use pgx::bgworkers::*;
use pgx::prelude::*;
use pgx::spi::SpiError;
use std::time::Duration;
use pgx::numeric::Error;
pgx::pg_module_magic!();

#[pg_extern]
fn hello_hello_world1() -> &'static str {
    "Hello, hello_world1"
}

#[pg_extern]
fn hello_add_two(a: int32, b: int32) -> int32 {
    a + b
}

#[pg_extern]
fn hello_do_stuff(a: int32, b: String) -> String {
    println!("received a as {} and b as {}", a, b);
    let ts = Spi::get_one::<int32>("SELECT 42;").expect("to find 42");
    info!("result is {}", ts);
    // pgx::bgworkers::BackgroundWorker::
    format!("{} and {}", a, b)
}

#[pg_guard]
pub extern "C" fn _PG_init() {
    BackgroundWorkerBuilder::new("Background Worker Example")
        .set_function("background_worker_main")
        .set_library("hello_world1")
        .set_argument(42i32.into_datum())
        .enable_spi_access()
        .load();
}

#[pg_guard]
#[no_mangle]
pub extern "C" fn background_worker_main(arg: pg_sys::Datum) {
    let arg = unsafe { i32::from_polymorphic_datum(arg, false, pg_sys::INT4OID) };

    // these are the signals we want to receive.  If we don't attach the SIGTERM handler, then
    // we'll never be able to exit via an external notification
    BackgroundWorker::attach_signal_handlers(SignalWakeFlags::SIGHUP | SignalWakeFlags::SIGTERM);

    // we want to be able to use SPI against the specified database (postgres), as the superuser which
    // did the initdb. You can specify a specific user with Some("my_user")
    BackgroundWorker::connect_worker_to_spi(Some("postgres"), None);

    log!(
        "Hello from inside the {} BGWorker!  Argument value={}",
        BackgroundWorker::get_name(),
        arg.unwrap()
    );

    // wake up every 10s or if we received a SIGTERM
    while BackgroundWorker::wait_latch(Some(Duration::from_secs(10))) {
        if BackgroundWorker::sighup_received() {
            // on SIGHUP, you might want to reload some external configuration or something
        }

        // within a transaction, execute an SQL statement, and log its results
        let result: Result<(), SpiError> = BackgroundWorker::transaction(|| {
            let res1 = Spi::connect(|client| {
                let tuple_table = client.select(
                    "SELECT 'Hi', id, ''||a FROM (SELECT id, 42 from generate_series(1,10) id) a ",
                    None,
                    None,
                );
                for tuple in tuple_table {
                    let a = tuple.by_ordinal(1)?.value::<String>().expect("ds");
                    let b = tuple.by_ordinal(2)?.value::<i32>().expect("ds");
                    let c = tuple.by_ordinal(3)?.value::<String>().expect("dsa");
                    log!("from bgworker: ({:?}, {:?}, {:?})", a, b, c);
                }
               Ok(Some(Ok(())))
                // Result::<Option<<int32>,_>(Ok(Some(32)))
            });
            res1.expect("dsd")
        });
        result.unwrap_or_else(|e| panic!("got an error: {:?}", e))
    }

    log!("Goodbye from inside the {} BGWorker! ", BackgroundWorker::get_name());
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_hello_world1() {
        assert_eq!("Hello, hello_world1", crate::hello_hello_world1());
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
