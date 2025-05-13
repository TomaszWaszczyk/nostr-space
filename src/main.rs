use leptos::{mount::mount_to_body, *};
use leptos::prelude::ElementChild;


fn main() {
    mount_to_body(|| view! { <p>"Hello, world!"</p> });
}
