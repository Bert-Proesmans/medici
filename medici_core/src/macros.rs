//! Defines useful macro's to simplify syntax of implementations

#[macro_export]
macro_rules! ctxt {
    ($target:expr; $error:path, $machine:ident) => {
        //
        {
            use $crate::error::FrontendErrorExt;
            let _temp = $target;
            let _temp = match _temp {
                Ok(v) => v,
                Err(e) => {
                    return Err(e.infuse($error, || $machine));
                }
            };
            _temp
        }
        //
    };
    // The default is a logic error!
    ($target:expr; $machine:ident) => {
        ctxt!($target; $crate::error::ErrorKind::LogicError, $machine);
    };
}

#[macro_export]
macro_rules! hydrate {
    ($target:expr; $machine:ident) => {
        //
        {
            use $crate::error::HydratedErrorExt;
            let _temp = $target;
            let _temp = match _temp {
                Ok(v) => v,
                Err(e) => {
                    return Err(e.hydrate(|| $machine));
                }
            };
            _temp
        }
        //
    };
}
