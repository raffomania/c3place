# c3place

## Core concepts

- https://wplace.live but on [c3nav](https://38c3.c3nav.de/)
- Only reachable from inside the C3 network
- MIT, contributions not welcome
- 18+ only
- Authentication via pregenerated and limited UUID tokens.
- One token per account
    - Rate limiting per token. Amount to be decided.
- Maps
    - Multiple maps - One per level
    - Estimated number of pixels:
        - 200 pixels per tile
        - 21x12 tiles
        - 5 levels
        - = ca. 250k pixels
    - store pixels in memory and DB at the same time for each request
    - render PNG tiles from memory in background job periodically
    - Push newly rendered PNG tiles to clients via sockets/or notify via SSE
- UI
    - Login
    - Register with optional name
    - Drag and drop to pan the map
    - Dropdown to select a level
    - Click to place a pixel
- Performance goals
    - ~15k total users
    - Targeting about ~5k requests/second
    - Should not be a problem
    
## Tech

- Axum backend
- egui frontend
- basic CI (formatting, tests, compilen, docs)
- nur tests wenn wir wirklich bock drauf haben
- PRs mit optionalen reviews
- Justfile

## Nice to have concepts

- Grouping accounts e.g. by assembly
- Account avatars
- UI
    - show per-pixel coordinates
    - Scroll to zoom
    - Optionally make art "transparent" to see underlying map
    - Code of conduct/Rules
    - Click a pixel to see who drew it
    - Optionally display room names above painted pixels
- moderation
    - reporting functionality
    - automatically ban users with many reports?
    - people abusing reports will get banned
    - what is not allowed?
        - harrassment, racism, fascism
        - doxxing
    - what is allowed?
        - painting over other's stuff
        - foul language that isn't explicitly disallowed
        - porn
        
## Timeline

- Backend boilerplate rüberziehen ins repo.
- Mit egui experimentieren
   - Image rendering
   - Panning
   - Irgendeine Form von UI
       - Navigation/Sidebar/Tool selection/Settings etc.
       - Login mockup
       - Dropdown für layer selection
   - Pixel setzen auf Canvas
   - Ohne networking
- API data types in separate crate exposen (Re-use von Frontend).
- User authentication, registration, login
- Image representation im Backend
    - Pixel byte vector im backend
    - Backend jobs um Bilder zu generieren
    - Pixel in die datenbank schreiben
- SSE im backend, um neue Bilder zum Frontend zu notifien.
- Egui mit Backend connection
    - API calls
    - SSE
- C3nav ins Backend einbacken
    - c3nav bilder runterladen, an's frontend schicken und anzeigen
    - c3nav text overlay daten laden, an's frontend schicken und anzeigen
- Rate limiting per user