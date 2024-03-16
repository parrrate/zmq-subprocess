use std::ffi::{c_int, c_void};

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
enum ErrorImpl {
    #[error(transparent)]
    Lib(#[from] libloading::Error),
    #[error("-1 returned (zmq error)")]
    ZmqInner,
    #[error("-2 returned (panic)")]
    Panic,
    #[error("{0} returned (unknown)")]
    Unknown(c_int),
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] ErrorImpl);

pub type Result = std::result::Result<(), Error>;

pub fn run(name: &str, socket: zmq::Socket) -> Result {
    let inner = || unsafe {
        let lib = libloading::Library::new(libloading::library_filename(name))?;
        let main: libloading::Symbol<unsafe extern "C" fn(*mut c_void) -> c_int> =
            lib.get(b"zmq_subproces_0_0_1")?;
        match main(socket.into_raw()) {
            0 => {
                lib.close()?;
                Ok(())
            }
            -1 => Err(ErrorImpl::ZmqInner),
            -2 => Err(ErrorImpl::Panic),
            unknown => Err(ErrorImpl::Unknown(unknown)),
        }
    };
    inner()?;
    Ok(())
}
