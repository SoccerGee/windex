use anyhow::Result;
use tray_icon::menu::{Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

/// Actions that can be triggered from the menu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    Settings,
    About,
    Quit,
    Unknown,
}

/// Menu item IDs
struct MenuIds {
    settings: MenuId,
    about: MenuId,
    quit: MenuId,
}

static mut MENU_IDS: Option<MenuIds> = None;

/// Build the tray icon and menu
pub fn build_tray() -> Result<TrayIcon> {
    let menu = Menu::new();

    let settings_item = MenuItem::new("Settings...", true, None);
    let about_item = MenuItem::new("About Windex", true, None);
    let quit_item = MenuItem::new("Quit Windex", true, None);

    // Store the menu item IDs for later lookup
    unsafe {
        MENU_IDS = Some(MenuIds {
            settings: settings_item.id().clone(),
            about: about_item.id().clone(),
            quit: quit_item.id().clone(),
        });
    }

    menu.append(&settings_item)?;
    menu.append(&PredefinedMenuItem::separator())?;
    menu.append(&about_item)?;
    menu.append(&PredefinedMenuItem::separator())?;
    menu.append(&quit_item)?;

    // Create a simple icon (16x16 white square for now)
    // TODO: Replace with proper icon asset
    let icon = create_default_icon()?;

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Windex - Window Manager")
        .with_icon(icon)
        .build()?;

    Ok(tray)
}

/// Create a simple default icon
fn create_default_icon() -> Result<Icon> {
    // Create a simple 16x16 icon with a grid pattern
    let size = 16;
    let mut rgba = vec![0u8; size * size * 4];

    for y in 0..size {
        for x in 0..size {
            let idx = (y * size + x) * 4;
            // Create a simple window-like icon pattern
            let is_border = x == 0 || x == size - 1 || y == 0 || y == size - 1;
            let is_titlebar = y < 4 && x > 0 && x < size - 1;
            let is_divider = x == size / 2 && y > 3;

            if is_border || is_titlebar || is_divider {
                // White color
                rgba[idx] = 255; // R
                rgba[idx + 1] = 255; // G
                rgba[idx + 2] = 255; // B
                rgba[idx + 3] = 255; // A
            } else {
                // Transparent
                rgba[idx] = 0;
                rgba[idx + 1] = 0;
                rgba[idx + 2] = 0;
                rgba[idx + 3] = 0;
            }
        }
    }

    Ok(Icon::from_rgba(rgba, size as u32, size as u32)?)
}

/// Get the menu event receiver
pub fn menu_receiver() -> &'static crossbeam_channel::Receiver<MenuEvent> {
    MenuEvent::receiver()
}

/// Convert a menu event to a MenuAction
pub fn event_to_action(event: &MenuEvent) -> MenuAction {
    unsafe {
        if let Some(ref ids) = MENU_IDS {
            if event.id == ids.settings {
                return MenuAction::Settings;
            }
            if event.id == ids.about {
                return MenuAction::About;
            }
            if event.id == ids.quit {
                return MenuAction::Quit;
            }
        }
    }
    MenuAction::Unknown
}
