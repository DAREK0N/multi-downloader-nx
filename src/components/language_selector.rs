use dioxus::prelude::*;
use crate::hooks::use_i18n::use_i18n;

/// Language information
struct LanguageInfo {
    code: &'static str,
    name: &'static str,
    native_name: &'static str,
}

const LANGUAGES: &[LanguageInfo] = &[
    LanguageInfo { code: "en", name: "English", native_name: "English" },
    LanguageInfo { code: "de", name: "German", native_name: "Deutsch" },
    LanguageInfo { code: "es", name: "Spanish", native_name: "Español" },
    LanguageInfo { code: "fr", name: "French", native_name: "Français" },
    LanguageInfo { code: "it", name: "Italian", native_name: "Italiano" },
    LanguageInfo { code: "pt", name: "Portuguese", native_name: "Português" },
    LanguageInfo { code: "pt-BR", name: "Brazilian Portuguese", native_name: "Português (Brasil)" },
    LanguageInfo { code: "ru", name: "Russian", native_name: "Русский" },
    LanguageInfo { code: "zh-CN", name: "Chinese (Simplified)", native_name: "简体中文" },
    LanguageInfo { code: "zh-TW", name: "Chinese (Traditional)", native_name: "繁體中文" },
    LanguageInfo { code: "ja", name: "Japanese", native_name: "日本語" },
    LanguageInfo { code: "ko", name: "Korean", native_name: "한국어" },
    LanguageInfo { code: "pl", name: "Polish", native_name: "Polski" },
    LanguageInfo { code: "nl", name: "Dutch", native_name: "Nederlands" },
    LanguageInfo { code: "tr", name: "Turkish", native_name: "Türkçe" },
    LanguageInfo { code: "ar", name: "Arabic", native_name: "العربية" },
];

/// Language selector component
#[component]
pub fn LanguageSelector() -> Element {
    let i18n = use_i18n();
    let mut show_dropdown = use_signal(|| false);
    let current_locale = i18n.locale();
    
    let current_lang = LANGUAGES.iter()
        .find(|lang| lang.code == current_locale)
        .unwrap_or(&LANGUAGES[0]);

    rsx! {
        div { class: "relative inline-block",
            button {
                class: "px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500",
                onclick: move |_| show_dropdown.set(!show_dropdown()),
                "🌐 {current_lang.native_name}"
            }
            
            if show_dropdown() {
                div { class: "absolute right-0 mt-2 w-64 bg-white dark:bg-gray-800 rounded-md shadow-lg z-10 max-h-96 overflow-y-auto",
                    for lang in LANGUAGES {
                        button {
                            key: "{lang.code}",
                            class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
                            onclick: move |_| {
                                i18n.set_locale(lang.code);
                                show_dropdown.set(false);
                            },
                            "{lang.native_name} ({lang.name})"
                        }
                    }
                }
            }
        }
    }
}
