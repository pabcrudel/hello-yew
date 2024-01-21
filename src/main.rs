use yew::prelude::*;

#[function_component]
fn App() -> Html {
  let counter: UseStateHandle<i32> = use_state(|| 0);

  let onclick = {
    let counter: UseStateHandle<i32> = counter.clone();

    move |_| {
      let value: i32 = *counter + 1;
      counter.set(value);
    }
  };

  html! {
    <>
      <header>
        <h1>{"Hello Yew"}</h1>
      </header>

      <main>
        <div>
          <button {onclick}>{ "+1" }</button>
          <p>{ *counter }</p>
        </div>
      </main>
    </>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
