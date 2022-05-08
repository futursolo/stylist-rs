use yew::prelude::*;

use crate::manager::StyleManager;

/// The properties for [`ManagerProvider`] Component, please see its documentation for usage.
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct ManagerProviderProps {
    pub manager: StyleManager,
    pub children: Children,
}

/// A Context Provider to provide a custom [`StyleManager`] to child components.
///
/// # Example:
///
/// ```
/// use stylist::manager::StyleManager;
/// use stylist::yew::ManagerProvider;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let mgr = use_state(|| {
///         StyleManager::builder()
///             .prefix("my-styles".into())
///             .build()
///             .unwrap()
///     });
///
///     let children = Html::default();
///     html! {
///         <ManagerProvider manager={(*mgr).clone()}>
///             {children}
///         </ManagerProvider>
///     }
/// }
/// ```
#[function_component(ManagerProvider)]
pub fn manager_provider(props: &ManagerProviderProps) -> Html {
    html! {
        <ContextProvider<StyleManager> context={props.manager.clone()}>
            {props.children.clone()}
        </ContextProvider<StyleManager>>
    }
}
