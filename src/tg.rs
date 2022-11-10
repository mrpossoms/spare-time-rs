use termios::*;
use std::io::*;

pub fn game_settings() -> Termios
{
	let stdin_fd = 0; // TODO: how can I get it from here std::io::stdin()
	// let stdout_fd = 0;

	let mut settings = Termios::from_fd(stdin_fd).unwrap();
	let old_settings = settings;

	// tcgetattr(STDIN_FILENO, old_settings);
	settings.c_lflag &= !ECHO;
	settings.c_lflag &= !ICANON;
	tcsetattr(stdin_fd, TCSANOW, &mut settings).expect("Could not apply terminal settings");

	// Hide cursor
	// fputs("\033[?25l", stdout_fd);
	std::io::stdout().lock().write(b"\033[?25l").expect("should write");

	old_settings
}

pub fn restore_settings(old_settings: &mut Termios)
{
	let stdin_fd = 0;
	tcsetattr(stdin_fd, TCSANOW, old_settings).expect("Could not apply terminal settings");
	// fputs("\033[?25h", stderr);
	std::io::stdout().lock().write(b"\033[?25h").expect("should write");
}

pub fn term_width() -> i32
{
	let info = terminfo::Database::from_env().unwrap();
	let cols : i32 = info.get::<terminfo::capability::Columns>().unwrap().into();

	return cols;
}

/**
 * @brief      Get the current height in rows of the terminal
 *
 * @return     height in rows, -1 if the value cannot be retrieved
 */
pub fn term_height() -> i32
{
	let info = terminfo::Database::from_env().unwrap();
	let rows : i32 = info.get::<terminfo::capability::Lines>().unwrap().into();

	return rows;
}

pub fn get_key() -> i32
{
	let mut key_buf : [u8;1] = [0];


	let mut read_fds = nix::sys::select::FdSet::new();
	let mut time_out = nix::sys::time::TimeVal::new(1, 0);//nix::sys::time::TimeValLike::seconds(1);
	// let mut time_out : nix::sys::time::TimeVal = nix::sys::time::TimeValLike::microseconds(33333);
	read_fds.insert(0);
	let _ = nix::sys::select::select(1, &mut read_fds, None, None, &mut time_out);
	for fd in read_fds.fds(None) {
		if fd == 0 {
			println!("stdin");
			let _ = std::io::stdin().read(&mut key_buf[..]);

			let remaining_us = time_out.tv_usec();
			let duration = std::time::Duration::from_micros(remaining_us.try_into().unwrap());
			// std::thread::sleep(duration);

			return key_buf[0].into();
		}
	}

	-1
}