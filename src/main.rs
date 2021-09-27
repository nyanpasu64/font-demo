use gtk::prelude::*;
use gtk::gdk;

fn main() {
	let app = gtk::Application::builder()
		.application_id("font-demo")
		.build();
	
	app.connect_activate(|app| {
		let window = gtk::ApplicationWindow::builder()
			.application(app)
			.title("GTK4 Font Demo")
			.default_width(1200)
			.default_height(800)
			.build();
		
		let provider = gtk::CssProvider::new();
		provider.load_from_data(include_bytes!("stylesheet.css"));
		
		gtk::StyleContext::add_provider_for_display(
			&gdk::Display::default().unwrap(),
			&provider,
			gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);
		
		let layout_box = gtk::Box::builder()
			.orientation(gtk::Orientation::Vertical)
			.build();
		
		let scroller = gtk::ScrolledWindow::builder()
			.vexpand(true)
			.build();
		
		let rows = gtk::Box::builder()
			.orientation(gtk::Orientation::Vertical)
			.build();
		
		for row in 0..10 {
			let row_box = gtk::Box::builder()
				.orientation(gtk::Orientation::Horizontal)
				.css_classes(match row % 3 {
					1 => vec!["dark".to_string()],
					2 => vec!["colored".to_string()],
					_ => vec!["light".to_string()],
				})
				.build();
			
			for column in 0..10 {
				let column_box = gtk::Box::builder()
					.orientation(gtk::Orientation::Vertical)
					.css_classes(match (row + column) % 4 {
						1 => vec!["column-box".to_string(), "bold".to_string()],
						2 => vec!["column-box".to_string(), "italic".to_string()],
						3 => vec!["column-box".to_string(), "bold".to_string(), "italic".to_string()],
						_ => vec!["column-box".to_string(), ],
					})
					.build();
				
				let title = gtk::Label::builder()
					.label("Font Demo")
					.build();
				
				let contents = gtk::Label::builder()
					.label(concat!(
						"This is a test for GTK4's font rendering.\n\n",
						"This is a test for GTK4's font rendering.\n\n",
						"iiiiiiiiiiiiiiiiiiii\n\n",
						"bbbbbbbbbbbbbbbbbbbb\n\n",
						"gggggggggggggggggggg\n\n",
						"This is a test for GTK4's font rendering. ",
						"This is a test for GTK4's font rendering. ",
						"This is a test for GTK4's font rendering. ",
						"This is a test for GTK4's font rendering.\n\n",
						"😀️ ❤️ 🐈‍⬛️",
					))
					.wrap(true)
					.build();
				
				column_box.append(&title);
				column_box.append(&contents);
				row_box.append(&column_box);
			}
			
			rows.append(&row_box);
		}
		
		scroller.set_child(Some(&rows));
		layout_box.append(&scroller);
		
		let controls_box = gtk::Box::builder()
			.orientation(gtk::Orientation::Horizontal)
			.build();
		
		let rotate_checkbox = gtk::CheckButton::builder()
			.label("Rotate")
			.build();
		
		rotate_checkbox.connect_toggled(|checkbox| {
			let layout_box = checkbox.parent().unwrap().parent().unwrap();
			
			if checkbox.is_active() {
				if !layout_box.has_css_class("rotate") {
					layout_box.add_css_class("rotate");
				}
			} else {
				if layout_box.has_css_class("rotate") {
					layout_box.remove_css_class("rotate");
				}
			}
		});
		
		controls_box.append(&rotate_checkbox);
		
		let scale_checkbox = gtk::CheckButton::builder()
			.label("Scale")
			.build();
		
		scale_checkbox.connect_toggled(|checkbox| {
			let layout_box = checkbox.parent().unwrap().parent().unwrap();
			
			if checkbox.is_active() {
				if !layout_box.has_css_class("scale") {
					layout_box.add_css_class("scale");
				}
			} else {
				if layout_box.has_css_class("scale") {
					layout_box.remove_css_class("scale");
				}
			}
		});
		
		controls_box.append(&scale_checkbox);
		
		let bounce_checkbox = gtk::CheckButton::builder()
			.label("Bouncy Text")
			.build();
		
		bounce_checkbox.connect_toggled(|checkbox| {
			let layout_box = checkbox.parent().unwrap().parent().unwrap();
			
			if checkbox.is_active() {
				if !layout_box.has_css_class("bounce") {
					layout_box.add_css_class("bounce");
				}
			} else {
				if layout_box.has_css_class("bounce") {
					layout_box.remove_css_class("bounce");
				}
			}
		});
		
		controls_box.append(&bounce_checkbox);
		
		layout_box.append(&controls_box);
		window.set_child(Some(&layout_box));
		
		window.present();
	});
	
	app.run();
}
