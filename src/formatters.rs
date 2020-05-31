use comrak::{markdown_to_html, ComrakOptions};

pub fn format_front(front: &String) -> String {
    convert_markdown_to_html(&front[3..].to_string())
}

pub fn format_back(back: &String) -> String {
    convert_markdown_to_html(back)
}

fn convert_markdown_to_html(input_markdown: &String) -> String {
    let mut html_string: String = markdown_to_html(&input_markdown, &ComrakOptions::default());

    html_string = str::replace(&html_string, "<pre", "<pre align=left ");
    html_string = str::replace(&html_string, "<ul", "<ul align=left ");
    html_string = str::replace(&html_string, "<ol", "<ol align=left ");

    html_string
}
