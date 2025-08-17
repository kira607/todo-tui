# Component

- Component is a base building block of TUI.
- Each component is a standalone structure, that is able to
  draw itself, handle events, change state, etc.
- Components form a tree-like structure, where one component
  can have multiple child components.
- Components communicate with each other with Messages.

## How a component is built

1. Messages enum. The messages produced by a component.
2. Component fields for storing data.
   1. `focused: bool` - special field for focusable components.
3. Child components. For composite components.
4. Core functions. Getters, setters, component data manipulation.
5. Draw function. For drawing (self and child components)
6. - [Event router](#events-routing). Routes events to child widgets and to self.
7. Event hadndlers. Functions that handle events routed to self.
8. Message handlers. Functions that handle messages from child components and from self.
9. Various std traits implementation (Default, From, etc.)

## Naming components

A component should be named in a simple way.

Good names examples:
- `TaskForm`
- `TextField`
- `WheatherInfo`

Bad names examples:
- `TransactionToOutcomeWidget`
- `PersonInformationCompoment`

## Components messages

A component must produce messaged when it's handling turn is finished.

A message is a simple enum, for example:

```rust
MyComponentMsg {
    Submited(u16),
    Canceled,
}
```

**Notice**: the name is `the name of a component` + `Msg`.
It should be true for all components.

## Components fields

A component can have free-form fields for its work.

For example: 

```rust
count: u8,
manager: ContactsManager,
title: String,
buffer: String,
...
```

Focusable components should have a `focused: bool` field.

## Child Components

A component can hold multiple sub components.

The component is not limited to drawing only sub components.
It can also add various other widgets around.

If there are multiple focusable sub components
only one sub component is active.

## Core functions

A component can implement free-form functions
for data and inernal state manupulation
as well as getters and setters for component fields.

This also includes construction stuff, like `::new()`

## Draw function

A component must have a `draw()` function wihich takes care
of drawing a component and its children to a terminal.

The signature of a draw function:

```rust
fn draw(&mut self, frame: &mut Frame, area: Rect);
```

A frame is used to get a `frame.buffer_mut()`
and area designates a terminal screen part where a
component should be drawn.

The draw function can do anything - hide or show child components,
do different things if a component is focused or not (for focusable components).

## Events routing

Events are read from the root component (the `App`).

From there they are either consumed by the app or routed down
to child components of the app.

Each component can also do the same thing - either consume the
event or route it down to the child widgets.

For the sake of flexebility, each component expects the full `Event`
rather than a specific event (e.g. `KeyEvent`).

The result of an event routing is a Message from a component.

The message is an `Option<TMsg>`, so the message can also be `None`.

### Component level events

A part of events are handled at a component level.

Handling an event at a component level can (and usually will) stop
this event from going down to child components, if at this stage
a component dicides to return a Message.

Each event is mapped to an event handler (see [Event handlers](#event-handlers)).

### Child components events routing

If an event passes through component level handling
it goes in child components events routing.

Here, an event is passed down to a currently focused sub component
and a sub component message is returned.

Boiler-plate for event router when listening to child component messages:

```rust
self.sub_component.handle_event(event).map(ComponentMsg::SubComponent)
```

## Event handlers

An impl block consisting of functions that handle events (like key presses).

A signature of an event handler looks like this:

```rust
fn do_something(&mut self) -> Option<TMsg> {
    ...
}
```

No parameters, return type is Option of a Component message.

## Messages handlers

**This section is under development**
**Anything written here is subject to change**

An impl block consisting of functions that handle messages.

This block is relevant only for the root component since
all messaged are bubbled up to the App.

## Std traits

For making components "rusty" they can implement various
std traits, like Default or From.

## Ideas

- macros that generates a specific component trait.