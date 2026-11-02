# Laurajay Diaries
A peer to peer shared diary to be able to share your days with your loved ones. Written in rust 
so ofcourse its blazingly fast, because that's how it works. 

# Shared Diary

A decentralized, local-first diary application for sharing entries between two users across mobile and desktop devices.

## Goals

- Cross-platform: iOS, Android, Windows, macOS, and Linux.
- Decentralized: no central server required for data storage or synchronization.
- Local-first: diary entries remain accessible offline.
- Peer-to-peer: devices communicate directly when reachable.
- Modular: storage, communication, synchronization, and application logic are independent.
- Privacy: devices authenticate one another and diary contents are encrypted during communication.

## Technology Stack

| Component | Technology |
|---|---|
| Language | Rust |
| Application framework | Tauri 2 |
| Async runtime | Tokio |
| Local storage | SQLite |
| P2P networking | rust-libp2p (future integration) |
| Initial transport | TCP |
| Local discovery | mDNS |
| Distributed data model | Versioned entries; CRDT integration considered later |

## Architecture

```text
diary/
├── crates/
│   ├── diary-model/
│   ├── diary-storage/
│   ├── diary-communication/
│   ├── diary-sync/
│   └── diary-app/
└── apps/
    ├── desktop/
    └── mobile/
```

### Modules

- **diary-model:** Defines diary entries, device identities, and shared data types.
- **diary-storage:** SQLite persistence and querying.
- **diary-communication:** Device discovery, connection establishment, and message transport.
- **diary-sync:** Synchronization, version tracking, and conflict resolution.
- **diary-app:** Coordinates modules and exposes application functionality.

## Data Models

### DiaryEntry

Represents an individual diary entry.

```rust
pub struct DiaryEntry {
    pub id: EntryId,
    pub date: DateTime<Utc>,
    pub author: String,
    entry: String,
}
```

Methods:

```rust
impl DiaryEntry {
    pub fn new(
        id: EntryId,
        date: DateTime<Utc>,
        author: String,
        contents: String,
    ) -> Self;

    pub fn read(&self) -> &str;

    pub fn edit(&mut self, contents: String);
}
```

Requirements:
- `id` is a stable, unique identifier.
- `date` represents creation time.
- `author` identifies the creator.
- `entry` contains the diary contents.
- `read()` returns a borrowed, read-only view.
- Entry contents are owned by the struct.

### Device

Represents a registered device.

```rust
pub struct Device {
    pub id: DeviceId,
    pub name: String,
    pub public_key: Vec<u8>,
}
```

Device identity is based on a cryptographic key, not an IP or MAC address.

## Storage

SQLite is the primary persistence layer.

```sql
CREATE TABLE diary_entries (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL,
    author TEXT NOT NULL,
    content TEXT NOT NULL
);
```

Requirements:
- Support creating, reading, updating, and deleting entries.
- Support querying by date and author.
- Maintain local persistence independently of network connectivity.
- Serialization is performed at the communication boundary.

A custom `StorageBlock` / `Vec<u8>` storage format is not required initially.

## Communication

The communication module is responsible for discovering and connecting devices.

### Initial implementation

- Discover nearby devices using mDNS on the local network.
- Establish direct TCP connections.
- Exchange serialized diary updates.
- Authenticate peers using registered device identities.
- Encrypt communication.
- Handle connection failures and reconnection.

