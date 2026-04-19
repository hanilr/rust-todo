use std::io::Write;
use colored::*;

use crate::todo::*;

macro_rules! printf {
    ($($arg:tt)*) => {
        print!($($arg)*);
        std::io::stdout().flush().unwrap();
    };
}

macro_rules! gotoxy {
    ($y:expr, $x:expr) => {
        print!("\x1B[{};{}H", $y, $x);
    };
}

pub struct UI {
    width: u8,
    height: u8,
    pos_x: u8,
    pos_y: u8,
}

impl UI {
    pub fn new(width: u8, height: u8, pos_x: u8, pos_y: u8) -> UI {
        UI {
            width: width,
            height: height,
            pos_x: pos_x,
            pos_y: pos_y,
        }
    }

    pub fn clone(ui: &UI) -> UI {
        UI {
            width: ui.width,
            height: ui.height,
            pos_x: ui.pos_x,
            pos_y: ui.pos_y,
        }
    }
}

pub struct Frame {
    horizontal_symbol: String,
    vertical_symbol: String,
    horizontal_color: String,
    vertical_color: String,
    horizontal_on_color: String,
    vertical_on_color: String,
    background_on_color: String,
}

impl Frame {
    pub fn new(horizontal_symbol: &str, vertical_symbol: &str,
        horizontal_color: &str, vertical_color: &str,
        horizontal_on_color: &str, vertical_on_color: &str,
        background_on_color: &str) -> Frame {
        Frame {
            horizontal_symbol: horizontal_symbol.to_string(),
            vertical_symbol: vertical_symbol.to_string(),
            horizontal_color: horizontal_color.to_string(),
            vertical_color: vertical_color.to_string(),
            horizontal_on_color: horizontal_on_color.to_string(),
            vertical_on_color: vertical_on_color.to_string(),
            background_on_color: background_on_color.to_string(),
        }
    }

    pub fn draw(&self, ui: UI) {
        gotoxy!(ui.pos_y + 1, ui.pos_x + 1);

        for i in 0..ui.height {
            gotoxy!(ui.pos_y + 1 + i, ui.pos_x + 1);
            printf!("{}", self.vertical_symbol
                .color(self.vertical_color.to_string())
                .on_color(self.vertical_on_color.to_string())
            );

            if i == 0 || i == ui.height - 1 {
                for i in 0..ui.width - 2 {
                    printf!("{}", self.horizontal_symbol
                        .color(self.horizontal_color.to_string())
                        .on_color(self.horizontal_on_color.to_string())
                    );
                }
            } else {
                for i in 0..ui.width - 2 {
                    printf!("{}", " ".on_color(self.background_on_color.to_string()));
                }
            }

            printf!("{}\n", self.vertical_symbol
                .color(self.vertical_color.to_string())
                .on_color(self.vertical_on_color.to_string())
            );
        }
    }
}

pub struct ListWidget {
    name: String,
    task_id: u8,
    task_title: String,
    task_description: String,
    task_is_complete: bool,
    list: List,
}

impl ListWidget {
    pub fn new(list: List, name: &str, task_id: u8, task_title: &str, task_description: &str, task_is_complete: bool) -> ListWidget {
        ListWidget {
            name: name.to_string(),
            task_id: task_id,
            task_title: task_title.to_string(),
            task_description: task_description.to_string(),
            task_is_complete: task_is_complete,
            list: list,
        }
    }

    pub fn draw(&self, ui: UI, frame: Frame, list: List, text_color: &str, text_on_color: &str) {
        let list_frame = Frame::new(&frame.horizontal_symbol, &frame.vertical_symbol,
            &frame.horizontal_color, &frame.vertical_color, &frame.horizontal_on_color,
            &frame.vertical_on_color,&frame.vertical_color);

        let list_ui = UI::clone(&ui);
        list_frame.draw(ui);

        let mut counter = 0;
        for i in list.tasks {
            gotoxy!(list_ui.pos_y + 2 + counter, list_ui.pos_x + 2);
            printf!("{}{}{}{}{}{}{}{}",
                "ID: ".to_string().color(text_color.to_string()).on_color(text_on_color.to_string()),
                i.id.to_string().color(text_color.to_string()).on_color(text_on_color.to_string()),

                ", Title: ".to_string().color(text_color.to_string()).on_color(text_on_color.to_string()),
                i.title.color(text_color.to_string()).on_color(text_on_color.to_string()),

                ", Description: ".to_string().color(text_color.to_string()).on_color(text_on_color.to_string()),
                i.description.color(text_color.to_string()).on_color(text_on_color.to_string()),

                ", Is Complete: ".to_string().color(text_color.to_string()).on_color(text_on_color.to_string()),
                i.is_complete.to_string().color(text_color.to_string()).on_color(text_on_color.to_string())
            );
            counter += 1;
        }
    }
}