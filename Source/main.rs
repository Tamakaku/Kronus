#[allow(non_snake_case)]

use std::thread;

fn main() {

	// if 

	let daemon = thread::spawn(|| {

	});

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

	daemon.join().unwrap();
}
