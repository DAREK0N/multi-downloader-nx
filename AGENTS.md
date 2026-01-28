# Migration Tasking (Dioxus 0.7.3)

## Mission
- Migrate legacy `/old` React content into a Dioxus **0.7.3** fullstack app.
- Current focus: **Phase 1–2** of the migration plan.

## Priorities
1) Deliver an updated `prompt.yaml` reflecting the Dioxus 0.7.3 target.
2) Keep existing GUI/UX parity while converting logic to idiomatic Rust/Dioxus.
3) Maintain Tailwind-driven styling and responsiveness baseline.

## Phase Expectations
- **Phase 1 (Foundation)**: Confirm project scaffolding, routing, and shared layout/components align with Dioxus 0.7.3; keep Tailwind wired.
- **Phase 2 (Core Features)**: Begin migrating download/server logic from React/TS to Dioxus server functions and Rust services; preserve queue and scheduling contracts.

## Working Notes
- Prefer Dioxus hooks (`use_signal`, `use_effect`, `use_resource`) over React hooks.
- Replace React server logic with Dioxus server functions; keep SSR + hydration working.
- Keep i18n language set and theme manager intact; ensure light/dark/system detection roadmap is preserved.
- Do not regress baseline functionality (downloads, queue, scheduled tasks, DRM status, webhooks).
