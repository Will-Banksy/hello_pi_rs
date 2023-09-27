use std::{process::ExitCode, io::{stdin, stdout, Write}, thread, time::Duration, sync::mpsc};

use rust_gpiozero::OutputDevice;

// trait WriteState {
// 	fn write_value(&mut self, value: bool);
// }

// impl WriteState for OutputDevice {
// 	fn write_value(&mut self, value: bool) {
// 		if value {
// 			self.on();
// 		} else {
// 			self.off();
// 		}
// 	}
// }

enum Instruction {
	Terminate,
}

fn main() -> ExitCode {
	print!("Enter blink delay (ms): ");
	stdout().flush().unwrap();
	let mut buf = String::new();
	stdin().read_line(&mut buf).unwrap();
	let buf = buf.trim();
	let delay: u32 = buf.parse().unwrap_or(1000);

	println!("Delay: {} ms", delay);

    let mut led1 = OutputDevice::new(23);
	let mut led2 = OutputDevice::new(25);

	led1.on();

	let (sender, reciever) = mpsc::channel::<Instruction>();

    let thandle = thread::spawn(move || {
		loop {
			led1.toggle();
			led2.toggle();
			thread::sleep(Duration::from_millis(delay as u64));
			
			if let Ok(_instruct) = reciever.try_recv() {
				break;
			}
		}
		led1.off();
		led2.off();
	});

	print!("Press enter to continue...");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

	sender.send(Instruction::Terminate).unwrap_or_default();

	thandle.join().unwrap();

    ExitCode::SUCCESS
}
