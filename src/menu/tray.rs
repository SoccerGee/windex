use anyhow::Result;
use std::sync::OnceLock;
use tray_icon::menu::{CheckMenuItem, Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

/// Actions that can be triggered from the menu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    Settings,
    AccessibilitySettings,
    LaunchAtLogin,
    About,
    Quit,
    Unknown,
}

/// Menu item IDs
struct MenuIds {
    settings: MenuId,
    accessibility: MenuId,
    launch_at_login: MenuId,
    about: MenuId,
    quit: MenuId,
}

/// Set once while the tray is built, then only read. `static mut` would be
/// undefined behaviour under Rust's aliasing rules even though the menu is
/// only touched from the main thread.
static MENU_IDS: OnceLock<MenuIds> = OnceLock::new();

/// The live menu bar item. Held for the lifetime of the app — dropping it
/// removes the icon from the menu bar.
pub struct Tray {
    _icon: TrayIcon,
    launch_at_login: CheckMenuItem,
}

impl Tray {
    /// Reflect the current launch-at-login state in the menu checkbox.
    pub fn set_launch_at_login(&self, checked: bool) {
        self.launch_at_login.set_checked(checked);
    }

    /// Whether the checkbox is currently ticked.
    pub fn launch_at_login_checked(&self) -> bool {
        self.launch_at_login.is_checked()
    }
}

/// Build the tray icon and menu
pub fn build_tray(launch_at_login: bool) -> Result<Tray> {
    let menu = Menu::new();

    let version_item = MenuItem::new(
        format!("Windex {}", env!("CARGO_PKG_VERSION")),
        false,
        None,
    );
    let settings_item = MenuItem::new("Edit Config…", true, None);
    let accessibility_item = MenuItem::new("Accessibility Access…", true, None);
    let launch_item = CheckMenuItem::new("Launch at Login", true, launch_at_login, None);
    let about_item = MenuItem::new("Keyboard Shortcuts…", true, None);
    let quit_item = MenuItem::new("Quit Windex", true, None);

    // Store the menu item IDs for later lookup
    let _ = MENU_IDS.set(MenuIds {
        settings: settings_item.id().clone(),
        accessibility: accessibility_item.id().clone(),
        launch_at_login: launch_item.id().clone(),
        about: about_item.id().clone(),
        quit: quit_item.id().clone(),
    });

    menu.append(&version_item)?;
    menu.append(&PredefinedMenuItem::separator())?;
    menu.append(&about_item)?;
    menu.append(&settings_item)?;
    menu.append(&accessibility_item)?;
    menu.append(&launch_item)?;
    menu.append(&PredefinedMenuItem::separator())?;
    menu.append(&quit_item)?;

    let icon = create_default_icon()?;

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip(format!("Windex {}", env!("CARGO_PKG_VERSION")))
        .with_icon(icon)
        // Template icons are recolored by macOS, so the glyph stays legible in
        // light mode, dark mode, and when the menu bar item is highlighted.
        .with_icon_as_template(true)
        .build()?;

    Ok(Tray {
        _icon: tray,
        launch_at_login: launch_item,
    })
}

/// Draw the menu bar glyph: the same three-pane layout as the app icon.
///
/// Rendered at 2x (36px) so it stays crisp on Retina displays; macOS scales it
/// down to the menu bar height. Template icons only use the alpha channel, so
/// the RGB values are black throughout.
fn create_default_icon() -> Result<Icon> {
    const SIZE: usize = 36;
    const INSET: usize = 4; // padding around the glyph inside the square
    const STROKE: usize = 3; // border / divider thickness

    let mut rgba = vec![0u8; SIZE * SIZE * 4];

    let left = INSET;
    let right = SIZE - INSET - 1;
    let top = INSET + 1;
    let bottom = SIZE - INSET - 2;
    let split_x = left + (right - left) * 52 / 100;
    let split_y = top + (bottom - top) / 2;

    for y in top..=bottom {
        for x in left..=right {
            // Rounded corners: skip the single outermost pixel of each corner.
            let corner = (x <= left && (y <= top || y >= bottom))
                || (x >= right && (y <= top || y >= bottom));
            if corner {
                continue;
            }

            let on_border = x < left + STROKE
                || x > right - STROKE
                || y < top + STROKE
                || y > bottom - STROKE;
            let on_divider = (x >= split_x && x < split_x + STROKE)
                || (x > split_x && y >= split_y && y < split_y + STROKE);

            if on_border || on_divider {
                let idx = (y * SIZE + x) * 4;
                rgba[idx] = 0;
                rgba[idx + 1] = 0;
                rgba[idx + 2] = 0;
                rgba[idx + 3] = 255;
            }
        }
    }

    Ok(Icon::from_rgba(rgba, SIZE as u32, SIZE as u32)?)
}

/// Get the menu event receiver
pub fn menu_receiver() -> &'static crossbeam_channel::Receiver<MenuEvent> {
    MenuEvent::receiver()
}

/// Convert a menu event to a MenuAction
pub fn event_to_action(event: &MenuEvent) -> MenuAction {
    let Some(ids) = MENU_IDS.get() else {
        return MenuAction::Unknown;
    };

    if event.id == ids.settings {
        MenuAction::Settings
    } else if event.id == ids.accessibility {
        MenuAction::AccessibilitySettings
    } else if event.id == ids.launch_at_login {
        MenuAction::LaunchAtLogin
    } else if event.id == ids.about {
        MenuAction::About
    } else if event.id == ids.quit {
        MenuAction::Quit
    } else {
        MenuAction::Unknown
    }
}
