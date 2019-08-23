
# Build My Own X Where X = Plugin System

A plugin system is more a programming pattern than a concrete component. It is a way to dynamically add and remove features based on a solid program. Generally, we use plugin system to extend customizablity.

Things that a plugin can do depends on the interfaces that plugin system (or rather, base program) provides, and of course are highly relavent to the intention and purpose of the program. So here I'm just illustrating basic plugin patterns, for example: a message `processor` which do nothing but holds a plugin system, an `upcase-plugin` that transform strings to lowercase, and an `echo-plugin` that prints received message.
