
extern crate log;
extern crate libloading;

use log::{info, debug};
use libloading::{Library, Symbol};

use std::any::Any;
use std::ffi::{OsStr, CStr};
use std::io::{Result, Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::os::raw::{c_void, c_char, c_int};

pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> String;
    fn on_load(&mut self) -> Result<()> { Ok(()) }
    fn on_unload(&mut self) -> Result<()> { Ok(()) }
}

struct CPluginData(Arc<Mutex<*mut c_void>>);
unsafe impl Send for CPluginData {}
unsafe impl Sync for CPluginData {}

struct CPlugin {
    loaded: bool,
    _name: String,
    library: Library,
    data: CPluginData,
}

impl CPlugin {
    pub fn build<P: AsRef<OsStr>>(filename: P) -> Result<CPlugin> {
        info!("Loading library: {}", filename.as_ref().to_str().unwrap());
        let library = Library::new(filename)?;
        info!("Getting plugin name...");
        let fn_name: Symbol<unsafe extern "C" fn() -> *const c_char> =
            unsafe { library.get(b"name\0") }?;
        let _name = unsafe { CStr::from_ptr(fn_name()) }.to_string_lossy().to_string();
        info!("...{}", _name);
        info!("Creating plugin data...");
        let fn_initialize: Symbol<unsafe extern "C" fn() -> *mut c_void> =
            unsafe { library.get(b"initialize\0") }?;
        let data = unsafe { fn_initialize() };
        let data = CPluginData(Arc::new(Mutex::new(data)));
        info!("Plugin Created: {}", _name);
        Ok(CPlugin {
            loaded: false,
            library,
            data,
            _name,
        })
    }
}

impl Plugin for CPlugin {
    fn name(&self) -> String {
        self._name.clone()
    }
    fn on_load(&mut self) -> Result<()> {
        if self.loaded {
            return Err(Error::new(ErrorKind::Other, format!("Already loaded: {}", self._name)));
        }
        self.loaded = true;
        let fn_on_load: Symbol<unsafe extern "C" fn(data: *mut c_void) -> c_int> =
            unsafe { self.library.get(b"on_load\0") }?;
        let data = *self.data.0.lock().unwrap();
        let result = unsafe { fn_on_load(data) };
        if result != 0 {
            return Err(Error::new(ErrorKind::Other, result.to_string()));
        }
        Ok(())
    }
    fn on_unload(&mut self) -> Result<()> {
        if !self.loaded {
            return Err(Error::new(ErrorKind::Other, format!("Not loaded yet: {}", self._name)));
        }
        self.loaded = false;
        let fn_on_unload: Symbol<unsafe extern "C" fn(data: *mut c_void) -> c_int> =
            unsafe { self.library.get(b"on_unload\0") }?;
        let data = *self.data.0.lock().unwrap();
        let result = unsafe { fn_on_unload(data) };
        if result != 0 {
            return Err(Error::new(ErrorKind::Other, result.to_string()));
        }
        Ok(())
    }
}

impl Drop for CPlugin {
    fn drop(&mut self) {
        if self.loaded {
            let _ = self.on_unload();
        }
    }
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    pub fn load(&mut self, filename: &str) -> Result<()> {
        info!("Loding {}", filename);
        let mut plugin = Box::new(CPlugin::build(filename)?);
        plugin.on_load()?;
        self.plugins.push(plugin);
        Ok(())
    }

    pub fn unload(&mut self, name: &str) -> Result<()> {
        let plugin_index = self.plugins.iter()
            .position(|plugin| plugin.name() == name)
            .ok_or_else(|| Error::new(
                ErrorKind::Other,
                format!("Failed to find specified plugin: {}", name)))?;
        self.unload_at(plugin_index)
    }

    pub fn unload_at(&mut self, index: usize) -> Result<()> {
        if index >= self.plugins.len() {
            return Err(Error::new(ErrorKind::Other, "Index out of bounds!"));
        }
        let mut plugin = self.plugins.remove(index);
        plugin.on_unload()?;
        Ok(())
    }
}
