use dioxus::prelude::*;
use crate::models::download::DownloadStatus;

#[derive(Clone, Copy, PartialEq)]
pub enum StatusVariant {
    Success,
    Warning,
    Error,
    Info,
    Pending,
}

impl From<&DownloadStatus> for StatusVariant {
    fn from(status: &DownloadStatus) -> Self {
        match status {
            DownloadStatus::Completed => StatusVariant::Success,
            DownloadStatus::Downloading => StatusVariant::Info,
            DownloadStatus::Processing => StatusVariant::Info,
            DownloadStatus::Pending => StatusVariant::Pending,
            DownloadStatus::Paused => StatusVariant::Warning,
            DownloadStatus::Failed => StatusVariant::Error,
            DownloadStatus::Cancelled => StatusVariant::Warning,
        }
    }
}

#[component]
pub fn StatusIndicator(
    #[props(default = StatusVariant::Info)] variant: StatusVariant,
    #[props(default = "".to_string())] label: String,
    #[props(default = false)] pulse: bool,
) -> Element {
    let (bg_color, text_color, dot_color) = match variant {
        StatusVariant::Success => ("bg-green-100", "text-green-800", "bg-green-500"),
        StatusVariant::Warning => ("bg-yellow-100", "text-yellow-800", "bg-yellow-500"),
        StatusVariant::Error => ("bg-red-100", "text-red-800", "bg-red-500"),
        StatusVariant::Info => ("bg-blue-100", "text-blue-800", "bg-blue-500"),
        StatusVariant::Pending => ("bg-gray-100", "text-gray-800", "bg-gray-500"),
    };
    
    let pulse_class = if pulse { "animate-pulse" } else { "" };
    
    rsx! {
        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {bg_color} {text_color}",
            span { class: "w-2 h-2 rounded-full mr-1.5 {dot_color} {pulse_class}" }
            if !label.is_empty() {
                {label}
            }
        }
    }
}

#[component]
pub fn DownloadStatusBadge(status: DownloadStatus) -> Element {
    let variant = StatusVariant::from(&status);
    let pulse = matches!(status, DownloadStatus::Downloading | DownloadStatus::Processing);
    let label = format!("{:?}", status);
    
    rsx! {
        StatusIndicator { variant, label, pulse }
    }
}
