use super::{Text, Translation};

pub const NO: Translation<&'static str> = Translation {
    id: "no",
    name: "NO - Norsk",
    text: Text {
        open_main_menu: "Åpne hovedmenyen",
        home_section_label: "Hjem",
        features_section_label: "Funksjoner",
        pricing_section_label: "Priser",
        contact_section_label: "Kontakt",
        select_language: "Velg språk",
        go_to_app: "Gå til appen",
        new_board: "Ny tavle",
        h1_main: "Delt oppgavehåndtering.",
        h1_sub: "Enkelt, uten registrering.",
        dense_button_label: "Tett",
        dark_button_label: "Mørk",
        mobile_button_label: "Mobil",
    },
};
