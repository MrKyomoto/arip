use std::fs;
use std::path::PathBuf;

use iced::Length::Fill;
use iced::{window, Task};
use iced::{widget::text, Element};
use iced::widget::{button, column, row};

#[derive(Debug)]
enum FileType {
    DIR,
    FILE, 
}

#[derive(Debug)]
struct AppState{
    current_dir: PathBuf,    
    current_files: Vec<(String,FileType)>
}

impl Default for AppState {
    fn default() -> Self {
        AppState { 
            current_dir: std::env::current_dir().unwrap(),
            current_files: Vec::default(),
        }
    } 
}

// 从用户界面传递到应用程序主体的信息
#[derive(Debug,Clone)]
enum Message{
    Exit,
}

// Task是从update中返回的一个任务,而后就会执行这个任务
fn update(state: &mut AppState, message: Message) -> Task<Message>{
    match message{
        Message::Exit => window::get_latest().and_then(window::close),
    }
}

fn view(state: &AppState) -> Element<'_, Message>{
    let mut cwd = String::from("cwd: ");
    cwd.push_str(state.current_dir.to_str().unwrap_or("unkown directory"));
    column![row![
        text(cwd).size(32).width(Fill),
        button(text("Up").size(24)).on_press(Message::Exit),
        button(text("Exit").size(24)).on_press(Message::Exit),
    ].spacing(8)
    ].into()
}
fn main() -> iced::Result{
    iced::application("Arip",update,view)
        .theme(|_s| iced::Theme::KanagawaDragon)
        .run()
}

fn get_files(path: &PathBuf) -> Vec<(String,FileType)>{
    let mut dirs = Vec::default();
    let mut files = Vec::default();

    for read in fs::read_dir(path).unwrap()
    dirs.append(&mut files);
    dirs
}