# Hello Yew

`Rust-generated` [WebAssembly](https://webassembly.org/) App with
[Yew](https://yew.rs/).

## The reason behind this project

I just started learning `Rust` and I read on their website that you can [build
WebAssembly with it](https://www.rust-lang.org/what/wasm). I searched on
the internet and I found `Yew`, a framework to do so in a way that reminds me
of React. So i decided to take a look.

## Sample Counter App

The first project that you can do to start learning `Yew` is a [Sample Counter
App](https://yew.rs/docs/getting-started/build-a-sample-app). There you can find
how to set up a small project. However, I want to explain some parts of it:

```rust
// Import all Yew module
use yew::prelude::*;

// Main component that returns an HTML
#[function_component]
fn App() -> Html {
  // Similar to `useState()` React Hook
  let counter: UseStateHandle<i32> = use_state(|| 0);

  // `onclick` callback function
  let onclick = {
    // Creates a copy of the current counter value
    let counter: UseStateHandle<i32> = counter.clone();

    /*
      This section doesn't ends with semicolon (;) because this functions
      returns it.

      `|_|` indicates that this anonymous function has no parameters.
    */
    move |_| {
      let value: i32 = *counter + 1;
      counter.set(value);
    }
  };

  // Like in React, this component returns an HTML.
  // `{}` is being used to insert `Rust` expressions.
  html! {
    <>
      <header>
        // Here is inserted static text as string literals
        <h1>{ "Hello Yew" }</h1>
      </header>

      <main>
        <div>
          // {onclick} is a shorthand of: onclick={onclick}
          // That is done because the callback fn has the same name
          <button {onclick}>{ "+1" }</button>
          <p>{ *counter }</p>
        </div>
      </main>
    </>
  }
}

// Rust App entry point that renders `App()` component
fn main() {
  yew::Renderer::<App>::new().render();
}
```

## Bugs or suggestions

If you found a bug or have a suggestion please don't hesitate to contact me or
open an
[issue on GitHub](https://github.com/pabcrudel/hello-yew/issues).
