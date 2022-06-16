use tauri::{Menu, MenuItem, Submenu};

pub fn main_menu() -> Menu {
	let menu_app = Menu::new()
		.add_native_item(MenuItem::Quit);

	let menu_edit = Menu::new()
		.add_native_item(MenuItem::Undo)
		.add_native_item(MenuItem::Redo)
		.add_native_item(MenuItem::Separator)
		.add_native_item(MenuItem::Cut)
		.add_native_item(MenuItem::Copy)
		.add_native_item(MenuItem::Paste)
		.add_native_item(MenuItem::SelectAll);

	let menu_view = Menu::new()
		.add_native_item(MenuItem::EnterFullScreen);

	let menu_window = Menu::new()
		.add_native_item(MenuItem::Minimize)
		.add_native_item(MenuItem::Zoom);

	Menu::new()
		.add_submenu(Submenu::new("App", menu_app))
		.add_submenu(Submenu::new("Edit", menu_edit))
		.add_submenu(Submenu::new("View", menu_view))
		.add_submenu(Submenu::new("Window", menu_window))
}
