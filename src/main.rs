use std::{process::ExitCode, io::{stdin, stdout, Write}, thread, time::Duration, sync::mpsc};

use rust_gpiozero::OutputDevice;

enum Instruction {
	Terminate,
}

fn main() -> ExitCode {
    // let mut led = LED::new(23);

    // led.blink(1.0, 1.0);

    // print!("Press enter to continue...");
    // stdout().flush().unwrap();
    // let mut buf = String::new();
    // stdin().read_line(&mut buf).unwrap();

    // led.off();

    let mut led = OutputDevice::new(23);

	let (sender, reciever) = mpsc::channel::<Instruction>();

    let thandle = thread::spawn(move || {
		loop {
			led.on();
			thread::sleep(Duration::from_secs(1));
			
			if let Ok(_instruct) = reciever.try_recv() {
				break;
			}
			
			led.off();
			thread::sleep(Duration::from_secs(1));
			
			if let Ok(_instruct) = reciever.try_recv() {
				break;
			}
		}
		led.off();
	});

	print!("Press enter to continue...");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

	sender.send(Instruction::Terminate).unwrap_or_default();

	thandle.join().unwrap();

    ExitCode::SUCCESS
}
