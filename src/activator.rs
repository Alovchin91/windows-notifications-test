use windows::Win32::{
    Foundation::CLASS_E_NOAGGREGATION,
    System::Com::{IClassFactory, IClassFactory_Impl},
    UI::Notifications::{
        INotificationActivationCallback, INotificationActivationCallback_Impl,
        NOTIFICATION_USER_INPUT_DATA,
    },
};
use windows_core::{BOOL, GUID, IUnknown, Interface, PCWSTR, Ref, Result, implement};

#[implement(INotificationActivationCallback)]
struct ToastActivator;

impl INotificationActivationCallback_Impl for ToastActivator_Impl {
    fn Activate(
        &self,
        _app_user_model_id: &PCWSTR,
        invoked_args: &PCWSTR,
        _data: *const NOTIFICATION_USER_INPUT_DATA,
        _count: u32,
    ) -> Result<()> {
        println!("toast activated! invoked args: {}", unsafe {
            invoked_args.display()
        });
        Ok(())
    }
}

#[implement(IClassFactory)]
pub struct ToastActivatorFactory;

impl IClassFactory_Impl for ToastActivatorFactory_Impl {
    fn CreateInstance(
        &self,
        punkouter: Ref<'_, IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
        if !punkouter.is_null() {
            return Err(CLASS_E_NOAGGREGATION.into());
        }
        let unk: IUnknown = ToastActivator.into();
        unsafe { unk.query(riid, ppvobject) }.ok()
    }

    fn LockServer(&self, _flock: BOOL) -> Result<()> {
        Ok(())
    }
}

#[allow(non_upper_case_globals)]
pub const CToastActivator: GUID = GUID::from_u128(0x402cb25a_34a9_4793_a89e_ed623c32decf);
