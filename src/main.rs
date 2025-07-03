use std::fs;
use std::path::PathBuf;

use iced::Length::Fill;
use iced::{window, Border, Shadow, Task};
use iced::{widget::text, Element};
use iced::widget::{button, column, horizontal_rule, row};

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
        let current_dir = std::env::current_dir().unwrap();
        let current_files = get_files(&current_dir);
        AppState { 
            current_dir, 
            current_files,
        }
    } 
}

// 从用户界面传递到应用程序主体的信息
#[derive(Debug,Clone)]
enum Message{
    Exit,
    CD(PathBuf),
}

// Task是从update中返回的一个任务,而后就会执行这个任务
fn update(state: &mut AppState, message: Message) -> Task<Message>{
    match message{
        Message::Exit => window::get_latest().and_then(window::close),
        Message::CD(path_buf) => {
            state.current_dir = path_buf;
            state.current_files = get_files(&state.current_dir);
            Task::none()
        }
    }
}

fn view(state: &AppState) -> Element<'_,Message>{
    let mut cwd = String::from("cwd: ");
    cwd.push_str(state.current_dir.to_str().unwrap_or("unkown directory"));

    let mut context = column![row![
        text(cwd).size(32).width(Fill),
        button(text("Up").size(24)).on_press(Message::CD(state.current_dir.parent().unwrap_or(&state.current_dir).to_path_buf())),
        button(text("Exit").size(24)).on_press(Message::Exit),
    ].spacing(8),
    
    ].spacing(2).padding(4);

    context = context.push(horizontal_rule(2));

    context = context.push(
        button(text("..").size(18)).on_press(Message::CD(state.current_dir.parent().unwrap_or(&state.current_dir).to_path_buf())).style(dir_button_style())
    );

    for file in &state.current_files{
        let file_name = text(&file.0).size(18);
        match &file.1 {
            // 这里还是有点说法的,context.push会返回一个新的实例,这是因为context在iced里被认为是不可改变的东西,所以要重新赋值
            FileType::DIR => {
                let mut file_path = state.current_dir.clone();
                file_path.push(&file.0);
                context = context.push(
                button(file_name)
                    .style(dir_button_style())
                    .on_press(Message::CD(file_path)),
            );
            },
            FileType::FILE => context = context.push(file_name),
        };
        
    }
    context.into()
}

fn dir_button_style() -> impl Fn(&iced::Theme,button::Status) -> button::Style{
    |_t,_e| button::Style{
        background: None,
        text_color: iced::Color::from_rgb(
            3.0/255.0,
            161.0/255.0,
            252.0/255.0,
        ),
        border: Border::default(),
        shadow: Shadow::default(),
    }
}
fn main() -> iced::Result{
    iced::application("Arip",update,view)
        .theme(|_s| iced::Theme::KanagawaDragon)
        .run()
}

fn get_files(path: &PathBuf) -> Vec<(String,FileType)>{
    let mut dirs: Vec<(String, FileType)> = Vec::default();
    let mut files = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path){
        for entry in read_dir{
            if let Ok(file) = entry{
                if let Ok(meta) = fs::metadata(&file.path()){
                    if meta.is_dir(){
                        dirs.push((file.file_name().to_str().unwrap_or("unknown dir").to_string(),FileType::DIR));
                    }else{
                        files.push((file.file_name().to_str().unwrap_or("unknown file").to_string(),FileType::FILE));
                    }
                }
            }
        }    
    }
    dirs.append(&mut files);
    dirs
}