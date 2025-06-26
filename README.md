# auto-registry -- Automatic registration via proc-macros

`auto-registry` is a crate that lets you automatically collect types inside a structure to later call a macro on them.
The main goal of this macro is to allow painless automatic registration by constructing the registered types on demand.

This crate uses the `proc_macros_span` span, thus requiring a nightly compiler.

# auto-registry in action

Basic example use of `auto-registry`:
```
pub trait Listener { ... }

#[auto_registry::auto_registry(registry = "listeners")]
pub struct KeyboardListener { ... }
impl Listener for KeyboardListener { ... }

#[auto_registry::auto_registry(registry = "listeners")]
pub struct MouseListener { ... }
impl Listener for MouseListener { ... }

macro_rules! collect_listeners { // Collects to a Vec<Box<dyn Listener>>
	( $($construct:expr);+ $(;)? ) => {{ // Macro must accepts `;`-separated arguments
		vec![$(Box::new($construct) as Box<dyn crate::Listener + Send + Sync>,)+]
	}};
}

/// Calling the `get_listeners!()` will generate the Vec containing all our listeners.
#[auto_registry::generate_registry(
    registry = "listeners",
    collector = collect_listeners,
    output = get_listeners)]
```

In this example, all registered types are mapped to expressions for the `collect_listeners` macro using `::default()`.

You can also specify how each types gets mapped to an expression via the `mapper` attribute:
```
pub trait Listener { ... }

#[auto_registry::auto_registry(registry = "listeners")]
pub struct KeyboardListener { ... }
impl Listener for KeyboardListener { ... }

#[auto_registry::auto_registry(registry = "listeners")]
pub struct MouseListener { ... }
impl Listener for MouseListener { ... }

macro_rules! map_listeners { // Maps a type to an expression
	($t:ty) => {{
        Box::new(<$t>::new()) as Box<dyn Listener + Send + Sync>
	}};
}

macro_rules! collect_listeners { // Collects to a Vec<Box<dyn Listener>>
	( $($construct:expr);+ $(;)? ) => {{ // Macro must accepts `;`-separated arguments
		vec![$($construct,)+]
	}};
}

/// Calling the `get_listeners!()` will generate the Vec containing all our listeners.
#[auto_registry::generate_registry(
    registry = "listeners",
    mapper = map_listeners,
    collector = collect_listeners,
    output = get_listeners)]
```

# License

This crate is licensed under the MIT license.
