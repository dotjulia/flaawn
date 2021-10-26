use crate::flaawn_renderer::flaawn_component::FlaawnComponent;
use crate::flaawn_renderer::flaawn_component_with_children::FlaawnComponentWithChildren;
use crate::flaawn_renderer::html_components::css_styleable::CSSStyleOptions;
use crate::options_struct;
use std::sync::Arc;

#[derive(Default)]
pub struct RandomHtmlId {
    pub id: String,
}

#[macro_export]
macro_rules! GenericHTMLTag {
    ($tag:expr, ( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        flaawn::flaawn_renderer::html_components::generic_html_component::GenericHTMLComponent {
            tag: $tag.to_string(),
            options: flaawn::flaawn_renderer::html_components::generic_html_component::GenericHTMLComponentOptions {
                $($on: Some($ov),)*
                ..Default::default()
            },
            child_components: vec![
                $(
                    std::sync::Arc::from($child),
                )*
            ],
        }
    }
}

#[macro_export]
macro_rules! generate_string_options_list {
    ($name:ident, [$($n:ident),+ ]) => {
        options_struct!($name, {
            $($n: String = None,)*
        });
    };
}

generate_string_options_list!(
    GenericHTMLComponentOptions,
    [
        accept,
        accept_charset,
        accesskey,
        action,
        align,
        allow,
        alt,
        async__,
        autocapitalize,
        autocomplete,
        autofocus,
        autoplay,
        background,
        bgcolor,
        border,
        buffered,
        capture,
        challenge,
        charset,
        checked,
        cite,
        class,
        code,
        codebase,
        color,
        cols,
        colspan,
        content,
        contenteditable,
        contextmenu,
        controls,
        coords,
        crossorigin,
        csp,
        data,
        datetime,
        decoding,
        default,
        defer,
        dir,
        dirname,
        disabled,
        download,
        draggable,
        enctype,
        enterkeyhint,
        for__,
        form,
        formaction,
        formenctype,
        formmethod,
        formnovalidate,
        formtarget,
        headers,
        height,
        hidden,
        high,
        href,
        hreflang,
        http_equiv,
        icon,
        id,
        importance,
        integrity,
        intrinsicsize,
        inputmode,
        ismap,
        itemprop,
        keytype,
        kind,
        label,
        lang,
        language,
        loading,
        list,
        loop__,
        low,
        manifest,
        max,
        maxlength,
        minlength,
        media,
        method,
        min,
        multiple,
        muted,
        name,
        novalidate,
        open,
        optimum,
        pattern,
        ping,
        placeholder,
        poster,
        preload,
        radiogroup,
        readonly,
        referrerpolicy,
        rel,
        required,
        reversed,
        rows,
        rowspan,
        sandbox,
        scope,
        scoped,
        selected,
        shape,
        size,
        sizes,
        slot,
        span,
        spellcheck,
        src,
        srcdoc,
        srclang,
        srcset,
        start,
        step,
        summary,
        tabindex,
        target,
        title,
        translate,
        type__,
        usemap,
        value,
        width,
        wrap
    ]
);

pub struct GenericHTMLComponent {
    pub tag: String,
    pub options: GenericHTMLComponentOptions,
    pub child_components: Vec<Arc<dyn FlaawnComponent>>,
}

impl GenericHTMLComponent {
    pub fn new(
        tag: String,
        options: GenericHTMLComponentOptions,
        child_components: Vec<Arc<dyn FlaawnComponent>>,
    ) -> GenericHTMLComponent {
        GenericHTMLComponent {
            tag,
            options,
            child_components: child_components,
        }
    }
}

impl FlaawnComponentWithChildren for GenericHTMLComponent {
    fn children(&self) -> Vec<Arc<dyn FlaawnComponent>> {
        self.child_components.clone()
    }
}

impl FlaawnComponent for GenericHTMLComponent {
    fn build(
        &self,
        session: &mut std::collections::HashMap<String, String>,
    ) -> std::string::String {
        format!(
            "<{} {}>{}</{}>",
            self.tag,
            self.options.build_attributes(),
            self.build_children(session),
            self.tag
        )
    }
}
