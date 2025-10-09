#[macro_export]
macro_rules! module_import_prelude {
    (
        $m_vis:vis mod prelude {
            $(use * from { $( $all_vis:vis mod $all_name:ident; )* })?
            $(use prelude::* from { $( $vis:vis mod $name:ident; )* })?
        }
    ) => {
        $($($all_vis mod $all_name;)*)?
        $($($vis mod $name;)*)?

        #[allow(unused)]
        $m_vis mod prelude {
            $($($all_vis use super::$all_name::*;)*)?
            $($($vis use super::$name::prelude::*;)*)?
        }
    };
}
