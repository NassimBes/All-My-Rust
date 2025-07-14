        ┌────────────┐
        │  main.rs   │
        └────┬───────┘
             │ initializes
             ▼
        ┌────────────┐
        │   App      │
        │────────────│
        │ screen: ▒▒▒ │────┐    screen enum tracks current UI screen
        │ menu_state  │    │
        │ chat_state  │    │
        │ should_exit │    │
        └────┬────────┘    │
             │             │
             │ match screen in draw() / handle_input()
             ▼             ▼
 ┌──────────────────┐   ┌────────────────────┐
 │    MenuState     │   │    ChatState       │
 │ (data + cursor)  │   │ (selected room etc)│
 └────┬─────────────┘   └─────┬──────────────┘
      │                      │
      │ render(...)          │ render(...)
      ▼                      ▼
┌────────────┐          ┌────────────┐
│ menu.rs    │          │ chat.rs    │
│ - render() │          │ - render() │
│ - cursor   │          │ - scroll   │
└────────────┘          └────────────┘
