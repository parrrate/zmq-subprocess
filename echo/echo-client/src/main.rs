fn main() {
    let ctx = zmq::Context::new();
    let host = ctx.socket(zmq::PAIR).unwrap();
    host.bind("inproc://echo").unwrap();
    let endpoint = &host.get_last_endpoint().unwrap().unwrap();
    let parasite = ctx.socket(zmq::PAIR).unwrap();
    parasite.connect(endpoint).unwrap();
    let r = std::thread::scope(|scope| {
        let handle = std::thread::Builder::new()
            .name("echozsp".into())
            .stack_size(32 * 1024)
            .spawn_scoped(scope, || zmq_subprocess_client::run("echozsp", parasite))
            .unwrap();
        let stdin = std::io::stdin();
        loop {
            let mut buffer = String::new();
            match stdin.read_line(&mut buffer) {
                Ok(_) => {
                    if buffer.is_empty() {
                        break;
                    }
                    if host.send(&buffer, 0).is_err() {
                        break;
                    }
                    let Ok(Ok(s)) = host.recv_string(0) else {
                        break;
                    };
                    print!("{s}");
                }
                Err(_) => break,
            }
        }
        let _ = host.send("", 0);
        handle.join()
    });
    eprintln!("{r:?}");
}
