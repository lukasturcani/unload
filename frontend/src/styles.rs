pub const BUTTON: &str = "
    text-white focus:ring-4 focus:outline-none
    font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5
    text-center bg-blue-600 hover:bg-blue-700 focus:ring-blue-800
";
pub const TEXT_INPUT: &str = "
    border text-base rounded-lg
    w-full p-2.5 bg-gray-700
    border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500
    focus:border-blue-500
";
pub const TEXT_INPUT_LABEL: &str = "
    text-sm font-medium text-white
";

pub const BOTTOM_BAR_BUTTON: &str = "
    h-full w-1/6
    grid place-items-center
    group
    first:border-s border-e border-gray-600
    active:bg-gray-800
    sm:hover:bg-gray-800 sm:border-none
";

pub const BOTTOM_BAR_ACTIVE_BUTTON: &str = "
    h-full w-1/6
    grid place-items-center
    group
    first:border-s border-e border-gray-600
    bg-gray-800
    sm:border-none
";

pub const BOTTOM_BAR: &str = "
    grow-0 shrink-0 w-full h-16
    bg-gray-700
    border-t border-gray-600
    flex flex-row justify-center
";

pub const TOOLTIP: &str = "
    pointer-events-none absolute -top-10 -left-2 w-max
    opacity-0 transition-opacity group-hover:opacity-100
    z-10 px-3 py-2 text-sm font-medium text-white
    rounded-lg shadow-sm tooltip bg-gray-800
    border border-gray-700
";
