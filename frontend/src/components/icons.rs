use dioxus::prelude::*;

#[component]
pub fn TrashIcon() -> Element {
    rsx! {
        Icon {
            d: "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0",
        }
    }
}

#[component]
pub fn CalendarIcon() -> Element {
    rsx! {
        Icon {
          d: "M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21 7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25 0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5m-9-6h.008v.008H12v-.008ZM12 15h.008v.008H12V15Zm0 2.25h.008v.008H12v-.008ZM9.75 15h.008v.008H9.75V15Zm0 2.25h.008v.008H9.75v-.008ZM7.5 15h.008v.008H7.5V15Zm0 2.25h.008v.008H7.5v-.008Zm6.75-4.5h.008v.008h-.008v-.008Zm0 2.25h.008v.008h-.008V15Zm0 2.25h.008v.008h-.008v-.008Zm2.25-4.5h.008v.008H16.5v-.008Zm0 2.25h.008v.008H16.5V15Z",
        }
    }
}

#[component]
pub fn EditIcon() -> Element {
    rsx! {
        Icon {
            d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
        }
    }
}

#[component]
pub fn ConfirmIcon() -> Element {
    rsx! {
        Icon {
            d: "m4.5 12.75 6 6 9-13.5",
        }
    }
}

#[component]
pub fn CancelIcon() -> Element {
    rsx! {
        Icon {
            d: "M6 18 18 6M6 6l12 12",
        }
    }
}

#[component]
pub fn PlusIcon() -> Element {
    rsx! {
        Icon {
            d: "M12 4.5v15m7.5-7.5h-15",
            stroke_width: "2",
        }
    }
}

#[component]
pub fn ToDoIcon() -> Element {
    rsx! {
        Icon {
            d: "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        }
    }
}

#[component]
pub fn InProgressIcon() -> Element {
    rsx! {
        Icon {
            d: "M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        }
    }
}

#[component]
pub fn DoneIcon() -> Element {
    rsx! {
        Icon {
            d: "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        }
    }
}

#[component]
pub fn BoltIcon() -> Element {
    rsx! {
        Icon {
            d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
        }
    }
}

#[component]
pub fn CopyIcon() -> Element {
    rsx! {
        Icon {
            d: "M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75",
        }
    }
}

#[component]
pub fn ArchiveIcon() -> Element {
    rsx! {
        Icon {
            d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
        }
    }
}

#[component]
pub fn RightIcon() -> Element {
    rsx! {
        Icon {
            d: "m8.25 4.5 7.5 7.5-7.5 7.5",
        }
    }
}

#[component]
pub fn UpIcon() -> Element {
    rsx! {
        Icon {
          d: "m4.5 15.75 7.5-7.5 7.5 7.5",
        }
    }
}

#[component]
pub fn DownIcon() -> Element {
    rsx! {
        Icon {
            d: "m19.5 8.25-7.5 7.5-7.5-7.5"
        }
    }
}

#[component]
fn Icon(d: &'static str, stroke_width: Option<&'static str>) -> Element {
    let stroke_width = stroke_width.unwrap_or("1.5");
    rsx! {
        svg {
            "aria-hidden": true,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            "viewBox": "0 0 24 24",
            "stroke-width": stroke_width,
            path {
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                d,
            }
        }
    }
}
