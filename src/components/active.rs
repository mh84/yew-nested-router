use crate::prelude::Target;
use crate::switch::use_switch;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ActiveProps<T: Target> {
    pub target: T,

    pub children: Children,

    #[prop_or_else(default::element)]
    pub element: String,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub active: Classes,

    #[prop_or_default]
    pub inactive: Classes,
}

mod default {
    pub fn element() -> String {
        "span".to_string()
    }
}

/// A style element, allowing to add cass classes based on the target's active state.
#[function_component(Active)]
pub fn active<T>(props: &ActiveProps<T>) -> Html
where
    T: Target,
{
    let switch = use_switch().expect("Need Router or Nested component");

    let mut class = props.class.clone();

    match switch.is_active(&props.target) {
        true => class.extend(props.active.clone()),
        false => class.extend(props.inactive.clone()),
    }

    html!(
        <@{props.element.clone()} {class}>
            { for props.children.iter() }
        </@>
    )
}
