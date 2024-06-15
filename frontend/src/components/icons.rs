use dioxus::prelude::*;

#[component]
pub fn StackIcon() -> Element {
    rsx! {
        Icon {
            d: "M6.429 9.75 2.25 12l4.179 2.25m0-4.5 5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0 4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0-5.571 3-5.571-3",
        }
    }
}

#[component]
pub fn BoardIcon() -> Element {
    rsx! {
        Icon {
            d: "M9 4.5v15m6-15v15m-10.875 0h15.75c.621 0 1.125-.504 1.125-1.125V5.625c0-.621-.504-1.125-1.125-1.125H4.125C3.504 4.5 3 5.004 3 5.625v12.75c0 .621.504 1.125 1.125 1.125Z",
        }
    }
}

#[component]
pub fn SolidBoardIcon() -> Element {
    rsx! {
        Icon {
            fill: "currentColor",
            stroke_width: "0",
            d: "M15 3.75H9v16.5h6V3.75ZM16.5 20.25h3.375c1.035 0 1.875-.84 1.875-1.875V5.625c0-1.036-.84-1.875-1.875-1.875H16.5v16.5ZM4.125 3.75H7.5v16.5H4.125a1.875 1.875 0 0 1-1.875-1.875V5.625c0-1.036.84-1.875 1.875-1.875Z",
        }
    }
}

#[component]
pub fn TagIcon() -> Element {
    rsx! {
        Svg {
            fill: "none",
            stroke_width: "1.5",
            body: rsx!{
                Path {
                    d: "M9.568 3H5.25A2.25 2.25 0 0 0 3 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 0 0 5.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 0 0 9.568 3Z"
                }
                Path {
                    d: "M6 6h.008v.008H6V6Z"
                }
            }
        }
    }
}

#[component]
pub fn SolidTagIcon() -> Element {
    rsx! {
        Svg {
            fill: "currentColor",
            stroke_width: "0",
            body: rsx!{
                path {
                    "fill-rule": "evenodd",
                    d: "M5.25 2.25a3 3 0 0 0-3 3v4.318a3 3 0 0 0 .879 2.121l9.58 9.581c.92.92 2.39 1.186 3.548.428a18.849 18.849 0 0 0 5.441-5.44c.758-1.16.492-2.629-.428-3.548l-9.58-9.581a3 3 0 0 0-2.122-.879H5.25ZM6.375 7.5a1.125 1.125 0 1 0 0-2.25 1.125 1.125 0 0 0 0 2.25Z",
                    "clip-rule": "evenodd",
                }
            }
        }
    }
}

#[component]
pub fn UsersIcon() -> Element {
    rsx! {
        Icon {
            d: "M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z",
        }
    }
}

#[component]
pub fn SolidUsersIcon() -> Element {
    rsx! {
        Svg {
            fill: "currentColor",
            stroke_width: "0",
            body: rsx! {
                path {
                    d: "M4.5 6.375a4.125 4.125 0 1 1 8.25 0 4.125 4.125 0 0 1-8.25 0ZM14.25 8.625a3.375 3.375 0 1 1 6.75 0 3.375 3.375 0 0 1-6.75 0ZM1.5 19.125a7.125 7.125 0 0 1 14.25 0v.003l-.001.119a.75.75 0 0 1-.363.63 13.067 13.067 0 0 1-6.761 1.873c-2.472 0-4.786-.684-6.76-1.873a.75.75 0 0 1-.364-.63l-.001-.122ZM17.25 19.128l-.001.144a2.25 2.25 0 0 1-.233.96 10.088 10.088 0 0 0 5.06-1.01.75.75 0 0 0 .42-.643 4.875 4.875 0 0 0-6.957-4.611 8.586 8.586 0 0 1 1.71 5.157v.003Z",
                }
            }
        }
    }
}

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
pub fn CircledPlusIcon() -> Element {
    rsx! {
        Icon {
            d: "M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
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
pub fn SolidDoneIcon() -> Element {
    rsx! {
        Svg {
            fill: "currentColor",
            stroke_width: "0",
            body: rsx!{
                path {
                    "fill-rule": "evenodd",
                    "clip-rule": "evenodd",
                    d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z",
                }
            }
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
pub fn UnarchiveIcon() -> Element {
    rsx! {
        Icon {
            d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
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
pub fn SolidArchiveIcon() -> Element {
    rsx! {
        Svg {
            fill: "currentColor",
            stroke_width: "0",
            body: rsx! {
                path {
                    d: "M3.375 3C2.339 3 1.5 3.84 1.5 4.875v.75c0 1.036.84 1.875 1.875 1.875h17.25c1.035 0 1.875-.84 1.875-1.875v-.75C22.5 3.839 21.66 3 20.625 3H3.375Z"
                }
                path {
                    "fill-rule": "evenodd",
                    d: "m3.087 9 .54 9.176A3 3 0 0 0 6.62 21h10.757a3 3 0 0 0 2.995-2.824L20.913 9H3.087ZM12 10.5a.75.75 0 0 1 .75.75v4.94l1.72-1.72a.75.75 0 1 1 1.06 1.06l-3 3a.75.75 0 0 1-1.06 0l-3-3a.75.75 0 1 1 1.06-1.06l1.72 1.72v-4.94a.75.75 0 0 1 .75-.75Z",
                    "clip-rule": "evenodd",
                }
            }
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
fn Icon(
    d: &'static str,
    fill: Option<&'static str>,
    stroke_width: Option<&'static str>,
) -> Element {
    let stroke_width = stroke_width.unwrap_or("1.5");
    let fill = fill.unwrap_or("none");
    rsx! {
        Svg {
            fill,
            stroke_width,
            body: rsx!{
                Path { d }
            }
        }
    }
}

#[component]
pub fn Svg(body: Element, fill: &'static str, stroke_width: &'static str) -> Element {
    rsx! {
        svg {
            "aria-hidden": true,
            xmlns: "http://www.w3.org/2000/svg",
            fill,
            "viewBox": "0 0 24 24",
            "stroke-width": stroke_width,
            {body}
        }
    }
}

#[component]
pub fn Path(d: &'static str) -> Element {
    rsx! {
        path {
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            d,
        }
    }
}
