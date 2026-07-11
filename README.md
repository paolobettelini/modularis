# Minecraft simple demo

Demo Minecraft minimale client/server composta tramite `modding_system`.
Client e server sono due applicazioni Bevy distinte, costruite dallo stesso
insieme di mod condivise e specializzate dai modpack:

- `modpacks/client.toml`: finestra, menu, settings, input, rendering e rete
  client;
- `modpacks/server.toml`: runner Bevy headless, mondo autoritativo, sessioni e
  rete server.

Il trasporto attuale usa UDP non bloccante e messaggi CBOR. Il protocollo e i
relativi messaggi Bevy tipizzati sono generati dal Composer.

## Funzionalità

- main menu e schermata Settings;
- nome player e indirizzo server configurabili, con default `Player` e
  `127.0.0.1:9999`;
- FOV e tasto di salto configurabili;
- menu pausa in-game trasparente con Resume e Settings;
- server locale headless in ascolto su `0.0.0.0:9999`;
- join/leave, assegnazione di un `PlayerId` e replica dei movimenti;
- avatar sferici con il nome sopra;
- richiesta dei chunk dal client e risposta autoritativa del server;
- rottura blocchi con click sinistro e piazzamento stone con click destro;
- replica autoritativa delle modifiche dei blocchi a tutti i client;
- cache chunk, streaming, collisioni e rendering esclusivamente client-side;
- mondo server interrogabile e modificabile, con terreno dirt/stone a
  scacchiera e chunk residenti entro 8 chunk dallo spawn;
- chunk compressi in RAM e sul network tramite palette e bit-packing;
- blocchi e settings registrati tramite codegen;
- texture possedute dalle mod e copiate in `assets/<nome-mod>/`.

## Composizione

Dalla root `videogames/`:

```bash
cargo run --manifest-path modding_system/composer-cli/Cargo.toml -- \
  compose \
  --modpack server \
  --modpacks-folder minecraft_simple_demo/modpacks \
  --mods-folder minecraft_simple_demo/mods \
  --cache minecraft_simple_demo/build-server

cargo run --manifest-path modding_system/composer-cli/Cargo.toml -- \
  compose \
  --modpack client \
  --modpacks-folder minecraft_simple_demo/modpacks \
  --mods-folder minecraft_simple_demo/mods \
  --cache minecraft_simple_demo/build-client
```

## Avvio

Prima avviare il server:

```bash
cargo run --manifest-path minecraft_simple_demo/build-server/server/Cargo.toml
```

Poi, in un secondo terminale, il client:

```bash
cargo run --manifest-path minecraft_simple_demo/build-client/client/Cargo.toml
```

Per provare più player si possono avviare più client. Controlli predefiniti:
`WASD`, mouse, `Space`, click sinistro per rompere e click destro per usare
l'item selezionato; `Esc` apre e chiude il menu pausa.

Le directory `build-client/`, `build-server/` e le crate
`mods/generated-*` sono output rigenerabili: non modificarle a mano.

Il trasporto attivo è TCP con framing length-prefixed. Trasporto, protocollo,
routing e gameplay restano mod separate, quindi possono essere sostituiti senza
accoppiare la logica di gioco ai socket.
