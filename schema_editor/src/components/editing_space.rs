use std::sync::Arc;

use generated_crate::prelude::*;

use super::workspace::{WorkspaceState, WorkspaceTab};
#[component]
pub fn EditingSpace() -> impl IntoView {
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();

    let list_view = move || {
        let selected_tab = selected_tab.clone();
        let list = match selected_tab.get() {
            WorkspaceTab::Template(_) => todo!(),
            WorkspaceTab::Operative(_) => todo!(),
            WorkspaceTab::Instance(_) => todo!(),
            WorkspaceTab::Trait(_) => todo!(),
        };
    };

    list_view
}
