# SWC Plugin Solid

### An attempt to add SolidJs support to SWC through a plugin.

The ultimate goal of this project is to fully support all of the features of the SolidJs babel plugin.
Where this differes from the existing plugin(s) is that it is going to consolidate
- babel-plugin-jsx-dom-expressions
- babel-preset-solid
- solid-refresh

all into one unified package. The 'dom expressions' part will likely still be separable from the Solid
specific implementations but that is not an explicit goal of this project.

If this plugin works successfully there will be a separate package provide a 'vite-plugin' implementation
of this, which will just be a thin wrapper around this making it easier to integrate into vite projects.

Currently the plugin works for the most part with standard client code. It does not support hydration, nor does it support
ssr or universal modes 

hmr is also not currently supported.
