
use macroquad::prelude::*;

const BG: Color = Color::new(0.035, 0.035, 0.045, 1.0);
const PANEL: Color = Color::new(0.075, 0.075, 0.09, 1.0);
const RED: Color = Color::new(0.92, 0.12, 0.22, 1.0);
const MUTED: Color = Color::new(0.55, 0.55, 0.61, 1.0);
const WHITE: Color = Color::new(0.94, 0.94, 0.97, 1.0);

struct ChatMessage {
    sender: String,
    body: String,
    mine: bool,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Nexured".to_owned(),
        window_width: 420,
        window_height: 820,
        high_dpi: true,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut messages = vec![
        ChatMessage {
            sender: "Nexured".into(),
            body: "Welcome to Nexured. Your conversations start here.".into(),
            mine: false,
        },
        ChatMessage {
            sender: "You".into(),
            body: "The interface is running.".into(),
            mine: true,
        },
    ];
    let mut input = String::new();
    let mut scroll = 0.0_f32;

    loop {
        clear_background(BG);

        let sw = screen_width();
        let sh = screen_height();

        draw_rectangle(0.0, 0.0, sw, 76.0, PANEL);
        draw_rectangle(0.0, 74.0, sw, 2.0, RED);
        draw_text("NEXURED", 20.0, 34.0, 25.0, WHITE);
        draw_text("MESSAGES", 21.0, 56.0, 11.0, MUTED);
        draw_circle(sw - 27.0, 31.0, 5.0, RED);

        draw_text("General chat", 20.0, 108.0, 20.0, WHITE);
        draw_text("Local interface preview", 20.0, 129.0, 12.0, MUTED);

        let input_height = 76.0;
        let bottom = sh - input_height;
        let mut y = 158.0 - scroll;

        for message in &messages {
            let max_width = (sw * 0.78).max(180.0);
            let text_width = measure_text(&message.body, None, 16, 1.0).width;
            let bubble_width = (text_width + 28.0).min(max_width);
            let lines = ((text_width / (max_width - 28.0)).ceil() as usize).max(1);
            let bubble_height = 36.0 + (lines.saturating_sub(1) as f32 * 20.0);
            let x = if message.mine {
                sw - bubble_width - 16.0
            } else {
                16.0
            };

            draw_text(
                &message.sender,
                x,
                y,
                12.0,
                if message.mine { RED } else { MUTED },
            );
            y += 8.0;
            draw_rectangle(
                x,
                y,
                bubble_width,
                bubble_height,
                if message.mine {
                    Color::new(0.25, 0.07, 0.11, 1.0)
                } else {
                    PANEL
                },
            );
            draw_rectangle(
                x,
                y,
                3.0,
                bubble_height,
                if message.mine {
                    RED
                } else {
                    Color::new(0.25, 0.25, 0.30, 1.0)
                },
            );

            let max_chars = ((max_width - 28.0) / 8.5).max(10.0) as usize;
            let mut line = String::new();
            let mut line_y = y + 23.0;
            for word in message.body.split_whitespace() {
                if !line.is_empty() && line.len() + word.len() + 1 > max_chars {
                    draw_text(&line, x + 14.0, line_y, 15.0, WHITE);
                    line.clear();
                    line_y += 20.0;
                }
                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(word);
            }
            if !line.is_empty() {
                draw_text(&line, x + 14.0, line_y, 15.0, WHITE);
            }

            y += bubble_height + 25.0;
        }

        draw_rectangle(0.0, bottom, sw, input_height, PANEL);
        draw_rectangle(0.0, bottom, sw, 1.0, Color::new(0.18, 0.18, 0.22, 1.0));
        draw_rectangle(14.0, bottom + 14.0, sw - 76.0, 46.0, BG);
        draw_text(
            if input.is_empty() {
                "Write a message..."
            } else {
                &input
            },
            26.0,
            bottom + 43.0,
            15.0,
            if input.is_empty() { MUTED } else { WHITE },
        );
        draw_circle(sw - 37.0, bottom + 37.0, 22.0, RED);
        draw_text(">", sw - 43.0, bottom + 44.0, 23.0, WHITE);

        // Keyboard input is currently intended for the desktop preview.
        // Android soft-keyboard integration will be implemented separately.
        while let Some(character) = get_char_pressed() {
            if !character.is_control() && input.chars().count() < 2000 {
                input.push(character);
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            input.pop();
        }

        if is_key_pressed(KeyCode::Enter) && !input.trim().is_empty() {
            messages.push(ChatMessage {
                sender: "You".into(),
                body: input.trim().to_string(),
                mine: true,
            });
            input.clear();
            scroll = (messages.len() as f32 * 80.0).max(0.0);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if mx > sw - 64.0 && my > bottom && my < sh && !input.trim().is_empty() {
                messages.push(ChatMessage {
                    sender: "You".into(),
                    body: input.trim().to_string(),
                    mine: true,
                });
                input.clear();
            }
        }

        if is_key_down(KeyCode::PageDown) {
            scroll += 8.0;
        }
        if is_key_down(KeyCode::PageUp) {
            scroll = (scroll - 8.0).max(0.0);
        }

        draw_line(0.0, bottom, sw, bottom, 1.0, RED);
        draw_text(
            "Nexured - early development build",
            16.0,
            sh - 4.0,
            9.0,
            MUTED,
        );

        next_frame().await;
    }
}
