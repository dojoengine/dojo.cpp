#ifndef TORII_CLIENT_H
#define TORII_CLIENT_H

#include "rust/cxx.h"
#include "rust/cxx_async.h"

// The first argument is the C++ type that the future yields, and the second argument is the
// fully-qualified name of the future, with `::` namespace separators replaced with commas. (For
// instance, if your future is named `mycompany::myproject::RustFutureString`, you might write
// `CXXASYNC_DEFINE_FUTURE(rust::String, mycompany, myproject, RustFutureString);`. The first
// argument is the C++ type that `cxx` maps your Rust type to: in this case, `String` maps to
// `rust::String`, so we supply `rust::String` here.
//
// This macro must be invoked at the top level, not in a namespace.
CXXASYNC_DEFINE_FUTURE(void, , org, dojo, RustFutureClient);

#endif // TORII_CLIENT_H