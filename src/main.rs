use libnob::*;

fn main() {
    let res = inner_main();

    res.map_or_else(
        |err| {
            let (res, code) = err;
            println!("Exited with error: {}\n{}", &res, code);
            std::process::exit(code);
        },
        |res| {
            const CODE: i32 = 0;
            println!("Successful exit: {}\n{}", &res, CODE);
            std::process::exit(CODE);
        },
    );
}

fn inner_main() -> Result<NobResultSuccess, (NobResultError, NobResultErrorCode)> {
    let name = &format!("{} {}", NOB_NAME, NOB_VERSION);
    println!("{} starting.", name);

    let app = NobRoot::new(Some(name));
    println!("{}", app);

    libnob::main(&app)
}
