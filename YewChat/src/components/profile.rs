use yew::prelude::*;
use crate::User;
use yew_router::prelude::*;
use crate::Route;
use js_sys;

// List avatar pilihan (nama => URL)
const AVATARS: &[(&str, &str)] = &[
    ("Cat1", "https://static.vecteezy.com/system/resources/previews/051/558/143/non_2x/shock-cat-meme-sticker-cute-illustration-vector.jpg"),
    ("Robot", "https://www.shutterstock.com/image-vector/sunglasses-doge-meme-sticker-tshirt-260nw-2550084977.jpg"),
    ("Ninja", "https://cdn.idntimes.com/content-images/post/20250304/tangkapan-layar-2025-03-04-pukul-130602-bcf3757607919df15775907d72c06307_600x400.png"),
];

const QUOTES: &[&str] = &[
    "“Code is like humor. When you have to explain it, it’s bad.” – Cory House",
    "“First, solve the problem. Then, write the code.” – John Johnson",
    "“Simplicity is the soul of efficiency.” – Austin Freeman",
];

#[function_component(Profile)]
pub fn profile() -> Html {
    // Ambil username dari context
    let user = use_context::<User>().expect("No context found.");
    let name = user.username.borrow().clone();

    // State untuk avatar yang terpilih (index)
    let avatar_idx = use_state(|| 0);
    // State untuk quote acak
    let quote = use_state(|| {
        let idx = js_sys::Math::floor(js_sys::Math::random() * (QUOTES.len() as f64)) as usize;
        QUOTES[idx].to_string()
    });

    let on_next_avatar = {
        let avatar_idx = avatar_idx.clone();
        Callback::from(move |_| {
            avatar_idx.set(((*avatar_idx + 1) % AVATARS.len()) as usize);
        })
    };

    html! {
        <div class="flex flex-col items-center justify-center h-screen bg-gradient-to-br from-purple-700 to-blue-600 text-white p-6">
            <h1 class="text-3xl font-bold mb-4">{"User Profile"}</h1>
            <img
                class="w-32 h-32 rounded-full shadow-lg mb-4"
                src={AVATARS[*avatar_idx].1.to_string()}
                alt={AVATARS[*avatar_idx].0.to_string()}
            />
            <div class="mb-2 text-xl"> { format!("Hello, {}!", name) } </div>
            <button
                onclick={on_next_avatar}
                class="bg-white text-purple-700 px-4 py-2 rounded-full shadow hover:bg-gray-200 transition"
            >
                { "Next Avatar" }
            </button>
            <p class="italic mt-6 text-center max-w-md"> { (*quote).clone() } </p>
            <Link<Route> to={Route::Chat}>
                <button class="mt-8 bg-white text-blue-600 px-6 py-2 rounded-full shadow hover:bg-gray-200 transition">
                    { "Back to Chat" }
                </button>
            </Link<Route>>
        </div>
    }
}