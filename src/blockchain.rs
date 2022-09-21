use spin_sleep::LoopHelper;

const INTERVAL_IN_SECONDS: 	f64 = 2.0;
const RATE_PER_SECOND: 		f64 = 1.0 / INTERVAL_IN_SECONDS;

pub fn run() {

	let mut loop_helper = LoopHelper::builder().build_with_target_rate(RATE_PER_SECOND);


	println!("Blockchain node started");

	loop {
		loop_helper.loop_start(); // or .loop_start_s() for f64 seconds

    	loop_helper.loop_sleep(); 
		eprint!(".");
	}
}
