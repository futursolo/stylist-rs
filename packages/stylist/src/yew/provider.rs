use yew::prelude::*;

use crate::manager::StyleManager;

/// The properties for [`ManagerProvider`] Component, please see its documentation for usage.
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct ManagerProviderProps {
    pub manager: StyleManager,
    #[prop_or_default]
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
pub fn manager_provider(props: &ManagerProviderProps) -> HtmlResult {
    let ManagerProviderProps { manager, children } = props.clone();

    #[cfg(any(feature = "ssr", feature = "hydration"))]
    {
        use crate::manager::StyleData;

        let _manager = manager.clone();
        let _style_data = use_transitive_state!(
            move |_| -> StyleData {
                _manager
                    .style_data()
                    .map(|m| m.clone())
                    .unwrap_or_else(StyleData::new)
            },
            ()
        )?;

        #[cfg(feature = "hydration")]
        {
            // We must load the styles immediately before child components are rendered.
            let manager = manager.clone();
            use_memo(
                move |manager| {
                    if let Some(m) = _style_data {
                        manager.load_style_data(m.as_ref());
                    }
                },
                manager,
            );
        }
    }

    Ok(html! {
        <ContextProvider<StyleManager> context={manager}>
            {children}
        </ContextProvider<StyleManager>>
    })
}
