use yew::prelude::*;

#[function_component]
fn App() -> Html {
  // Similar to `useState()` React Hook
  let counter: UseStateHandle<i32> = use_state(|| 0);

  // `onclick` callback function
  let onclick = {
    let counter: UseStateHandle<i32> = counter.clone();

    /*
      This section doesn't ends with semicolon because this functions
      returns it
    */
    move |_| {
      let value: i32 = *counter + 1;
      counter.set(value);
    }
  };

  // Like in React, this component returns an HTML
  html! {
    <div>
      <header>
        <h1>{"Hello Yew"}</h1>
      </header>

      <main>
        <div>
          // {onclick} is a shorthand of: onclick={onclick}
          // That is done because the callback fn has the same name
          <button {onclick}>{ "+1" }</button>
          <p>{ *counter }</p>
        </div>
      </main>
    </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
