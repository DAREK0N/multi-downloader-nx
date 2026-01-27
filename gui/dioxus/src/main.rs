#![allow(non_snake_case)]
//! Dioxus-based web UI for Multi Downloader NX with rudimentary i18n and responsive layout.
//! This is a minimal scaffold translated from the existing React GUI to unblock future work.

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Lang {
	En,
	De,
	Es,
	Fr,
	Ja,
}

#[derive(Clone, Copy, PartialEq)]
struct LangContext {
	lang: Lang,
}

fn main() {
	launch(App);
}

fn App() -> Element {
	let lang = use_signal(|| LangContext { lang: Lang::En });

	rsx! {
		LocaleProvider { lang: lang() }
		Container { lang: lang }
	}
}

#[component]
fn LocaleProvider(lang: LangContext, children: Element) -> Element {
	use_context_provider(|| lang);
	children
}

#[component]
fn Container(lang: Signal<LangContext>) -> Element {
	let current = lang().lang;

	let t = |key: &str| translate(&current, key);

	let mut lang_signal = lang;

	rsx! {
		div {
			style: "min-height: 100vh; display: flex; flex-direction: column; background: #121212; color: #fff; font-family: Inter, system-ui, -apple-system, BlinkMacSystemFont, sans-serif; padding: 1rem; gap: 1rem;",
			header {
				style: "display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center; justify-content: center;",
				h1 { style: "margin: 0; font-size: 1.6rem;", "{t(\"title\")}" }
				div {
					style: "display: flex; gap: 0.5rem; flex-wrap: wrap; justify-content: center;",
					button { onclick: move |_| lang_signal.write().lang = Lang::En, "EN" }
					button { onclick: move |_| lang_signal.write().lang = Lang::De, "DE" }
					button { onclick: move |_| lang_signal.write().lang = Lang::Es, "ES" }
					button { onclick: move |_| lang_signal.write().lang = Lang::Fr, "FR" }
					button { onclick: move |_| lang_signal.write().lang = Lang::Ja, "日本語" }
				}
			}
			main {
				style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1rem; width: 100%;",
				section {
					style: card_style(),
					h2 { "{t(\"actions\")}" }
					div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
						button { "{t(\"login\")}" }
						button { "{t(\"logout\")}" }
						button { "{t(\"open_output\")}" }
						button { "{t(\"clear_queue\")}" }
						button { "{t(\"add_to_queue\")}" }
						button { "{t(\"start_queue\")}" }
					}
				}
				section {
					style: card_style(),
					h2 { "{t(\"queue\")}" }
					p { "{t(\"queue_empty\")}" }
					progress { max: "100", value: "0" }
					div { style: "display: flex; flex-direction: column; gap: 0.25rem;",
						span { "{t(\"progress_meta\")}" }
					}
				}
			}
			footer { style: "margin-top: auto; text-align: center; font-size: 0.9rem; color: #aaa;",
				"{t(\"footer\")}"
			}
		}
	}
}

fn card_style() -> &'static str {
	"background: #1f1f1f; border-radius: 12px; padding: 1rem; box-shadow: 0 10px 30px rgba(0,0,0,0.35);"
}

fn translate(lang: &Lang, key: &str) -> &'static str {
	match (lang, key) {
		(_, "title") => "AniDL Downloader",
		(Lang::En, "actions") => "Actions",
		(Lang::De, "actions") => "Aktionen",
		(Lang::Es, "actions") => "Acciones",
		(Lang::Fr, "actions") => "Actions",
		(Lang::Ja, "actions") => "操作",
		(Lang::En, "login") => "Login",
		(Lang::De, "login") => "Anmelden",
		(Lang::Es, "login") => "Iniciar sesión",
		(Lang::Fr, "login") => "Connexion",
		(Lang::Ja, "login") => "ログイン",
		(Lang::En, "logout") => "Logout",
		(Lang::De, "logout") => "Abmelden",
		(Lang::Es, "logout") => "Cerrar sesión",
		(Lang::Fr, "logout") => "Déconnexion",
		(Lang::Ja, "logout") => "ログアウト",
		(Lang::En, "open_output") => "Open Output Directory",
		(Lang::De, "open_output") => "Ausgabeverzeichnis öffnen",
		(Lang::Es, "open_output") => "Abrir carpeta de salida",
		(Lang::Fr, "open_output") => "Ouvrir le dossier de sortie",
		(Lang::Ja, "open_output") => "出力フォルダーを開く",
		(Lang::En, "clear_queue") => "Clear Queue",
		(Lang::De, "clear_queue") => "Warteschlange leeren",
		(Lang::Es, "clear_queue") => "Limpiar cola",
		(Lang::Fr, "clear_queue") => "Vider la file",
		(Lang::Ja, "clear_queue") => "キューをクリア",
		(Lang::En, "add_to_queue") => "Add to Queue",
		(Lang::De, "add_to_queue") => "Zur Warteschlange hinzufügen",
		(Lang::Es, "add_to_queue") => "Añadir a la cola",
		(Lang::Fr, "add_to_queue") => "Ajouter à la file",
		(Lang::Ja, "add_to_queue") => "キューに追加",
		(Lang::En, "start_queue") => "Start Queue",
		(Lang::De, "start_queue") => "Warteschlange starten",
		(Lang::Es, "start_queue") => "Iniciar cola",
		(Lang::Fr, "start_queue") => "Démarrer la file",
		(Lang::Ja, "start_queue") => "キューを開始",
		(Lang::En, "queue") => "Queue",
		(Lang::De, "queue") => "Warteschlange",
		(Lang::Es, "queue") => "Cola",
		(Lang::Fr, "queue") => "File d'attente",
		(Lang::Ja, "queue") => "キュー",
		(Lang::En, "queue_empty") => "No items in queue.",
		(Lang::De, "queue_empty") => "Keine Elemente in der Warteschlange.",
		(Lang::Es, "queue_empty") => "Sin elementos en la cola.",
		(Lang::Fr, "queue_empty") => "Aucun élément dans la file.",
		(Lang::Ja, "queue_empty") => "キューにアイテムがありません。",
		(Lang::En, "progress_meta") => "Progress: 0% | 0 / 0 parts",
		(Lang::De, "progress_meta") => "Fortschritt: 0% | 0 / 0 Teile",
		(Lang::Es, "progress_meta") => "Progreso: 0% | 0 / 0 partes",
		(Lang::Fr, "progress_meta") => "Progression : 0 % | 0 / 0 parties",
		(Lang::Ja, "progress_meta") => "進行状況: 0% | 0 / 0 パート",
		(Lang::En, "footer") => "Responsive Dioxus UI with i18n",
		(Lang::De, "footer") => "Responsives Dioxus-UI mit i18n",
		(Lang::Es, "footer") => "UI Dioxus adaptable con i18n",
		(Lang::Fr, "footer") => "Interface Dioxus réactive avec i18n",
		(Lang::Ja, "footer") => "i18n対応のレスポンシブDioxus UI",
		(_, _) => "Missing translation",
	}
}
