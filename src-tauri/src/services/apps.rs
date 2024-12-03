/* sys lib */
use std::{
	ffi::OsString,
	os::windows::ffi::OsStringExt, sync::{Arc, Mutex}, time::Duration
};

use winapi::{
	shared::{
		minwindef::DWORD,
		windef::HWND__
	},
	um::{
		processthreadsapi::OpenProcess,
		winbase::QueryFullProcessImageNameW,
		winnt::PROCESS_QUERY_INFORMATION,
		winuser::{
			EnumWindows,
			GetWindowTextA,
			GetWindowTextLengthA,
			GetWindowThreadProcessId,
			IsWindowVisible,
			SetWindowPos,
			HWND_NOTOPMOST,
			HWND_TOPMOST,
			SWP_NOMOVE,
			SWP_NOSIZE
		}
	}
};
use windows::Win32::Foundation::HWND;

/* models */
use crate::models::{
	active_app::ActiveApp,
	app_info::AppInfo,
	response::Response
};

static mut STOP_FLAG: bool = false;

fn get_process_path_from_hwnd(hwnd: *mut HWND__) -> Option<String> {
	let mut process_id: DWORD = 0;
	unsafe {
		GetWindowThreadProcessId(hwnd, &mut process_id);
	}

	if process_id != 0 {
		unsafe {
			let handle = OpenProcess(PROCESS_QUERY_INFORMATION, false as i32, process_id);
			if !handle.is_null() {
				let mut buffer: [u16; 4096] = [0; 4096];
				let mut buffer_size = buffer.len() as DWORD;
				if QueryFullProcessImageNameW(handle, 0, buffer.as_mut_ptr(), &mut buffer_size) != 0 {
					let os_str = OsString::from_wide(&buffer[..(buffer_size as usize)]);
					return Some(os_str.to_str().unwrap().to_string());
				}
			}
		}
	}

	None
}

pub fn get_list_windows() -> Response {
  let mut list_windows: Vec<AppInfo> = Vec::new();

  unsafe extern "system" fn enum_windows_proc(hwnd_mut: *mut HWND__, param: isize) -> i32 {
		let temp_list = param as *mut Vec<AppInfo>;

    if IsWindowVisible(hwnd_mut) != 0 {
			let mut info_map = AppInfo {
				title: String::new(),
        hwnd: String::new(),
        exe_file: String::new(),
				status: String::new(),
			};

			let title_length = GetWindowTextLengthA(hwnd_mut.clone());
			if title_length > 0 {
				let mut buffer = Vec::with_capacity((title_length + 1) as usize);
				GetWindowTextA(
					hwnd_mut.clone(),
					buffer.as_mut_ptr() as _,
					buffer.capacity() as _,
				);
				buffer.set_len(title_length as usize);

				info_map.title = String::from_utf8_lossy(&buffer).into_owned();
				info_map.hwnd = (hwnd_mut as usize).to_string();
				if let Some(process_path) = get_process_path_from_hwnd(hwnd_mut) {
					info_map.exe_file = process_path.clone().to_string();
				}
				info_map.status = "Running".to_string();

				(*temp_list).push(info_map);
			}
		}

		1
  }

	unsafe {
		let temp_list = &mut list_windows as *mut Vec<AppInfo>;
		EnumWindows(Some(enum_windows_proc), temp_list as isize);
	}

	Response {
		status: "success".to_string(),
		message: "".to_string(),
		data: serde_json::to_string(&list_windows).unwrap(),
	}
}

pub fn start_move(temp_hwnd: Arc<Mutex<usize>>) {
	loop {
		if unsafe { STOP_FLAG } {
			break;
		}

		let parse_temp_hwnd = *temp_hwnd.lock().unwrap() as *mut HWND__;

		if !winvd::is_window_on_current_desktop(HWND(parse_temp_hwnd as _)).unwrap() {
			let desktop = winvd::get_current_desktop().unwrap();
			let _ = winvd::move_window_to_desktop(desktop, &HWND(parse_temp_hwnd as _));
		}

		std::thread::sleep(Duration::from_secs(1 / 2));
	}
}

pub fn start_action_on_app(list_active_apps: Vec<ActiveApp>) -> Response {
	unsafe { STOP_FLAG = true; }
	std::thread::sleep(Duration::from_secs(1));
	unsafe { STOP_FLAG = false; }

	for app in list_active_apps.iter() {
		if app.hwnd != "" {
			let temp_hwnd = app.hwnd.parse::<usize>().unwrap() as *mut HWND__;

			if !temp_hwnd.is_null() {
				if unsafe { IsWindowVisible(temp_hwnd) != 0 } {
					let arc_temp_hwnd = Arc::new(Mutex::new(temp_hwnd as usize));

					if app.isMove {
						std::thread::spawn(move || {
							start_move(arc_temp_hwnd);
						});
					}

					if app.isOnTop {
						unsafe {
							let result = SetWindowPos(
								temp_hwnd,
								HWND_TOPMOST,
								0,
								0,
								0,
								0,
								SWP_NOSIZE | SWP_NOMOVE,
							);

							if result == 0 {
								eprintln!("Failed to set the window always on top!");
								return Response {
									status: "error".to_string(),
									message: "Failed to set the window always on top!".to_string(),
									data: "".to_string(),
								};
							}
						};
					} else {
						unsafe {
							let result = SetWindowPos(
								temp_hwnd,
								HWND_NOTOPMOST,
								0,
								0,
								0,
								0,
								SWP_NOSIZE | SWP_NOMOVE,
							);

							if result == 0 {
								eprintln!("Failed to set the window not always on top!");
								return Response {
									status: "error".to_string(),
									message: "Failed to set the window not always on top!".to_string(),
									data: "".to_string(),
								};
							}
						};
					}
				}
			} else {
				eprintln!("Window not found!");
				return Response {
					status: "error".to_string(),
					message: "Window not found!".to_string(),
					data: "".to_string(),
				};
			}
		}
	}

	Response {
		status: "success".to_string(),
    message: "".to_string(),
    data: "".to_string(),
	}
}