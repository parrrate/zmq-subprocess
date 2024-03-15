fn main(socket: zmq::Socket) -> zmq::Result<()> {
    loop {
        let msg = socket.recv_bytes(0)?;
        if msg.is_empty() {
            break;
        }
        socket.send(msg, 0)?;
    }
    Ok(())
}

zmq_subprocess::zsp!(main);
