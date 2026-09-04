// -----------------------------------------------------------------------------
// Модуль os_dialogs
// Содержит реализацию системных диалогов (OpenFile, CloseFile и т.д.)
// -----------------------------------------------------------------------------
use std::sync::OnceLock;
use std::path::PathBuf;

use crate::APP_TITLE;

// Глобальное хранилище для дескриптора главного окна
pub static PARENT_WINDOW: OnceLock<AppWindow> = OnceLock::new();

// -----------------------------------------------------------------------------
// Нелперы для Windows
// -----------------------------------------------------------------------------
//#[cfg(windows)]
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
    RawWindowHandle, RawDisplayHandle
};

// --- СИСТЕМНАЯ СБОРКА ДЛЯ ИСТИННОЙ МОДАЛЬНОСТИ НА WINDOWS ---
pub struct AppWindow {
    pub window:  RawWindowHandle,
    pub display: RawDisplayHandle,
}
// Разрешаем безопасно передавать структуру между потоками и
// конкурентный доступ по ссылкам (&AppWindow)
unsafe impl Send for AppWindow {}
unsafe impl Sync for AppWindow {}

impl HasWindowHandle for AppWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unsafe { Ok(WindowHandle::borrow_raw(self.window)) }
    }
}

impl HasDisplayHandle for AppWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe { Ok(DisplayHandle::borrow_raw(self.display)) }
    }
}

/// Находит HWND вашей запущенной программы Iced на Windows для жесткого закрепления
//#[cfg(windows)]
// fn get_win32_parent() -> Option<AppWindow> {
//     use raw_window_handle::{
//         RawDisplayHandle, RawWindowHandle, Win32WindowHandle, WindowsDisplayHandle,
//     };
//     use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, GA_ROOT, GetAncestor};

//     let window_title: Vec<u16> = APP_TITLE.encode_utf16().collect();
//     unsafe {
//         let hwnd = FindWindowW(std::ptr::null(), window_title.as_ptr());
//         if !hwnd.is_null() {
//             let root_hwnd = GetAncestor(hwnd, GA_ROOT);
//             if let Some(nonzero_hwnd) = std::num::NonZeroIsize::new(root_hwnd as isize) {
//                 let win32_handle = Win32WindowHandle::new(nonzero_hwnd);
//                 let raw_win_handle = RawWindowHandle::Win32(win32_handle);
//                 let final_win_handle = WindowHandle::borrow_raw(raw_win_handle);

//                 let win_display = WindowsDisplayHandle::new();
//                 let raw_disp_handle = RawDisplayHandle::Windows(win_display);
//                 let final_disp_handle = DisplayHandle::borrow_raw(raw_disp_handle);

//                 return Some(AppWindow {
//                     window: final_win_handle,
//                     display: final_disp_handle,
//                 });
//             }
//         }
//     }
//     None
// }

// ====================================================================
// а) ДИАЛОГ СООБЩЕНИЙ (MESSAGE DIALOG)
// ====================================================================

/// Перечисление типов системных уведомлений
pub enum DialogLevel {
    Info,
    Warning,
    Error,
}

/// Вызывает модальное окно сообщения. Принимает: заголовок, тип, текст.
pub fn show_message_box(title: &str, level: DialogLevel, message: &str) {
    let rfd_level = match level {
        DialogLevel::Info => rfd::MessageLevel::Info,
        DialogLevel::Warning => rfd::MessageLevel::Warning,
        DialogLevel::Error => rfd::MessageLevel::Error,
    };

    let mut dialog = rfd::MessageDialog::new()
        .set_title(title)
        .set_description(message)
        .set_buttons(rfd::MessageButtons::Ok)
        .set_level(rfd_level);

    // Привязываем родительское окно на Windows (на Mac/Linux RFD связывает процессы нативно)
    // #[cfg(windows)]
    // if let Some(parent) = get_win32_parent() {
    //     dialog = dialog.set_parent(&parent);
    // }
    // #[cfg(target_os="linux")]
    // if let Some(parent) = iced_window_handle {
    //     dialog = dialog.set_parent(&parent);
    // }
    if let Some(parent_window) = PARENT_WINDOW.get() {
        dialog = dialog.set_parent(&parent_window);
    }
    
    dialog.show();
}

/// Вызывает модальное окно подтверждения (Да/Нет).
/// Возвращает true, если пользователь нажал "Да", и false, если выбрал "Нет".
pub fn show_confirm_box(title: &str, message: &str) -> bool {
    log::info!("Вызов модального диалогового окна '{}'", title);

    let mut dialog = rfd::MessageDialog::new()
        .set_title(title)
        .set_description(message)
        .set_buttons(rfd::MessageButtons::YesNo)    // Две кнопки: Да и Нет
        .set_level(rfd::MessageLevel::Warning);     // Иконка предупреждения (Желтый треугольник)

    #[cfg(windows)]
    // if let Some(parent) = get_win32_parent() {
    //     dialog = dialog.set_parent(&parent);
    // }
    if let Some(parent_window) = PARENT_WINDOW.get() {
        dialog = dialog.set_parent(&parent_window);
    }

    // .show() для YesNo возвращает true при нажатии на "Yes"
    dialog.show() == rfd::MessageDialogResult::Yes
}

// ====================================================================
// б) ДИАЛОГ ОТКРЫТИЯ ФАЙЛА (OPEN FILE DIALOG)
// ====================================================================

/// Вызывает модальный диалог открытия файла.
/// Принимает: название окна, начальный каталог, расширение-фильтр (например, "json")
pub fn show_open_dialog(title: &str, initial_dir: &str, filter_ext: &str) -> Option<PathBuf> {
    let mut dialog = rfd::FileDialog::new()
        .set_title(title)
        .add_filter(&format!("Проект (*.{})", filter_ext), &[filter_ext]);

    if !initial_dir.is_empty() {
        dialog = dialog.set_directory(initial_dir);
    }

    // #[cfg(windows)]
    // if let Some(parent) = get_win32_parent() {
    //     dialog = dialog.set_parent(&parent);
    // }
    if let Some(parent_window) = PARENT_WINDOW.get() {
        dialog = dialog.set_parent(&parent_window);
    }

    dialog.pick_file()
}

// ====================================================================
// в) ДИАЛОГ СОХРАНЕНИЯ ФАЙЛА (SAVE FILE DIALOG)
// ====================================================================

/// Вызывает модальный диалог сохранения файла.
/// Принимает: название окна, имя файла по умолчанию, расширение-фильтр
pub fn show_save_dialog(title: &str, default_name: &str, filter_ext: &str) -> Option<PathBuf> {
    let mut dialog = rfd::FileDialog::new()
        .set_title(title)
        .set_file_name(default_name)
        .add_filter(&format!("Проект (*.{})", filter_ext), &[filter_ext]);

    // #[cfg(windows)]
    // if let Some(parent) = get_win32_parent() {
    //     dialog = dialog.set_parent(&parent);
    // }
    if let Some(parent_window) = PARENT_WINDOW.get() {
        dialog = dialog.set_parent(&parent_window);
    }

    dialog.save_file()
}
