// loading text function still in testing
async fn loading(text: &str, loaded: bool) {
    println!("");
    'textloop: loop {
        print!("\r{} /", text);
        thread::sleep(Duration::from_millis(100));
        std::io::stdout().flush().unwrap();
        print!("\r{} -", text);
        thread::sleep(Duration::from_millis(100));
        std::io::stdout().flush().unwrap();
        print!("\r{} \\", text);
        thread::sleep(Duration::from_millis(100));
        std::io::stdout().flush().unwrap();
        print!("\r{} |", text);
        thread::sleep(Duration::from_millis(100));
        std::io::stdout().flush().unwrap();
        if loaded == true {
            println!("");
            break 'textloop;
        }
    }
}
