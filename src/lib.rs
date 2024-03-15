#[macro_export]
macro_rules! zsp {
    ($main:expr) => {
        #[no_mangle]
        unsafe extern "C" fn zmq_subproces_0_0_1(socket: *mut std::ffi::c_void) -> std::ffi::c_int {
            let main: fn(zmq::Socket) -> zmq::Result<()> = main;
            let socket = zmq::Socket::from_raw(socket);
            match std::panic::catch_unwind(|| main(socket)) {
                Ok(Ok(())) => 0,
                Ok(Err(_)) => -1,
                Err(_) => -2,
            }
        }
    };
}
