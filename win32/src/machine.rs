use crate::winapi;
use builtin_kernel32 as kernel32;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use win32_system::dll::{DLLResolution, Shim};
use win32_system::memory::Memory;
use win32_system::{ArcEvent, System, Wait, WaitResult, host};
use win32_winapi::{ERROR, HANDLE, HMODULE};

#[cfg(feature = "x86-emu")]
pub use crate::machine_emu::Machine;
#[cfg(feature = "x86-64")]
pub use crate::machine_raw::Machine;
#[cfg(feature = "x86-unicorn")]
pub use crate::machine_unicorn::Machine;

/// Integrates the X86 CPU emulator with the Windows OS support.
pub struct MachineX<Emu> {
    pub status: Status,
    pub emu: Emu,
    pub memory: Memory,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub state2: std::cell::UnsafeCell<HashMap<TypeId, Box<dyn Any>>>,
    pub external_dlls: Vec<String>,
}

impl<Emu> MachineX<Emu>
where
    MachineX<Emu>: System,
{
    /// Hackily make a null pointer, for use in tests when we know the pointer isn't needed.
    #[cfg(test)]
    pub fn null() -> &'static mut MachineX<Emu> {
        #[allow(invalid_value)]
        unsafe {
            std::mem::transmute(0usize)
        }
    }

    pub fn set_external_dlls(&mut self, dlls: Vec<String>) {
        self.external_dlls = dlls
            .into_iter()
            .map(|dll| kernel32::loader::normalize_module_name(&dll))
            .collect();
    }

    pub fn break_on_startup(&mut self) {
        kernel32::get_state(self).break_on_startup = true;
    }
}

impl System for Machine {
    fn mem(&self) -> memory::Mem {
        self.mem()
    }
    fn memory(&self) -> &Memory {
        &self.memory
    }
    fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    fn machine(&mut self) -> *mut () {
        self as *mut _ as *mut _
    }

    fn host(&self) -> &dyn host::Host {
        self.host.as_ref()
    }

    fn call_x86(
        &mut self,
        func: u32,
        args: Vec<u32>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32> + '_>> {
        Box::pin(self.call_x86(func, args))
    }

    fn new_thread(&mut self, new_cpu: bool, stack_size: u32, start_addr: u32, args: &[u32]) -> u32 {
        Machine::new_thread_impl(self, new_cpu, stack_size, start_addr, args)
    }

    fn block(
        &mut self,
        wait: Option<u32>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + '_>> {
        Box::pin(Machine::block(self, wait))
    }

    fn unblock(&mut self) {
        #[cfg(feature = "x86-emu")]
        {
            self.unblock_all();
        }

        #[cfg(not(feature = "x86-emu"))]
        {
            todo!();
        }
    }

    fn sleep(&mut self, ms: u32) -> std::pin::Pin<Box<dyn Future<Output = ()> + '_>> {
        Box::pin(kernel32::Sleep(self, ms))
    }

    fn wait_for_events(
        &mut self,
        events: &[ArcEvent],
        wait_all: bool,
        wait: Wait,
    ) -> std::pin::Pin<Box<dyn Future<Output = WaitResult> + '_>> {
        Box::pin(kernel32::wait_for_events(
            self,
            events.into(),
            wait_all,
            wait,
        ))
    }

    fn wait_for_objects(
        &mut self,
        events: &[HANDLE<()>],
        wait_all: bool,
        wait: Wait,
    ) -> std::pin::Pin<Box<dyn Future<Output = WaitResult> + '_>> {
        let objects = {
            let state = kernel32::get_state(self);
            events
                .into_iter()
                .map(|handle| state.objects.get(*handle).unwrap().get_event().clone())
                .collect()
        };
        Box::pin(kernel32::wait_for_events(self, objects, wait_all, wait))
    }

    fn set_last_error(&self, err: ERROR) {
        kernel32::teb_mut(self).LastErrorValue = err.into();
    }

    fn get_library(&self, name: &str) -> HMODULE {
        let name = kernel32::loader::normalize_module_name(name);

        if let Some((hmodule, _)) = kernel32::get_state(self)
            .modules
            .iter()
            .find(|(_, dll)| dll.name == name)
        {
            *hmodule
        } else {
            HMODULE::null()
        }
    }

    fn load_library(&mut self, dll: &str) -> std::pin::Pin<Box<dyn Future<Output = HMODULE> + '_>> {
        let dll = dll.to_string();
        Box::pin(async move {
            let res = kernel32::loader::load_dll(self, &dll).await;
            // TODO: forward errors to caller?
            res.unwrap()
        })
    }

    fn get_symbol(&self, dll: &str, name: &str) -> u32 {
        kernel32::loader::get_symbol(self, dll, name)
    }

    fn get_resources(&self, module: HMODULE) -> Option<&[u8]> {
        let state = kernel32::get_state(self);
        let module = state.modules.get(&module)?;
        module.resources(self.mem())
    }

    fn get_thread_id(&self) -> u32 {
        kernel32::current_thread(self).to_raw()
    }

    fn exit(&mut self, status: u32) {
        Machine::exit(self, status);
    }
    fn exit_thread(&mut self, status: u32) {
        Machine::exit_thread(self, status);
    }

    fn state(&self, id: &TypeId) -> &dyn Any {
        if id == &TypeId::of::<std::cell::RefCell<builtin_user32::State>>() {
            &self.state.user32
        } else if id == &TypeId::of::<std::cell::RefCell<builtin_gdi32::State>>() {
            &self.state.gdi32
        } else if id == &TypeId::of::<std::cell::RefCell<builtin_dsound::State>>() {
            &self.state.dsound
        } else if id == &TypeId::of::<std::cell::RefCell<builtin_ddraw::State>>() {
            &self.state.ddraw
        } else if id == &TypeId::of::<std::cell::RefCell<builtin_winmm::State>>() {
            &self.state.winmm
        } else {
            panic!()
        }
    }

    fn state2(&self, id: &TypeId, init: fn() -> Box<dyn std::any::Any>) -> &dyn Any {
        // Safety: we only ever insert into this hashmap, so existing references
        // to values remain valid even though here we mutate it.
        let state2 = unsafe { &mut *self.state2.get() };
        if let Some(state) = state2.get(id) {
            return state.as_ref();
        }
        let state2 = unsafe { &mut *self.state2.get() };
        state2.insert(id.clone(), init());
        state2.get(id).unwrap().as_ref()
    }

    fn teb_addr(&self) -> u32 {
        self.teb_addr()
    }

    fn debug_break(&mut self) {
        self.emu.x86.cpu_mut().state = x86::CPUState::DebugBreak;
    }

    fn resolve_dll(&self, filename: &str) -> DLLResolution {
        winapi::builtin::resolve_dll(&self.external_dlls, &filename)
    }

    fn register_shims(&mut self, export_to_shim: &[(u32, &'static Shim)]) {
        for &(addr, shim) in export_to_shim {
            self.emu.shims.register(addr, Ok(shim));
        }
    }
}

/// Status of the machine/process.  Separate from CPU state because multiple threads
/// can be in different states.
#[derive(PartialEq, Eq, Default, Debug)]
pub enum Status {
    /// Running normally.
    #[default]
    Running,
    /// All threads are blocked awaiting results.
    Blocked,
    /// CPU error.
    Error {
        message: String,
        // TODO:
        // signal: u8
    },
    /// Hit a breakpoint.
    DebugBreak,
    /// Process exited.
    Exit(u32),
}

impl Status {
    pub fn is_running(&self) -> bool {
        matches!(self, Status::Running)
    }
}
