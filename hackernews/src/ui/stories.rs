use crate::{
    api::{get_story_comments, get_top_stories},
    ui::CommentsState,
    StoryItem,
};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(20));

    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
            ul {
                for story in stories {
                        StoryItem{ story: story.clone() }
                }
            }
        },
        Some(Err(err)) => rsx! {
            div { class: "mt-6 text-red-500",
                p { "Failed loading stories" }
                p { "{err}" }
            }
        },
        None => rsx! {
            div { class: "mt-6 ",
                p { "Loading stories..." }
            }
        },
    }
}

#[component]
pub fn StoryItem(story: StoryItem) -> Element {
    let mut comments_state = use_context::<Signal<CommentsState>>();
    rsx! {
        li { class: "py-5 border-b px-3 transition hover:bg-indigo-100",
            a { href: "#", class: "flex justify-between items-center",
                h3 { class: "text-lg font-semibold", "{story.title}" }
                p { class: "text-md text-gray-400" }
            }
            div { class: "text-md italic text-gray-400",
                span { " {story.score} points by {story.by} {story.time} | " }
                a {
                    href: "#",
                    prevent_default: "onclick",
                    onclick: move |event| {
                        info!("Clicked on story: {} with event: {:#?}", story.title, event);
                        let story = story.clone();
                        async move {
                            *comments_state.write() = CommentsState::Loading;
                            if let Ok(story_data) = get_story_comments(story).await {
                                *comments_state.write() = CommentsState::Loaded(Box::new(story_data));
                            }
                        }
                    },
                    "{story.kids.len()} comments"
                }
            }
        }
    }
}
