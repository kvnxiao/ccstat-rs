use ccstat_rs::render_statusline;

fn main() {
  match render_statusline() {
    Ok(output) => println!("{}", output),
    Err(e) => println!("ERROR: {}", e),
  }
}
