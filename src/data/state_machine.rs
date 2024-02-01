use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

#[doc(hidden)]
mod imp {
    use super::*;

    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct StateMachine {
        pub type_iface: glib::gobject_ffi::GTypeInterface,
        pub press: fn(&super::StateMachine),
        pub release: fn(&super::StateMachine),
        pub press_timeout: fn(&super::StateMachine),
        pub tick: fn(&super::StateMachine),
    }

    #[glib::object_interface]
    unsafe impl ObjectInterface for StateMachine {
        const NAME: &'static str = "PtStateMachine";
        type Prerequisites = ();
    }
}

glib::wrapper! {
    /// The interface for timer state machines.
    pub struct StateMachine(ObjectInterface<imp::StateMachine>);
}

/// Trait that contains defined method in `StateMachine`
pub trait StateMachineExt: 'static {
    fn press(&self);
    fn release(&self);
    fn press_timeout(&self);
    fn tick(&self);
}

impl<O: IsA<StateMachine>> StateMachineExt for O {
    fn press(&self) {
        let iface = self.interface::<StateMachine>().unwrap();
        (iface.as_ref().press)(self.upcast_ref())
    }

    fn release(&self) {
        let iface = self.interface::<StateMachine>().unwrap();
        (iface.as_ref().release)(self.upcast_ref())
    }

    fn press_timeout(&self) {
        let iface = self.interface::<StateMachine>().unwrap();
        (iface.as_ref().press_timeout)(self.upcast_ref())
    }

    fn tick(&self) {
        let iface = self.interface::<StateMachine>().unwrap();
        (iface.as_ref().tick)(self.upcast_ref())
    }
}

/// Trait that must be implemented by objects that implements `StateMachine`.
pub trait StateMachineImpl: ObjectImpl {
    fn press(&self);
    fn release(&self);
    fn press_timeout(&self);
    fn tick(&self);
}

unsafe impl<T> IsImplementable<T> for StateMachine
where
    T: StateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<StateMachine>,
{
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.press = state_machine_press_trampoline::<T>;
        iface.release = state_machine_release_trampoline::<T>;
        iface.press_timeout = state_machine_press_timeout_trampoline::<T>;
        iface.tick = state_machine_tick_trampoline::<T>;
    }
}

fn state_machine_press_trampoline<T>(state_machine: &StateMachine)
where
    T: StateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<StateMachine>,
{
    state_machine
        .downcast_ref::<T::Type>()
        .unwrap()
        .imp()
        .press();
}

fn state_machine_release_trampoline<T>(state_machine: &StateMachine)
where
    T: StateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<StateMachine>,
{
    state_machine
        .downcast_ref::<T::Type>()
        .unwrap()
        .imp()
        .release();
}

fn state_machine_press_timeout_trampoline<T>(state_machine: &StateMachine)
where
    T: StateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<StateMachine>,
{
    state_machine
        .downcast_ref::<T::Type>()
        .unwrap()
        .imp()
        .press_timeout();
}

fn state_machine_tick_trampoline<T>(state_machine: &StateMachine)
where
    T: StateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<StateMachine>,
{
    state_machine
        .downcast_ref::<T::Type>()
        .unwrap()
        .imp()
        .tick();
}
