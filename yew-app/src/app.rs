use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew::Callback;

#[derive(PartialEq, Clone, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    videos: Vec<Video>,
    //  final goal here is to display the selected video. To do that, VideosList component needs to
    //  "notify" its parent when a video is selected, which is done via a Callback
    on_click: Callback<Video>,
}

#[function_component(VideosList)]
fn videos_list(video_list: &VideosListProps) -> Html {
    // making the html creation process as component
    let on_click = video_list.on_click.clone();
    video_list
        .videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };
            html! {
                // <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
                <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };
    let details = selected_video.as_ref().map(|video| {
        html! {
           <VideoDetails video={video.clone()} />
        }
    });
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];
    // let videos = use_state(|| vec![]);
    // {
    //     let videos = videos.clone();
    //     use_effect_with((), move |_| {
    //         let videos = videos.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
    //                 .send()
    //                 .await
    //                 .unwrap()
    //                 .json()
    //                 .await
    //                 .unwrap();
    //             videos.set(fetched_videos);
    //         });
    //         || ()
    //     });
    // }
    // let video_html: Html = videos
    //     .iter()
    //     .map(|v| {
    //         html! {
    //         <p key={v.id}>{format!("{}: {}", v.speaker, v.title)}</p>
    //         }
    //     })
    //     .collect();

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                // { video_html }
                <VideosList videos={videos} on_click={on_video_select.clone()} />
            </div>
            // { for details }. Option<_> implements Iterator
            // so we can use it to display the only
            // element returned by the Iterator with a special
            // { for ... } syntax supported by the html! macro.
            {for details}
            <div>
                <h3>{ "John Doe: Building and breaking things" }</h3>
                <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
            </div>
        </>
    }
}
