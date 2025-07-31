mod organizer;
mod categories;

use clap::Parser;

/// Rustify – органайзер файлів
#[derive(Parser)]
#[command(name = "Rustify")]
#[command(about = "Sorts and organizes files by type", long_about = None)]
pub struct Cli {
    /// Шлях до папки, яку потрібно впорядкувати
    pub path: String,

    /// Dry-run: тільки показати, що буде зроблено, без фактичного переміщення
    #[arg(short, long)]
    pub dry_run: bool,
}

fn main() {
    env_logger::init(); // Ініціалізація логування

    let args = Cli::parse(); // Парсимо аргументи з командного рядка

    if let Err(e) = organizer::organize(&args.path, args.dry_run) {
        eprintln!("Помилка: {}", e);
    }
}

