#![feature(trace_macros)]

use std::sync::{LazyLock, Mutex};
trace_macros!(true);
pub trait Listener {
	fn name(&self) -> &'static str;
}
pub trait AA {}

#[auto_registry::auto_registry(registry = "listeners", path = "crate")]
#[derive(Default)]
pub struct KeyboardListener {}
impl Listener for KeyboardListener {
    fn name(&self) -> &'static str {
        "Keyboard"
    }
}

#[auto_registry::auto_registry(registry = "listeners", path = "crate")]
#[derive(Default)]
pub struct MouseListener {}
impl Listener for MouseListener {
    fn name(&self) -> &'static str {
        "Mouse"
    }
}

macro_rules! collect_listeners { // Collects to a Vec<Box<dyn Listener>>
	( $($construct:expr);+ $(;)? ) => {{ // Macro must accepts `;`-separated arguments
		vec![$(Box::new($construct) as Box<dyn crate::Listener + Send + Sync>,)+]
	}};
}

#[auto_registry::generate_registry(registry = "listeners", collector = collect_listeners, output = get_listeners)]

#[test]
fn test_basic_buildup() {
	let x = KeyboardListener{};
	let listeners = get_listeners!();

	assert_eq!(listeners.len(), 2);
	assert_eq!(listeners[0].name(), "Keyboard");
	assert_eq!(listeners[1].name(), "Mouse");
}

static LISTENERS: LazyLock<Mutex<Vec<Box<dyn Listener + Send + Sync>>>> = LazyLock::new(|| Mutex::new(Vec::default()));

macro_rules! register_listener { // Register a single listener
	($t:ty) => {{
		let mut listeners = LISTENERS.lock();
		listeners
			.unwrap()
			.push(Box::new(<$t>::default()) as Box<dyn Listener + Send + Sync>);
	}};
}

#[auto_registry::generate_registry(registry = "listeners", mapper = register_listener, output = register_all_listeners)]

#[test]
fn test_basic_registration() {
	register_all_listeners!();
	assert_eq!(LISTENERS.lock().unwrap().len(), 2);
	assert_eq!(LISTENERS.lock().unwrap()[0].name(), "Keyboard");
	assert_eq!(LISTENERS.lock().unwrap()[1].name(), "Mouse");
}
