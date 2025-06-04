use std::env;
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let min_width = 63;
    let upper = input.to_uppercase();

    let content_len = upper.len();
    let total_width = if content_len > min_width { content_len } else { min_width };

    let padding = (total_width - content_len) / 2;
    let extra_space = (total_width - content_len) % 2;

    let border = format!("// {}", "=".repeat(total_width));
    let centered_line = format!(
        "// │{}{}{}│",
        " ".repeat(padding),
        upper,
        " ".repeat(padding + extra_space)
    );

    let output = format!("{}\n{}\n{}", border, centered_line, border);

    println!("{}", output);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output).unwrap();
}
