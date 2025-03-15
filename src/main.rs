use windows::{
    Data::Xml::Dom::XmlDocument,
    UI::Notifications::{ToastNotification, ToastNotificationManager},
    Win32::{
        System::Com::{
            CLSCTX_LOCAL_SERVER, COINIT_MULTITHREADED, CoInitializeEx, CoRegisterClassObject,
            CoRevokeClassObject, REGCLS_MULTIPLEUSE,
        },
        UI::Shell::SetCurrentProcessExplicitAppUserModelID,
    },
};
use windows_core::{IUnknown, Result, h, w};

mod activator;

fn main() -> Result<()> {
    unsafe { SetCurrentProcessExplicitAppUserModelID(w!("Uh.Oh.WindowsNotificationsTest"))? };

    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.ok()?;

    if std::env::args().any(|arg| arg == "-ToastActivated") {
        return on_toast_activated();
    }

    let doc = XmlDocument::new()?;
    doc.LoadXml(h!(r#"<toast>
        <visual>
            <binding template="ToastGeneric">
                <text></text>
                <text></text>
            </binding>
        </visual>
    </toast>"#))?;
    doc.SelectSingleNode(h!("//text[1]"))?
        .SetInnerText(h!("Andrew sent you a picture"))?;
    doc.SelectSingleNode(h!("//text[2]"))?
        .SetInnerText(h!("Check this out, The Enchantments in Washington!"))?;

    let notification = ToastNotification::CreateToastNotification(&doc)?;

    let toast_notifier =
        ToastNotificationManager::CreateToastNotifierWithId(h!("Uh.Oh.WindowsNotificationsTest"))?;

    toast_notifier.Show(&notification)
}

fn on_toast_activated() -> Result<()> {
    println!("press <ENTER> to exit ...");

    let factory: IUnknown = activator::ToastActivatorFactory.into();
    let registration = unsafe {
        CoRegisterClassObject(
            &activator::CToastActivator,
            &factory,
            CLSCTX_LOCAL_SERVER,
            REGCLS_MULTIPLEUSE,
        )?
    };

    let mut _discard = String::new();
    std::io::stdin()
        .read_line(&mut _discard)
        .expect("failed to read line");

    unsafe { CoRevokeClassObject(registration) }
}
