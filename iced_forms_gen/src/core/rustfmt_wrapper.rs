use std::process::{Command, Stdio};
use std::io::Write;



pub fn rustfmt(input: &str) -> Result<String, String> {
    // Напрямую вызываем системный процесс, форсируя современную редакцию
    let mut child = Command::new("rustfmt")
        .arg("--emit")
        .arg("stdout")
        .arg("--edition")
        .arg("2021") // Iced 0.14 требует Rust 2021
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Не удалось запустить rustfmt в системе: {}", e))?;

    // Записываем "грязный" код в стандартный ввод процесса
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes()).map_err(|e| e.to_string())?;
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| e.to_string())
    } else {
        // Передаем реальный лог ошибки наружу в ваш макрос log::error
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        Err(stderr)
    }
}