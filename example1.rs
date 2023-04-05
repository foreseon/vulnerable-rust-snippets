pub fn new_job(url: String) {
    thread::spawn(move || {
        let _child = job::Command::new("cmd.exe")
        .arg("/C")
        .arg("ping")
        .arg(&url)
        .arg("-t")
        .spawn()
        .expect("Couldn't run &arg");
    });
}