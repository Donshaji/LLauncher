use std::cell::Cell;
use std::process::{Command, Output};
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation, ScrolledWindow};

fn show_error(app: &Application, parent: &ApplicationWindow, message: &str) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Action unavailable")
        .transient_for(parent)
        .modal(true)
        .default_width(360)
        .default_height(160)
        .build();

    let content = GtkBox::new(Orientation::Vertical, 12);
    content.set_margin_top(16);
    content.set_margin_bottom(16);
    content.set_margin_start(16);
    content.set_margin_end(16);

    let label = Label::new(Some(message));
    label.set_wrap(true);
    content.append(&label);

    let close = Button::with_label("Close");
    let error_window = window.clone();
    close.connect_clicked(move |_| error_window.close());
    content.append(&close);

    window.set_child(Some(&content));
    window.present();
}

fn run_command(
    app: &Application,
    parent: &ApplicationWindow,
    program: &str,
    args: &[&str],
) -> Option<Output> {
    match Command::new(program).args(args).output() {
        Ok(output) if output.status.success() => Some(output),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            show_error(
                app,
                parent,
                &format!(
                    "{program} failed: {}",
                    if stderr.trim().is_empty() {
                        format!("exit status {}", output.status)
                    } else {
                        stderr.trim().to_owned()
                    }
                ),
            );
            None
        }
        Err(error) => {
            show_error(
                app,
                parent,
                &format!("Could not run {program}: {error}"),
            );
            None
        }
    }
}

fn main() {
    let app = Application::builder()
        .application_id("dev.llauncher.LLauncher")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("LLauncher")
            .default_width(432)
            .default_height(900)
            .build();

        let layout = GtkBox::new(Orientation::Vertical, 0);
        let home = Label::new(Some("LLauncher is ready"));
        home.set_vexpand(true);
        layout.append(&home);

        let navigation = GtkBox::new(Orientation::Horizontal, 4);
        navigation.set_homogeneous(true);
        navigation.set_margin_top(8);
        navigation.set_margin_bottom(8);
        navigation.set_margin_start(8);
        navigation.set_margin_end(8);

        let recent = Button::with_label("Recent\nApps");
        recent.set_tooltip_text(Some("Recent applications"));
        let home_button = Button::with_label("Home");
        let back = Button::with_label("Back");
        let rotate = Button::with_label("Rotate");
        let close = Button::with_label("Close\nApp");
        close.set_tooltip_text(Some("Close current application"));

        for button in [&recent, &home_button, &back, &rotate, &close] {
            button.set_hexpand(true);
            button.set_vexpand(true);
            navigation.append(button);
        }
        layout.append(&navigation);
        window.set_child(Some(&layout));

        let last_app = Rc::new(Cell::new(None::<String>));

        let recent_app = app.clone();
        let recent_parent = window.clone();
        let recent_target = last_app.clone();
        recent.connect_clicked(move |_| {
            let Some(output) = run_command(&recent_app, &recent_parent, "wmctrl", &["-l"]) else {
                return;
            };

            let recent_window = ApplicationWindow::builder()
                .application(&recent_app)
                .title("Recent applications")
                .transient_for(&recent_parent)
                .default_width(360)
                .default_height(480)
                .build();
            let list = GtkBox::new(Orientation::Vertical, 6);
            list.set_margin_top(12);
            list.set_margin_bottom(12);
            list.set_margin_start(12);
            list.set_margin_end(12);

            let windows = String::from_utf8_lossy(&output.stdout);
            let mut found_window = false;
            for line in windows.lines() {
                let mut fields = line.split_whitespace();
                let Some(id) = fields.next() else { continue };
                let (Some(_desktop), Some(_host)) = (fields.next(), fields.next()) else {
                    continue;
                };
                let title = fields.collect::<Vec<_>>().join(" ");
                if title.is_empty() || title == "LLauncher" {
                    continue;
                }

                found_window = true;
                let app_button = Button::with_label(&title);
                let target = recent_target.clone();
                let app = recent_app.clone();
                let parent = recent_parent.clone();
                let popup = recent_window.clone();
                let id = id.to_owned();
                app_button.connect_clicked(move |_| {
                    if run_command(&app, &parent, "wmctrl", &["-ia", &id]).is_some() {
                        target.set(Some(id.clone()));
                        popup.close();
                    }
                });
                list.append(&app_button);
            }

            if !found_window {
                list.append(&Label::new(Some("No other application windows found.")));
            }

            let scroll = ScrolledWindow::builder()
                .vexpand(true)
                .child(&list)
                .build();
            recent_window.set_child(Some(&scroll));
            recent_window.present();
        });

        let home_window = window.clone();
        home_button.connect_clicked(move |_| home_window.present());

        let back_app = app.clone();
        let back_parent = window.clone();
        let back_target = last_app.clone();
        back.connect_clicked(move |_| {
            let Some(id) = back_target.take() else {
                show_error(
                    &back_app,
                    &back_parent,
                    "Choose an application from Recent Apps before using Back.",
                );
                return;
            };
            if run_command(&back_app, &back_parent, "wmctrl", &["-ia", &id]).is_some() {
                let _ = run_command(
                    &back_app,
                    &back_parent,
                    "xdotool",
                    &["key", "--clearmodifiers", "Escape"],
                );
            }
            back_target.set(Some(id));
        });

        let rotate_app = app.clone();
        let rotate_parent = window.clone();
        let rotate_left = Rc::new(Cell::new(true));
        let rotation_direction = rotate_left.clone();
        rotate.connect_clicked(move |_| {
            let Some(output) = run_command(&rotate_app, &rotate_parent, "xrandr", &["--query"])
            else {
                return;
            };
            let output_name = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains(" connected "))
                .and_then(|line| line.split_whitespace().next())
                .map(str::to_owned);
            let Some(output_name) = output_name else {
                show_error(&rotate_app, &rotate_parent, "No connected display was found.");
                return;
            };

            let direction = if rotation_direction.get() {
                "left"
            } else {
                "right"
            };
            if run_command(
                &rotate_app,
                &rotate_parent,
                "xrandr",
                &["--output", &output_name, "--rotate", direction],
            )
            .is_some()
            {
                rotation_direction.set(!rotation_direction.get());
            }
        });

        let close_app = app.clone();
        let close_parent = window.clone();
        let close_target = last_app.clone();
        close.connect_clicked(move |_| {
            let Some(id) = close_target.take() else {
                show_error(
                    &close_app,
                    &close_parent,
                    "Choose an application from Recent Apps before closing it.",
                );
                return;
            };
            if run_command(&close_app, &close_parent, "wmctrl", &["-ic", &id]).is_none() {
                close_target.set(Some(id));
            }
        });

        window.present();
    });

    app.run();
}
