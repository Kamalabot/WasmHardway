Sure, here are five examples of props in Yew that can populate and control the front end:

### **1. Text Prop**

- **Description**: Passing text data to display on the frontend.
- **Usage**: Simple text display, like headings, paragraphs, or labels.

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct TextProps {
    pub message: String,
}

#[function_component(DisplayText)]
fn display_text(props: &TextProps) -> Html {
    html! { <p>{ &props.message }</p> }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <DisplayText message="Hello, Yew!".to_string() />
    }
}
```

### **2. Color Prop**

- **Description**: Passing a color value to style elements dynamically.
- **Usage**: Changes the appearance of elements based on user selection or dynamic data.

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ColorProps {
    pub color: String,
}

#[function_component(ColoredBox)]
fn colored_box(props: &ColorProps) -> Html {
    html! {
        <div style={format!("background-color: {};", props.color)}>{ "Colored Box" }</div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <ColoredBox color="lightblue".to_string() />
    }
}
```

### **3. Image URL Prop**

- **Description**: Passes a URL for an image to be displayed on the frontend.
- **Usage**: Dynamically shows images based on user data or content fetched from APIs.

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ImageProps {
    pub url: String,
}

#[function_component(DisplayImage)]
fn display_image(props: &ImageProps) -> Html {
    html! {
        <img src={&props.url} alt="Dynamic Image" />
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <DisplayImage url="https://via.placeholder.com/150".to_string() />
    }
}
```

### **4. List Items Prop**

- **Description**: Passing a list of items to generate dynamic lists on the frontend.
- **Usage**: Useful for displaying data-driven content like product lists, menus, or posts.

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ListProps {
    pub items: Vec<String>,
}

#[function_component(DisplayList)]
fn display_list(props: &ListProps) -> Html {
    html! {
        <ul>
            { for props.items.iter().map(|item| 
                html! { <li>{ item }</li> }) }
        </ul>
    }
}

#[function_component(App)]
fn app() -> Html {
    let items = vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string()];
    html! {
        <DisplayList {items} />
    }
}
```

### **5. Boolean Prop**

- **Description**: Passes a boolean value to toggle the display or styling of elements.
- **Usage**: Controls visibility, styles, or functionality like button states.

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ToggleProps {
    pub is_visible: bool,
}

#[function_component(ToggleDisplay)]
fn toggle_display(props: &ToggleProps) -> Html {
    if props.is_visible {
        html! { <p>{ "Visible Content" }</p> }
    } else {
        html! { <p>{ "Content Hidden" }</p> }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <ToggleDisplay is_visible={true} />
    }
}
```

### **Summary**

1. **Text Prop**: Displays dynamic text content.
2. **Color Prop**: Alters styling based on props.
3. **Image URL Prop**: Renders images dynamically.
4. **List Items Prop**: Populates lists on the frontend.
5. **Boolean Prop**: Controls visibility or state of elements.

These props allow flexible and dynamic data handling on the frontend, making Yew applications interactive and responsive to data changes.

### **Understanding Callbacks in Yew**

In Yew, a **callback** is a mechanism that allows data and events to flow from child components back to parent components, enabling communication between different parts of your application. It’s useful when you need to trigger actions or update state based on user interactions within child components.

### **Significance of Callbacks:**

1. **Event Handling**: Callbacks handle user interactions such as button clicks, form submissions, or any other UI event that should trigger logic in the parent component.

2. **Data Flow**: Enables data to flow upwards from child to parent, contrary to the usual one-way data flow where data flows from parent to child through props.

3. **Component Communication**: Facilitates communication between components, allowing child components to update the parent’s state or trigger functions.

### **Example: Displaying Data Flow to the Frontend**

Let's create an example where a child component sends data to the parent component using a callback, which then updates the frontend.

```rust
use yew::prelude::*;

// Define properties for the Child component, including a callback
#[derive(Properties, PartialEq)]
struct ChildProps {
    pub on_data_send: Callback<String>,
}

// Child component that triggers the callback
#[function_component(Child)]
fn child(props: &ChildProps) -> Html {
    // This button triggers the callback with some data when clicked
    let on_button_click = {
        let on_data_send = props.on_data_send.clone();
        move |_| {
            // Emit the data to the parent when the button is clicked
            on_data_send.emit("Hello from Child".to_string());
        }
    };

    html! {
        <button onclick={on_button_click}>{ "Send Data to Parent" }</button>
    }
}

// Parent component that receives data from the Child and displays it
#[function_component(Parent)]
fn parent() -> Html {
    // Callback to handle data sent from the Child component
    let on_data_send = Callback::from(|data: String| {
        // Print the data received from the child to the console
        web_sys::console::log_1(&data.into());
        // In a real-world scenario, you might update state here instead
    });

    html! {
        <div>
            // Render the Child component, passing the callback as a prop
            <Child on_data_send={on_data_send} />
            <p>{ "Parent Component" }</p>
        </div>
    }
}

// Main application component
#[function_component(App)]
fn app() -> Html {
    html! {
        <Parent />
    }
}

```

### **Explanation:**

1. **Child Component (`Child`)**:
   
   - Takes a callback (`on_data_send`) as a prop.
   - When the button is clicked, it triggers the callback and sends the string `"Hello from Child"` back to the parent.

2. **Parent Component (`Parent`)**:
   
   - Defines a callback (`on_data_send`) that receives data from the child.
   - Logs the received data to the console.

3. **Data Flow**:
   
   - The data ("Hello from Child") is emitted by the child component and handled by the parent's callback function.



### **Use Case Example:**

This pattern is commonly used in forms, interactive lists, or when child components need to communicate their state or events back to the main application logic.

If you want the callback to emit when the child component is rendered without any user interaction, you can trigger the callback directly within the child component's rendering logic. Below is a modified version of the previous example where the callback is invoked immediately when the child component renders:

### **Callback Emit on Render Example**

```rust
use yew::prelude::*;

// Define properties for the Child component, including a callback
#[derive(Properties, PartialEq)]
struct ChildProps {
    pub on_data_send: Callback<String>,
}

// Child component that triggers the callback immediately on render
#[function_component(Child)]
fn child(props: &ChildProps) -> Html {
    // Emit the callback when the component is rendered
    props.on_data_send.emit("Hello from Child on Render".to_string());

    html! {
        <p>{ "Child Component Rendered" }</p>
    }
}

// Parent component that receives data from the Child and displays it
#[function_component(Parent)]
fn parent() -> Html {
    // Callback to handle data sent from the Child component
    let on_data_send = Callback::from(|data: String| {
        // Print the data received from the child to the console
        web_sys::console::log_1(&data.into());
    });

    html! {
        <div>
            // Render the Child component, passing the callback as a prop
            <Child on_data_send={on_data_send} />
            <p>{ "Parent Component" }</p>
        </div>
    }
}

// Main application component
#[function_component(App)]
fn app() -> Html {
    html! {
        <Parent />
    }
}
```

### **Explanation:**

1. **Child Component (`Child`)**:
   
   - The `emit` method on the `on_data_send` callback is called immediately when the child component is rendered.
   - This triggers the callback without waiting for any user interaction.

2. **Parent Component (`Parent`)**:
   
   - Receives the data sent from the child component during its initial render and logs it to the console.

This approach ensures that the callback is executed as soon as the child component is rendered, sending data automatically to the parent.

### Working with States & Hooks

To pass the `greet` string from the `App` component to the frontend via the `HelloWorld` component and also log it to 

inside `App` whenever the callback is triggered. Here's a complete approach to achieve this.

In Yew, states (`use_state`) are crucial for managing component data that can change over time, allowing components to re-render when the data they depend on changes. States are particularly useful when handling user interactions, asynchronous data fetching, or managing dynamic content.

### **Common Use Cases of States in Yew**

1. **User Input Handling:**
   
   - Managing form input values like text fields, checkboxes, or dropdowns.

2. **Asynchronous Data Fetching:**
   
   - Displaying data fetched from APIs and handling loading or error states.

3. **UI Updates on User Actions:**
   
   - Handling button clicks, toggling elements, or conditionally rendering content.

4. **Communication Between Components:**
   
   - Passing state-changing callbacks to child components for communication between parent and child.

### **Code Example: Basic State Management in Yew**

Let's look at a simple example where we handle user input and a button click to update and display the state, showing how state is used effectively.

```rust
use yew::prelude::*;

// Define the main App component
#[function_component(App)]
fn app() -> Html {
    // Define state to hold the user's input
    let input_value = use_state(|| String::new()); // Holds the current text input
    let display_value = use_state(|| String::new()); // Holds the displayed text on the screen

    // Callback to update the input_value state 
    // when the input changes
    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |event: InputEvent| {
            let input = 
event.target_unchecked_into::<web_sys::HtmlInputElement>();\/
            input_value.set(input.value());
        })
    };

    // Callback to update the display_value when
    // the button is clicked
    let on_click = {
        let input_value = input_value.clone();
        let display_value = display_value.clone();
        Callback::from(move |_| {
            let message = format!("Hello, {}!", 
*input_value); // Format a greeting message
            web_sys::console::log_1(&message.clone().into()); // Log to the console
            display_value.set(message); // Update the display_value state
        })
    };

    // Render the UI elements
    html! {
        <div>
            <input type="text" 
placeholder="Enter your name" 
oninput={on_input} /> // Input box
            <button onclick={on_click}>{ "Greet Me" }
            </button> // Button
            <p>{ &*display_value }</p> // Display the greeting message
        </div>
    }
}
```

### **Significance and Usage Explanation**

1. **`use_state`:**
   
   - `use_state` is used to create reactive state variables. Here, `input_value` holds the current value of the text input, and `display_value` stores the greeting message displayed on the screen.

2. **Updating State on User Input:**
   
   - `on_input` is a callback triggered when the user types in the input box. It updates `input_value` with the current text.

3. **Button Click Handling:**
   
   - `on_click` is triggered when the button is clicked. It formats a greeting message, logs it to the console, and updates the `display_value` state to show the greeting on the frontend.

4. **Dynamic UI Updates:**
   
   - When `display_value` is updated, the component automatically re-renders, showing the new message in the `<p>` tag.

### **Real-World Use Case**

This pattern is commonly used in interactive web applications where data needs to be captured, processed, and dynamically displayed, such as:

- **Form Submissions**: Capturing user input and updating the display based on validation or user actions.
- **Search Filters**: Updating results based on filter selections.
- **Chat Applications**: Displaying messages in real-time as they are sent or received.

Using state in this manner makes components responsive and interactive, providing users with immediate feedback based on their actions.

When you're working in Rust, especially in asynchronous or stateful environments (like when building UIs or handling web apps with frameworks like Yew or Seed), `use_state` plays a critical role in managing state, while `String::new()` doesn't provide the same functionality.

Here's why `use_state` is important and differs from `String::new()`:

### `String::new()`

- **What it does**: It simply creates a new empty `String`. The `String` can grow or change, but it's just a basic, mutable value.
- **Scope**: The `String` you create exists in the scope where it's initialized. Once it's dropped (when the scope ends), the data is gone.
- **Immutability**: Even if you create a mutable `String` (`let mut s = String::new()`), the mutability only lasts within the scope.

### `use_state`

- **What it does**: `use_state` is commonly used in environments like asynchronous apps (Tokio, async web servers, or UIs) to maintain state across renders or tasks. It creates state that can persist between function calls, task switches, or UI re-renders.
- **Persistence**: The state managed by `use_state` is stored in a way that it persists beyond a single scope (for example, across renders in a UI framework).
- **Shared Ownership**: `use_state` provides a way to safely share state between tasks or across components, often wrapping values like `Rc<RefCell<T>>` (in single-threaded contexts) or `Arc<Mutex<T>>` (in multi-threaded environments). This ensures that multiple parts of the program can access or modify the state without conflicts, enabling safe sharing of addresses.

### Impact of `use_state` on Address Sharing

- **Concurrency and Safety**: When multiple tasks or components need to access or modify the state concurrently, `use_state` ensures that this is done safely by managing the ownership and borrowing rules. It handles scenarios where multiple references to the state might exist.
- **Controlled State Updates**: `use_state` wraps the state in a way that only allows updates in specific controlled manners (usually through closures that update the value).
- **Avoiding Borrow Checker Issues**: `use_state` works around Rust's strict ownership and borrowing system by providing a way to mutate shared state without running into lifetime or borrowing issues. If you just used `String::new()` and tried to share it between tasks or components, you'd run into issues with Rust's ownership model.

In short, `String::new()` is for simple, local strings, while `use_state` is designed to manage state in complex, shared, or async contexts, ensuring that the state persists and is safely accessible across different parts of a program.



In Yew, besides `use_state`, other state hooks provide different functionalities for managing component states, allowing more advanced state management patterns. Below are some of the commonly used state hooks in Yew:

### **1. `use_reducer`**

- **Significance**: Provides a way to manage complex state logic by using a reducer function similar to Redux or React's `useReducer`. It is useful when the state management logic involves multiple actions or transitions.

- **Usage**: Defines a reducer function that updates the state based on actions, allowing organized state changes.

- **Example**:
  
  ```rust
  use yew::prelude::*;
  use std::rc::Rc;
  
  #[derive(Clone, PartialEq)]
  struct CounterState {
      count: i32,
  }
  
  enum CounterAction {
      Increment,
      Decrement,
  }
  
  fn reducer(state: Rc<CounterState>, action: CounterAction) -> Rc<CounterState> {
      match action {
          CounterAction::Increment => Rc::new(CounterState { count: state.count + 1 }),
          CounterAction::Decrement => Rc::new(CounterState { count: state.count - 1 }),
      }
  }
  
  #[function_component(Counter)]
  fn counter() -> Html {
      let state = use_reducer(|| 
      Rc::new(CounterState { count: 0 }), reducer);
  
      html! {
          <div>
              <button onclick={state.dispatch(CounterAction
              ::Decrement)}>{ "Decrement" }</button>
              <p>{ state.count }</p>
              <button onclick={state.dispatch(CounterAction::Increment)}>{ "Increment" }</button>
          </div>
      }
  }
  ```

### **2. `use_ref`**

- **Significance**: Provides a way to store mutable references to data that do not trigger re-renders when modified. This is similar to React’s `useRef`, useful for accessing and modifying DOM elements or other data without affecting the component’s render cycle.

- **Usage**: Often used to directly manipulate or store references to values that are not part of the render state.

- **Example**:
  
  ```rust
  use yew::prelude::*;
  
  #[function_component(RefExample)]
  fn ref_example() -> Html {
      let input_ref = use_ref(|| NodeRef::default()); // Reference to the input element
  
      let focus_input = {
          let input_ref = input_ref.clone();
          Callback::from(move |_| {
              if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                  input.focus().unwrap(); // Focuses the input when button is clicked
              }
          })
      };
  
      html! {
          <div>
              <input ref={input_ref} type="text" placeholder="Click button to focus" />
              <button onclick={focus_input}>{ "Focus Input" }</button>
          </div>
      }
  }
  ```

### **3. `use_context`**

- **Significance**: Allows sharing state or data between components without passing props explicitly through every component level. Useful for managing global or shared state.

- **Usage**: Used to provide and consume context data across component trees.

- **Example**:
  
  ```rust
  use yew::prelude::*;
  
  // Define a context to hold a shared value
  #[derive(Clone, PartialEq, Debug)]
  struct Theme(String);
  
  #[function_component(Parent)]
  fn parent() -> Html {
      let theme = use_state(|| Theme("Light".to_string()));
  
      html! {
          <ContextProvider<Theme> context={(*theme).clone()}>
              <Child />
          </ContextProvider<Theme>>
      }
  }
  
  #[function_component(Child)]
  fn child() -> Html {
      let theme = use_context::<Theme>().expect("No Theme context found"); // Consume the context value
  
      html! {
          <div>{ format!("Current theme: {:?}", theme) }</div>
      }
  }
  ```

### **4. `use_effect`**

- **Significance**: Manages side effects in functional components, such as fetching data, setting up subscriptions, or directly interacting with the DOM. Runs when dependencies change, similar to React's `useEffect`.

- **Usage**: Commonly used for data fetching, setting up timers, or subscribing to data changes.

- **Example**:
  
  ```rust
  use yew::prelude::*;
  use web_sys::console;
  
  #[function_component(EffectExample)]
  fn effect_example() -> Html {
      let counter = use_state(|| 0);
  
      // Effect that runs on counter change
      {
          let counter = counter.clone();
          use_effect(move || {
              console::log_1(&format!("Counter changed: {}", counter).into());
              || {} // No cleanup needed here
          });
      }
  
      html! {
          <div>
              <p>{ format!("Counter: {}", *counter) }</p>
              <button onclick={Callback::from(move |_| counter.set(*counter + 1))}>{ "Increment" }</button>
          </div>
      }
  }
  ```

### **Summary of State Hooks**

- **`use_state`**: Simple state management for reactive components.
- **`use_reducer`**: Complex state management with actions and reducers, useful for structured state updates.
- **`use_ref`**: Mutable reference for data without causing re-renders, ideal for DOM manipulation or storing values directly.
- **`use_context`**: Shared state management across component hierarchies, avoiding prop drilling.
- **`use_effect`**: Handles side effects, like data fetching and subscriptions, responding to dependency changes.

These state hooks offer a flexible and powerful way to manage data and effects in Yew, allowing for highly interactive and responsive web applications.
