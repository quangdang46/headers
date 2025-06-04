use std::env;
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let total_width = 63; 
    let border = "// ================================================================";

    let trimmed = if input.len() > total_width {
        input[..total_width].to_string()
    } else {
        input.clone()
    };

    let padding = (total_width - trimmed.len()) / 2;
    let extra_space = (total_width - trimmed.len()) % 2; 

    let centered_line = format!(
        "// │{}{}{}│",
        " ".repeat(padding),
        trimmed,
        " ".repeat(padding + extra_space)
    );

    let output = format!("{}\n{}\n{}", border, centered_line, border);


    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output).unwrap(); 
}
