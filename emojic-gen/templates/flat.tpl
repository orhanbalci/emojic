// Code generated by github.com/orhanbalci/emojic/emojic-gen DO NOT EDIT.

// Source: {{ Link }}
// Created at: {{ Date }}

#![allow(unused_imports)]

//! Flat list of all emojis without sub modules.
//!
//! This module contains the same set of emojis as the [`crate::grouped`] module, but
//! without the sub modules. This make it a bit more messy, but allows for shorter
//! references from code.
//!
//! # Examples
//!
//! ```rust
//! // prints: 🖼️
//! println!("{}", emojic::flat::FRAMED_PICTURE);
//! ```

{% for grp in Constants %}
// begin {{ grp.identifier }} {{ grp.preview_emojis }}
	{% for sub in grp.subgroups %}
	// begin {{ sub.identifier }} {{ sub.preview_emojis }}
		{% for emoji in sub.emojis %}
		// {{ emoji.identifier }} {{ emoji.preview_emojis }}
		#[doc(inline)]
		pub use crate::grouped::{{ grp.identifier }}::{{ sub.identifier }}::{{ emoji.identifier }};
		{% endfor %}
	// end {{ sub.identifier }}
	{% endfor %}
// end {{ grp.identifier }}
{% endfor %}

// EOF