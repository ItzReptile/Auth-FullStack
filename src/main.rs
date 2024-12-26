use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    phone: String,
    website: String,
    address: Address,
    company: Company,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Company {
    name: String,
    catchPhrase: String,
    bs: String,
}

#[function_component(App)]
fn app() -> Html {
    let users = use_state(Vec::new);
    let loading = use_state(|| true);
    let search_term = use_state(|| String::new());

    {
        let users = users.clone();
        let loading = loading.clone();
        
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match fetch_users().await {
                        Ok(fetched_users) => {
                            users.set(fetched_users);
                            loading.set(false);
                        }
                        Err(e) => {
                            console::log_1(&format!("Error fetching users: {:?}", e).into());
                            loading.set(false);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let on_search = {
        let search_term = search_term.clone();
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            search_term.set(value);
        })
    };

    let filtered_users = users.iter().filter(|user| {
        let search = (*search_term).to_lowercase();
        user.name.to_lowercase().contains(&search) || 
        user.email.to_lowercase().contains(&search) ||
        user.username.to_lowercase().contains(&search)
    });

    html! {
        <div class="container mx-auto p-4">
            <h1 class="text-3xl font-bold mb-6">{"User Directory"}</h1>
            
            // Search bar
            <div class="mb-6">
                <input 
                    type="text"
                    placeholder="Search by name, username, or email..."
                    class="w-full px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    onchange={on_search}
                />
            </div>

            if *loading {
                <div class="flex justify-center">
                    <p>{"Loading..."}</p>
                </div>
            } else {
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {filtered_users.map(|user| html! {
                        <UserCard user={user.clone()} />
                    }).collect::<Html>()}
                </div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct UserCardProps {
    user: User,
}

#[function_component(UserCard)]
fn user_card(props: &UserCardProps) -> Html {
    html! {
        <div class="bg-white shadow rounded-lg p-6 hover:shadow-lg transition-shadow">
            <h2 class="text-xl font-semibold mb-2">{&props.user.name}</h2>
            <p class="text-gray-600 mb-1">{"@"}{&props.user.username}</p>
            <p class="text-gray-600 mb-3">{&props.user.email}</p>
            
            <div class="border-t pt-3">
                <h3 class="font-semibold mb-1">{"Company"}</h3>
                <p class="text-gray-600">{&props.user.company.name}</p>
            </div>
            
            <div class="border-t mt-3 pt-3">
                <h3 class="font-semibold mb-1">{"Contact"}</h3>
                <p class="text-gray-600 mb-1">{&props.user.phone}</p>
                <a href={format!("https://{}", &props.user.website)} 
                   class="text-blue-600 hover:underline"
                   target="_blank">
                    {&props.user.website}
                </a>
            </div>
        </div>
    }
}

async fn fetch_users() -> Result<Vec<User>, reqwest::Error> {
    let users = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/users")
        .send()
        .await?
        .json::<Vec<User>>()
        .await?;
    Ok(users)
}

fn main() {
    yew::Renderer::<App>::new().render();
}